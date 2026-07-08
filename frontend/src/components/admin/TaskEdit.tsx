/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import {
  Edit,
  SimpleForm,
  TextInput,
  BooleanInput,
  SelectInput,
  DateInput,
  useRecordContext,
} from "react-admin";

const TitlePreview = () => {
  const record = useRecordContext();
  return <span>Edit Task: {record ? record.title : ""}</span>;
};

export const AdminTaskEdit = (props: any) => (
  <Edit {...props} title={<TitlePreview />}>
    <SimpleForm>
      <TextInput source="title" label="Title" fullWidth />
      <BooleanInput source="completed" label="Completed" />
      <SelectInput
        source="priority"
        label="Priority"
        choices={[
          { id: "low", name: "🟢 Low" },
          { id: "medium", name: "🟡 Medium" },
          { id: "high", name: "🔴 High" },
        ]}
      />
      <DateInput source="due_date" label="Due Date" />
      <TextInput source="user_id" label="User ID" disabled fullWidth />
    </SimpleForm>
  </Edit>
);
