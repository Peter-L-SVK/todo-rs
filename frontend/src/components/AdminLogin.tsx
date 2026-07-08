/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import React, { useState } from "react";
import { login, setAuthToken, getCurrentUser } from "../api/authApi";
import { AxiosError } from "axios";
import "./Auth.css";

interface AdminLoginProps {
  onLogin: () => void;
}

interface ErrorResponse {
  message?: string;
}

const AdminLogin: React.FC<AdminLoginProps> = ({ onLogin }) => {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);

    try {
      // 1. Login to get JWT token
      const response = await login({ email, password });
      setAuthToken(response.token);

      // 2. Verify user has admin role
      const user = await getCurrentUser();
      if (user.role !== "admin") {
        setError("Admin access required");
        setLoading(false);
        return;
      }

      // 3. Notify parent component about successful login
      // App.tsx will react to authTokenChanged event and update UI
      onLogin();
    } catch (err) {
      const error = err as AxiosError<ErrorResponse>;
      setError(error.response?.data?.message || "Login failed");
      setLoading(false);
    }
  };

  return (
    <div className="auth-container">
      <h2>🔐 Admin Login</h2>
      <form onSubmit={handleSubmit} className="auth-form">
        <input
          type="email"
          placeholder="Admin Email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          required
        />
        <input
          type="password"
          placeholder="Password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          required
        />
        {error && <div className="auth-error">{error}</div>}
        <button type="submit" disabled={loading}>
          {loading ? "Loading..." : "Login as Admin"}
        </button>
      </form>
    </div>
  );
};

export default AdminLogin;
