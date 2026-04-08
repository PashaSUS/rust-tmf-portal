import { Loader2, AlertCircle, Inbox } from "lucide-react";

export function LoadingState({ message = "Loading…" }: { message?: string }) {
  return (
    <div className="flex flex-col items-center justify-center py-16 text-muted-foreground">
      <Loader2 className="h-8 w-8 animate-spin mb-3" />
      <p className="text-sm">{message}</p>
    </div>
  );
}

export function ErrorState({ message = "Something went wrong" }: { message?: string }) {
  return (
    <div className="flex flex-col items-center justify-center py-16 text-destructive">
      <AlertCircle className="h-8 w-8 mb-3" />
      <p className="text-sm">{message}</p>
    </div>
  );
}

export function EmptyState({ message = "No data found" }: { message?: string }) {
  return (
    <div className="flex flex-col items-center justify-center py-16 text-muted-foreground">
      <Inbox className="h-8 w-8 mb-3" />
      <p className="text-sm">{message}</p>
    </div>
  );
}
