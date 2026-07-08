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
  DateField,
  Filter,
  TextInput,
  useRecordContext,
} from "react-admin";

const RoleField = () => {
  const record = useRecordContext();
  if (!record) return null;

  const isAdmin = record.role === "admin";
  return (
    <span
      style={{
        backgroundColor: isAdmin ? "#4CAF50" : "#747d8c",
        color: "white",
        padding: "2px 12px",
        borderRadius: "12px",
        fontSize: "11px",
        fontWeight: "bold",
        textTransform: "uppercase",
        display: "inline-block",
      }}
    >
      {record.role || "user"}
    </span>
  );
};

const UserFilter = (props: any) => (
  <Filter {...props}>
    <TextInput label="Search by username" source="username" alwaysOn />
    <TextInput label="Search by email" source="email" />
  </Filter>
);

export const AdminUserList = (props: any) => (
  <List {...props} filters={<UserFilter />} perPage={25}>
    <Datagrid bulkActionButtons={false}>
      <TextField source="id" label="ID" />
      <TextField source="username" label="Username" />
      <TextField source="email" label="Email" />
      <RoleField source="role" label="Role" />
      <DateField source="created_at" label="Joined" showTime />
    </Datagrid>
  </List>
);
