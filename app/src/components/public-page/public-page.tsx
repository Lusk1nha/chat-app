"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";
import { Path } from "@/shared/enums/Path.enum";

import authService from "@/shared/factories/auth-factory";

interface PublicOnlyProps {
  children: React.ReactNode;
}

export default function PublicOnly({ children }: Readonly<PublicOnlyProps>) {
  const router = useRouter();

  useEffect(() => {
    const checkAuth = async () => {
      let isAuthenticated = false;

      try {
        isAuthenticated = await authService.validateSession();
      } finally {
        if (isAuthenticated) {
          router.push(Path.Home);
        }
      }
    };

    checkAuth();
  }, [router]);

  return <>{children}</>;
}
