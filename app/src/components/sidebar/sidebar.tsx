"use client";

import { useState } from "react";
import { MessagesType, MessagesTypeTabs } from "../messages-type-tabs";
import { Card, CardContent } from "../ui/card";
import { SidebarUserContainer } from "./sidebar-user-container";
import { MessagesWrapper } from "../messages-wrapper";
import { MessageSquareText, Pin } from "lucide-react";

export function Sidebar() {
  const [selectedTab, setSelectedTab] = useState<MessagesType | undefined>(
    "all"
  );

  return (
    <Card className="w-full h-full">
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
            teste
          </MessagesWrapper>

          <MessagesWrapper label="Messages" icon={MessageSquareText}>
            teste
          </MessagesWrapper>
        </div>
      </CardContent>
    </Card>
  );
}
