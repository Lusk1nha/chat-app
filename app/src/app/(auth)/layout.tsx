export default function AuthLayout({
  children
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <div className="bg-background w-full h-screen flex items-center justify-center px-4">
      {children}
    </div>
  );
}
