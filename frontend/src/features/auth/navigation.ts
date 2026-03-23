import type { AuthMenuNode } from "./types";

export type NavigationItem = {
  id: string;
  label: string;
  path: string;
  icon: string | null;
  permission: string | null;
};

export type NavigationSection = {
  id: string;
  label: string;
  icon: string | null;
  items: NavigationItem[];
};

const fallbackNavigationSections: NavigationSection[] = [
  {
    id: "dashboard",
    label: "工作台",
    icon: "dashboard",
    items: [
      {
        id: "dashboard-home",
        label: "首页概览",
        path: "/",
        icon: "home",
        permission: "system:dashboard:view",
      },
    ],
  },
  {
    id: "system",
    label: "系统管理",
    icon: "settings",
    items: [
      {
        id: "system-users",
        label: "用户管理",
        path: "/system/users",
        icon: "users",
        permission: "system:user:list",
      },
      {
        id: "system-roles",
        label: "角色管理",
        path: "/system/roles",
        icon: "shield",
        permission: "system:role:list",
      },
      {
        id: "system-departments",
        label: "部门管理",
        path: "/system/departments",
        icon: "building",
        permission: "system:dept:list",
      },
      {
        id: "system-menus",
        label: "菜单管理",
        path: "/system/menus",
        icon: "menu",
        permission: "system:menu:list",
      },
    ],
  },
];

function isVisible(menu: AuthMenuNode) {
  return menu.visible && menu.status === "ACTIVE";
}

function sortByOrder<T extends { sortOrder: number }>(items: T[]) {
  return [...items].sort((left, right) => left.sortOrder - right.sortOrder);
}

function toNavigationItem(menu: AuthMenuNode): NavigationItem | null {
  if (!menu.routePath) {
    return null;
  }

  return {
    id: menu.id,
    label: menu.name,
    path: menu.routePath,
    icon: menu.icon,
    permission: menu.permission,
  };
}

function collectItems(menus: AuthMenuNode[]): NavigationItem[] {
  const items: NavigationItem[] = [];

  for (const menu of sortByOrder(menus)) {
    if (!isVisible(menu)) {
      continue;
    }

    if (menu.menuType === "BUTTON") {
      continue;
    }

    if (menu.menuType === "DIRECTORY") {
      items.push(...collectItems(menu.children ?? []));
      continue;
    }

    const item = toNavigationItem(menu);

    if (item) {
      items.push(item);
    }
  }

  return items;
}

export function buildNavigationSections(
  menus: AuthMenuNode[],
): NavigationSection[] {
  const sections = sortByOrder(menus)
    .filter(isVisible)
    .flatMap((menu): NavigationSection[] => {
      if (menu.menuType === "BUTTON") {
        return [];
      }

      if (menu.menuType === "DIRECTORY") {
        return [
          {
            id: menu.id,
            label: menu.name,
            icon: menu.icon,
            items: collectItems(menu.children ?? []),
          },
        ];
      }

      const item = toNavigationItem(menu);

      return item
        ? [
            {
              id: menu.id,
              label: menu.name,
              icon: menu.icon,
              items: [item],
            },
          ]
        : [];
    });

  return sections.length > 0 ? sections : fallbackNavigationSections;
}

export { fallbackNavigationSections };
