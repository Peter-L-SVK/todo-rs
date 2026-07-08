/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import React, { useState, useEffect, useCallback } from 'react';
import { BrowserRouter, Routes, Route, Link, useLocation, Navigate } from 'react-router-dom';
import TaskList from './components/TaskList';
import AdminPanel from './components/Admin';
import { getCurrentUser, getAuthToken } from './api/authApi';
import './App.css';

const AppContent: React.FC = () => {
  const location = useLocation();
  const isAdminRoute = location.pathname.startsWith('/admin');
  const [isAdminUser, setIsAdminUser] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(true);

  const checkAdminRole = useCallback(async () => {
    const token = getAuthToken();
    if (!token) {
      setIsAdminUser(false);
      setLoading(false);
      return;
    }

    try {
      const user = await getCurrentUser();
      setIsAdminUser(user.role === 'admin');
    } catch (error) {
      console.error('Failed to check admin role:', error);
      setIsAdminUser(false);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    checkAdminRole();
  }, [checkAdminRole]);

  useEffect(() => {
    const handleAuthChange = (event: Event) => {
      if (event.type === 'storage' || event.type === 'authTokenChanged') {
        setLoading(true);
        checkAdminRole();
      }
    };

    window.addEventListener('storage', handleAuthChange);
    window.addEventListener('authTokenChanged', handleAuthChange);

    return () => {
      window.removeEventListener('storage', handleAuthChange);
      window.removeEventListener('authTokenChanged', handleAuthChange);
    };
  }, [checkAdminRole]);

  // Redirect if not admin
  if (isAdminRoute && !loading && !isAdminUser) {
    return <Navigate to="/" replace />;
  }

  // Hide main navigation on admin routes
  const showNavigation = !isAdminRoute;

  return (
    <div className="app">
      {showNavigation && !loading && isAdminUser && (
        <nav className="app-nav">
          <Link to="/">📋 Todo List</Link>
          <Link to="/admin">⚙️ Admin</Link>
        </nav>
      )}

      <Routes>
        <Route path="/" element={<TaskList />} />
        <Route path="/admin/*" element={<AdminPanel />} />
      </Routes>
    </div>
  );
};

const App: React.FC = () => {
  return (
    <BrowserRouter>
      <AppContent />
    </BrowserRouter>
  );
};

export default App;
