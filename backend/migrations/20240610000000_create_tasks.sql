/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0,
    priority TEXT DEFAULT 'medium',
    due_date TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX idx_tasks_completed ON tasks(completed);
CREATE INDEX idx_tasks_priority ON tasks(priority);
CREATE INDEX idx_tasks_created_at ON tasks(created_at DESC);