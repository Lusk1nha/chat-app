import { Sidebar } from "@/components/sidebar/sidebar";

export default function Dashboard() {
  return (
    <div className="min-w-full h-screen flex gap-2 p-4">
      <div className="w-80 h-full">
        <Sidebar />
      </div>
    </div>
  );
}
