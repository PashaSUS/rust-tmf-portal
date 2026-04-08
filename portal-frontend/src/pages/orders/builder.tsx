import { useState, useCallback } from "react";
import { useNavigate } from "react-router-dom";
import { useMutation } from "@tanstack/react-query";
import { createTmfItem } from "@/lib/api";
import type { OrderDraft, OrderItem, PartyRef } from "./types";
import { emptyDraft } from "./types";
import { OrderItemEditor } from "./item-editor";
import { PartyEditor } from "./party-place-editor";
import { PageHeader } from "@/components/shared/page-header";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";
import { toast } from "sonner";
import { Send, FileText, Eye } from "lucide-react";

export default function OrderBuilderPage() {
  const navigate = useNavigate();
  const [draft, setDraft] = useState<OrderDraft>(emptyDraft);
  const [showPreview, setShowPreview] = useState(false);

  const update = useCallback(
    <K extends keyof OrderDraft>(key: K, val: OrderDraft[K]) =>
      setDraft((d) => ({ ...d, [key]: val })),
    [],
  );

  const mutation = useMutation({
    mutationFn: () => createTmfItem("tmf622", "productOrder", buildPayload(draft)),
    onSuccess: (data) => {
      const id = (data as Record<string, unknown>)?.id;
      toast.success("Order created" + (id ? ` (${id})` : ""));
      if (id) navigate(`/api/tmf622/productOrder/${id}`);
      else navigate("/api/tmf622");
    },
    onError: (err) => toast.error(`Failed: ${err.message}`),
  });

  const itemCount = draft.orderItem.length;
  const canSubmit = itemCount > 0 && draft.orderItem.every((i) => i.productOffering.id);

  return (
    <div className="space-y-6">
      <PageHeader
        title="Order Builder"
        description="TMF622 · Build a product order from catalog offerings"
        backTo="/api/tmf622"
      />

      {/* Order header fields */}
      <Card>
        <CardHeader>
          <CardTitle className="text-sm flex items-center gap-2">
            <FileText className="h-4 w-4" /> Order Details
          </CardTitle>
        </CardHeader>
        <CardContent className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
          <div className="space-y-1">
            <Label className="text-xs">External ID</Label>
            <Input className="h-8 text-sm" placeholder="Your reference…" value={draft.externalId} onChange={(e) => update("externalId", e.target.value)} />
          </div>
          <div className="space-y-1 sm:col-span-2 lg:col-span-2">
            <Label className="text-xs">Description</Label>
            <Textarea className="text-sm min-h-[2rem]" placeholder="Order description…" value={draft.description} onChange={(e) => update("description", e.target.value)} />
          </div>
          <div className="space-y-1">
            <Label className="text-xs">Priority</Label>
            <Select value={draft.priority} onValueChange={(v: string | null) => v && update("priority", v)}>
              <SelectTrigger className="h-8 text-sm">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {["0", "1", "2", "3", "4"].map((p) => (
                  <SelectItem key={p} value={p}>
                    {p} — {["Critical", "High", "Medium", "Low", "Planning"][Number(p)]}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          <div className="space-y-1">
            <Label className="text-xs">Requested Start</Label>
            <Input type="datetime-local" className="h-8 text-sm" value={draft.requestedStartDate} onChange={(e) => update("requestedStartDate", e.target.value)} />
          </div>
          <div className="space-y-1">
            <Label className="text-xs">Requested Completion</Label>
            <Input type="datetime-local" className="h-8 text-sm" value={draft.requestedCompletionDate} onChange={(e) => update("requestedCompletionDate", e.target.value)} />
          </div>
        </CardContent>
      </Card>

      {/* Related parties */}
      <PartyEditor parties={draft.relatedParty} onChange={(p: PartyRef[]) => update("relatedParty", p)} />

      {/* Order items */}
      <OrderItemEditor items={draft.orderItem} onChange={(items: OrderItem[]) => update("orderItem", items)} />

      {/* Actions */}
      <div className="flex items-center gap-3 sticky bottom-4 bg-background/80 backdrop-blur p-4 border rounded-lg">
        <Button onClick={() => mutation.mutate()} disabled={!canSubmit || mutation.isPending}>
          <Send className="h-4 w-4 mr-2" />
          {mutation.isPending ? "Submitting…" : "Submit Order"}
        </Button>
        <Button variant="outline" onClick={() => setShowPreview(!showPreview)}>
          <Eye className="h-4 w-4 mr-2" />
          {showPreview ? "Hide" : "Preview"} JSON
        </Button>
        <div className="flex-1" />
        <Badge variant={canSubmit ? "default" : "secondary"}>
          {itemCount} item{itemCount !== 1 ? "s" : ""}
        </Badge>
      </div>

      {showPreview && (
        <Card>
          <CardHeader>
            <CardTitle className="text-sm">Payload Preview</CardTitle>
          </CardHeader>
          <CardContent>
            <pre className="text-xs font-mono bg-muted p-3 rounded overflow-auto max-h-96">
              {JSON.stringify(buildPayload(draft), null, 2)}
            </pre>
          </CardContent>
        </Card>
      )}
    </div>
  );
}

function buildPayload(draft: OrderDraft): Record<string, unknown> {
  const payload: Record<string, unknown> = {
    "@type": "ProductOrder",
  };

  if (draft.externalId) payload.externalId = draft.externalId;
  if (draft.description) payload.description = draft.description;
  if (draft.priority) payload.priority = draft.priority;
  if (draft.requestedStartDate) payload.requestedStartDate = new Date(draft.requestedStartDate).toISOString();
  if (draft.requestedCompletionDate) payload.requestedCompletionDate = new Date(draft.requestedCompletionDate).toISOString();

  if (draft.relatedParty.length > 0) {
    payload.relatedParty = draft.relatedParty
      .filter((p) => p.id || p.name)
      .map((p) => ({
        id: p.id,
        name: p.name,
        role: p.role,
        "@referredType": "RelatedParty",
      }));
  }

  payload.orderItem = draft.orderItem.map((item, idx) => {
    const oi: Record<string, unknown> = {
      id: String(idx + 1),
      action: item.action,
      productOffering: {
        id: item.productOffering.id,
        name: item.productOffering.name,
      },
    };

    const product: Record<string, unknown> = {};
    if (item.product?.productSpecification?.id) {
      product.productSpecification = {
        id: item.product.productSpecification.id,
        name: item.product.productSpecification.name,
      };
    }
    const chars = (item.product?.productCharacteristic ?? []).filter((c) => c.name);
    if (chars.length > 0) {
      product.productCharacteristic = chars.map((c) => ({
        name: c.name,
        value: c.value,
        ...(c.valueType ? { valueType: c.valueType } : {}),
      }));
    }
    if (Object.keys(product).length > 0) oi.product = product;
    if (item.quantity && item.quantity > 1) oi.quantity = item.quantity;

    return oi;
  });

  return payload;
}
