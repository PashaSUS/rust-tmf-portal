import { useCallback } from "react";
import type { PartyRef, PlaceRef } from "./types";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Plus, Trash2, Users, MapPin } from "lucide-react";

interface PartyEditorProps {
  parties: PartyRef[];
  onChange: (p: PartyRef[]) => void;
}

export function PartyEditor({ parties, onChange }: PartyEditorProps) {
  const add = useCallback(() => {
    onChange([...parties, { id: "", name: "", role: "Buyer" }]);
  }, [parties, onChange]);

  const update = useCallback(
    (idx: number, patch: Partial<PartyRef>) =>
      onChange(parties.map((p, i) => (i === idx ? { ...p, ...patch } : p))),
    [parties, onChange],
  );

  const remove = useCallback(
    (idx: number) => onChange(parties.filter((_, i) => i !== idx)),
    [parties, onChange],
  );

  return (
    <Card>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="text-sm flex items-center gap-2">
            <Users className="h-4 w-4" /> Related Parties
            {parties.length > 0 && <Badge variant="outline">{parties.length}</Badge>}
          </CardTitle>
          <Button size="sm" variant="outline" onClick={add}>
            <Plus className="h-4 w-4 mr-1" /> Add Party
          </Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        {parties.length === 0 && (
          <p className="text-xs text-muted-foreground">No related parties added</p>
        )}
        {parties.map((p, idx) => (
          <div key={idx} className="flex gap-2 items-end">
            <div className="flex-1 space-y-1">
              <Label className="text-xs">Name</Label>
              <Input className="h-7 text-xs" value={p.name} onChange={(e) => update(idx, { name: e.target.value })} />
            </div>
            <div className="w-32 space-y-1">
              <Label className="text-xs">ID</Label>
              <Input className="h-7 text-xs" placeholder="Party ID" value={p.id} onChange={(e) => update(idx, { id: e.target.value })} />
            </div>
            <div className="w-36 space-y-1">
              <Label className="text-xs">Role</Label>
              <Select value={p.role} onValueChange={(v: string | null) => v && update(idx, { role: v })}>
                <SelectTrigger className="h-7 text-xs">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {["Buyer", "Seller", "Requester", "Technical Contact", "Bill Receiver"].map((r) => (
                    <SelectItem key={r} value={r}>{r}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <Button size="icon" variant="ghost" className="h-7 w-7" onClick={() => remove(idx)}>
              <Trash2 className="h-4 w-4 text-destructive" />
            </Button>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}

interface PlaceEditorProps {
  places: PlaceRef[];
  onChange: (p: PlaceRef[]) => void;
}

export function PlaceEditor({ places, onChange }: PlaceEditorProps) {
  const add = useCallback(() => {
    onChange([...places, { id: "", name: "", role: "Install Address" }]);
  }, [places, onChange]);

  const update = useCallback(
    (idx: number, patch: Partial<PlaceRef>) =>
      onChange(places.map((p, i) => (i === idx ? { ...p, ...patch } : p))),
    [places, onChange],
  );

  const remove = useCallback(
    (idx: number) => onChange(places.filter((_, i) => i !== idx)),
    [places, onChange],
  );

  return (
    <Card>
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="text-sm flex items-center gap-2">
            <MapPin className="h-4 w-4" /> Places
            {places.length > 0 && <Badge variant="outline">{places.length}</Badge>}
          </CardTitle>
          <Button size="sm" variant="outline" onClick={add}>
            <Plus className="h-4 w-4 mr-1" /> Add Place
          </Button>
        </div>
      </CardHeader>
      <CardContent className="space-y-3">
        {places.length === 0 && (
          <p className="text-xs text-muted-foreground">No places added</p>
        )}
        {places.map((p, idx) => (
          <div key={idx} className="flex gap-2 items-end">
            <div className="flex-1 space-y-1">
              <Label className="text-xs">Name / Address</Label>
              <Input className="h-7 text-xs" value={p.name} onChange={(e) => update(idx, { name: e.target.value })} />
            </div>
            <div className="w-32 space-y-1">
              <Label className="text-xs">ID</Label>
              <Input className="h-7 text-xs" placeholder="Place ID" value={p.id} onChange={(e) => update(idx, { id: e.target.value })} />
            </div>
            <div className="w-36 space-y-1">
              <Label className="text-xs">Role</Label>
              <Select value={p.role} onValueChange={(v: string | null) => v && update(idx, { role: v })}>
                <SelectTrigger className="h-7 text-xs">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {["Install Address", "Delivery Address", "Service Address", "Billing Address"].map((r) => (
                    <SelectItem key={r} value={r}>{r}</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <Button size="icon" variant="ghost" className="h-7 w-7" onClick={() => remove(idx)}>
              <Trash2 className="h-4 w-4 text-destructive" />
            </Button>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}
