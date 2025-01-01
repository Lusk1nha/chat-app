"use client";

import { DragAndDrop } from "../drag-and-drop/drag-and-drop";
import { DragAndDropStrings } from "../drag-and-drop/drag-and-drop-content";
import { UploadAvatarContent } from "./upload-avatar-content";
import { UploadRemoveButton } from "./upload-remove-button";

export interface IUploadAvatarProps {
  name: string;
  onChange: (file: File | null) => void;
  onBlur?: () => void;

  value?: File;

  dragAndDropStrings?: DragAndDropStrings;

  error?: boolean;
  disabled?: boolean;
}

export function UploadAvatar(props: Readonly<IUploadAvatarProps>) {
  const { name, onChange, onBlur, value, error, disabled, dragAndDropStrings } =
    props;

  const componentName = "upload-avatar-input-" + name;

  function handleChange(files: File[]) {
    if (files.length === 0) {
      return;
    }

    const firstFile = files[0];
    onChange(firstFile);
  }

  function handleClear() {
    onChange(null);
  }

  if (!value) {
    return (
      <div id={componentName} className="flex flex-col">
        <DragAndDrop
          name={name}
          onChange={handleChange}
          value={value ?? []}
          onBlur={onBlur}
          multiple={false}
          accept={{
            "image/png": [".png"],
            "image/jpeg": [".jpg", ".jpeg"]
          }}
          strings={dragAndDropStrings}
          error={error}
          disabled={disabled}
        />
      </div>
    );
  }

  return (
    <div id={componentName} className="w-full flex items-end gap-4">
      <UploadAvatarContent value={value} />

      <div className="flex flex-col gap-2">
        <UploadRemoveButton onClick={handleClear} />
      </div>
    </div>
  );
}
