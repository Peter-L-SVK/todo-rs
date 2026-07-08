/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import {
  List,
  Datagrid,
  TextField,
  BooleanField,
  EditButton,
  DeleteButton,
  Filter,
  TextInput,
  SelectInput,
  DateField,
  useRecordContext,
} from "react-admin";

// Priority badge component (shared)
const PriorityField = ({ source }: { source: string }) => {
  const record = useRecordContext();
  if (!record) return null;

  const priority = record[source];
  const getColor = (p: string) => {
    switch (p) {
      case "high":
        return "#ff4757";
      case "medium":
        return "#ffa502";
      case "low":
        return "#2ed573";
      default:
        return "#747d8c";
    }
  };

  return (
    <span
      style={{
        backgroundColor: getColor(priority),
        color: "white",
        padding: "2px 10px",
        borderRadius: "12px",
        fontSize: "11px",
        fontWeight: "bold",
        textTransform: "uppercase",
        display: "inline-block",
      }}
    >
      {priority || "none"}
    </span>
  );
};

// Admin filters - extra fields for admin
const TaskFilter = (props: any) => (
  <Filter {...props}>
    <TextInput label="Search by title" source="title" alwaysOn />
    <TextInput label="User ID" source="user_id" />
    <SelectInput
      label="Priority"
      source="priority"
      choices={[
        { id: "low", name: "Low" },
        { id: "medium", name: "Medium" },
        { id: "high", name: "High" },
      ]}
    />
    <SelectInput
      label="Status"
      source="completed"
      choices={[
        { id: true, name: "Completed" },
        { id: false, name: "Active" },
      ]}
    />
  </Filter>
);

export const AdminTaskList = (props: any) => (
  <List
    {...props}
    filters={<TaskFilter />}
    perPage={25}
    sort={{ field: "created_at", order: "DESC" }}
  >
    <Datagrid rowClick="edit" bulkActionButtons>
      <TextField source="id" label="ID" />
      <TextField source="title" label="Title" />
      <BooleanField source="completed" label="Done" />
      <PriorityField source="priority" label="Priority" />
      <DateField source="due_date" label="Due Date" />
      <DateField source="created_at" label="Created" showTime />
      <TextField source="user_id" label="User ID" />
      <EditButton />
      <DeleteButton />
    </Datagrid>
  </List>
);
