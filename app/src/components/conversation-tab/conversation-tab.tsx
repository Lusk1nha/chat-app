import { CheckCheck, LucideIcon, Pin } from "lucide-react";
import { Persona } from "../persona/persona";

export function ConversationTab() {
  return (
    <div className="w-full h-14 flex items-center gap-x-4">
      <div>
        <Persona
          src="https://github.com/Lusk1nha.png"
          initials="LP"
          size="sm"
        />
      </div>

      <div className="w-full h-full flex flex-col justify-between py-2">
        <ConversationHeaderInfo name="Lucas Pedro" time="12:30 PM" />
        <MessagePreview
          icon={CheckCheck}
          message="I'll be there in 5 minutes"
        />
      </div>
    </div>
  );
}

interface IConversationHeaderInfoProps {
  name: string;
  time: string;
}

function ConversationHeaderInfo(props: Readonly<IConversationHeaderInfoProps>) {
  const { name, time } = props;

  return (
    <div className="w-full flex items-center justify-between">
      <h4 className="text-xs md:text-sm font-medium">{name}</h4>
      <p className="flex items-center text-xs gap-x-1">
        <Pin size={14} />
        <span>{time}</span>
      </p>
    </div>
  );
}

interface IMessagePreviewProps {
  message: string;
  icon: LucideIcon;
}

function MessagePreview(props: Readonly<IMessagePreviewProps>) {
  const { message, icon: Icon } = props;

  return (
    <div className="w-full flex items-center justify-between">
      <p className="text-xs text-gray-500 font-normal">{message}</p>
      <div>
        <Icon size={18} />
      </div>
    </div>
  );
}
