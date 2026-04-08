import type { ApiStatusResponse, ListResponse } from "./types";

declare global {
  interface Window {
    __RUNTIME_CONFIG?: { apiUrl?: string };
  }
}

const BASE = window.__RUNTIME_CONFIG?.apiUrl ?? "";

async function fetchJson<T>(url: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${url}`, {
    ...init,
    headers: { "Content-Type": "application/json", ...init?.headers },
  });
  if (!res.ok) {
    const body = await res.text();
    throw new Error(`API error ${res.status}: ${body}`);
  }
  return res.json();
}

function qs(params?: Record<string, string | number | undefined>): string {
  if (!params) return "";
  const entries = Object.entries(params).filter(([, v]) => v !== undefined && v !== "");
  if (entries.length === 0) return "";
  return "?" + new URLSearchParams(entries.map(([k, v]) => [k, String(v)])).toString();
}

// Admin
export const fetchApiStatus = () => fetchJson<ApiStatusResponse>("/api/admin/status");

// Generic TMF proxy functions
export const fetchTmfList = (apiId: string, resource: string, p?: Record<string, string | number | undefined>) =>
  fetchJson<ListResponse>(`/api/tmf/${apiId}/${resource}${qs(p)}`);

export const fetchTmfItem = (apiId: string, resource: string, id: string) =>
  fetchJson(`/api/tmf/${apiId}/${resource}/${id}`);

export const createTmfItem = (apiId: string, resource: string, body: unknown) =>
  fetchJson(`/api/tmf/${apiId}/${resource}`, { method: "POST", body: JSON.stringify(body) });

export const patchTmfItem = (apiId: string, resource: string, id: string, body: unknown) =>
  fetchJson(`/api/tmf/${apiId}/${resource}/${id}`, { method: "PATCH", body: JSON.stringify(body) });

export const deleteTmfItem = (apiId: string, resource: string, id: string) =>
  fetch(`${BASE}/api/tmf/${apiId}/${resource}/${id}`, { method: "DELETE" });
