import { Control, FieldValues, Path } from "react-hook-form";
import {
  IUploadAvatarProps,
  UploadAvatar
} from "./upload-avatar/upload-avatar";
import {
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage
} from "../ui/form";

interface IFormUploadAvatarProps<T extends FieldValues> {
  name: Path<T>;

  label: string;
  control: Control<T>;

  field?: IUploadAvatarProps;

  required?: boolean;

  description?: string;

  onLabelPos?: React.ReactNode;
}

export default function FormUploadAvatar<T extends FieldValues>(
  props: Readonly<IFormUploadAvatarProps<T>>
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
            <UploadAvatar {...field} {...fieldAttributes} />
          </FormControl>

          {description && <FormDescription>{description}</FormDescription>}

          <FormMessage />
        </FormItem>
      )}
    />
  );
}
