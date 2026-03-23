import { Navigate } from "@tanstack/react-router";
import type { PropsWithChildren } from "react";

import { hasAuthToken } from "@/lib/auth";

type AuthGuardProps = PropsWithChildren<{
  fallbackTo?: string;
}>;

export function AuthGuard({ children, fallbackTo = "/login" }: AuthGuardProps) {
  if (!hasAuthToken()) {
    return <Navigate to={fallbackTo} replace />;
  }

  return <>{children}</>;
}
