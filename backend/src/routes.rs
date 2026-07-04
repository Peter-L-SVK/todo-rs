/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

//! REST API routes. Returns full Task objects on create/update to keep frontend state consistent.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    routing::{get, post, patch, delete},
};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use uuid::Uuid;
use validator::Validate;
use axum_csrf::CsrfToken;
use crate::models::{Task, CreateTask, UpdateTask};

pub fn create_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/api/tasks", get(get_tasks).post(create_task))
        .route("/api/tasks/{id}", patch(update_task).delete(delete_task)) 
        .route("/api/csrf", get(get_csrf_token))
        .with_state(pool)
}

#[axum::debug_handler]
async fn get_csrf_token(token: CsrfToken) -> (StatusCode, Json<Value>) {
    let token_value = token.authenticity_token().unwrap_or_default();
    (StatusCode::OK, Json(json!({ "csrfToken": token_value })))
}

async fn create_task(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTask>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    let insert_result = sqlx::query(
        "INSERT INTO tasks (id, title, completed, priority, due_date, created_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&payload.title)
    .bind(false)
    .bind(&payload.priority)
    .bind(&payload.due_date)
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
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                        "status": "error",
                        "message": "Task created but failed to fetch"
                    }))).into_response()
                }
            }
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "status": "error",
                "message": "Failed to create task"
            }))).into_response()
        }
    }
}

async fn get_tasks(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match sqlx::query_as::<_, Task>("SELECT * FROM tasks ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
    {
        Ok(tasks) => (StatusCode::OK, Json(tasks)).into_response(),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "status": "error",
                "message": "Failed to fetch tasks"
            }))).into_response()
        },
    }
}

// Fetches task first to preserve existing values for missing fields
async fn update_task(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<UpdateTask>,
) -> impl IntoResponse {
    let existing_task: Option<Task> = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
        .bind(&id)
        .fetch_optional(&pool)
        .await
        .unwrap_or(None);

    if let Some(task) = existing_task {
        let title = payload.title.unwrap_or(task.title);
        let completed = payload.completed.unwrap_or(task.completed);
        let priority = payload.priority.or(task.priority);
        let due_date = payload.due_date.or(task.due_date);

        let update_result = sqlx::query(
            "UPDATE tasks SET title = ?, completed = ?, priority = ?, due_date = ? WHERE id = ?"
        )
        .bind(&title)
        .bind(&completed)
        .bind(&priority)
        .bind(&due_date)
        .bind(&id)
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
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                            "status": "error",
                            "message": "Task updated but failed to fetch"
                        }))).into_response()
                    }
                }
            },
            Err(e) => {
                eprintln!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                    "status": "error",
                    "message": "Failed to update task"
                }))).into_response()
            }
        }
    } else {
        (StatusCode::NOT_FOUND, Json(json!({
            "status": "error",
            "message": "Task not found"
        }))).into_response()
    }
}

async fn delete_task(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    match sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                (StatusCode::OK, Json(json!({ "status": "success" }))).into_response()
            } else {
                (StatusCode::NOT_FOUND, Json(json!({
                    "status": "error",
                    "message": "Task not found"
                }))).into_response()
            }
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "status": "error",
                "message": "Failed to delete task"
            }))).into_response()
        },
    }
}

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
                created_at TEXT NOT NULL
            )"
        )
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_create_task_validation() {
        let pool = setup_test_db().await;
        
        // Valid task
        let valid_task = CreateTask {
            title: "Test".to_string(),
            priority: Some("high".to_string()),
            due_date: Some("2024-12-31".to_string()),
        };
        assert!(valid_task.validate().is_ok());

        // Invalid task - empty title
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

        // Insert test task directly
        let task_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        sqlx::query(
            "INSERT INTO tasks (id, title, completed, priority, due_date, created_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&task_id)
        .bind("Test task")
        .bind(false)
        .bind::<Option<String>>(None)
        .bind::<Option<String>>(None)
        .bind(now.to_rfc3339())
        .execute(&pool)
        .await
        .unwrap();

        // Fetch tasks
        let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Test task");
        assert_eq!(tasks[0].id, task_id);
    }

    #[tokio::test]
    async fn test_update_task() {
        let pool = setup_test_db().await;
        
        // Insert test task
        let task_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        sqlx::query(
            "INSERT INTO tasks (id, title, completed, priority, due_date, created_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&task_id)
        .bind("Original title")
        .bind(false)
        .bind::<Option<String>>(None)
        .bind::<Option<String>>(None)
        .bind(now.to_rfc3339())
        .execute(&pool)
        .await
        .unwrap();

        // Update task
        let updated = sqlx::query_as::<_, Task>(
            "UPDATE tasks SET title = ?, completed = ? WHERE id = ? RETURNING *"
        )
        .bind("Updated title")
        .bind(true)
        .bind(&task_id)
        .fetch_one(&pool)
        .await
        .unwrap();

        assert_eq!(updated.title, "Updated title");
        assert!(updated.completed);
    }

    #[tokio::test]
    async fn test_delete_task() {
        let pool = setup_test_db().await;
        
        // Insert test task
        let task_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        sqlx::query(
            "INSERT INTO tasks (id, title, completed, priority, due_date, created_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&task_id)
        .bind("Task to delete")
        .bind(false)
        .bind::<Option<String>>(None)
        .bind::<Option<String>>(None)
        .bind(now.to_rfc3339())
        .execute(&pool)
        .await
        .unwrap();

        // Delete task
        let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind(&task_id)
            .execute(&pool)
            .await
            .unwrap();

        assert_eq!(result.rows_affected(), 1);

        // Verify task is gone
        let tasks = sqlx::query_as::<_, Task>("SELECT * FROM tasks")
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_delete_nonexistent_task() {
        let pool = setup_test_db().await;
        
        let result = sqlx::query("DELETE FROM tasks WHERE id = ?")
            .bind("nonexistent-id")
            .execute(&pool)
            .await
            .unwrap();

        assert_eq!(result.rows_affected(), 0);
    }

    #[tokio::test]
    async fn test_create_task_with_priority() {
        let pool = setup_test_db().await;

        // Insert test task with priority
        let task_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        sqlx::query(
            "INSERT INTO tasks (id, title, completed, priority, due_date, created_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&task_id)
        .bind("High priority task")
        .bind(false)
        .bind("high")
        .bind::<Option<String>>(None)
        .bind(now.to_rfc3339())
        .execute(&pool)
        .await
        .unwrap();

        // Fetch task
        let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
            .bind(&task_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(task.priority, Some("high".to_string()));
    }

    #[tokio::test]
    async fn test_create_task_with_due_date() {
        let pool = setup_test_db().await;

        // Insert test task with due date
        let task_id = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        sqlx::query(
            "INSERT INTO tasks (id, title, completed, priority, due_date, created_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&task_id)
        .bind("Task with due date")
        .bind(false)
        .bind::<Option<String>>(None)
        .bind("2024-12-31")
        .bind(now.to_rfc3339())
        .execute(&pool)
        .await
        .unwrap();

        // Fetch task
        let task = sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE id = ?")
            .bind(&task_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(task.due_date, Some("2024-12-31".to_string()));
    }
}
