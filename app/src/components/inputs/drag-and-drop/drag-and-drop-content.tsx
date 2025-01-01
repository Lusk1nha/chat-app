import { cn } from "@/lib/utils";
import { ImageUp } from "lucide-react";
import { DropzoneRootProps } from "react-dropzone";
import { AnimatePresence } from "framer-motion";

import { motion } from "framer-motion";

export type DragAndDropStrings = {
  dragAndDrop?: string;
  dragAndDropDescription?: string;
  acceptDescription?: string;
};

interface IDragAndDropTextProps {
  children: React.ReactNode;

  icon?: React.ReactNode;
  strings?: DragAndDropStrings;

  getRootProps: <T extends DropzoneRootProps>(props?: T) => T;

  hasError: boolean;
}

export function DragAndDropContent(props: Readonly<IDragAndDropTextProps>) {
  const { children, icon, strings, getRootProps, hasError } = props;

  return (
    <div
      className={cn(
        "border-2 border-dashed rounded-lg flex flex-col gap-1 p-6 items-center select-none cursor-pointer hover:border-primary hover:bg-accent transition-all",
        hasError ? "border-destructive" : "border-gray-200"
      )}
      {...getRootProps()}
    >
      {children}

      <AnimatePresence>
        <motion.div
          className={cn(
            "flex flex-col gap-2 text-center",
            hasError ? "text-destructive" : "text-muted-foreground"
          )}
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
        >
          <div className="flex items-center justify-center rounded-lg text-secondary-foreground">
            {icon ?? <ImageUp className="w-8 h-8" />}
          </div>

          {strings?.dragAndDrop && (
            <span className="text-sm font-medium">{strings.dragAndDrop}</span>
          )}

          {strings?.dragAndDropDescription && (
            <span className="text-xs">{strings.dragAndDropDescription}</span>
          )}

          {strings?.acceptDescription && (
            <span className="text-xs">({strings.acceptDescription})</span>
          )}
        </motion.div>
      </AnimatePresence>
    </div>
  );
}
