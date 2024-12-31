import { Logo } from "@/components/logo";
import { cn } from "@/lib/utils";

interface IAuthHeaderProps {
  className?: string;
  title: string;
  subtitle: string;
}

export function AuthHeader(props: Readonly<IAuthHeaderProps>) {
  const { className, title, subtitle } = props;

  return (
    <div
      className={cn(
        "flex flex-col items-center justify-center gap-y-4",
        className
      )}
    >
      <div className="flex flex-col items-center justify-center gap-y-2">
        <Logo />
        <h2 className="text-2xl font-bold text-center">{title}</h2>
        <h5 className="text-sm text-muted-foreground text-center">
          {subtitle}
        </h5>
      </div>
    </div>
  );
}
