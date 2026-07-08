import { useList } from "react-admin";
import { useEffect } from "react";

export const TestComponent = () => {
  const { data, isLoading, error } = useList("tasks");

  useEffect(() => {
    console.log("🔍 useList data:", data);
    console.log("🔍 useList loading:", isLoading);
    console.log("🔍 useList error:", error);
  }, [data, isLoading, error]);

  return null;
};
