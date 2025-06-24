# Vibe Kanban

> A visual project management tool for developers that integrates with git repositories and coding agents like Claude Code and Amp.

## Quick Start

Run vibe kanban instantly without installation:

```bash
npx vibe-kanban
```

This will launch the application locally and open it in your browser automatically.

## What is Vibe Kanban?

Vibe Kanban is a modern project management tool designed specifically for developers. It helps you organize your coding projects with kanban-style task management while providing powerful integrations with git repositories and AI coding agents.

### ✨ Key Features

**🗂️ Project Management**
- Add git repositories as projects (existing or create new ones)
- Automatic git integration and repository validation
- Project search functionality across all files
- Custom setup and development scripts per project

**📋 Task Management**
- Create and manage tasks with kanban-style boards
- Task status tracking (Todo, In Progress, Done)
- Rich task descriptions and notes
- Task execution with multiple AI agents

**🤖 AI Agent Integration**
- **Claude**: Advanced AI coding assistant
- **Amp**: Powerful development agent
- **Echo**: Simple testing/debugging agent
- Create tasks and immediately start agent execution
- Follow-up task execution for iterative development

**⚡ Development Workflow**
- Create isolated git worktrees for each task attempt
- View diffs of changes made by agents
- Merge successful changes back to main branch
- Rebase task branches to stay up-to-date
- Manual file editing and deletion
- Integrated development server support

**🎛️ Developer Tools**
- Browse and validate git repositories from filesystem
- Open task worktrees in your preferred editor (VS Code, Cursor, Windsurf, IntelliJ, Zed)
- Real-time execution monitoring and process control
- Stop running processes individually or all at once
- Sound notifications for task completion

## How It Works

1. **Add Projects**: Import existing git repositories or create new ones
2. **Create Tasks**: Define what needs to be built or fixed
3. **Execute with AI**: Let coding agents work on your tasks in isolated environments
4. **Review Changes**: See exactly what was modified using git diffs
5. **Merge Results**: Incorporate successful changes into your main codebase

## API Endpoints

Vibe Kanban provides a comprehensive REST API:

### Projects (`/api/projects`)
- `GET /projects` - List all projects
- `POST /projects` - Create new project
- `GET /projects/:id` - Get project details
- `PUT /projects/:id` - Update project
- `DELETE /projects/:id` - Delete project
- `GET /projects/:id/search` - Search files in project

### Tasks (`/api/projects/:project_id/tasks`)
- `GET /tasks` - List project tasks
- `POST /tasks` - Create new task
- `POST /tasks/create-and-start` - Create task and start execution
- `GET /tasks/:task_id` - Get task details
- `PUT /tasks/:task_id` - Update task
- `DELETE /tasks/:task_id` - Delete task

### Task Attempts (`/api/projects/:project_id/tasks/:task_id/attempts`)
- `GET /attempts` - List task attempts
- `POST /attempts` - Create new attempt
- `GET /attempts/:attempt_id/activities` - Get execution activities
- `GET /attempts/:attempt_id/diff` - View changes made
- `POST /attempts/:attempt_id/merge` - Merge changes to main
- `POST /attempts/:attempt_id/rebase` - Rebase onto main branch
- `GET /attempts/:attempt_id/branch-status` - Check branch status
- `POST /attempts/:attempt_id/open-editor` - Open in code editor
- `POST /attempts/:attempt_id/follow-up` - Continue with follow-up execution
- `POST /attempts/:attempt_id/start-dev-server` - Start development server
- `POST /attempts/:attempt_id/stop` - Stop all execution processes

### Configuration (`/api/config`)
- `GET /config` - Get current configuration
- `POST /config` - Update configuration
- `GET /config/constants` - Get available options

### Filesystem (`/api/filesystem`)
- `GET /filesystem/list` - Browse directories
- `GET /filesystem/validate-git` - Validate git repository
- `GET /filesystem/create-git` - Initialize new git repository

### Health (`/api/health`)
- `GET /health` - Service health check

## Configuration

Vibe Kanban supports customization through its configuration system:

- **Editor Integration**: Choose your preferred code editor
- **Sound Notifications**: Customize completion sounds
- **Project Defaults**: Set default setup and development scripts

## Technical Architecture

- **Backend**: Rust with Axum web framework
- **Frontend**: React with TypeScript
- **Database**: SQLite for local data storage
- **Git Integration**: Native git operations for repository management
- **Process Management**: Tokio-based async execution monitoring

## Requirements

- Node.js (for npx execution)
- Git (for repository operations)
- Your preferred code editor (optional, for opening task worktrees)

## Use Cases

**🔧 Bug Fixes**
- Create a task describing the bug
- Let an AI agent analyze and fix the issue
- Review the proposed changes
- Merge if satisfied, or provide follow-up instructions

**✨ Feature Development**
- Break down features into manageable tasks
- Use agents for initial implementation
- Iterate with follow-up executions
- Test using integrated development servers

**🚀 Project Setup**
- Bootstrap new projects with AI assistance
- Set up development environments
- Configure build and deployment scripts

**📚 Code Documentation**
- Generate documentation for existing code
- Create README files and API documentation
- Maintain up-to-date project information

---

**Ready to supercharge your development workflow?**

```bash
npx vibe-kanban
```

*Start managing your projects with the power of AI coding agents today!*
