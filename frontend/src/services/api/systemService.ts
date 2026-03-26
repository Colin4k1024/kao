import api from '@/lib/api'
import type { ApiResponse, PageResult, ListParams } from '@/types/api'
import type { User, Role, Department, Menu, Post, UserQueryParams, RoleQueryParams, DeptQueryParams, MenuQueryParams } from '@/types/user'

export const userApi = {
  list: (params: ListParams & UserQueryParams) => {
    return api.get<PageResult<User>>('/system/users', params)
  },

  getById: (id: number) => {
    return api.get<User>(`/system/users/${id}`)
  },

  create: (data: Partial<User>) => {
    return api.post<User>('/system/users', data)
  },

  update: (id: number, data: Partial<User>) => {
    return api.put<User>(`/system/users/${id}`, data)
  },

  delete: (id: number) => {
    return api.delete(`/system/users/${id}`)
  },

  batchDelete: (ids: number[]) => {
    return api.post('/system/users/batch-delete', { ids })
  },

  updateStatus: (id: number, status: number) => {
    return api.patch(`/system/users/${id}/status`, { status })
  },

  resetPassword: (id: number) => {
    return api.post(`/system/users/${id}/reset-password`)
  },
}

export const roleApi = {
  list: (params: ListParams & RoleQueryParams) => {
    return api.get<PageResult<Role>>('/system/roles', params)
  },

  getById: (id: number) => {
    return api.get<Role>(`/system/roles/${id}`)
  },

  create: (data: Partial<Role>) => {
    return api.post<Role>('/system/roles', data)
  },

  update: (id: number, data: Partial<Role>) => {
    return api.put<Role>(`/system/roles/${id}`, data)
  },

  delete: (id: number) => {
    return api.delete(`/system/roles/${id}`)
  },

  updateStatus: (id: number, status: number) => {
    return api.patch(`/system/roles/${id}/status`, { status })
  },

  getMenuTree: (id: number) => {
    return api.get<Menu[]>(`/system/roles/${id}/menus`)
  },

  assignMenus: (id: number, menuIds: number[]) => {
    return api.post(`/system/roles/${id}/menus`, { menuIds })
  },
}

export const deptApi = {
  list: (params: DeptQueryParams) => {
    return api.get<Department[]>('/system/departments', params)
  },

  getById: (id: number) => {
    return api.get<Department>(`/system/departments/${id}`)
  },

  create: (data: Partial<Department>) => {
    return api.post<Department>('/system/departments', data)
  },

  update: (id: number, data: Partial<Department>) => {
    return api.put<Department>(`/system/departments/${id}`, data)
  },

  delete: (id: number) => {
    return api.delete(`/system/departments/${id}`)
  },

  updateStatus: (id: number, status: number) => {
    return api.patch(`/system/departments/${id}/status`, { status })
  },
}

export const menuApi = {
  list: (params: MenuQueryParams) => {
    return api.get<Menu[]>('/system/menus', params)
  },

  getById: (id: number) => {
    return api.get<Menu>(`/system/menus/${id}`)
  },

  create: (data: Partial<Menu>) => {
    return api.post<Menu>('/system/menus', data)
  },

  update: (id: number, data: Partial<Menu>) => {
    return api.put<Menu>(`/system/menus/${id}`, data)
  },

  delete: (id: number) => {
    return api.delete(`/system/menus/${id}`)
  },

  updateStatus: (id: number, status: number) => {
    return api.patch(`/system/menus/${id}/status`, { status })
  },

  getMenuTree: () => {
    return api.get<Menu[]>('/system/menus/tree')
  },
}

export const postApi = {
  list: (params: ListParams) => {
    return api.get<PageResult<Post>>('/system/posts', params)
  },

  getById: (id: number) => {
    return api.get<Post>(`/system/posts/${id}`)
  },

  create: (data: Partial<Post>) => {
    return api.post<Post>('/system/posts', data)
  },

  update: (id: number, data: Partial<Post>) => {
    return api.put<Post>(`/system/posts/${id}`, data)
  },

  delete: (id: number) => {
    return api.delete(`/system/posts/${id}`)
  },

  updateStatus: (id: number, status: number) => {
    return api.patch(`/system/posts/${id}/status`, { status })
  },
}
