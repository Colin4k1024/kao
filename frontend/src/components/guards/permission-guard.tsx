import type { PropsWithChildren, ReactNode } from "react";

import { usePermissions, type PermissionRequirement } from "@/features/auth";

type PermissionGuardProps = PropsWithChildren<{
  required: PermissionRequirement;
  fallback?: ReactNode;
}>;

export function PermissionGuard({
  required,
  fallback = null,
  children,
}: PermissionGuardProps) {
  const { hasPermission, isLoading } = usePermissions();

  if (isLoading) {
    return <>{fallback}</>;
  }

  if (!hasPermission(required)) {
    return <>{fallback}</>;
  }

  return <>{children}</>;
}
