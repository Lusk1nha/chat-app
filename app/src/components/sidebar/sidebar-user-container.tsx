import { Search } from "lucide-react";
import { PersonaWithTitle } from "../persona/persona-with-title";
import { Button } from "../ui/button";

export function SidebarUserContainer() {
  return (
    <div className="w-full flex items-center justify-between gap-4">
      <PersonaWithTitle
        title="Lucas Pedro"
        aux="Software Engineer"
        src="https://github.com/Lusk1nha.png"
        initials="LP"
        size="md"
      />

      <Button className="min-w-8 h-8" size="icon" type="button" variant="ghost">
        <Search size={24} />
      </Button>
    </div>
  );
}
