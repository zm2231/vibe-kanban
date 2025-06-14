import { useState, useEffect } from 'react'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Alert, AlertDescription } from '@/components/ui/alert'
import { ProjectsPage } from '@/components/projects/projects-page'
import { UsersPage } from '@/components/users/users-page'
import { LoginForm } from '@/components/auth/login-form'
import { ApiResponse } from 'shared/types'
import { authStorage, isAuthenticated, logout, makeAuthenticatedRequest } from '@/lib/auth'
import { ArrowLeft, Heart, Activity, FolderOpen, Users, CheckCircle, AlertCircle, LogOut } from 'lucide-react'

function App() {
  const [currentPage, setCurrentPage] = useState<'home' | 'projects' | 'users'>('home')
  const [message, setMessage] = useState<string>('')
  const [messageType, setMessageType] = useState<'success' | 'error'>('success')
  const [loading, setLoading] = useState(false)
  const [authenticated, setAuthenticated] = useState(false)
  
  const currentUser = authStorage.getUser()

  useEffect(() => {
    setAuthenticated(isAuthenticated())
  }, [])

  const handleLogin = () => {
    setAuthenticated(true)
    setCurrentPage('home')
  }

  const handleLogout = () => {
    logout()
    setAuthenticated(false)
    setCurrentPage('home')
  }

  const fetchHello = async () => {
    setLoading(true)
    try {
      const response = await makeAuthenticatedRequest('/api/hello?name=Bloop')
      const data = await response.json()
      setMessage(data.message)
      setMessageType('success')
    } catch (error) {
      setMessage('Error connecting to backend')
      setMessageType('error')
    } finally {
      setLoading(false)
    }
  }

  const checkHealth = async () => {
    setLoading(true)
    try {
      const response = await makeAuthenticatedRequest('/api/health')
      const data: ApiResponse<string> = await response.json()
      setMessage(data.message || 'Health check completed')
      setMessageType('success')
    } catch (error) {
      setMessage('Backend health check failed')
      setMessageType('error')
    } finally {
      setLoading(false)
    }
  }

  if (!authenticated) {
    return <LoginForm onSuccess={handleLogin} />
  }

  if (currentPage === 'projects' || currentPage === 'users') {
    return (
      <div className="min-h-screen bg-background">
        <div className="border-b">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="flex items-center justify-between h-16">
              <div className="flex items-center space-x-6">
                <h2 className="text-lg font-semibold">Bloop</h2>
                <div className="flex items-center space-x-1">
                  <Button
                    variant={currentPage === 'projects' ? 'default' : 'ghost'}
                    size="sm"
                    onClick={() => setCurrentPage('projects')}
                  >
                    <FolderOpen className="mr-2 h-4 w-4" />
                    Projects
                  </Button>
                  {currentUser?.is_admin && (
                    <Button
                      variant={currentPage === 'users' ? 'default' : 'ghost'}
                      size="sm"
                      onClick={() => setCurrentPage('users')}
                    >
                      <Users className="mr-2 h-4 w-4" />
                      Users
                    </Button>
                  )}
                </div>
              </div>
              <div className="flex items-center space-x-4">
                <div className="text-sm text-muted-foreground">
                  Welcome, {currentUser?.email}
                </div>
                <Button variant="ghost" onClick={() => setCurrentPage('home')}>
                  <ArrowLeft className="mr-2 h-4 w-4" />
                  Home
                </Button>
                <Button variant="ghost" onClick={handleLogout}>
                  <LogOut className="mr-2 h-4 w-4" />
                  Logout
                </Button>
              </div>
            </div>
          </div>
        </div>
        <div className="max-w-7xl mx-auto p-6 sm:p-8">
          {currentPage === 'projects' ? <ProjectsPage /> : <UsersPage />}
        </div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-background to-muted/20">
      <div className="container mx-auto px-4 py-12">
        <div className="max-w-4xl mx-auto">
          <div className="text-center mb-12">
            <div className="flex items-center justify-center mb-6">
              <div className="rounded-full bg-primary/10 p-4">
                <Heart className="h-8 w-8 text-primary" />
              </div>
            </div>
            <h1 className="text-4xl font-bold tracking-tight mb-4">
              Welcome to Bloop
            </h1>
            <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
              A modern full-stack monorepo built with Rust backend and React frontend.
              Get started by exploring our features below.
            </p>
          </div>

          <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3 mb-8">
            <Card className="hover:shadow-md transition-shadow">
              <CardHeader>
                <div className="flex items-center">
                  <div className="rounded-lg bg-blue-100 p-2 mr-3">
                    <Heart className="h-5 w-5 text-blue-600" />
                  </div>
                  <CardTitle className="text-lg">API Test</CardTitle>
                </div>
                <CardDescription>
                  Test the connection between frontend and backend
                </CardDescription>
              </CardHeader>
              <CardContent>
                <Button 
                  onClick={fetchHello} 
                  disabled={loading}
                  className="w-full"
                  size="sm"
                >
                  <Heart className="mr-2 h-4 w-4" />
                  Say Hello
                </Button>
              </CardContent>
            </Card>

            <Card className="hover:shadow-md transition-shadow">
              <CardHeader>
                <div className="flex items-center">
                  <div className="rounded-lg bg-green-100 p-2 mr-3">
                    <Activity className="h-5 w-5 text-green-600" />
                  </div>
                  <CardTitle className="text-lg">Health Check</CardTitle>
                </div>
                <CardDescription>
                  Monitor the health status of your backend services
                </CardDescription>
              </CardHeader>
              <CardContent>
                <Button 
                  onClick={checkHealth} 
                  variant="outline" 
                  disabled={loading}
                  className="w-full"
                  size="sm"
                >
                  <Activity className="mr-2 h-4 w-4" />
                  Check Health
                </Button>
              </CardContent>
            </Card>

            <Card className="hover:shadow-md transition-shadow">
              <CardHeader>
                <div className="flex items-center">
                  <div className="rounded-lg bg-purple-100 p-2 mr-3">
                    <FolderOpen className="h-5 w-5 text-purple-600" />
                  </div>
                  <CardTitle className="text-lg">Projects</CardTitle>
                </div>
                <CardDescription>
                  Manage your projects with full CRUD operations
                </CardDescription>
              </CardHeader>
              <CardContent>
                <Button 
                  onClick={() => setCurrentPage('projects')}
                  className="w-full"
                  size="sm"
                >
                  <FolderOpen className="mr-2 h-4 w-4" />
                  View Projects
                </Button>
              </CardContent>
            </Card>

            {currentUser?.is_admin && (
              <Card className="hover:shadow-md transition-shadow">
                <CardHeader>
                  <div className="flex items-center">
                    <div className="rounded-lg bg-orange-100 p-2 mr-3">
                      <Users className="h-5 w-5 text-orange-600" />
                    </div>
                    <CardTitle className="text-lg">Users</CardTitle>
                  </div>
                  <CardDescription>
                    Manage user accounts and permissions
                  </CardDescription>
                </CardHeader>
                <CardContent>
                  <Button 
                    onClick={() => setCurrentPage('users')}
                    className="w-full"
                    size="sm"
                  >
                    <Users className="mr-2 h-4 w-4" />
                    Manage Users
                  </Button>
                </CardContent>
              </Card>
            )}

            <Card className="hover:shadow-md transition-shadow">
              <CardHeader>
                <div className="flex items-center">
                  <div className="rounded-lg bg-red-100 p-2 mr-3">
                    <LogOut className="h-5 w-5 text-red-600" />
                  </div>
                  <CardTitle className="text-lg">Account</CardTitle>
                </div>
                <CardDescription>
                  Logged in as {currentUser?.email}
                </CardDescription>
              </CardHeader>
              <CardContent>
                <Button 
                  onClick={handleLogout}
                  variant="outline"
                  className="w-full"
                  size="sm"
                >
                  <LogOut className="mr-2 h-4 w-4" />
                  Logout
                </Button>
              </CardContent>
            </Card>
          </div>

          {message && (
            <Alert variant={messageType === 'error' ? 'destructive' : 'default'} className="max-w-2xl mx-auto">
              {messageType === 'error' ? (
                <AlertCircle className="h-4 w-4" />
              ) : (
                <CheckCircle className="h-4 w-4" />
              )}
              <AlertDescription>
                {message}
              </AlertDescription>
            </Alert>
          )}

          <div className="mt-12 text-center">
            <p className="text-sm text-muted-foreground">
              Built with ❤️ using Rust, React, TypeScript, and Tailwind CSS
            </p>
          </div>
        </div>
      </div>
    </div>
  )
}

export default App
