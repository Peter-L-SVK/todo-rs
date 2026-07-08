/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import { Card, CardContent, Typography, Grid } from "@mui/material";
import { useDataProvider } from "react-admin";
import { useEffect, useState } from "react";

export const AdminDashboard = () => {
  const dataProvider = useDataProvider();
  const [stats, setStats] = useState({
    totalTasks: 0,
    completedTasks: 0,
    totalUsers: 0,
    highPriority: 0,
  });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchStats = async () => {
      try {
        const [tasks, users] = await Promise.all([
          dataProvider.getList("tasks", {
            pagination: { page: 1, perPage: 1000 },
            sort: { field: "id", order: "ASC" },
            filter: {},
          }),
          dataProvider.getList("users", {
            pagination: { page: 1, perPage: 1000 },
            sort: { field: "id", order: "ASC" },
            filter: {},
          }),
        ]);

        const taskData = tasks.data || [];
        const completed = taskData.filter((t: any) => t.completed).length;
        const high = taskData.filter((t: any) => t.priority === "high").length;

        setStats({
          totalTasks: taskData.length,
          completedTasks: completed,
          totalUsers: (users.data || []).length,
          highPriority: high,
        });
      } catch (error) {
        console.error("Failed to fetch stats:", error);
      } finally {
        setLoading(false);
      }
    };

    fetchStats();
  }, [dataProvider]);

  if (loading) {
    return <Typography>Loading dashboard...</Typography>;
  }

  return (
    <Grid container spacing={3}>
      <Grid item xs={12} md={3}>
        <Card>
          <CardContent>
            <Typography color="textSecondary" gutterBottom>
              Total Tasks
            </Typography>
            <Typography variant="h4">{stats.totalTasks}</Typography>
          </CardContent>
        </Card>
      </Grid>
      <Grid item xs={12} md={3}>
        <Card>
          <CardContent>
            <Typography color="textSecondary" gutterBottom>
              Completed Tasks
            </Typography>
            <Typography variant="h4" style={{ color: "#4CAF50" }}>
              {stats.completedTasks}
            </Typography>
          </CardContent>
        </Card>
      </Grid>
      <Grid item xs={12} md={3}>
        <Card>
          <CardContent>
            <Typography color="textSecondary" gutterBottom>
              High Priority
            </Typography>
            <Typography variant="h4" style={{ color: "#ff4757" }}>
              {stats.highPriority}
            </Typography>
          </CardContent>
        </Card>
      </Grid>
      <Grid item xs={12} md={3}>
        <Card>
          <CardContent>
            <Typography color="textSecondary" gutterBottom>
              Total Users
            </Typography>
            <Typography variant="h4">{stats.totalUsers}</Typography>
          </CardContent>
        </Card>
      </Grid>
    </Grid>
  );
};
