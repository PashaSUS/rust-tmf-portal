import { useQuery } from "@tanstack/react-query";
import { fetchApiStatus } from "@/lib/api";
import { queryKeys } from "@/lib/query-keys";
import type { ApiStatusEntry } from "@/lib/types";
import { useMemo } from "react";

export function useApiStatus() {
  return useQuery({
    queryKey: queryKeys.admin.status,
    queryFn: fetchApiStatus,
    staleTime: 30_000,
    refetchInterval: 60_000,
  });
}

export function useConfiguredApis(): ApiStatusEntry[] {
  const { data } = useApiStatus();
  return data?.apis.filter((a) => a.configured) ?? [];
}

export function useAvailableApis(): ApiStatusEntry[] {
  const { data } = useApiStatus();
  return data?.apis.filter((a) => a.configured && a.available) ?? [];
}

export function useIsApiAvailable(apiId: string): boolean {
  const apis = useAvailableApis();
  return apis.some((a) => a.id === apiId);
}

export interface NavSection {
  domain: string;
  apis: ApiStatusEntry[];
}

export function useNavSections(): NavSection[] {
  const configured = useConfiguredApis();
  return useMemo(() => {
    const domainMap = new Map<string, ApiStatusEntry[]>();
    for (const api of configured) {
      const list = domainMap.get(api.domain) ?? [];
      list.push(api);
      domainMap.set(api.domain, list);
    }
    const sections: NavSection[] = [];
    for (const [domain, apis] of domainMap) {
      apis.sort((a, b) => a.id.localeCompare(b.id));
      sections.push({ domain, apis });
    }
    sections.sort((a, b) => a.domain.localeCompare(b.domain));
    return sections;
  }, [configured]);
}
