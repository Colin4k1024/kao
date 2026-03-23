import { type ReactNode } from "react";

import { RouterProvider, createMemoryHistory, createRouter } from "@tanstack/react-router";
import { render } from "@testing-library/react";

import { AppProviders } from "@/app/providers";
import { rootRoute } from "@/routes/__root";
import { indexRoute } from "@/routes/index";
import { loginRoute } from "@/routes/login";

export function createTestRouter(initialPath: string) {
  const routeTree = rootRoute.addChildren([indexRoute, loginRoute]);

  return createRouter({
    routeTree,
    history: createMemoryHistory({
      initialEntries: [initialPath],
    }),
    defaultPreload: "intent",
  });
}

export function renderWithAppRouter(initialPath = "/login") {
  const router = createTestRouter(initialPath);

  function Wrapper({ children }: { children: ReactNode }) {
    return <AppProviders>{children}</AppProviders>;
  }

  return {
    router,
    ...render(<RouterProvider router={router} />, {
      wrapper: Wrapper,
    }),
  };
}
