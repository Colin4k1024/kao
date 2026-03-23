import { Outlet, createRoute } from "@tanstack/react-router";

import { AuthGuard } from "@/components/guards/auth-guard";
import { AppShell } from "@/components/layout/app-shell";

import { rootRoute } from "../__root";

export const systemRoute = createRoute({
  getParentRoute: () => rootRoute,
  path: "/system",
  component: SystemLayout,
});

function SystemLayout() {
  return (
    <AuthGuard>
      <AppShell>
        <Outlet />
      </AppShell>
    </AuthGuard>
  );
}
