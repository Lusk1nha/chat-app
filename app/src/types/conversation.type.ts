export interface ConversationType {
  id: string;

  entityType: "user" | "group";
  entityId: string;

  updatedAt: Date;
  createdAt: Date;
}
