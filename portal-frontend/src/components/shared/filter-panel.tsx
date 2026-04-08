import { useState, useCallback } from "react";
import type { FilterDef } from "@/lib/types";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Checkbox } from "@/components/ui/checkbox";
import { Filter, X } from "lucide-react";

interface FilterPanelProps {
  filters: FilterDef[];
  values: Record<string, string>;
  onChange: (values: Record<string, string>) => void;
}

export function FilterPanel({ filters, values, onChange }: FilterPanelProps) {
  const [expanded, setExpanded] = useState(false);

  const activeCount = Object.values(values).filter((v) => v !== "").length;

  const update = useCallback(
    (key: string, val: string) => {
      onChange({ ...values, [key]: val });
    },
    [values, onChange],
  );

  const clear = useCallback(() => {
    onChange({});
  }, [onChange]);

  if (filters.length === 0) return null;

  return (
    <div className="space-y-3">
      <div className="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          onClick={() => setExpanded(!expanded)}
        >
          <Filter className="h-4 w-4 mr-1" />
          Filters
          {activeCount > 0 && (
            <span className="ml-1 bg-primary text-primary-foreground rounded-full px-1.5 text-xs">
              {activeCount}
            </span>
          )}
        </Button>
        {activeCount > 0 && (
          <Button variant="ghost" size="sm" onClick={clear}>
            <X className="h-4 w-4 mr-1" />
            Clear
          </Button>
        )}
      </div>
      {expanded && (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 p-4 border rounded-lg bg-muted/30">
          {filters.map((f) => (
            <FilterField key={f.key} def={f} value={values[f.key] ?? ""} onChange={(v) => update(f.key, v)} />
          ))}
        </div>
      )}
    </div>
  );
}

function FilterField({ def, value, onChange }: { def: FilterDef; value: string; onChange: (v: string) => void }) {
  switch (def.filterType) {
    case "select":
      return (
        <div className="space-y-1">
          <Label className="text-xs">{def.label}</Label>
          <Select value={value} onValueChange={(v: string | null) => onChange(!v || v === "__all__" ? "" : v)}>
            <SelectTrigger className="h-8 text-sm">
              <SelectValue placeholder={`All ${def.label}`} />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="__all__">All</SelectItem>
              {(def.options ?? []).map((opt) => (
                <SelectItem key={opt} value={opt}>{opt}</SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      );
    case "boolean":
      return (
        <div className="space-y-1">
          <Label className="text-xs">{def.label}</Label>
          <div className="flex items-center gap-2 h-8">
            <Checkbox
              checked={value === "true"}
              onCheckedChange={(checked) => onChange(checked ? "true" : "")}
            />
            <span className="text-sm">Yes</span>
          </div>
        </div>
      );
    case "dateRange":
      return (
        <div className="space-y-1">
          <Label className="text-xs">{def.label}</Label>
          <Input
            type="date"
            className="h-8 text-sm"
            value={value}
            onChange={(e) => onChange(e.target.value)}
          />
        </div>
      );
    case "number":
      return (
        <div className="space-y-1">
          <Label className="text-xs">{def.label}</Label>
          <Input
            type="number"
            className="h-8 text-sm"
            placeholder={def.placeholder ?? def.label}
            value={value}
            onChange={(e) => onChange(e.target.value)}
          />
        </div>
      );
    default:
      return (
        <div className="space-y-1">
          <Label className="text-xs">{def.label}</Label>
          <Input
            className="h-8 text-sm"
            placeholder={def.placeholder ?? `Filter by ${def.label.toLowerCase()}…`}
            value={value}
            onChange={(e) => onChange(e.target.value)}
          />
        </div>
      );
  }
}
