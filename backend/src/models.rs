/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

//! Task entity and DTOs. DTOs separate from Task because id/created_at are auto-generated.

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
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
