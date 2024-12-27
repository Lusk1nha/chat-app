"use client";

import { useState } from "react";
import { MessagesType, MessagesTypeTabs } from "../messages-type-tabs";
import { Card, CardContent } from "../ui/card";
import { SidebarUserContainer } from "./sidebar-user-container";
import { MessagesWrapper } from "../messages-wrapper";
import { MessageSquareText, Pin } from "lucide-react";
import { ConversationTab } from "../conversation-tab/conversation-tab";
import { ConversationMockup } from "@/tests/conversation.mockup";

export function Sidebar() {
  const [selectedTab, setSelectedTab] = useState<MessagesType | undefined>(
    "all"
  );

  function generateConversation(length: number) {
    const { list } = new ConversationMockup();
    return list(length);
  }

  const conversations = generateConversation(10);
  console.log({
    conversations
  });

  return (
    <Card className="w-full h-full rounded-400">
      <CardContent className="flex flex-col px-5 pt-6 pb-0 gap-y-4">
        <div className="w-full flex flex-col gap-4">
          <SidebarUserContainer />
          <MessagesTypeTabs
            defaultValue="all"
            onChange={(value) => setSelectedTab(value)}
          />
        </div>

        <div className="w-full h-full flex flex-col gap-1">
          <MessagesWrapper label="Pinned messages" icon={Pin}>
            {conversations.map((_, index) => (
              <ConversationTab key={index} />
            ))}
          </MessagesWrapper>

          <MessagesWrapper label="Messages" icon={MessageSquareText}>
            teste
          </MessagesWrapper>
        </div>
      </CardContent>
    </Card>
  );
}
