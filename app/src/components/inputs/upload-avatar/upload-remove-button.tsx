"use client";

import { Button } from "@/components/ui/button";
import { Trash2 } from "lucide-react";

interface IUploadRemoveButtonProps {
  onClick: () => void;
}

export function UploadRemoveButton(props: Readonly<IUploadRemoveButtonProps>) {
  const { onClick } = props;

  return (
    <Button type="button" size="icon" variant="destructive" onClick={onClick}>
      <Trash2 className="w-6 h-6" />
    </Button>
  );
}
