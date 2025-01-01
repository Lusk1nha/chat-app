"use client";

import { AuthProvider } from "@/contexts/auth-context";
import { Toaster } from "./ui/sonner";

interface IProvidersProps {
  children: React.ReactNode;
}

export function Providers({ children }: Readonly<IProvidersProps>) {
  return (
    <AuthProvider>
      {children}
      <Toaster />
    </AuthProvider>
  );
}
