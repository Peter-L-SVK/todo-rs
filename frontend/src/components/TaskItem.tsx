/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

// Task item with inline editing (double-click or edit button).
// Supports priority and due date changes via dropdown/date picker.

import React, { useState, useRef, useEffect } from 'react';
import { FiTrash2, FiCircle, FiCheckCircle, FiEdit2 } from 'react-icons/fi';
import { Task } from '@/types/task.types';
import { updateTask } from '@/api/tasksApi';

interface TaskItemProps {
  task: Task;
  onToggle: (id: string) => void;
  onDelete: (id: string) => void;
  onUpdate?: (task: Task) => void;
}

interface UpdatePayload {
  title: string;
  priority?: string;
  due_date?: string | null;
}

const TaskItem: React.FC<TaskItemProps> = ({ task, onToggle, onDelete, onUpdate }) => {
  const [isEditing, setIsEditing] = useState(false);
  const [editText, setEditText] = useState(task.title);
  const [editPriority, setEditPriority] = useState(task.priority || 'medium');
  const [editDueDate, setEditDueDate] = useState(task.due_date || '');
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isEditing && inputRef.current) {
      inputRef.current.focus();
      inputRef.current.select();
    }
  }, [isEditing]);

  const getPriorityColor = (priority?: string): string => {
    switch (priority) {
      case 'high': return '#ff4757';
      case 'medium': return '#ffa502';
      case 'low': return '#2ed573';
      default: return '#747d8c';
    }
  };

  const getPriorityLabel = (priority?: string): string => {
    switch (priority) {
      case 'high': return '🔴 High';
      case 'medium': return '🟡 Medium';
      case 'low': return '🟢 Low';
      default: return '⚪ None';
    }
  };

  const formatDate = (dateString?: string | null): string => {
    if (!dateString) return '';
    try {
      const date = new Date(dateString);
      return date.toLocaleDateString('sk-SK', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric'
      });
    } catch {
      return '';
    }
  };

  const handleDoubleClick = (): void => {
    setIsEditing(true);
    setEditText(task.title);
    setEditPriority(task.priority || 'medium');
    setEditDueDate(task.due_date || '');
  };

  const handleSave = async (): Promise<void> => {
    const trimmedText = editText.trim();
    if (!trimmedText) {
      setIsEditing(false);
      return;
    }

    try {
      const updates: UpdatePayload = { title: trimmedText };
      
      if (editPriority !== (task.priority || 'medium')) {
        updates.priority = editPriority;
      }
      if (editDueDate !== (task.due_date || '')) {
        updates.due_date = editDueDate || null;
      }

      const updatedTask = await updateTask(task.id, updates);
      if (onUpdate) {
        onUpdate(updatedTask);
      }
      setIsEditing(false);
    } catch (error) {
      console.error('Failed to update task:', error);
    }
  };

  const handleCancel = (): void => {
    setIsEditing(false);
    setEditText(task.title);
    setEditPriority(task.priority || 'medium');
    setEditDueDate(task.due_date || '');
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>): void => {
    if (e.key === 'Enter') {
      e.preventDefault();
      void handleSave();  // Explicitly mark as void
    } else if (e.key === 'Escape') {
      e.preventDefault();
      handleCancel();
    }
  };

  if (isEditing) {
    return (
      <li className={`task-item ${task.completed ? 'completed' : ''}`}>
        <button 
          className="toggle-btn"
          onClick={() => onToggle(task.id)}
          aria-label={task.completed ? "Mark incomplete" : "Mark complete"}
        >
          {task.completed ? <FiCheckCircle /> : <FiCircle />}
        </button>
        <input
          ref={inputRef}
          type="text"
          value={editText}
          onChange={(e) => setEditText(e.target.value)}
          onKeyDown={handleKeyDown}
          className="edit-input"
          placeholder="Task title..."
        />
        <select
          value={editPriority}
          onChange={(e) => setEditPriority(e.target.value)}
          className="edit-select"
        >
          <option value="low">🟢 Low</option>
          <option value="medium">🟡 Medium</option>
          <option value="high">🔴 High</option>
        </select>
        <input
          type="date"
          value={editDueDate}
          onChange={(e) => setEditDueDate(e.target.value)}
          className="edit-date"
        />
        <button 
          className="delete-btn"
          onClick={() => onDelete(task.id)}
          aria-label="Delete task"
        >
          <FiTrash2 />
        </button>
      </li>
    );
  }

  return (
    <li className={`task-item ${task.completed ? 'completed' : ''}`}>
      <button 
        className="toggle-btn"
        onClick={() => onToggle(task.id)}
        aria-label={task.completed ? "Mark incomplete" : "Mark complete"}
      >
        {task.completed ? <FiCheckCircle /> : <FiCircle />}
      </button>
      <span 
        className="task-title"
        onDoubleClick={handleDoubleClick}
        style={{ cursor: 'pointer' }}
      >
        {task.title}
      </span>
      <button 
        className="edit-btn"
        onClick={handleDoubleClick}
        aria-label="Edit task"
        title="Edit task (double-click or click here)"
      >
        <FiEdit2 size={16} />
      </button>
      {task.priority && (
        <span 
          className="priority-badge"
	  data-priority={task.priority}
          style={{ backgroundColor: getPriorityColor(task.priority) }}
          title={getPriorityLabel(task.priority)}
        >
          {task.priority}
        </span>
      )}
      {task.due_date && (
        <span className="due-date">
          📅 {formatDate(task.due_date)}
        </span>
      )}
      <button 
        className="delete-btn"
        onClick={() => onDelete(task.id)}
        aria-label="Delete task"
      >
        <FiTrash2 />
      </button>
    </li>
  );
};

export default TaskItem;
