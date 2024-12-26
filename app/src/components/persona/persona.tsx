import { cn } from "@/lib/utils";
import { Avatar, AvatarImage, AvatarFallback } from "../ui/avatar";

type PersonaSizes = "xs" | "sm" | "md" | "lg";

export interface IPersonaProps {
  src: string;
  initials: string;

  alt?: string;
  size?: PersonaSizes;
}

const sizes: Record<PersonaSizes, string> = {
  xs: "w-8 h-8",
  sm: "w-10 h-10",
  md: "w-12 h-12",
  lg: "w-14 h-14"
};

export function Persona(props: Readonly<IPersonaProps>) {
  const { src, initials, alt, size = "sm" } = props;

  const classSize = sizes[size];

  return (
    <Avatar className={cn("w-10 h-10", classSize)}>
      <AvatarImage src={src} alt={alt} />
      <AvatarFallback>{initials}</AvatarFallback>
    </Avatar>
  );
}
