import { createRouter } from "@tanstack/react-router";

import { rootRoute } from "@/routes/__root";
import { indexRoute } from "@/routes/index";
import { systemRoute } from "@/routes/system/root";
import { departmentsRoute } from "@/routes/system/departments";
import { menusRoute } from "@/routes/system/menus";
import { rolesRoute } from "@/routes/system/roles";
import { usersRoute } from "@/routes/system/users";
import { loginRoute } from "@/routes/login";

const routeTree = rootRoute.addChildren([
  indexRoute,
  loginRoute,
  systemRoute.addChildren([usersRoute, rolesRoute, departmentsRoute, menusRoute]),
]);

export const router = createRouter({
  routeTree,
  defaultPreload: "intent",
});

declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}
