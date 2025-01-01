"use client";

import { useState } from "react";
import { MessagesType, MessagesTypeTabs } from "../messages-type-tabs";
import { Card, CardContent } from "../ui/card";
import { SidebarUserContainer } from "./sidebar-user-container";
import { MessagesWrapper } from "../messages-wrapper";
import { MessageSquareText, Pin } from "lucide-react";
import { ConversationTab } from "../conversation-tab/conversation-tab";
import { Separator } from "../ui/separator";
import { Button } from "../ui/button";
import authService from "@/shared/factories/auth-factory";
import { toast } from "sonner";

import { useRouter } from "next/navigation";
import { Path } from "@/shared/enums/Path.enum";

export function Sidebar() {
  const [selectedTab, setSelectedTab] = useState<MessagesType | undefined>(
    "all"
  );

  const router = useRouter();

  async function logout() {
    await authService.logout();
    toast.success("Logout realizado com sucesso");
    router.push(Path.Login);
  }

  return (
    <Card className="w-full h-full rounded-400">
      <CardContent className="flex flex-col px-5 pt-6 pb-4 gap-y-4">
        <div className="h-full flex flex-col justify-between">
          <div className="w-full h-full flex flex-col gap-4">
            <div className="w-full flex flex-col gap-4">
              <SidebarUserContainer />
              <MessagesTypeTabs
                defaultValue="all"
                onChange={(value) => setSelectedTab(value)}
              />
            </div>

            <div className="w-full h-full flex flex-col gap-1">
              <MessagesWrapper label="Pinned messages" icon={Pin}>
                <ConversationTab key={1} />
              </MessagesWrapper>

              <MessagesWrapper label="Messages" icon={MessageSquareText}>
                teste
              </MessagesWrapper>
            </div>
          </div>

          <div className="w-full flex flex-col gap-4">
            <Separator />

            <Button variant="secondary" type="button" onClick={logout}>
              Logout
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
