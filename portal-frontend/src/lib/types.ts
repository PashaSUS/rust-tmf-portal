// Schema types (from tmf-client)

export type ColumnType = "string" | "number" | "date" | "boolean" | "badge";
export type FilterType = "text" | "select" | "dateRange" | "boolean" | "number";

export interface ColumnDef {
  key: string;
  label: string;
  colType: ColumnType;
  sortable: boolean;
}

export interface FilterDef {
  key: string;
  label: string;
  filterType: FilterType;
  options?: string[];
  placeholder?: string;
}

export interface ResourceSchema {
  name: string;
  pluralLabel: string;
  columns: ColumnDef[];
  filters: FilterDef[];
  defaultSort?: string;
}

// API types

export interface ApiStatusEntry {
  id: string;
  name: string;
  domain: string;
  basePath: string;
  version: string;
  versions: string[];
  resources: string[];
  configured: boolean;
  available: boolean;
  baseUrl?: string;
  resourceSchemas: Record<string, ResourceSchema>;
}

export interface ApiStatusResponse {
  apis: ApiStatusEntry[];
}

export interface ListResponse<T = unknown> {
  data: T[];
  totalCount?: number;
}
