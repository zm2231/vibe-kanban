// Shared types for the frontend application
export interface User {
  id: string
  email: string
  is_admin: boolean
  created_at?: Date
  updated_at?: Date
}

export interface ApiResponse<T> {
  success: boolean
  data: T | null
  message: string | null
}

export interface LoginRequest {
  email: string
  password: string
}

export interface LoginResponse {
  user: User
  token: string
}

export interface Project {
  id: string
  name: string
  owner_id: string
  created_at: Date
  updated_at: Date
}

export interface CreateProject {
  name: string
}

export interface UpdateProject {
  name: string | null
}

export interface Task {
  id: string
  project_id: string
  title: string
  description: string | null
  status: TaskStatus
  created_at: string
  updated_at: string
}

export type TaskStatus = "todo" | "inprogress" | "inreview" | "done" | "cancelled"

export interface CreateTask {
  project_id: string
  title: string
  description: string | null
}

export interface UpdateTask {
  title: string | null
  description: string | null
  status: TaskStatus | null
}

export interface CreateUser {
  email: string
  password: string
  is_admin: boolean | null
}

export interface UpdateUser {
  email: string | null
  password: string | null
  is_admin: boolean | null
}
