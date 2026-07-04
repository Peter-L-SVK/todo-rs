/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import React from 'react';
import { Task } from '@/types/task.types';
import TaskItem from './TaskItem';

interface TasksContainerProps {
  tasks: Task[];
  onToggle: (id: string) => void;
  onDelete: (id: string) => void;
  onUpdate?: (task: Task) => void;
}

const TasksContainer: React.FC<TasksContainerProps> = ({ 
  tasks, 
  onToggle, 
  onDelete,
  onUpdate
}) => {
  return (
    <ul className="task-list">
      {tasks.length > 0 ? (
        tasks.map(task => (
          <TaskItem
            key={task.id}
            task={task}
            onToggle={onToggle}
            onDelete={onDelete}
            onUpdate={onUpdate}
          />
        ))
      ) : (
        <li className="empty-state">No tasks yet. Add one above!</li>
      )}
    </ul>
  );
};

export default TasksContainer;
