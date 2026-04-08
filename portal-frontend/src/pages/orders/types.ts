/** Types for the TMF622 order builder */

export interface OrderItem {
  id: string;
  action: "add" | "modify" | "delete" | "noChange";
  productOffering: { id: string; name: string; href?: string };
  product?: {
    productCharacteristic?: Characteristic[];
    productSpecification?: { id: string; name: string };
    place?: PlaceRef[];
    relatedParty?: PartyRef[];
  };
  quantity?: number;
}

export interface Characteristic {
  name: string;
  value: string;
  valueType?: string;
}

export interface PartyRef {
  id: string;
  name: string;
  role: string;
  "@referredType"?: string;
}

export interface PlaceRef {
  id: string;
  name: string;
  role: string;
  "@referredType"?: string;
}

export interface OrderDraft {
  description: string;
  externalId: string;
  priority: string;
  requestedStartDate: string;
  requestedCompletionDate: string;
  channel?: { id: string; name: string }[];
  relatedParty: PartyRef[];
  orderItem: OrderItem[];
  note?: { text: string; author?: string }[];
}

export function emptyDraft(): OrderDraft {
  return {
    description: "",
    externalId: "",
    priority: "4",
    requestedStartDate: "",
    requestedCompletionDate: "",
    relatedParty: [],
    orderItem: [],
  };
}

export function emptyOrderItem(): OrderItem {
  return {
    id: crypto.randomUUID().slice(0, 8),
    action: "add",
    productOffering: { id: "", name: "" },
    product: { productCharacteristic: [], relatedParty: [], place: [] },
  };
}
