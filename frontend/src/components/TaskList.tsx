/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

// Main task container. Manages all tasks state, CRUD operations, and filtering.

import React, { useState, useEffect } from 'react';
import { Task, FilterType, CreateTaskDto } from '@/types/task.types';
import { 
  getCsrfToken, 
  fetchTasks, 
  createTask, 
  updateTask, 
  deleteTask,
  getErrorMessage 
} from '@/api/tasksApi';
import { getCurrentUser, removeAuthToken, getAuthToken, setAuthToken } from '@/api/authApi';
import TaskForm from './TaskForm';
import TasksContainer from './TasksContainer';
import TaskFilters from './TaskFilters';
import Login from './Login';
import Register from './Register';
import './TaskList.css';
import './Auth.css';

const TaskList: React.FC = () => {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [newTask, setNewTask] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  const [filter, setFilter] = useState<FilterType>('all');
  const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false);
  const [username, setUsername] = useState<string>('');
  const [showRegister, setShowRegister] = useState<boolean>(false);

  // Check authentication on mount
  useEffect(() => {
    const token = getAuthToken();
    if (token) {
      setAuthToken(token);
      setIsAuthenticated(true);
      getCurrentUser()
        .then(user => setUsername(user.username))
        .catch(() => {
          removeAuthToken();
          setIsAuthenticated(false);
        });
    } else {
      setIsAuthenticated(false);
    }
  }, []);

  // Load tasks when authenticated
  useEffect(() => {
    if (isAuthenticated) {
      const initialize = async (): Promise<void> => {
        try {
          await getCsrfToken();
          const tasksData = await fetchTasks();
          setTasks(tasksData);
        } catch (err) {
          setError(getErrorMessage(err));
        } finally {
          setLoading(false);
        }
      };
      void initialize();
    }
  }, [isAuthenticated]);

  const handleLogin = async () => {
    try {
      const user = await getCurrentUser();
      setUsername(user.username);
      setIsAuthenticated(true);
      setLoading(true);
    } catch (err) {
      setError('Failed to load user data');
    }
  };

  const handleLogout = () => {
    removeAuthToken();
    setIsAuthenticated(false);
    setUsername('');
    setTasks([]);
    setError(null);
  };

  const handleAddTask = async (e: React.FormEvent): Promise<void> => {
      e.preventDefault();
      const trimmedTitle = newTask.trim();
      if (!trimmedTitle) return;

      try {
	  const taskData: CreateTaskDto = { 
	      title: trimmedTitle,
	      priority: 'medium'
	  };
	  const newTaskData = await createTask(taskData);
	  setTasks([newTaskData, ...tasks]);
	  setNewTask('');
      } catch (err) {
	  setError(getErrorMessage(err));
      }
  };

  const handleToggleTask = async (taskId: string): Promise<void> => {
    try {
      const task = tasks.find(t => t.id === taskId);
      if (!task) return;

      const updatedTask = await updateTask(taskId, {
        title: task.title,
        completed: !task.completed
      });

      setTasks(tasks.map(t => t.id === taskId ? updatedTask : t));
    } catch (err) {
      setError(getErrorMessage(err));
    }
  };

  const handleDeleteTask = async (taskId: string): Promise<void> => {
    try {
      await deleteTask(taskId);
      setTasks(tasks.filter(t => t.id !== taskId));
    } catch (err) {
      setError(getErrorMessage(err));
    }
  };

  const handleUpdateTask = (updatedTask: Task): void => {
    setTasks(tasks.map(t => t.id === updatedTask.id ? updatedTask : t));
  };

  // Non-async wrappers for event handlers
  const onAddTask = (e: React.FormEvent): void => {
    void handleAddTask(e);
  };

  const onToggleTask = (taskId: string): void => {
    void handleToggleTask(taskId);
  };

  const onDeleteTask = (taskId: string): void => {
    void handleDeleteTask(taskId);
  };

  const filteredTasks = tasks.filter(task => {
    if (filter === 'active') return !task.completed;
    if (filter === 'completed') return task.completed;
    return true;
  });

  const totalTasks = tasks.length;
  const completedTasks = tasks.filter(t => t.completed).length;
  const completionPercentage = totalTasks === 0 ? 0 : Math.round((completedTasks / totalTasks) * 100);

  if (!isAuthenticated) {
    return (
      <div className="auth-wrapper">
        {showRegister ? (
          <Register 
            onRegister={handleLogin} 
            onSwitchToLogin={() => setShowRegister(false)} 
          />
        ) : (
          <Login 
            onLogin={handleLogin} 
            onSwitchToRegister={() => setShowRegister(true)} 
          />
        )}
      </div>
    );
  }

  if (loading) return <div className="container loading">Loading...</div>;
  if (error) return <div className="container error">Error: {error}</div>;

  return (
    <div className="container">
      <div className="header">
        <h1>
          <img 
            src="/to-do-list.svg"
            alt="Todo List" 
            className="to-do-list"
            width="32"
            height="32"
          />
          {' '}Todo List
        </h1>
        <div className="user-info">
          <span>👤 {username}</span>
          <button onClick={handleLogout} className="logout-btn">Logout</button>
        </div>
      </div>
      
      <TaskForm 
        newTask={newTask}
        setNewTask={setNewTask}
        handleAddTask={onAddTask}
      />

      <div className="progress-container">
        <div className="progress-info">
          <span>📊 Progress: {completionPercentage}%</span>
          <span>✅ {completedTasks} / {totalTasks} done</span>
        </div>
        <div className="progress-bar">
          <div 
            className={`progress-fill ${completionPercentage === 100 ? 'complete' : completionPercentage < 30 ? 'low' : ''}`}
            style={{ width: `${completionPercentage}%` }}
          />
        </div>
      </div>

      <TaskFilters 
        currentFilter={filter}
        onFilterChange={setFilter}
        tasksCount={tasks.length}
        activeCount={tasks.filter(t => !t.completed).length}
      />

      <TasksContainer 
        tasks={filteredTasks}
        onToggle={onToggleTask}
        onDelete={onDeleteTask}
        onUpdate={handleUpdateTask}
      />
    </div>
  );
};

export default TaskList;
