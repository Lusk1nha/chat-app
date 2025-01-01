import { cn } from "@/lib/utils";
import Link from "next/link";

interface IAuthRedirectProps {
  className?: string;
  children: string;

  href: string;
  redirectText: string;
}

export function AuthRedirect(props: Readonly<IAuthRedirectProps>) {
  const { className, children, href, redirectText } = props;

  return (
    <p className={cn("text-sm text-muted-foreground text-center", className)}>
      {children}{" "}
      <Link href={href} className="text-primary hover:underline">
        {redirectText}
      </Link>
    </p>
  );
}
