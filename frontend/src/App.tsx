/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import React from 'react';
import TaskList from './components/TaskList';
import './App.css';

const App: React.FC = () => {
  return (
    <div className="app">
      <TaskList />
    </div>
  );
};

export default App;
