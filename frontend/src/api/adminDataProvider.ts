/*
 * To Do WebApp
 * Copyright (c) 2026 Peter Leukanič
 * Under MIT License
 * Feel free to share and modify
 *
 */

import { fetchUtils, DataProvider } from "react-admin";

export const dataProviderFactory = (
  apiUrl: string,
  token?: string,
): DataProvider => {
  const httpClient = (url: string, options: any = {}) => {
    if (!options.headers) {
      options.headers = new Headers({ "Content-Type": "application/json" });
    }

    if (token) {
      options.headers.set("Authorization", `Bearer ${token}`);
    }

    const csrfToken = localStorage.getItem("csrf_token");
    if (csrfToken) {
      options.headers.set("X-CSRF-Token", csrfToken);
    }

    console.log("🔍 Admin HTTP request:", url);
    return fetchUtils.fetchJson(url, options);
  };

  return {
    getList: async (resource: string, params: any) => {
      console.log("🔍 getList called for resource:", resource);

      // Use admin endpoints for tasks when in admin panel
      let url;
      if (resource === "tasks") {
        url = `${apiUrl}/admin/tasks`; // All tasks for admin
      } else {
        url = `${apiUrl}/${resource}`;
      }

      // Add filters if present
      if (params.filter && Object.keys(params.filter).length > 0) {
        const searchParams = new URLSearchParams();
        Object.entries(params.filter).forEach(([key, value]) => {
          if (value) searchParams.append(key, String(value));
        });
        if (searchParams.toString()) {
          url += `?${searchParams.toString()}`;
        }
      }

      console.log("🔍 Fetching:", url);

      try {
        const { json } = await httpClient(url);
        console.log("🔍 Response:", json);

        if (Array.isArray(json)) {
          return {
            data: json,
            total: json.length,
          };
        }

        if (json && typeof json === "object" && "data" in json) {
          return json;
        }

        return {
          data: [],
          total: 0,
        };
      } catch (error) {
        console.error("🔍 getList error:", error);
        return {
          data: [],
          total: 0,
        };
      }
    },

    getOne: async (resource: string, params: any) => {
      const url = `${apiUrl}/${resource}/${params.id}`;
      const { json } = await httpClient(url);
      return { data: json };
    },

    getMany: async (resource: string, params: any) => {
      const url = `${apiUrl}/${resource}`;
      const { json } = await httpClient(url);
      const data = Array.isArray(json) ? json : [];
      return {
        data: data.filter((item: any) => params.ids.includes(item.id)),
      };
    },

    getManyReference: async (resource: string, params: any) => {
      const url = `${apiUrl}/${resource}`;
      const { json } = await httpClient(url);
      const data = Array.isArray(json) ? json : [];
      return {
        data: data,
        total: data.length,
      };
    },

    create: async (resource: string, params: any) => {
      const url = `${apiUrl}/${resource}`;
      const { json } = await httpClient(url, {
        method: "POST",
        body: JSON.stringify(params.data),
      });
      return { data: json };
    },

    update: async (resource: string, params: any) => {
      const url = `${apiUrl}/${resource}/${params.id}`;
      const { json } = await httpClient(url, {
        method: "PATCH",
        body: JSON.stringify(params.data),
      });
      return { data: json };
    },

    updateMany: async (resource: string, params: any) => {
      const promises = params.ids.map((id: string) => {
        const url = `${apiUrl}/${resource}/${id}`;
        return httpClient(url, {
          method: "PATCH",
          body: JSON.stringify(params.data),
        });
      });
      const results = await Promise.all(promises);
      return { data: results.map((r: any) => r.json) };
    },

    delete: async (resource: string, params: any) => {
      const url = `${apiUrl}/${resource}/${params.id}`;
      const { json } = await httpClient(url, {
        method: "DELETE",
      });
      return { data: json };
    },

    deleteMany: async (resource: string, params: any) => {
      const promises = params.ids.map((id: string) => {
        const url = `${apiUrl}/${resource}/${id}`;
        return httpClient(url, {
          method: "DELETE",
        });
      });
      await Promise.all(promises);
      return { data: params.ids };
    },
  };
};
