export const queryKeys = {
  admin: {
    status: ["admin", "status"] as const,
  },
  tmf: {
    list: (apiId: string, resource: string, params?: Record<string, unknown>) =>
      ["tmf", apiId, resource, "list", params] as const,
    detail: (apiId: string, resource: string, id: string) =>
      ["tmf", apiId, resource, id] as const,
  },
} as const;
