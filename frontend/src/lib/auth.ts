import { User } from 'shared/types'

const TOKEN_KEY = 'auth_token'
const USER_KEY = 'auth_user'

export const authStorage = {
  getToken: (): string | null => {
    return localStorage.getItem(TOKEN_KEY)
  },

  setToken: (token: string): void => {
    localStorage.setItem(TOKEN_KEY, token)
  },

  removeToken: (): void => {
    localStorage.removeItem(TOKEN_KEY)
  },

  getUser: (): User | null => {
    const user = localStorage.getItem(USER_KEY)
    return user ? JSON.parse(user) : null
  },

  setUser: (user: User): void => {
    localStorage.setItem(USER_KEY, JSON.stringify(user))
  },

  removeUser: (): void => {
    localStorage.removeItem(USER_KEY)
  },

  clear: (): void => {
    localStorage.removeItem(TOKEN_KEY)
    localStorage.removeItem(USER_KEY)
  }
}

export const getAuthHeaders = (): Record<string, string> => {
  const token = authStorage.getToken()
  return token ? { Authorization: `Bearer ${token}` } : {}
}

export const makeAuthenticatedRequest = async (url: string, options: RequestInit = {}) => {
  const headers = {
    'Content-Type': 'application/json',
    ...getAuthHeaders(),
    ...(options.headers || {})
  }

  return fetch(url, {
    ...options,
    headers
  })
}

export const isAuthenticated = (): boolean => {
  return !!authStorage.getToken()
}
