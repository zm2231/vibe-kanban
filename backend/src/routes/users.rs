use axum::{
    routing::{get, post},
    Router,
    Json,
    response::Json as ResponseJson,
    extract::{Path, Extension},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::models::{ApiResponse, user::{User, CreateUser, UpdateUser, LoginRequest, LoginResponse, UserResponse}};
use crate::auth::{AuthUser, create_token, hash_password, verify_password};

pub async fn login(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<LoginRequest>
) -> Result<ResponseJson<ApiResponse<LoginResponse>>, StatusCode> {
    match sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, is_admin, created_at, updated_at FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(user)) => {
            match verify_password(&payload.password, &user.password_hash) {
                Ok(true) => {
                    match create_token(user.id, user.email.clone(), user.is_admin) {
                        Ok(token) => {
                            Ok(ResponseJson(ApiResponse {
                                success: true,
                                data: Some(LoginResponse {
                                    user: user.into(),
                                    token,
                                }),
                                message: Some("Login successful".to_string()),
                            }))
                        }
                        Err(e) => {
                            tracing::error!("Failed to create token: {}", e);
                            Err(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                }
                Ok(false) => Err(StatusCode::UNAUTHORIZED),
                Err(e) => {
                    tracing::error!("Password verification error: {}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Ok(None) => Err(StatusCode::UNAUTHORIZED),
        Err(e) => {
            tracing::error!("Failed to fetch user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_users(
    _auth: AuthUser,
    Extension(pool): Extension<PgPool>
) -> Result<ResponseJson<ApiResponse<Vec<UserResponse>>>, StatusCode> {
    match sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, is_admin, created_at, updated_at FROM users ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await
    {
        Ok(users) => {
            let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();
            Ok(ResponseJson(ApiResponse {
                success: true,
                data: Some(user_responses),
                message: None,
            }))
        }
        Err(e) => {
            tracing::error!("Failed to fetch users: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_user(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>
) -> Result<ResponseJson<ApiResponse<UserResponse>>, StatusCode> {
    // Users can only view their own profile unless they're admin
    if auth.user_id != id && !auth.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    match sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, is_admin, created_at, updated_at FROM users WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(user)) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(user.into()),
            message: None,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to fetch user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_user(
    auth: AuthUser,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUser>
) -> Result<ResponseJson<ApiResponse<UserResponse>>, StatusCode> {
    // Only admins can create users
    if !auth.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let id = Uuid::new_v4();
    let now = Utc::now();
    let is_admin = payload.is_admin.unwrap_or(false);

    let password_hash = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match sqlx::query_as!(
        User,
        "INSERT INTO users (id, email, password_hash, is_admin, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, email, password_hash, is_admin, created_at, updated_at",
        id,
        payload.email,
        password_hash,
        is_admin,
        now,
        now
    )
    .fetch_one(&pool)
    .await
    {
        Ok(user) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(user.into()),
            message: Some("User created successfully".to_string()),
        })),
        Err(e) => {
            tracing::error!("Failed to create user: {}", e);
            if e.to_string().contains("users_email_key") {
                Err(StatusCode::CONFLICT) // Email already exists
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

pub async fn update_user(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<UpdateUser>
) -> Result<ResponseJson<ApiResponse<UserResponse>>, StatusCode> {
    // Users can only update their own profile unless they're admin
    if auth.user_id != id && !auth.is_admin {
        return Err(StatusCode::FORBIDDEN);
    }

    let now = Utc::now();

    // Get existing user
    let existing_user = match sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, is_admin, created_at, updated_at FROM users WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to check user existence: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let email = payload.email.unwrap_or(existing_user.email);
    let is_admin = if auth.is_admin {
        payload.is_admin.unwrap_or(existing_user.is_admin)
    } else {
        existing_user.is_admin // Non-admins can't change admin status
    };

    let password_hash = if let Some(new_password) = payload.password {
        match hash_password(&new_password) {
            Ok(hash) => hash,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        existing_user.password_hash
    };

    match sqlx::query_as!(
        User,
        "UPDATE users SET email = $2, password_hash = $3, is_admin = $4, updated_at = $5 WHERE id = $1 RETURNING id, email, password_hash, is_admin, created_at, updated_at",
        id,
        email,
        password_hash,
        is_admin,
        now
    )
    .fetch_one(&pool)
    .await
    {
        Ok(user) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(user.into()),
            message: Some("User updated successfully".to_string()),
        })),
        Err(e) => {
            tracing::error!("Failed to update user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_user(
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Extension(pool): Extension<PgPool>
) -> Result<ResponseJson<ApiResponse<()>>, StatusCode> {
    // Only admins can delete users, and they can't delete themselves
    if !auth.is_admin || auth.user_id == id {
        return Err(StatusCode::FORBIDDEN);
    }

    match sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() == 0 {
                Err(StatusCode::NOT_FOUND)
            } else {
                Ok(ResponseJson(ApiResponse {
                    success: true,
                    data: None,
                    message: Some("User deleted successfully".to_string()),
                }))
            }
        }
        Err(e) => {
            tracing::error!("Failed to delete user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_current_user(
    auth: AuthUser,
    Extension(pool): Extension<PgPool>
) -> Result<ResponseJson<ApiResponse<UserResponse>>, StatusCode> {
    match sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, is_admin, created_at, updated_at FROM users WHERE id = $1",
        auth.user_id
    )
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(user)) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(user.into()),
            message: None,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to fetch current user: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn users_router() -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/me", get(get_current_user))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
}
