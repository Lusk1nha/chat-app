import { Sidebar } from "@/components/sidebar/sidebar";

interface IDashboardLayoutProps {
  children: React.ReactNode;
}

export default async function DashboardLayout(
  props: Readonly<IDashboardLayoutProps>
) {
  const { children } = props;

  return (
    <div className="min-w-full h-screen flex gap-2 p-4">
      <div className="w-80 h-full">
        <Sidebar />
      </div>
      <div>{children}</div>
    </div>
  );
}
