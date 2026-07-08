export interface Task {
  id: string;
  title: string;
  completed: boolean;
  created_at: string;
  priority?: "low" | "medium" | "high";
  due_date?: string | null;
}

export interface CreateTaskDto {
  title: string;
  priority?: "low" | "medium" | "high";
  due_date?: string | null;
}

export interface UpdateTaskDto {
  title?: string;
  completed?: boolean;
  priority?: "low" | "medium" | "high";
  due_date?: string | null;
}

// Použi unknown namiesto any
export interface ApiResponse<T = unknown> {
  status: "success" | "error";
  data?: T;
  message?: string;
}

export type FilterType = "all" | "active" | "completed";
