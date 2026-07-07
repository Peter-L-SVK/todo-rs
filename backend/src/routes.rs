/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

//! REST API routes. Returns full Task objects on create/update to keep frontend state consistent.

use crate::auth::extract_user_id;
use crate::models::{
    AuthResponse, CreateTask, LoginRequest, RegisterRequest, Task, UpdateTask, User, UserInfo,
};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch, post},
};
use axum_csrf::CsrfToken;
use serde_json::{Value, json};
use sqlx::SqlitePool;
use uuid::Uuid;
use validator::Validate;

pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/tasks", get(get_tasks).post(create_task))
        .route("/api/tasks/{id}", patch(update_task).delete(delete_task))
        .route("/api/csrf", get(get_csrf_token))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/me", get(get_current_user))
        .with_state(pool)
}

#[axum::debug_handler]
async fn get_csrf_token(token: CsrfToken) -> (StatusCode, Json<Value>) {
    let token_value = token.authenticity_token().unwrap_or_default();
    (StatusCode::OK, Json(json!({ "csrfToken": token_value })))
}

// ============================================
// AUTH HANDLERS
// ============================================

async fn register(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    use crate::auth::hash_password;

    if let Err(e) = payload.validate() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "status": "error",
                "message": format!("Validation error: {}", e)
            })),
        )
            .into_response();
    }

    let existing = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await;

    if let Ok(Some(_)) = existing {
        return (
            StatusCode::CONFLICT,
            Json(json!({
                "status": "error",
                "message": "User with this email already exists"
            })),
        )
            .into_response();
    }

    let password_hash = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": e
                })),
            )
                .into_response();
        }
    };

    let user_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    match sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&user_id)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(now.to_rfc3339())
    .execute(&pool)
    .await
    {
        Ok(_) => match crate::auth::generate_token(&user_id, &payload.email) {
            Ok(token) => {
                let response = AuthResponse {
                    token,
                    user: UserInfo {
                        id: user_id,
                        username: payload.username,
                        email: payload.email,
                    },
                };
                (StatusCode::CREATED, Json(json!(response))).into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": e
                })),
            )
                .into_response(),
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to create user"
                })),
            )
                .into_response()
        }
    }
}

async fn login(
    State(pool): State<SqlitePool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    use crate::auth::verify_password;

    let user: User = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_one(&pool)
        .await
    {
        Ok(user) => user,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "error",
                    "message": "Invalid email or password"
                })),
            )
                .into_response();
        }
    };

    match verify_password(&payload.password, &user.password_hash) {
        Ok(true) => match crate::auth::generate_token(&user.id, &user.email) {
            Ok(token) => {
                let response = AuthResponse {
                    token,
                    user: user.into(),
                };
                (StatusCode::OK, Json(json!(response))).into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": e
                })),
            )
                .into_response(),
        },
        _ => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status": "error",
                "message": "Invalid email or password"
            })),
        )
            .into_response(),
    }
}

async fn get_current_user(
    State(pool): State<SqlitePool>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok());

    let user_id = match extract_user_id(auth_header) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "error",
                    "message": e
                })),
            )
                .into_response();
        }
    };

    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_one(&pool)
        .await
    {
        Ok(user) => {
            let user_info: UserInfo = user.into();
            (StatusCode::OK, Json(json!(user_info))).into_response()
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "User not found"
            })),
        )
            .into_response(),
    }
}

// ============================================
// TASK HANDLERS (Protected with JWT)
// ============================================

async fn create_task(
    State(pool): State<SqlitePool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok());

    let user_id = match extract_user_id(auth_header) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "error",
                    "message": e
                })),
            )
                .into_response();
        }
    };

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    let insert_result = sqlx::query(
        "INSERT INTO tasks (id, title, completed, priority, due_date, user_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload.title)
    .bind(false)
    .bind(&payload.priority)
    .bind(&payload.due_date)
    .bind(&user_id)
    .bind(now.to_rfc3339())
    .execute(&pool)
    .await;

    match insert_result {
        Ok(_) => {
            match sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
                .bind(&id)
                .fetch_one(&pool)
                .await
            {
                Ok(task) => (StatusCode::CREATED, Json(json!(task))).into_response(),
                Err(e) => {
                    eprintln!("Failed to fetch created task: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "status": "error",
                            "message": "Task created but failed to fetch"
                        })),
                    )
                        .into_response()
                }
            }
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to create task"
                })),
            )
                .into_response()
        }
    }
}

async fn get_tasks(
    State(pool): State<SqlitePool>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok());

    let user_id = match extract_user_id(auth_header) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "error",
                    "message": e
                })),
            )
                .into_response();
        }
    };

    match sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE user_id = ? ORDER BY created_at DESC",
    )
    .bind(&user_id)
    .fetch_all(&pool)
    .await
    {
        Ok(tasks) => (StatusCode::OK, Json(tasks)).into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to fetch tasks"
                })),
            )
                .into_response()
        }
    }
}

async fn update_task(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<UpdateTask>,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok());

    let user_id = match extract_user_id(auth_header) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "error",
                    "message": e
                })),
            )
                .into_response();
        }
    };

    let existing_task: Option<Task> =
        sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ? AND user_id = ?")
            .bind(&id)
            .bind(&user_id)
            .fetch_optional(&pool)
            .await
            .unwrap_or(None);

    if let Some(task) = existing_task {
        let title = payload.title.unwrap_or(task.title);
        let completed = payload.completed.unwrap_or(task.completed);
        let priority = payload.priority.or(task.priority);
        let due_date = payload.due_date.or(task.due_date);

        let update_result = sqlx::query(
            "UPDATE tasks SET title = ?, completed = ?, priority = ?, due_date = ? WHERE id = ? AND user_id = ?"
        )
        .bind(&title)
        .bind(completed)
        .bind(&priority)
        .bind(&due_date)
        .bind(&id)
        .bind(&user_id)
        .execute(&pool)
        .await;

        match update_result {
            Ok(_) => {
                match sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
                    .bind(&id)
                    .fetch_one(&pool)
                    .await
                {
                    Ok(updated_task) => (StatusCode::OK, Json(json!(updated_task))).into_response(),
                    Err(e) => {
                        eprintln!("Failed to fetch updated task: {}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({
                                "status": "error",
                                "message": "Task updated but failed to fetch"
                            })),
                        )
                            .into_response()
                    }
                }
            }
            Err(e) => {
                eprintln!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": "error",
                        "message": "Failed to update task"
                    })),
                )
                    .into_response()
            }
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": "error",
                "message": "Task not found or access denied"
            })),
        )
            .into_response()
    }
}

async fn delete_task(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok());

    let user_id = match extract_user_id(auth_header) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "status": "error",
                    "message": e
                })),
            )
                .into_response();
        }
    };

    match sqlx::query("DELETE FROM tasks WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&user_id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                (StatusCode::OK, Json(json!({ "status": "success" }))).into_response()
            } else {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "status": "error",
                        "message": "Task not found or access denied"
                    })),
                )
                    .into_response()
            }
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": "error",
                    "message": "Failed to delete task"
                })),
            )
                .into_response()
        }
    }
}

// ============================================
// TESTS
// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use uuid::Uuid;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

        sqlx::query(
            "CREATE TABLE tasks (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0,
                priority TEXT DEFAULT 'medium',
                due_date TEXT,
                user_id TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "CREATE TABLE users (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                email TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_create_task_validation() {
        let pool = setup_test_db().await;

        let valid_task = CreateTask {
            title: "Test".to_string(),
            priority: Some("high".to_string()),
            due_date: Some("2024-12-31".to_string()),
        };
        assert!(valid_task.validate().is_ok());

        let invalid_task = CreateTask {
            title: "".to_string(),
            priority: None,
            due_date: None,
        };
        assert!(invalid_task.validate().is_err());
    }

    #[tokio::test]
    async fn test_create_and_get_task() {
        let pool = setup_test_db().await;
        let user_id = Uuid::new_v4().to_string();

        let task_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();

        sqlx::query(
            "INSERT INTO tasks (id, title, completed, priority, due_date, user_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&task_id)
        .bind("Test task")
        .bind(false)
        .bind::<Option<String>>(None)
        .bind::<Option<String>>(None)
        .bind(&user_id)
        .bind(now.to_rfc3339())
        .execute(&pool)
        .await
        .unwrap();

        let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE user_id = ?")
            .bind(&user_id)
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Test task");
        assert_eq!(tasks[0].user_id, user_id);
    }

    #[tokio::test]
    async fn test_update_task() {
        let pool = setup_test_db().await;
        let user_id = Uuid::new_v4().to_string();

        let task_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();

        sqlx::query(
            "INSERT INTO tasks (id, title, completed, priority, due_date, user_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&task_id)
        .bind("Original title")
        .bind(false)
        .bind::<Option<String>>(None)
        .bind::<Option<String>>(None)
        .bind(&user_id)
        .bind(now.to_rfc3339())
        .execute(&pool)
        .await
        .unwrap();

        let updated = sqlx::query_as::<_, Task>(
            "UPDATE tasks SET title = ?, completed = ? WHERE id = ? AND user_id = ? RETURNING *",
        )
        .bind("Updated title")
        .bind(true)
        .bind(&task_id)
        .bind(&user_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(updated.title, "Updated title");
        assert!(updated.completed);
    }

    #[tokio::test]
    async fn test_delete_task() {
        let pool = setup_test_db().await;
        let user_id = Uuid::new_v4().to_string();

        let task_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();

        sqlx::query(
            "INSERT INTO tasks (id, title, completed, priority, due_date, user_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&task_id)
        .bind("Task to delete")
        .bind(false)
        .bind::<Option<String>>(None)
        .bind::<Option<String>>(None)
        .bind(&user_id)
        .bind(now.to_rfc3339())
        .execute(&pool)
        .await
        .unwrap();

        let result = sqlx::query("DELETE FROM tasks WHERE id = ? AND user_id = ?")
            .bind(&task_id)
            .bind(&user_id)
            .execute(&pool)
            .await
            .unwrap();

        assert_eq!(result.rows_affected(), 1);

        let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_user_cannot_see_other_users_tasks() {
        let pool = setup_test_db().await;
        let user1_id = Uuid::new_v4().to_string();
        let user2_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();

        let task_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO tasks (id, title, completed, priority, due_date, user_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&task_id)
        .bind("User 1 task")
        .bind(false)
        .bind::<Option<String>>(None)
        .bind::<Option<String>>(None)
        .bind(&user1_id)
        .bind(now.to_rfc3339())
        .execute(&pool)
        .await
        .unwrap();

        let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE user_id = ?")
            .bind(&user2_id)
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(tasks.len(), 0);
    }
}
