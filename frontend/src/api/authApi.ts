/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import { api } from "./tasksApi";
import axios from "axios";

export interface User {
  id: string;
  username: string;
  email: string;
  role: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export const register = async (
  data: RegisterRequest,
): Promise<AuthResponse> => {
  const response = await api.post<AuthResponse>("/api/auth/register", data);
  return response.data;
};

export const login = async (data: LoginRequest): Promise<AuthResponse> => {
  const response = await api.post<AuthResponse>("/api/auth/login", data);
  return response.data;
};

export const getCurrentUser = async (): Promise<User> => {
  try {
    const token = getAuthToken();
    if (!token) {
      throw new Error("No auth token found");
    }

    api.defaults.headers.common["Authorization"] = `Bearer ${token}`;

    const response = await api.get<User>("/api/auth/me");
    return response.data;
  } catch (error) {
    if (axios.isAxiosError(error) && error.response?.status === 401) {
      removeAuthToken();
    }
    throw error;
  }
};

// Store token and dispatch event for real-time UI updates
export const setAuthToken = (token: string): void => {
  localStorage.setItem("auth_token", token);
  api.defaults.headers.common["Authorization"] = `Bearer ${token}`;

  // Notify app about auth change (same-tab)
  window.dispatchEvent(new Event("authTokenChanged"));
};

// Remove token and dispatch event for real-time UI updates
export const removeAuthToken = (): void => {
  localStorage.removeItem("auth_token");
  delete api.defaults.headers.common["Authorization"];

  // Notify app about auth change (same-tab)
  window.dispatchEvent(new Event("authTokenChanged"));
};

export const isAuthenticated = (): boolean => {
  return !!localStorage.getItem("auth_token");
};

export const getAuthToken = (): string | null => {
  return localStorage.getItem("auth_token");
};
