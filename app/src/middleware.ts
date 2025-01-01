import { NextRequest, NextResponse } from "next/server";
import { Path } from "./shared/enums/Path.enum";

export function middleware(req: NextRequest) {
  const pathname = req.nextUrl.pathname;
  const refreshToken = req.cookies.get("refresh_token");

  const isAuthGroup = pathname.startsWith(Path.AuthGroup); // Exemplo: /auth
  const isDashboardGroup = pathname.startsWith(Path.DashboardGroup); // Exemplo: /dashboard
  const isRedirectingToHome = pathname === Path.Home; // Exemplo: /
  const isRedirectingToLogin = pathname === Path.Login; // Exemplo: /auth/login

  if (isRedirectingToHome) {
    return NextResponse.redirect(new URL(Path.DashboardGroup, req.url));
  }

  // Usuário autenticado tentando acessar rotas de autenticação
  if (refreshToken && isAuthGroup) {
    // Evitar loop de redirecionamento para a home
    if (!isRedirectingToHome) {
      return NextResponse.redirect(new URL(Path.DashboardGroup, req.url));
    }
    return NextResponse.next();
  }

  // Usuário não autenticado tentando acessar rotas protegidas
  if (!refreshToken && isDashboardGroup) {
    // Evitar loop de redirecionamento para o login
    if (!isRedirectingToLogin) {
      return NextResponse.redirect(new URL(Path.Login, req.url));
    }
    return NextResponse.next();
  }

  // Continue para a rota solicitada
  return NextResponse.next();
}

// Configuração das rotas protegidas
export const config = {
  matcher: ["/:path*"] // Protege todas as rotas
};
