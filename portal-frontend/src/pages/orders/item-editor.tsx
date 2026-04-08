import { useState, useCallback } from "react";
import { useQuery } from "@tanstack/react-query";
import { fetchTmfList } from "@/lib/api";
import { queryKeys } from "@/lib/query-keys";
import type { Characteristic, OrderItem } from "./types";
import { emptyOrderItem } from "./types";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { LoadingState } from "@/components/shared/loading-state";
import { Plus, Search, Trash2, Package } from "lucide-react";

interface Props {
  items: OrderItem[];
  onChange: (items: OrderItem[]) => void;
}

export function OrderItemEditor({ items, onChange }: Props) {
  const addItem = useCallback(() => {
    onChange([...items, emptyOrderItem()]);
  }, [items, onChange]);

  const updateItem = useCallback(
    (idx: number, patch: Partial<OrderItem>) => {
      const next = items.map((it, i) => (i === idx ? { ...it, ...patch } : it));
      onChange(next);
    },
    [items, onChange],
  );

  const removeItem = useCallback(
    (idx: number) => onChange(items.filter((_, i) => i !== idx)),
    [items, onChange],
  );

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h3 className="font-semibold text-sm">Order Items</h3>
        <Button size="sm" variant="outline" onClick={addItem}>
          <Plus className="h-4 w-4 mr-1" /> Add Item
        </Button>
      </div>
      {items.length === 0 && (
        <Card>
          <CardContent className="py-8 text-center text-muted-foreground text-sm">
            No items yet. Click "Add Item" to start building the order.
          </CardContent>
        </Card>
      )}
      {items.map((item, idx) => (
        <OrderItemCard
          key={item.id}
          item={item}
          index={idx}
          onUpdate={(patch) => updateItem(idx, patch)}
          onRemove={() => removeItem(idx)}
        />
      ))}
    </div>
  );
}

function OrderItemCard({
  item, index, onUpdate, onRemove,
}: {
  item: OrderItem; index: number;
  onUpdate: (patch: Partial<OrderItem>) => void;
  onRemove: () => void;
}) {
  return (
    <Card>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="text-sm flex items-center gap-2">
            <Package className="h-4 w-4" />
            Item #{index + 1}
            {item.productOffering.name && (
              <Badge variant="outline">{item.productOffering.name}</Badge>
            )}
          </CardTitle>
          <div className="flex items-center gap-2">
            <Select
              value={item.action}
              onValueChange={(v: string | null) => v && onUpdate({ action: v as OrderItem["action"] })}
            >
              <SelectTrigger className="h-7 text-xs w-28">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="add">Add</SelectItem>
                <SelectItem value="modify">Modify</SelectItem>
                <SelectItem value="delete">Delete</SelectItem>
                <SelectItem value="noChange">No Change</SelectItem>
              </SelectContent>
            </Select>
            <Button size="icon" variant="ghost" onClick={onRemove} className="h-7 w-7">
              <Trash2 className="h-4 w-4 text-destructive" />
            </Button>
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        <OfferingPicker item={item} onUpdate={onUpdate} />
        <CharacteristicEditor
          chars={item.product?.productCharacteristic ?? []}
          onChange={(chars) =>
            onUpdate({ product: { ...item.product, productCharacteristic: chars } })
          }
        />
      </CardContent>
    </Card>
  );
}

function OfferingPicker({ item, onUpdate }: { item: OrderItem; onUpdate: (p: Partial<OrderItem>) => void }) {
  const [search, setSearch] = useState("");
  const [open, setOpen] = useState(!item.productOffering.id);

  const { data, isLoading } = useQuery({
    queryKey: queryKeys.tmf.list("tmf620", "productOffering", { limit: 20, name: search || undefined }),
    queryFn: () => fetchTmfList("tmf620", "productOffering", { limit: 20, ...(search ? { "name.contains": search } : {}) }),
    enabled: open,
  });

  const offerings = (data?.data ?? []) as Record<string, unknown>[];

  if (!open && item.productOffering.id) {
    return (
      <div className="flex items-center gap-2 text-sm">
        <span className="text-muted-foreground">Offering:</span>
        <Badge>{item.productOffering.name}</Badge>
        <span className="text-xs text-muted-foreground">({item.productOffering.id})</span>
        <Button size="sm" variant="ghost" className="h-6 text-xs" onClick={() => setOpen(true)}>
          Change
        </Button>
      </div>
    );
  }

  return (
    <div className="space-y-2">
      <Label className="text-xs">Select Product Offering (TMF620)</Label>
      <div className="relative">
        <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
        <Input
          className="pl-8 h-8 text-sm"
          placeholder="Search offerings by name…"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </div>
      {isLoading && <LoadingState message="Searching offerings…" />}
      {!isLoading && offerings.length === 0 && (
        <p className="text-xs text-muted-foreground">No offerings found</p>
      )}
      <div className="max-h-48 overflow-y-auto space-y-1">
        {offerings.map((o) => (
          <button
            key={String(o.id)}
            className="w-full text-left px-3 py-2 rounded hover:bg-muted text-sm flex justify-between items-center"
            onClick={() => {
              onUpdate({
                productOffering: { id: String(o.id), name: String(o.name ?? o.id) },
                product: {
                  ...item.product,
                  productSpecification: o.productSpecification
                    ? { id: String((o.productSpecification as Record<string, unknown>).id ?? ""), name: String((o.productSpecification as Record<string, unknown>).name ?? "") }
                    : undefined,
                },
              });
              setOpen(false);
            }}
          >
            <div>
              <span className="font-medium">{String(o.name ?? "Unnamed")}</span>
              <span className="text-xs text-muted-foreground ml-2">{String(o.id)}</span>
            </div>
            {typeof o.lifecycleStatus === "string" && <Badge variant="outline" className="text-xs">{o.lifecycleStatus}</Badge>}
          </button>
        ))}
      </div>
    </div>
  );
}

function CharacteristicEditor({
  chars, onChange,
}: {
  chars: Characteristic[];
  onChange: (c: Characteristic[]) => void;
}) {
  const addChar = () => onChange([...chars, { name: "", value: "" }]);
  const updateChar = (idx: number, patch: Partial<Characteristic>) =>
    onChange(chars.map((c, i) => (i === idx ? { ...c, ...patch } : c)));
  const removeChar = (idx: number) => onChange(chars.filter((_, i) => i !== idx));

  return (
    <div className="space-y-2">
      <div className="flex items-center justify-between">
        <Label className="text-xs">Characteristics</Label>
        <Button size="sm" variant="ghost" className="h-6 text-xs" onClick={addChar}>
          <Plus className="h-3 w-3 mr-1" /> Add
        </Button>
      </div>
      {chars.length === 0 && (
        <p className="text-xs text-muted-foreground">No characteristics configured</p>
      )}
      {chars.map((ch, idx) => (
        <div key={idx} className="flex gap-2 items-center">
          <Input
            className="h-7 text-xs flex-1"
            placeholder="Name"
            value={ch.name}
            onChange={(e) => updateChar(idx, { name: e.target.value })}
          />
          <Input
            className="h-7 text-xs flex-1"
            placeholder="Value"
            value={ch.value}
            onChange={(e) => updateChar(idx, { value: e.target.value })}
          />
          <Button size="icon" variant="ghost" className="h-7 w-7" onClick={() => removeChar(idx)}>
            <Trash2 className="h-3 w-3" />
          </Button>
        </div>
      ))}
    </div>
  );
}
