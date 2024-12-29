import { LucideIcon } from "lucide-react";

interface IMessagesWrapperProps {
  label: string;

  icon: LucideIcon;
  iconAriaLabel?: string;

  children?: React.ReactNode;
}

export function MessagesWrapper(props: Readonly<IMessagesWrapperProps>) {
  const { label, icon: Icon, iconAriaLabel } = props;

  return (
    <div className="w-full flex flex-col">
      <div className="w-full flex items-center justify-between py-4">
        <h5 className="text-sm text-accent-foreground">{label}</h5>
        <Icon
          aria-label={iconAriaLabel}
          className="text-accent-foreground"
          size={14}
        />
      </div>

      <div className="w-full h-full flex flex-col gap-y-2">{props.children}</div>
    </div>
  );
}
