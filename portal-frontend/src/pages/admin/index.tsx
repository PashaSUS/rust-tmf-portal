import { useApiStatus } from "@/hooks/use-api-status";
import { PageHeader } from "@/components/shared/page-header";
import { LoadingState, ErrorState } from "@/components/shared/loading-state";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { CheckCircle2, XCircle, Settings2 } from "lucide-react";

export default function AdminPage() {
  const { data, isLoading, error } = useApiStatus();

  if (isLoading) return <LoadingState />;
  if (error) return <ErrorState message={error.message} />;

  const apis = data?.apis ?? [];
  const available = apis.filter((a) => a.available).length;
  const configured = apis.filter((a) => a.configured).length;

  return (
    <div className="space-y-6">
      <PageHeader title="API Status" description="Overview of all TMF API connectivity" />
      <div className="grid grid-cols-3 gap-4">
        <StatCard label="Total APIs" value={apis.length} />
        <StatCard label="Configured" value={configured} />
        <StatCard label="Available" value={available} variant={available > 0 ? "success" : "muted"} />
      </div>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {apis.filter((a) => a.configured).map((api) => (
          <Card key={api.id}>
            <CardHeader className="pb-2">
              <div className="flex items-center justify-between">
                <CardTitle className="text-sm font-medium">{api.name}</CardTitle>
                <Badge variant={api.available ? "default" : "secondary"}>
                  {api.id.toUpperCase()}
                </Badge>
              </div>
            </CardHeader>
            <CardContent>
              <div className="flex items-center gap-2 text-sm">
                {api.available ? (
                  <><CheckCircle2 className="h-4 w-4 text-green-500" /><span>Connected</span></>
                ) : (
                  <><XCircle className="h-4 w-4 text-yellow-500" /><span>Unreachable</span></>
                )}
              </div>
              <div className="flex items-center gap-1.5 mt-1.5 flex-wrap">
                <Badge variant="outline" className="text-xs">{api.version}</Badge>
                <Badge variant="outline" className="text-xs">{api.domain}</Badge>
                <Badge variant="outline" className="text-xs">{api.resources.length} resources</Badge>
              </div>
              {api.baseUrl && (
                <p className="text-xs text-muted-foreground mt-1 truncate">{api.baseUrl}</p>
              )}
            </CardContent>
          </Card>
        ))}
      </div>
      {apis.some((a) => !a.configured) && (
        <>
          <h2 className="text-lg font-semibold mt-8">Unconfigured APIs ({apis.filter((a) => !a.configured).length})</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-3">
            {apis.filter((a) => !a.configured).map((api) => (
              <Card key={api.id} className="opacity-60">
                <CardContent className="pt-4">
                  <div className="flex items-center gap-2 text-sm">
                    <Settings2 className="h-4 w-4 text-muted-foreground" />
                    <span className="font-mono text-xs">{api.id.toUpperCase()}</span>
                    <span className="truncate">{api.name}</span>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        </>
      )}
    </div>
  );
}

function StatCard({ label, value, variant = "default" }: { label: string; value: number; variant?: string }) {
  return (
    <Card>
      <CardContent className="pt-6">
        <p className="text-sm text-muted-foreground">{label}</p>
        <p className={`text-3xl font-bold ${variant === "success" ? "text-green-500" : ""}`}>{value}</p>
      </CardContent>
    </Card>
  );
}
