import Link from "next/link";
import { CircleX } from "lucide-react";
import { Path } from "@/shared/enums/Path.enum";
import { buttonVariants } from "@/components/ui/button";

export default function NotFound() {
  return (
    <div className="bg-background w-full h-screen flex items-center justify-center">
      <div className="max-w-[520px] w-full h-full flex flex-col items-center justify-center gap-y-2 px-4">
        <CircleX className="text-primary" size={120} />
        <h2 className="text-2xl font-bold">Page not found</h2>
        <p className="texd-sm text-center">
          Could not find requested resource, please try again.
        </p>
        <Link
          className={buttonVariants({
            variant: "outline",
            className: "w-full mt-4"
          })}
          href={Path.Home}
        >
          Go back to home
        </Link>
      </div>
    </div>
  );
}
