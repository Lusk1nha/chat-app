import { Control, FieldValues, Path } from "react-hook-form";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "../ui/form";

import { HTMLProps } from "react";
import { Textarea } from "../ui/textarea";

interface IFormTextAreaProps<T extends FieldValues> {
  name: Path<T>;
  label: string;
  control: Control<T>;

  field?: HTMLProps<HTMLTextAreaElement>;

  required?: boolean;

  description?: string;

  onLabelPos?: React.ReactNode;
}

export function FormTextarea<T extends FieldValues>(
  props: Readonly<IFormTextAreaProps<T>>
) {
  const {
    name,
    control,
    label,
    required = false,
    description,
    field: fieldAttributes,
    onLabelPos
  } = props;

  return (
    <FormField
      name={name}
      control={control}
      render={({ field }) => (
        <FormItem>
          <div className="w-full flex items-center justify-between">
            <FormLabel required={required}>{label}</FormLabel>
            {onLabelPos}
          </div>
          <FormControl>
            <Textarea
              className="w-full text-xs"
              {...field}
              {...fieldAttributes}
            />
          </FormControl>

          {description && <FormDescription>{description}</FormDescription>}

          <FormMessage />
        </FormItem>
      )}
    />
  );
}
