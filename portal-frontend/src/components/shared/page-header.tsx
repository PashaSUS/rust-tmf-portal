import type { ReactNode } from "react";
import { Button } from "@/components/ui/button";
import { useNavigate } from "react-router-dom";
import { ArrowLeft } from "lucide-react";

interface PageHeaderProps {
  title: string;
  description?: string;
  backTo?: string;
  actions?: ReactNode;
}

export function PageHeader({ title, description, backTo, actions }: PageHeaderProps) {
  const navigate = useNavigate();

  return (
    <div className="flex items-start justify-between mb-6">
      <div className="flex items-center gap-3">
        {backTo && (
          <Button variant="ghost" size="icon" onClick={() => navigate(backTo)}>
            <ArrowLeft className="h-4 w-4" />
          </Button>
        )}
        <div>
          <h1 className="text-2xl font-semibold tracking-tight">{title}</h1>
          {description && <p className="text-sm text-muted-foreground mt-1">{description}</p>}
        </div>
      </div>
      {actions && <div className="flex items-center gap-2">{actions}</div>}
    </div>
  );
}
