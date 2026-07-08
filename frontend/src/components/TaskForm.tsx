/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import React from "react";

interface TaskFormProps {
  newTask: string;
  setNewTask: (value: string) => void;
  handleAddTask: (e: React.FormEvent) => void;
}

const TaskForm: React.FC<TaskFormProps> = ({
  newTask,
  setNewTask,
  handleAddTask,
}) => {
  return (
    <form onSubmit={handleAddTask} className="task-form">
      <input
        type="text"
        value={newTask}
        onChange={(e) => setNewTask(e.target.value)}
        placeholder="Add a new task..."
        autoFocus
      />
      <button type="submit">Add</button>
    </form>
  );
};

export default TaskForm;
