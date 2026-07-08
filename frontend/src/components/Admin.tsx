/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import React, { useState, useEffect } from "react";
import {
  Admin,
  Resource,
  List,
  Datagrid,
  TextField,
  BooleanField,
  Edit,
  SimpleForm,
  TextInput,
  BooleanInput,
  SelectInput,
  DateInput,
  useRecordContext,
  Layout,
  AppBar,
} from "react-admin";
import { dataProviderFactory } from "../api/adminDataProvider";
import { getCurrentUser, removeAuthToken, getAuthToken } from "../api/authApi";
import AdminLogin from "./AdminLogin";
import { Link } from "react-router-dom";
import "./Admin.css";

// Material UI icons
import TaskIcon from "@mui/icons-material/Assignment";
import UserIcon from "@mui/icons-material/People";

// ============================================
// DASHBOARD
// ============================================

const Dashboard = () => (
  <div style={{ padding: "20px" }}>
    <h2>Admin Dashboard</h2>
    <p>Welcome to the admin panel. Here you can manage tasks and users.</p>
  </div>
);

// ============================================
// CUSTOM APP BAR WITH AUTO-HIDE
// ============================================

const CustomAppBar = (props: any) => {
  const [isVisible, setIsVisible] = useState(true);
  const [lastScrollY, setLastScrollY] = useState(0);

  // Auto-hide menu on scroll down, show on scroll up or at top
  useEffect(() => {
    const handleScroll = () => {
      const currentScrollY = window.scrollY;

      if (currentScrollY === 0) {
        setIsVisible(true);
      } else if (currentScrollY > lastScrollY) {
        setIsVisible(false);
      } else {
        setIsVisible(true);
      }

      setLastScrollY(currentScrollY);
    };

    window.addEventListener("scroll", handleScroll);
    return () => window.removeEventListener("scroll", handleScroll);
  }, [lastScrollY]);

  // Show menu on mouse hover over top area
  const handleMouseEnter = () => setIsVisible(true);
  const handleMouseLeave = () => {
    if (window.scrollY > 0) {
      setIsVisible(false);
    }
  };

  return (
    <div
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      style={{
        position: "sticky",
        top: 0,
        zIndex: 1200,
        transition: "transform 0.3s ease-in-out",
        transform: isVisible ? "translateY(0)" : "translateY(-100%)",
        marginTop: "-42px",
        paddingTop: "38px",
      }}
    >
      <AppBar
        {...props}
        userMenu={false}
        sx={{
          backgroundColor: "#1a1a2e",
          boxShadow: "none",
          minHeight: "40px",
          "& .MuiToolbar-root": {
            minHeight: "40px !important",
            padding: "0 12px",
          },
        }}
      >
        <div
          style={{
            display: "flex",
            alignItems: "center",
            width: "100%",
            justifyContent: "space-between",
            padding: "0 12px",
            height: "40px",
          }}
        >
          <div style={{ display: "flex", alignItems: "center", gap: "16px" }}>
            <span
              style={{
                color: "white",
                fontSize: "15px",
                fontWeight: "400",
                letterSpacing: "0.5px",
              }}
            >
              Admin Panel
            </span>
            <Link
              to="/"
              style={{
                color: "rgba(255,255,255,0.7)",
                textDecoration: "none",
                fontSize: "13px",
                transition: "color 0.2s",
              }}
              onMouseEnter={(e) => (e.currentTarget.style.color = "white")}
              onMouseLeave={(e) =>
                (e.currentTarget.style.color = "rgba(255,255,255,0.7)")
              }
            >
              Back to App
            </Link>
          </div>
          <button
            onClick={() => {
              localStorage.removeItem("auth_token");
              window.location.href = "/";
            }}
            style={{
              padding: "4px 14px",
              background: "rgba(220, 53, 69, 0.85)",
              color: "white",
              border: "none",
              borderRadius: "4px",
              cursor: "pointer",
              fontSize: "13px",
              transition: "background 0.2s",
            }}
            onMouseEnter={(e) => (e.currentTarget.style.background = "#dc3545")}
            onMouseLeave={(e) =>
              (e.currentTarget.style.background = "rgba(220, 53, 69, 0.85)")
            }
          >
            Logout
          </button>
        </div>
      </AppBar>
    </div>
  );
};

// ============================================
// CUSTOM LAYOUT
// ============================================

const CustomLayout = (props: any) => (
  <Layout {...props} appBar={CustomAppBar} />
);

// ============================================
// TASK LIST
// ============================================

const TaskList = (props: any) => (
  <List {...props} sort={{ field: "created_at", order: "DESC" }}>
    <Datagrid rowClick="edit">
      <TextField source="id" label="ID" />
      <TextField source="title" label="Title" />
      <BooleanField source="completed" label="Done" />
      <TextField source="priority" label="Priority" />
      <TextField source="due_date" label="Due Date" />
      <TextField source="user_id" label="User ID" />
    </Datagrid>
  </List>
);

// ============================================
// TASK EDIT
// ============================================

const TaskTitle = () => {
  const record = useRecordContext();
  return <span>Edit Task: {record ? record.title : ""}</span>;
};

const TaskEdit = (props: any) => (
  <Edit {...props} title={<TaskTitle />}>
    <SimpleForm>
      <TextInput source="title" label="Title" fullWidth />
      <BooleanInput source="completed" label="Completed" />
      <SelectInput
        source="priority"
        label="Priority"
        choices={[
          { id: "low", name: "Low" },
          { id: "medium", name: "Medium" },
          { id: "high", name: "High" },
        ]}
      />
      <DateInput source="due_date" label="Due Date" />
      <TextInput source="user_id" label="User ID" disabled fullWidth />
    </SimpleForm>
  </Edit>
);

// ============================================
// USER LIST
// ============================================

const UserList = (props: any) => (
  <List {...props}>
    <Datagrid>
      <TextField source="id" label="ID" />
      <TextField source="username" label="Username" />
      <TextField source="email" label="Email" />
      <TextField source="role" label="Role" />
    </Datagrid>
  </List>
);

// ============================================
// ADMIN PANEL
// ============================================

const AdminPanel: React.FC<{ onAdminLogin?: () => void }> = ({
  onAdminLogin,
}) => {
  const [isAdmin, setIsAdmin] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(true);
  const [dataProvider, setDataProvider] = useState<any>(null);

  useEffect(() => {
    const checkAdmin = async () => {
      const token = getAuthToken();
      console.log("Token:", token ? "YES" : "NO");

      if (!token) {
        setIsAdmin(false);
        setLoading(false);
        return;
      }

      try {
        const user = await getCurrentUser();
        console.log("User role:", user.role);

        if (user.role === "admin") {
          console.log("Creating dataProvider...");
          const provider = dataProviderFactory(
            "http://localhost:8000/api",
            token,
          );
          setDataProvider(provider);
          setIsAdmin(true);
          console.log("DataProvider created successfully");
        } else {
          console.log("User is not admin");
          setIsAdmin(false);
        }
      } catch (error) {
        console.error("Admin check failed:", error);
        removeAuthToken();
        setIsAdmin(false);
      } finally {
        setLoading(false);
      }
    };

    checkAdmin();
  }, []);

  const handleAdminLogin = () => {
    const token = getAuthToken();
    if (token) {
      const provider = dataProviderFactory("http://localhost:8000/api", token);
      setDataProvider(provider);
      setIsAdmin(true);
      if (onAdminLogin) {
        onAdminLogin();
      }
    }
  };

  const handleLogout = () => {
    removeAuthToken();
    setIsAdmin(false);
    setDataProvider(null);
    window.location.reload();
  };

  if (loading) {
    return <div className="admin-loading">Loading...</div>;
  }

  if (!isAdmin || !dataProvider) {
    return <AdminLogin onLogin={handleAdminLogin} />;
  }

  console.log("Rendering Admin component");

  return (
    <div className="admin-container">
      <Admin
        dataProvider={dataProvider}
        disableTelemetry
        dashboard={Dashboard}
        layout={CustomLayout}
        basename="/admin"
      >
        <Resource
          name="tasks"
          list={TaskList}
          edit={TaskEdit}
          icon={TaskIcon}
          recordRepresentation="title"
          options={{ label: "Tasks" }}
        />
        <Resource
          name="users"
          list={UserList}
          icon={UserIcon}
          recordRepresentation="username"
          options={{ label: "Users" }}
        />
      </Admin>
    </div>
  );
};

export default AdminPanel;
