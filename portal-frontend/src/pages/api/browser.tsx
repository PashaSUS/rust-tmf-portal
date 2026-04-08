import { useState, useMemo, useCallback } from "react";
import { useParams, useNavigate, Link } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { fetchTmfList } from "@/lib/api";
import { queryKeys } from "@/lib/query-keys";
import { useApiStatus } from "@/hooks/use-api-status";
import type { ResourceSchema, ColumnDef } from "@/lib/types";
import { PageHeader } from "@/components/shared/page-header";
import { DataTable, type Column } from "@/components/shared/data-table";
import { FilterPanel } from "@/components/shared/filter-panel";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { ErrorState, LoadingState } from "@/components/shared/loading-state";
import { ShoppingCart } from "lucide-react";

export default function ApiBrowserPage() {
  const { apiId } = useParams<{ apiId: string }>();
  const { data: status } = useApiStatus();
  const api = status?.apis.find((a) => a.id === apiId);

  if (!api) return <LoadingState message="Loading API info…" />;
  if (!api.configured) return <ErrorState message={`${apiId} is not configured`} />;

  const firstResource = api.resources[0] ?? "";

  return (
    <div className="space-y-6">
      <PageHeader
        title={api.name}
        description={`${api.id.toUpperCase()} · ${api.version} · ${api.domain}`}
      />
      {apiId === "tmf622" && (
        <Link to="/orders/new">
          <Button variant="outline" size="sm">
            <ShoppingCart className="h-4 w-4 mr-2" /> Order Builder
          </Button>
        </Link>
      )}
      {api.resources.length === 1 ? (
        <ResourceTable apiId={api.id} resource={firstResource} schema={api.resourceSchemas?.[firstResource]} />
      ) : (
        <Tabs defaultValue={firstResource}>
          <TabsList className="flex-wrap h-auto gap-1">
            {api.resources.map((r) => (
              <TabsTrigger key={r} value={r}>
                {api.resourceSchemas?.[r]?.pluralLabel ?? r}
              </TabsTrigger>
            ))}
          </TabsList>
          {api.resources.map((r) => (
            <TabsContent key={r} value={r}>
              <ResourceTable apiId={api.id} resource={r} schema={api.resourceSchemas?.[r]} />
            </TabsContent>
          ))}
        </Tabs>
      )}
    </div>
  );
}

function ResourceTable({ apiId, resource, schema }: { apiId: string; resource: string; schema?: ResourceSchema }) {
  const navigate = useNavigate();
  const [offset, setOffset] = useState(0);
  const [filters, setFilters] = useState<Record<string, string>>({});
  const [sortField, setSortField] = useState(schema?.defaultSort ?? "");
  const limit = 20;

  const activeFilters = useMemo(
    () => Object.fromEntries(Object.entries(filters).filter(([, v]) => v !== "")),
    [filters],
  );

  const queryParams = useMemo(
    () => ({
      limit,
      offset,
      ...(sortField ? { sort: sortField } : {}),
      ...activeFilters,
    }),
    [limit, offset, sortField, activeFilters],
  );

  const { data, isLoading, error } = useQuery({
    queryKey: queryKeys.tmf.list(apiId, resource, queryParams),
    queryFn: () => fetchTmfList(apiId, resource, queryParams),
  });

  const rows = data?.data ?? [];

  const columns: Column<Record<string, unknown>>[] = useMemo(
    () => (schema ? schemaColumns(schema.columns) : buildFallbackColumns(rows)),
    [schema, rows],
  );

  const handleFilterChange = useCallback((v: Record<string, string>) => {
    setFilters(v);
    setOffset(0);
  }, []);

  const sortableFields = useMemo(
    () => schema?.columns.filter((c) => c.sortable).map((c) => c.key) ?? [],
    [schema],
  );

  return (
    <div className="space-y-4">
      {schema && <FilterPanel filters={schema.filters} values={filters} onChange={handleFilterChange} />}
      {sortableFields.length > 0 && (
        <SortControls fields={sortableFields} schema={schema!} value={sortField} onChange={setSortField} />
      )}
      <DataTable
        columns={columns}
        data={rows as Record<string, unknown>[]}
        totalCount={data?.totalCount ?? 0}
        pageSize={limit}
        currentOffset={offset}
        onPageChange={setOffset}
        isLoading={isLoading}
        error={error}
        onRowClick={(row) => {
          if (row.id) navigate(`/api/${apiId}/${resource}/${row.id}`);
        }}
      />
    </div>
  );
}

function SortControls({ fields, schema, value, onChange }: { fields: string[]; schema: ResourceSchema; value: string; onChange: (v: string) => void }) {
  const colMap = useMemo(() => {
    const m = new Map<string, string>();
    for (const c of schema.columns) m.set(c.key, c.label);
    return m;
  }, [schema]);

  return (
    <div className="flex items-center gap-2 text-sm">
      <span className="text-muted-foreground">Sort:</span>
      {fields.map((f) => {
        const isActive = value === f || value === `-${f}`;
        const isDesc = value === `-${f}`;
        return (
          <button
            key={f}
            onClick={() => {
              if (value === f) onChange(`-${f}`);
              else if (value === `-${f}`) onChange("");
              else onChange(f);
            }}
            className={`px-2 py-0.5 rounded text-xs border ${isActive ? "bg-primary text-primary-foreground" : "hover:bg-muted"}`}
          >
            {colMap.get(f) ?? f} {isActive ? (isDesc ? "↓" : "↑") : ""}
          </button>
        );
      })}
    </div>
  );
}

function schemaColumns(defs: ColumnDef[]): Column<Record<string, unknown>>[] {
  return defs.map((def) => ({
    key: def.key,
    header: def.label,
    render: (row: Record<string, unknown>) => {
      const val = row[def.key];
      if (val === null || val === undefined) return "—";
      switch (def.colType) {
        case "badge":
          return <Badge variant="outline">{String(val)}</Badge>;
        case "boolean":
          return <Badge variant={val ? "default" : "secondary"}>{String(val)}</Badge>;
        case "date": {
          const s = String(val);
          return s.length > 10 ? s.slice(0, 10) : s;
        }
        default: {
          const s = String(val);
          return s.length > 60 ? s.slice(0, 57) + "…" : s;
        }
      }
    },
  }));
}

function buildFallbackColumns(rows: unknown[]): Column<Record<string, unknown>>[] {
  if (rows.length === 0) {
    return [
      { key: "id", header: "ID", render: (r) => String(r.id ?? "—") },
      { key: "name", header: "Name", render: (r) => String(r.name ?? "—") },
    ];
  }
  const sample = rows[0] as Record<string, unknown>;
  const preferred = ["id", "name", "description", "state", "status", "lifecycleStatus", "version", "lastUpdate"];
  const keys = preferred.filter((k) => k in sample);
  const others = Object.keys(sample).filter(
    (k) => !keys.includes(k) && !k.startsWith("@") && !k.startsWith("href") && typeof sample[k] !== "object",
  );
  const allKeys = [...keys, ...others].slice(0, 6);
  return allKeys.map((key) => ({
    key,
    header: key.replace(/([A-Z])/g, " $1").replace(/^./, (s) => s.toUpperCase()),
    render: (row: Record<string, unknown>) => {
      const val = row[key];
      if (val === null || val === undefined) return "—";
      if (typeof val === "boolean") return <Badge variant={val ? "default" : "secondary"}>{String(val)}</Badge>;
      const str = String(val);
      return str.length > 60 ? str.slice(0, 57) + "…" : str;
    },
  }));
}
