import { Control, FieldValues, Path } from "react-hook-form";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "../ui/form";
import { Input } from "../ui/input";
import { HTMLProps } from "react";

interface IFormInputProps<T extends FieldValues> {
  name: Path<T>;
  label: string;
  control: Control<T>;
  field: HTMLProps<HTMLInputElement>;

  required?: boolean;

  description?: string;

  onLabelPos?: React.ReactNode;
}

export function FormInput<T extends FieldValues>(
  props: Readonly<IFormInputProps<T>>
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
            <Input
              className="w-full h-[42px] text-xs"
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
