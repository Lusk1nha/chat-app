export interface Message {
  id: string;

  conversationId: string;
  senderId: string;
  receiverId: string;

  body: string;

  updatedAt: Date;
  createdAt: Date;
}
