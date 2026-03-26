import request from '@/lib/api';
import { PageParams } from '@/lib/api';

// 用户类型
export interface User {
  id: string;
  username: string;
  nickname?: string;
  email?: string;
  phone?: string;
  avatar?: string;
  status: number;
  department_id?: string;
  department_name?: string;
  post_id?: string;
  post_name?: string;
  roles: string[];
  created_at?: string;
  updated_at?: string;
}

// 部门类型
export interface Department {
  id: string;
  parent_id?: string;
  ancestors: string;
  department_name: string;
  display_order: number;
  leader?: string;
  phone?: string;
  email?: string;
  status: number;
  children?: Department[];
  created_at?: string;
  updated_at?: string;
}

// 岗位类型
export interface Post {
  id: string;
  post_code: string;
  post_name: string;
  display_order: number;
  status: number;
  created_at?: string;
  updated_at?: string;
}

// 角色类型
export interface Role {
  id: string;
  role_name: string;
  role_code: string;
  display_order: number;
  status: number;
  role_type: number;
  data_scope: number;
  remark?: string;
  menu_ids: string[];
  created_at?: string;
  updated_at?: string;
}

// 菜单类型
export interface Menu {
  id: string;
  parent_id?: string;
  ancestors: string;
  menu_name: string;
  menu_type: string;
  icon?: string;
  route_name?: string;
  route_path?: string;
  component?: string;
  permission?: string;
  display_order: number;
  is_cache: string;
  is_visible: string;
  status: number;
  children?: Menu[];
  created_at?: string;
  updated_at?: string;
}

// 用户服务
export const userService = {
  list(params: PageParams & { username?: string; status?: number }) {
    return request.get<{ list: User[]; total: number; page: number; pageSize: number }>('/api/system/users', { params });
  },
  get(id: string) {
    return request.get<User>(`/api/system/users/${id}`);
  },
  create(data: Partial<User>) {
    return request.post<User>('/api/system/users', data);
  },
  update(id: string, data: Partial<User>) {
    return request.put<User>(`/api/system/users/${id}`, data);
  },
  delete(id: string) {
    return request.delete(`/api/system/users/${id}`);
  },
  resetPassword(id: string) {
    return request.put(`/api/system/users/${id}/reset-password`);
  },
  assignRoles(id: string, roleIds: string[]) {
    return request.put(`/api/system/users/${id}/roles`, roleIds);
  },
};

// 部门服务
export const departmentService = {
  list(params?: PageParams) {
    return request.get<Department[]>('/api/system/departments', { params });
  },
  tree() {
    return request.get<Department[]>('/api/system/departments/tree');
  },
  get(id: string) {
    return request.get<Department>(`/api/system/departments/${id}`);
  },
  create(data: Partial<Department>) {
    return request.post<Department>('/api/system/departments', data);
  },
  update(id: string, data: Partial<Department>) {
    return request.put<Department>(`/api/system/departments/${id}`, data);
  },
  delete(id: string) {
    return request.delete(`/api/system/departments/${id}`);
  },
};

// 岗位服务
export const postService = {
  list(params?: PageParams) {
    return request.get<Post[]>('/api/system/posts', { params });
  },
  get(id: string) {
    return request.get<Post>(`/api/system/posts/${id}`);
  },
  create(data: Partial<Post>) {
    return request.post<Post>('/api/system/posts', data);
  },
  update(id: string, data: Partial<Post>) {
    return request.put<Post>(`/api/system/posts/${id}`, data);
  },
  delete(id: string) {
    return request.delete(`/api/system/posts/${id}`);
  },
};

// 角色服务
export const roleService = {
  list(params?: PageParams) {
    return request.get<Role[]>('/api/system/roles', { params });
  },
  get(id: string) {
    return request.get<Role>(`/api/system/roles/${id}`);
  },
  create(data: Partial<Role>) {
    return request.post<Role>('/api/system/roles', data);
  },
  update(id: string, data: Partial<Role>) {
    return request.put<Role>(`/api/system/roles/${id}`, data);
  },
  delete(id: string) {
    return request.delete(`/api/system/roles/${id}`);
  },
  assignMenus(id: string, menuIds: string[]) {
    return request.put(`/api/system/roles/${id}/menus`, menuIds);
  },
};

// 菜单服务
export const menuService = {
  list(params?: PageParams) {
    return request.get<Menu[]>('/api/system/menus', { params });
  },
  tree() {
    return request.get<Menu[]>('/api/system/menus/tree');
  },
  get(id: string) {
    return request.get<Menu>(`/api/system/menus/${id}`);
  },
  create(data: Partial<Menu>) {
    return request.post<Menu>('/api/system/menus', data);
  },
  update(id: string, data: Partial<Menu>) {
    return request.put<Menu>(`/api/system/menus/${id}`, data);
  },
  delete(id: string) {
    return request.delete(`/api/system/menus/${id}`);
  },
};

export default {
  user: userService,
  department: departmentService,
  post: postService,
  role: roleService,
  menu: menuService,
};
