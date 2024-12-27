import { Conversation } from "@/modules/conversation/conversation.schema";

import { faker } from "@faker-js/faker";

export class ConversationMockup {
  constructor() {
    this.generate = this.generate.bind(this);
    this.list = this.list.bind(this);
  }

  list(length: number): Conversation[] {
    const mockups = Array.from({ length }, (_, index) => this.generate(index));
    return mockups;
  }

  generate(index: number): Conversation {
    const body = {
      index,

      id: faker.string.uuid(),
      entityType: faker.helpers.arrayElement(["user", "group"]),
      entityId: faker.string.uuid(),

      updatedAt: faker.date.recent(),
      createdAt: faker.date.recent()
    };

    const conversation = new Conversation(
      body.id,
      body.entityType,
      body.entityId,
      body.updatedAt,
      body.createdAt
    );

    return conversation;
  }
}
