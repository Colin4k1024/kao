export type UserStatus = "ACTIVE" | "DISABLED" | "LOCKED";

export type UserRoleSummary = {
  id: string;
  code: string;
  name: string;
  dataScope: string;
};

export type UserListItem = {
  id: string;
  username: string;
  displayName: string;
  email: string | null;
  phone: string | null;
  deptId: string | null;
  deptName: string | null;
  status: UserStatus;
  isSuperAdmin: boolean;
  roles: UserRoleSummary[];
  createdAt: string;
  updatedAt: string;
};

export type UserListResponse = {
  items: UserListItem[];
  total: number;
};

export type CreateUserInput = {
  username: string;
  password: string;
  displayName: string;
  email?: string | null;
  phone?: string | null;
  deptId?: string | null;
  roleIds: string[];
};

export type UpdateUserInput = {
  displayName: string;
  email?: string | null;
  phone?: string | null;
  deptId?: string | null;
  status: UserStatus;
  roleIds: string[];
  resetPassword?: string;
};
