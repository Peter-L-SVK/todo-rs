-- Add user_id column to tasks table
ALTER TABLE tasks ADD COLUMN user_id TEXT NOT NULL DEFAULT '';

-- Create index for faster user-specific queries
CREATE INDEX idx_tasks_user_id ON tasks(user_id);

-- Note: Existing tasks will have empty user_id
-- In production, you'd migrate existing data or set a default user
