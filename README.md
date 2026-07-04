# Rust + React TypeScript Todo App

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) 
[![Top Language](https://img.shields.io/github/languages/top/Peter-L-SVK/TermTalk)](https://github.com/Peter-L-SVK/todo-rs)
[![GitHub release (latest by date)](https://img.shields.io/github/v/release/Peter-L-SVK/TermTalk)](https://github.com/Peter-L-SVK/todo-rs/releases/latest)
[![GitHub last commit](https://img.shields.io/github/last-commit/Peter-L-SVK/TermTalk)](https://github.com/Peter-L-SVK/todo-rs/commits/main)

Full-stack todo application with Rust (Axum) backend and React TypeScript frontend.

![ToDo demo](demo.png) 

## Features

- Create, read, update, delete tasks
- Priority levels (Low/Medium/High) with color coding
- Due dates for tasks
- Progress tracking with percentage bar
- Task filtering (All/Active/Completed)
- Inline editing (double-click or edit button)
- CSRF protection
- SQLite database
- Responsive design

## Tech Stack

**Backend:** Rust, Axum, Tokio, SQLx, SQLite, Serde, UUID, Chrono, Validator

**Frontend:** React 19, TypeScript 5.5, Vite 6.3, Axios, React Icons

## Project Structure

```
todo-app-rs/
├── backend/
│   ├── src/
│   │   ├── main.rs          # Entry point
│   │   ├── database.rs      # DB connection
│   │   ├── models.rs        # Data models
│   │   └── routes.rs        # API routes
│   ├── migrations/          # SQL migrations
│   └── Cargo.toml
└── frontend/
    ├── src/
    │   ├── api/
    │   │   └── tasksApi.ts
    │   ├── components/
    │   │   ├── TaskList.tsx
    │   │   ├── TaskForm.tsx
    │   │   ├── TaskItem.tsx
    │   │   ├── TaskFilters.tsx
    │   │   └── TasksContainer.tsx
    │   ├── types/
    │   │   └── task.types.ts
    │   ├── App.tsx
    │   └── main.tsx
    ├── package.json
    └── tsconfig.json
```

## Setup

### Backend
```bash
cd backend
echo "DATABASE_URL=sqlite:todo.db" > .env
sqlx migrate run
cargo run
# Server: http://localhost:8000
```

### Frontend
```bash
cd frontend
npm install
npm run dev
# App: http://localhost:5173
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/csrf` | Get CSRF token |
| GET | `/api/tasks` | Get all tasks |
| POST | `/api/tasks` | Create task |
| PATCH | `/api/tasks/{id}` | Update task |
| DELETE | `/api/tasks/{id}` | Delete task |

### Data Models
```typescript
Task {
  id: string
  title: string
  completed: boolean
  priority?: "low" | "medium" | "high"
  due_date?: string  // YYYY-MM-DD
  created_at: number
}
```

## Testing

### Backend Tests (Rust)

Run all backend unit and integration tests:

```bash
cd backend
cargo test
```

Run specific test:

```bash
cargo test test_create_and_get_task
```

Run tests with output:

```bash
cargo test -- --nocapture
```

**Test Coverage:**

| Test | Description |
|------|-------------|
| `test_api_response_success` | ApiResponse success format |
| `test_api_response_error` | ApiResponse error format |
| `test_create_task_valid` | Valid task validation |
| `test_create_task_empty_title` | Empty title validation |
| `test_create_task_title_too_long` | Title length validation |
| `test_create_pool` | Database connection |
| `test_pool_connections` | Connection pool |
| `test_create_and_get_task` | Create + fetch task |
| `test_update_task` | Update task |
| `test_delete_task` | Delete task |
| `test_delete_nonexistent_task` | Delete non-existent task |
| `test_create_task_with_priority` | Create task with priority |
| `test_create_task_with_due_date` | Create task with due date |

### Frontend Tests (React + Vitest)

Run all frontend tests:

```bash
cd frontend
npm run test
```

Run tests in watch mode:

```bash
npm run test -- --watch
```

Run tests with coverage:

```bash
npm run test -- --coverage
```

**Test Coverage:**

| Test | Description |
|------|-------------|
| `TaskItem` | Rendering, edit mode, toggle, delete |
| `TaskList` | Loading, filtering, add, toggle, delete |

### API Testing with curl

```bash
# Get CSRF token
CSRF_TOKEN=$(curl -s http://localhost:8000/api/csrf | grep -o '"csrfToken":"[^"]*"' | cut -d'"' -f4)

# Create task
curl -X POST http://localhost:8000/api/tasks \
  -H "Content-Type: application/json" \
  -H "X-CSRF-Token: $CSRF_TOKEN" \
  -d '{"title":"Learn Rust","priority":"high"}'

# Get all tasks
curl http://localhost:8000/api/tasks

# Update task (replace {id} with actual task ID)
curl -X PATCH http://localhost:8000/api/tasks/{id} \
  -H "Content-Type: application/json" \
  -H "X-CSRF-Token: $CSRF_TOKEN" \
  -d '{"completed":true}'

# Delete task (replace {id} with actual task ID)
curl -X DELETE http://localhost:8000/api/tasks/{id} \
  -H "X-CSRF-Token: $CSRF_TOKEN"
```

### API Testing with Postman

1. Import the collection (optional)
2. Set environment variable: `base_url = http://localhost:8000`
3. Test endpoints in this order:
   1. `GET /api/csrf` - Store CSRF token
   2. `POST /api/tasks` - Create task
   3. `GET /api/tasks` - Get all tasks
   4. `PATCH /api/tasks/{id}` - Update task
   5. `DELETE /api/tasks/{id}` - Delete task

**Postman Headers:**
```
Content-Type: application/json
X-CSRF-Token: {{csrf_token}}
```

### Linting

```bash
cd frontend
npm run lint
```

## Database Schema

```sql
CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    completed BOOLEAN DEFAULT 0,
    priority TEXT DEFAULT 'medium',
    due_date TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX idx_tasks_completed ON tasks(completed);
CREATE INDEX idx_tasks_priority ON tasks(priority);
CREATE INDEX idx_tasks_created_at ON tasks(created_at DESC);
```

## Development Commands

### Backend
```bash
cargo run        # Run server
cargo build --release  # Build release
cargo test       # Run tests
sqlx migrate run  # Run migrations
```

### Frontend
```bash
npm run dev       # Development
npm run build     # Production build
npm run preview   # Preview build
npm run test      # Run tests
npm run lint      # Run ESLint
npx tsc --noEmit  # Type check
```

## Contributing

1. Fork the repository
2. Create feature branch
3. Make changes
4. Submit pull request

Keep changes:
- Type-safe (TypeScript strict mode)
- Error handled
- Consistent API responses
- Well commented

## License

MIT License - See [LICENSE](LICENSE) for details.
