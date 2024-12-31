import { Card, CardContent } from "@/components/ui/card";
import { cn } from "@/lib/utils";

interface IAuthCardProps {
  className?: string;
  children: React.ReactNode;
}

export function AuthCard(props: Readonly<IAuthCardProps>) {
  const { className, children } = props;

  return (
    <Card className={cn("w-full max-w-[540px]", className)}>
      <CardContent className="flex flex-col p-8 gap-y-4">
        {children}
      </CardContent>
    </Card>
  );
}
