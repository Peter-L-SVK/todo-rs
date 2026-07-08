/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import React from "react";
import { FilterType } from "@/types/task.types";

interface TaskFiltersProps {
  currentFilter: FilterType;
  onFilterChange: (filter: FilterType) => void;
  tasksCount: number;
  activeCount: number;
}

const TaskFilters: React.FC<TaskFiltersProps> = ({
  currentFilter,
  onFilterChange,
  tasksCount,
  activeCount,
}) => {
  const filters: { value: FilterType; label: string }[] = [
    { value: "all", label: `All (${tasksCount})` },
    { value: "active", label: `Active (${activeCount})` },
    { value: "completed", label: `Completed (${tasksCount - activeCount})` },
  ];

  return (
    <div className="task-filters">
      {filters.map(({ value, label }) => (
        <button
          key={value}
          className={`filter-btn ${currentFilter === value ? "active" : ""}`}
          onClick={() => onFilterChange(value)}
        >
          {label}
        </button>
      ))}
    </div>
  );
};

export default TaskFilters;
