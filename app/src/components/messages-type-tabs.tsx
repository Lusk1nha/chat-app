"use client";

import { Tabs, TabsList, TabsTrigger } from "./ui/tabs";

export type MessagesType = "all" | "personal" | "group";

type MessageTab = {
  label: string;
  value: MessagesType;
};

interface IMessagesTypeTabsProps {
  defaultValue?: MessagesType;
  value?: MessagesType;

  onChange?: (value: MessagesType | undefined) => void;
}

const TABS: MessageTab[] = [
  { label: "All", value: "all" },
  { label: "Personal", value: "personal" },
  { label: "Group", value: "group" }
];

export function MessagesTypeTabs(props: Readonly<IMessagesTypeTabsProps>) {
  const { defaultValue = "all", value, onChange } = props;

  function handleOnChange(value: string) {
    if (onChange) {
      onChange(value as MessagesType);
    }
  }

  return (
    <Tabs
      defaultValue={defaultValue}
      value={value}
      onValueChange={handleOnChange}
    >
      <TabsList className="w-full h-10 justify-between px-4 rounded-3xl">
        {TABS.map((tab) => (
          <TabsTrigger
            className="rounded-3xl text-sm font-medium data-[state='active']:text-primary"
            key={tab.value}
            value={tab.value}
          >
            {tab.label}
          </TabsTrigger>
        ))}
      </TabsList>
    </Tabs>
  );
}
