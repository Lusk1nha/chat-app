export class Message {
  private readonly _id: string;
  private readonly _conversationId: string;
  private readonly _senderId: string;
  private readonly _receiverId: string;
  private readonly _body: string;
  private readonly _updatedAt: Date;
  private readonly _createdAt: Date;

  constructor(
    id: string,
    conversationId: string,
    senderId: string,
    receiverId: string,
    body: string,
    updatedAt: Date,
    createdAt: Date
  ) {
    this._id = id;
    this._conversationId = conversationId;
    this._senderId = senderId;
    this._receiverId = receiverId;
    this._body = body;
    this._updatedAt = updatedAt;
    this._createdAt = createdAt;
  }

  get id(): string {
    return this._id;
  }

  get conversationId(): string {
    return this._conversationId;
  }

  get senderId(): string {
    return this._senderId;
  }

  get receiverId(): string {
    return this._receiverId;
  }

  get body(): string {
    return this._body;
  }

  get updatedAt(): Date {
    return this._updatedAt;
  }

  get createdAt(): Date {
    return this._createdAt;
  }
}
