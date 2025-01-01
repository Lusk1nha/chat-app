"use client";

import { useEffect, ReactNode } from "react";
import { useRouter } from "next/navigation"; // Altere para 'next/navigation'
import authService from "@/shared/factories/auth-factory";
import { Path } from "@/shared/enums/Path.enum";
import { toast } from "sonner";

interface ProtectedComponentProps {
  children: ReactNode;
}

export default function ProtectedComponent({
  children
}: Readonly<ProtectedComponentProps>) {
  const router = useRouter();

  useEffect(() => {
    const checkAuth = async () => {
      try {
        const isAuthenticated = await authService.validateSession();

        if (!isAuthenticated) {
          router.push(Path.Login);
        }
      } catch (error) {
        toast.error("Sessão expirada, faça login novamente");
        router.push(Path.Login);
      }
    };

    checkAuth();
  }, [router]);

  return <>{children}</>;
}
