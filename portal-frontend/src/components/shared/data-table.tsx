import { useState } from "react";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { ChevronLeft, ChevronRight, Search } from "lucide-react";
import { EmptyState, LoadingState, ErrorState } from "./loading-state";

export interface Column<T> {
  key: string;
  header: string;
  render: (row: T) => React.ReactNode;
  className?: string;
}

export interface DataTableProps<T> {
  columns: Column<T>[];
  data: T[];
  totalCount?: number;
  searchPlaceholder?: string;
  onSearch?: (q: string) => void;
  onPageChange?: (offset: number) => void;
  pageSize?: number;
  currentOffset?: number;
  onRowClick?: (row: T) => void;
  isLoading?: boolean;
  error?: Error | null;
}

export function DataTable<T>({
  columns, data, totalCount, searchPlaceholder = "Search…",
  onSearch, onPageChange, pageSize = 10, currentOffset = 0, onRowClick,
  isLoading, error,
}: DataTableProps<T>) {
  const [search, setSearch] = useState("");

  const handleSearch = (val: string) => {
    setSearch(val);
    onSearch?.(val);
  };

  if (isLoading) return <LoadingState />;
  if (error) return <ErrorState message={error.message} />;

  const page = Math.floor(currentOffset / pageSize) + 1;
  const totalPages = totalCount ? Math.ceil(totalCount / pageSize) : undefined;

  return (
    <div className="space-y-4">
      {onSearch && (
        <div className="relative max-w-sm">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder={searchPlaceholder}
            value={search}
            onChange={(e) => handleSearch(e.target.value)}
            className="pl-9"
          />
        </div>
      )}

      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              {columns.map((col) => (
                <TableHead key={col.key} className={col.className}>{col.header}</TableHead>
              ))}
            </TableRow>
          </TableHeader>
          <TableBody>
            {data.length === 0 ? (
              <TableRow>
                <TableCell colSpan={columns.length}>
                  <EmptyState />
                </TableCell>
              </TableRow>
            ) : (
              data.map((row, i) => (
                <TableRow
                  key={i}
                  className={onRowClick ? "cursor-pointer hover:bg-muted/50" : ""}
                  onClick={() => onRowClick?.(row)}
                >
                  {columns.map((col) => (
                    <TableCell key={col.key} className={col.className}>
                      {col.render(row)}
                    </TableCell>
                  ))}
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </div>

      {onPageChange && (
        <div className="flex items-center justify-between">
          <p className="text-sm text-muted-foreground">
            {totalCount !== undefined ? `${totalCount} total` : `${data.length} items`}
          </p>
          <div className="flex items-center gap-2">
            <Button
              variant="outline" size="sm"
              disabled={currentOffset === 0}
              onClick={() => onPageChange(Math.max(0, currentOffset - pageSize))}
            >
              <ChevronLeft className="h-4 w-4" />
            </Button>
            <span className="text-sm">
              Page {page}{totalPages ? ` of ${totalPages}` : ""}
            </span>
            <Button
              variant="outline" size="sm"
              disabled={totalPages ? page >= totalPages : data.length < pageSize}
              onClick={() => onPageChange(currentOffset + pageSize)}
            >
              <ChevronRight className="h-4 w-4" />
            </Button>
          </div>
        </div>
      )}
    </div>
  );
}
