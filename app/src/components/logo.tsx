import { MessageSquareQuote } from "lucide-react";

export function Logo() {
  return (
    <div className="flex items-center gap-x-2">
      <MessageSquareQuote className="text-primary" size={36} />
      <h1 className="text-2xl font-bold text-primary">Papinho</h1>
    </div>
  );
}
