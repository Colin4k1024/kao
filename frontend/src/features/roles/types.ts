export type RoleStatus = "ACTIVE" | "DISABLED";

export type RoleDataScope = "ALL" | "CUSTOM" | "DEPT" | "DEPT_AND_CHILD" | "SELF";

export type RoleListItem = {
  id: string;
  code: string;
  name: string;
  description: string | null;
  dataScope: RoleDataScope;
  status: RoleStatus;
  isSystem: boolean;
  userCount: number;
  permissionCount: number;
  updatedAt: string;
};

export type RoleListResponse = {
  items: RoleListItem[];
  total: number;
};

export type CreateRoleInput = {
  code: string;
  name: string;
  description?: string | null;
  dataScope: RoleDataScope;
  status: RoleStatus;
  permissionIds: string[];
  departmentIds: string[];
};

export type UpdateRoleInput = CreateRoleInput;
