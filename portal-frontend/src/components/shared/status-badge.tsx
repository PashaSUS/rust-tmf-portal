import { Badge } from "@/components/ui/badge";

const statusColors: Record<string, string> = {
  active: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
  launched: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
  completed: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
  approved: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
  inProgress: "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
  acknowledged: "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
  pending: "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
  held: "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
  partial: "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200",
  cancelled: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
  failed: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
  rejected: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
  retired: "bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200",
  obsolete: "bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200",
};

export function StatusBadge({ status }: { status?: string }) {
  if (!status) return <Badge variant="outline">Unknown</Badge>;

  const key = status.toLowerCase().replace(/[\s_-]/g, "");
  const colorClass = Object.entries(statusColors).find(([k]) => key.includes(k))?.[1]
    ?? "bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200";

  return <Badge className={colorClass}>{status}</Badge>;
}
