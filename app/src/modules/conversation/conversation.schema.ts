export class Conversation {
  private readonly _id: string;
  private readonly _entityType: "user" | "group";
  private readonly _entityId: string;
  private readonly _updatedAt: Date;
  private readonly _createdAt: Date;

  constructor(
    id: string,
    entityType: "user" | "group",
    entityId: string,
    updatedAt: Date,
    createdAt: Date
  ) {
    this._id = id;
    this._entityType = entityType;
    this._entityId = entityId;
    this._updatedAt = updatedAt;
    this._createdAt = createdAt;
  }

  get id(): string {
    return this._id;
  }

  get entityType(): "user" | "group" {
    return this._entityType;
  }

  get entityId(): string {
    return this._entityId;
  }

  get updatedAt(): Date {
    return this._updatedAt;
  }

  get createdAt(): Date {
    return this._createdAt;
  }
}
