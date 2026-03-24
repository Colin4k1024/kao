export type TableListItem = {
  id: string;
  username: string;
  display_name?: string;
  email?: string;
  phone?: string;
  dept_id?: string;
  avatar_url?: string;
  status: string;
  is_super_admin?: boolean;
  last_login_at?: string;
  last_login_ip?: string;
  created_at: string;
  updated_at: string;
};

export type TableListParams = {
  page?: number;
  pageSize?: number;
  dept_id?: string;
  status?: string;
};

export type RoleItem = {
  id: string;
  code: string;
  name: string;
  description?: string;
  data_scope: string;
  status: string;
  is_system?: boolean;
  created_at: string;
  updated_at: string;
};

export type DepartmentItem = {
  id: string;
  parent_id?: string;
  code: string;
  name: string;
  ancestors?: string;
  path?: string;
  sort_order?: number;
  leader?: string;
  phone?: string;
  email?: string;
  status: string;
  children?: DepartmentItem[];
};

export type MenuItem = {
  id: string;
  parent_id?: string;
  name: string;
  menu_type: string;
  route_path?: string;
  component?: string;
  permission?: string;
  icon?: string;
  sort_order?: number;
  visible?: boolean;
  status: string;
  children?: MenuItem[];
};

export type LoginResult = {
  access_token: string;
  token_type: string;
  expires_in: number;
};

export type CurrentUser = {
  id: string;
  username: string;
  display_name?: string;
  email?: string;
  dept_id?: string;
  avatar_url?: string;
  permissions: string[];
  roles: string[];
};

export type MenuNode = {
  id: string;
  name: string;
  icon?: string;
  path?: string;
  children?: MenuNode[];
};
