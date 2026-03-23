export type DepartmentStatus = "ACTIVE" | "DISABLED";

export type DepartmentTreeNode = {
  id: string;
  code: string;
  name: string;
  parentId: string | null;
  ancestors: string;
  path: string;
  sortOrder: number;
  leader: string | null;
  phone: string | null;
  email: string | null;
  status: DepartmentStatus;
  children: DepartmentTreeNode[];
};

export type DepartmentTreeResponse = {
  items: DepartmentTreeNode[];
};

export type CreateDepartmentInput = {
  code: string;
  name: string;
  parentId?: string | null;
  sortOrder: number;
  leader?: string | null;
  phone?: string | null;
  email?: string | null;
  status: DepartmentStatus;
};

export type UpdateDepartmentInput = CreateDepartmentInput;
