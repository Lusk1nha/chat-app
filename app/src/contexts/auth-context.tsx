"use client";

import { createContext, useEffect, useMemo, useState } from "react";

interface IAuthProviderProps {
  children: React.ReactNode;
  defaultToken?: string | null;
}

interface IAuthContext {
  accessToken: string | null;
  setAccessToken: (token: string | null) => void;
}

export const AuthContext = createContext<IAuthContext>({
  accessToken: null,
  setAccessToken: () => {}
});

export const AuthProvider = (props: Readonly<IAuthProviderProps>) => {
  const { children, defaultToken } = props;

  const [accessToken, setAccessToken] = useState<string | null>(
    defaultToken ?? null
  );

  useEffect(() => {
    const token = localStorage.getItem("accessToken");
    if (token) {
      setAccessToken(token);
    }
  }, []);

  const value = useMemo(
    () => ({
      accessToken,
      setAccessToken
    }),
    [accessToken, setAccessToken]
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};
