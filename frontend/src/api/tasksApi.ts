/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import axios, { AxiosInstance, AxiosError } from "axios";
import {
  Task,
  CreateTaskDto,
  UpdateTaskDto,
  ApiResponse,
} from "@/types/task.types";

interface CsrfResponse {
  csrfToken: string;
}

interface ImportMetaEnv {
  VITE_API_URL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

const API_URL =
  (import.meta as unknown as ImportMeta).env?.VITE_API_URL ||
  "http://localhost:8000";

// EXPORTUJ api aby ho mohli používať ostatné súbory
export const api: AxiosInstance = axios.create({
  baseURL: API_URL,
  withCredentials: true,
  headers: {
    "Content-Type": "application/json",
    Accept: "application/json",
  },
});

export const getCsrfToken = async (): Promise<string> => {
  try {
    const response = await api.get<CsrfResponse>("/api/csrf");
    const token = response.data.csrfToken;
    api.defaults.headers.common["X-CSRF-Token"] = token;
    return token;
  } catch (error) {
    console.error("Failed to fetch CSRF token:", error);
    throw error;
  }
};

export const fetchTasks = async (): Promise<Task[]> => {
  const response = await api.get<Task[]>("/api/tasks");
  return response.data;
};

export const createTask = async (taskData: CreateTaskDto): Promise<Task> => {
  const response = await api.post<Task>("/api/tasks", taskData);
  return response.data;
};

export const updateTask = async (
  taskId: string,
  updates: UpdateTaskDto,
): Promise<Task> => {
  const response = await api.patch<Task>(`/api/tasks/${taskId}`, updates);
  return response.data;
};

export const deleteTask = async (taskId: string): Promise<void> => {
  await api.delete(`/api/tasks/${taskId}`);
};

export const getErrorMessage = (error: unknown): string => {
  if (axios.isAxiosError(error)) {
    const axiosError = error as AxiosError<ApiResponse>;
    return (
      axiosError.response?.data?.message ||
      axiosError.message ||
      "Network error"
    );
  }
  if (error instanceof Error) {
    return error.message;
  }
  return "Unknown error occurred";
};
