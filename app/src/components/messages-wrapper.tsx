import { LucideIcon } from "lucide-react";

interface IMessagesWrapperProps {
  label: string;
  icon: LucideIcon;

  children?: React.ReactNode;
}

export function MessagesWrapper(props: Readonly<IMessagesWrapperProps>) {
  const { label, icon: Icon } = props;

  return (
    <div className="w-full flex flex-col">
      <div className="w-full flex items-center justify-between py-4">
        <h5 className="text-sm text-accent-foreground">{label}</h5>
        <Icon className="text-accent-foreground" size={14} />
      </div>

      <div className="w-full h-full">{props.children}</div>
    </div>
  );
}
