import { createContext, useContext, useState, useEffect, ReactNode } from 'react'
import { isAuthenticated, authStorage, makeAuthenticatedRequest } from '@/lib/auth'
import { User } from '@/types'

interface AuthContextType {
  user: User | null
  isAuthenticated: boolean
  isLoading: boolean
  login: (user: User, token: string) => void
  logout: () => void
  refreshAuthStatus: () => Promise<void>
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

interface AuthProviderProps {
  children: ReactNode
}

export function AuthProvider({ children }: AuthProviderProps) {
  const [user, setUser] = useState<User | null>(null)
  const [isAuthenticatedState, setIsAuthenticated] = useState(false)
  const [isLoading, setIsLoading] = useState(true)

  const checkAuthStatus = async (): Promise<boolean> => {
    const token = authStorage.getToken()
    if (!token) {
      return false
    }

    try {
      const response = await makeAuthenticatedRequest('/api/auth/status')
      
      if (response.ok) {
        const data = await response.json()
        if (data.success && data.data?.authenticated) {
          // Update user data from server response
          if (data.data.user_id && data.data.email) {
            const userData: User = {
              id: data.data.user_id,
              email: data.data.email,
              is_admin: data.data.is_admin || false
            }
            authStorage.setUser(userData)
            setUser(userData)
          }
          return true
        }
      }
      
      // If we get here, the token is invalid
      return false
    } catch (error) {
      console.error('Auth status check failed:', error)
      return false
    }
  }

  const refreshAuthStatus = async () => {
    setIsLoading(true)
    
    if (isAuthenticated()) {
      const isValid = await checkAuthStatus()
      if (isValid) {
        setIsAuthenticated(true)
      } else {
        // Clear invalid auth state
        authStorage.clear()
        setUser(null)
        setIsAuthenticated(false)
      }
    } else {
      setUser(null)
      setIsAuthenticated(false)
    }
    
    setIsLoading(false)
  }

  useEffect(() => {
    refreshAuthStatus()
  }, [])

  const login = (userData: User, token: string) => {
    authStorage.setToken(token)
    authStorage.setUser(userData)
    setUser(userData)
    setIsAuthenticated(true)
  }

  const logout = () => {
    authStorage.clear()
    setUser(null)
    setIsAuthenticated(false)
    window.location.href = '/'
  }

  const value: AuthContextType = {
    user,
    isAuthenticated: isAuthenticatedState,
    isLoading,
    login,
    logout,
    refreshAuthStatus
  }

  return (
    <AuthContext.Provider value={value}>
      {children}
    </AuthContext.Provider>
  )
}

export function useAuth() {
  const context = useContext(AuthContext)
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider')
  }
  return context
}
