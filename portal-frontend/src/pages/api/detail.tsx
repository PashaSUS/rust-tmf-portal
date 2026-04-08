import { useParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import { fetchTmfItem } from "@/lib/api";
import { queryKeys } from "@/lib/query-keys";
import { PageHeader } from "@/components/shared/page-header";
import { LoadingState, ErrorState } from "@/components/shared/loading-state";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";

export default function ResourceDetailPage() {
  const { apiId, resource, id } = useParams<{ apiId: string; resource: string; id: string }>();

  const { data, isLoading, error } = useQuery({
    queryKey: queryKeys.tmf.detail(apiId!, resource!, id!),
    queryFn: () => fetchTmfItem(apiId!, resource!, id!),
    enabled: !!apiId && !!resource && !!id,
  });

  if (isLoading) return <LoadingState />;
  if (error) return <ErrorState message={error.message} />;
  if (!data) return <ErrorState message="Resource not found" />;

  const record = data as Record<string, unknown>;

  const scalars: [string, unknown][] = [];
  const objects: [string, unknown][] = [];
  for (const [key, val] of Object.entries(record)) {
    if (key.startsWith("@") || key === "href") continue;
    if (val !== null && typeof val === "object") {
      objects.push([key, val]);
    } else {
      scalars.push([key, val]);
    }
  }

  return (
    <div className="space-y-6">
      <PageHeader
        title={String(record.name ?? record.id ?? resource)}
        description={`${apiId?.toUpperCase()} / ${resource} / ${id}`}
        backTo={`/api/${apiId}`}
      />
      <Card>
        <CardHeader><CardTitle className="text-base">Properties</CardTitle></CardHeader>
        <CardContent className="space-y-2 text-sm">
          {scalars.map(([key, val]) => (
            <div key={key} className="flex gap-2">
              <span className="text-muted-foreground w-48 shrink-0">{key}:</span>
              <span>{val === null || val === undefined ? "—" : String(val)}</span>
            </div>
          ))}
        </CardContent>
      </Card>
      {objects.map(([key, val]) => (
        <Card key={key}>
          <CardHeader>
            <CardTitle className="text-base flex items-center gap-2">
              {key}
              {Array.isArray(val) && <Badge variant="outline">{(val as unknown[]).length} items</Badge>}
            </CardTitle>
          </CardHeader>
          <CardContent>
            <pre className="text-xs font-mono bg-muted p-3 rounded overflow-auto max-h-80">
              {JSON.stringify(val, null, 2)}
            </pre>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
