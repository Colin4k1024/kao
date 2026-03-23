export type AuthLoginInput = {
  username: string;
  password: string;
};

export type AuthRole = {
  id: string;
  code: string;
  name: string;
};

export type AuthSessionUser = {
  id: string;
  username: string;
  displayName: string;
  email: string | null;
  avatarUrl: string | null;
  deptId: string | null;
  deptName?: string | null;
  status: "ACTIVE" | "DISABLED" | "LOCKED";
  isSuperAdmin: boolean;
  roles: AuthRole[];
};

export type AuthMenuType = "DIRECTORY" | "MENU" | "BUTTON";

export type AuthMenuNode = {
  id: string;
  parentId: string | null;
  name: string;
  menuType: AuthMenuType;
  routePath: string | null;
  component: string | null;
  permission: string | null;
  icon: string | null;
  sortOrder: number;
  visible: boolean;
  keepAlive: boolean;
  status: "ACTIVE" | "DISABLED";
  children?: AuthMenuNode[];
};

export type AuthLoginResponse = {
  access_token: string;
  user: AuthSessionUser;
  permissions: string[];
  menus: AuthMenuNode[];
};

export type AuthProfileResponse = {
  user: AuthSessionUser;
};

export type AuthPermissionsResponse = {
  permissions: string[];
};

export type AuthMenusResponse = {
  menus: AuthMenuNode[];
};

export type PermissionRequirement = string | string[];
