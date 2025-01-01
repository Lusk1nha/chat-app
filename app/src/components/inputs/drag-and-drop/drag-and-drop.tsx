"use client";

import Dropzone, { Accept, FileRejection } from "react-dropzone";
import {
  DragAndDropContent,
  DragAndDropStrings
} from "./drag-and-drop-content";
import { useCallback, useState } from "react";
import { toast } from "sonner";

interface IDragAndDropProps {
  name: string;
  onChange: (files: File[]) => void;
  onBlur?: () => void;

  value: File[];

  icon?: React.ReactNode;
  strings?: DragAndDropStrings;

  error?: boolean;

  maxSize?: number;
  maxFiles?: number;
  accept?: Accept;
  multiple?: boolean;
  disabled?: boolean;
}

export function DragAndDrop(props: Readonly<IDragAndDropProps>) {
  const {
    icon,
    strings,
    onChange,
    onBlur,

    maxFiles = 1,
    maxSize = 5 * 1024 * 1024,
    accept = undefined,
    multiple = true,
    disabled = false,
    error = false
  } = props;

  const [_, setIsDraggingInside] = useState(false);

  const onDrop = useCallback((files: File[]) => {
    if (files.length > maxFiles) {
      onChange([]);
      return;
    }

    onChange(files);
  }, []);

  const onDropRejected = useCallback((rejectedFiles: FileRejection[]) => {
    if (rejectedFiles.length === 0) {
      return;
    }

    rejectedFiles.forEach((reject) => {
      const description = reject.errors
        .map((error) => error.message)
        .join(", ");

      toast.error(`File ${reject.file.name} was rejected: ${description}`, {
        description: "Please try again with another file."
      });
    });
  }, []);

  return (
    <Dropzone
      maxFiles={maxFiles}
      onDrop={onDrop}
      onDropRejected={onDropRejected}
      maxSize={maxSize}
      accept={accept}
      multiple={multiple}
      disabled={disabled}
      onDragEnter={() => setIsDraggingInside(true)}
      onDragLeave={() => setIsDraggingInside(false)}
    >
      {({ getInputProps, getRootProps }) => (
        <DragAndDropContent
          icon={icon}
          strings={strings}
          getRootProps={getRootProps}
          hasError={error}
        >
          <input onBlur={onBlur} {...getInputProps()} />
        </DragAndDropContent>
      )}
    </Dropzone>
  );
}
