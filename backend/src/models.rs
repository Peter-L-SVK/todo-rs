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

// ============================================
// TASK MODELS
// ============================================

/// Main Task entity - represents a todo item in the database
#[derive(Debug, Serialize, Deserialize, FromRow, Validate, Clone)]
pub struct Task {
    pub id: String,
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    pub completed: bool,
    pub priority: Option<String>,
    pub due_date: Option<String>,
    pub user_id: String,  // Foreign key to User
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

/// DTO for creating a new task - client provides title only
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTask {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    #[validate(length(min = 1, max = 10))]
    pub priority: Option<String>,
    pub due_date: Option<String>,
}

/// DTO for updating a task - all fields optional for partial updates
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTask {
    #[validate(length(min = 1, max = 100))]
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub priority: Option<String>,
    pub due_date: Option<String>,
}

// ============================================
// API RESPONSE WRAPPER
// ============================================

/// Generic API response wrapper for consistent error handling
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

// ============================================
// USER MODELS
// ============================================

/// User entity - stored in the database with hashed password
#[derive(Debug, Serialize, Deserialize, FromRow, Validate, Clone)]
pub struct User {
    pub id: String,
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[serde(skip_serializing)]  // Never send password hash to client
    pub password_hash: String,
    pub role: String,  // 'user' or 'admin'
    pub created_at: DateTime<Utc>,
}

/// DTO for user registration
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

/// DTO for user login
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Response after successful authentication
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

/// Public user information (safe to expose)
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub role: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
        }
    }
}

// ============================================
// JWT CLAIMS
// ============================================

/// JWT claims structure for token generation and validation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,        // User ID (subject)
    pub email: String,      // User email
    pub exp: usize,         // Expiration timestamp (UNIX epoch)
}

// ============================================
// TESTS
// ============================================

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
