/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

//! Task entity and DTOs. DTOs separate from Task because id/created_at are auto-generated.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Validate, Clone)]
pub struct Task {
    pub id: String,
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    pub completed: bool,
    pub priority: Option<String>,
    pub due_date: Option<String>,
    pub user_id: String, // NEW - links task to user
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTask {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(min = 1, max = 10))]
    pub priority: Option<String>,
    pub due_date: Option<String>,
    // user_id is added by backend from JWT, not by client
}

// All fields optional for partial updates (PATCH)
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTask {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub priority: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: "success",
            data: Some(data),
            message: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            status: "error",
            data: None,
            message: Some(message),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, Validate, Clone)]
pub struct User {
    pub id: String,
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user_id
    pub email: String,
    pub exp: usize, // expiration timestamp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task_valid() {
        let task = CreateTask {
            title: "Test task".to_string(),
            priority: Some("high".to_string()),
            due_date: Some("2024-12-31".to_string()),
        };
        assert!(task.validate().is_ok());
    }

    #[test]
    fn test_create_task_title_too_long() {
        let task = CreateTask {
            title: "a".repeat(101),
            priority: None,
            due_date: None,
        };
        assert!(task.validate().is_err());
    }

    #[test]
    fn test_create_task_empty_title() {
        let task = CreateTask {
            title: "".to_string(),
            priority: None,
            due_date: None,
        };
        assert!(task.validate().is_err());
    }

    #[test]
    fn test_api_response_success() {
        let response: ApiResponse<String> = ApiResponse::success("test data".to_string());
        assert_eq!(response.status, "success");
        assert_eq!(response.data, Some("test data".to_string()));
        assert!(response.message.is_none());
    }

    #[test]
    fn test_api_response_error() {
        let response: ApiResponse<String> = ApiResponse::error("something went wrong".to_string());
        assert_eq!(response.status, "error");
        assert!(response.data.is_none());
        assert_eq!(response.message, Some("something went wrong".to_string()));
    }
}
