import request from '@/lib/api'
import type { ApiResponse, PageResult, ListParams } from '@/types/api'
import type { User, Role, Department, Menu, Post, UserQueryParams, RoleQueryParams, DeptQueryParams, MenuQueryParams } from '@/types/user'

export const userApi = {
  list: (params: ListParams & UserQueryParams) => {
    return request.get<PageResult<User>>('/system/users', { params })
  },

  getById: (id: number) => {
    return request.get<User>(`/system/users/${id}`)
  },

  create: (data: Partial<User>) => {
    return request.post<User>('/system/users', data)
  },

  update: (id: number, data: Partial<User>) => {
    return request.put<User>(`/system/users/${id}`, data)
  },

  delete: (id: number) => {
    return request.delete(`/system/users/${id}`)
  },

  batchDelete: (ids: number[]) => {
    return request.post('/system/users/batch-delete', { ids })
  },

  updateStatus: (id: number, status: number) => {
    return request.post(`/system/users/${id}/status`, { status })
  },

  resetPassword: (id: number) => {
    return request.post(`/system/users/${id}/reset-password`)
  },
}

export const roleApi = {
  list: (params: ListParams & RoleQueryParams) => {
    return request.get<PageResult<Role>>('/system/roles', { params })
  },

  getById: (id: number) => {
    return request.get<Role>(`/system/roles/${id}`)
  },

  create: (data: Partial<Role>) => {
    return request.post<Role>('/system/roles', data)
  },

  update: (id: number, data: Partial<Role>) => {
    return request.put<Role>(`/system/roles/${id}`, data)
  },

  delete: (id: number) => {
    return request.delete(`/system/roles/${id}`)
  },

  updateStatus: (id: number, status: number) => {
    return request.post(`/system/roles/${id}/status`, { status })
  },

  getMenuTree: (id: number) => {
    return request.get<Menu[]>(`/system/roles/${id}/menus`)
  },

  assignMenus: (id: number, menuIds: number[]) => {
    return request.post(`/system/roles/${id}/menus`, { menuIds })
  },
}

export const deptApi = {
  list: (params: DeptQueryParams) => {
    return request.get<Department[]>('/system/departments', { params })
  },

  getById: (id: number) => {
    return request.get<Department>(`/system/departments/${id}`)
  },

  create: (data: Partial<Department>) => {
    return request.post<Department>('/system/departments', data)
  },

  update: (id: number, data: Partial<Department>) => {
    return request.put<Department>(`/system/departments/${id}`, data)
  },

  delete: (id: number) => {
    return request.delete(`/system/departments/${id}`)
  },

  updateStatus: (id: number, status: number) => {
    return request.post(`/system/departments/${id}/status`, { status })
  },
}

export const menuApi = {
  list: (params: MenuQueryParams) => {
    return request.get<Menu[]>('/system/menus', { params })
  },

  getById: (id: number) => {
    return request.get<Menu>(`/system/menus/${id}`)
  },

  create: (data: Partial<Menu>) => {
    return request.post<Menu>('/system/menus', data)
  },

  update: (id: number, data: Partial<Menu>) => {
    return request.put<Menu>(`/system/menus/${id}`, data)
  },

  delete: (id: number) => {
    return request.delete(`/system/menus/${id}`)
  },

  updateStatus: (id: number, status: number) => {
    return request.post(`/system/menus/${id}/status`, { status })
  },

  getMenuTree: () => {
    return request.get<Menu[]>('/system/menus/tree')
  },
}

export const postApi = {
  list: (params: ListParams) => {
    return request.get<PageResult<Post>>('/system/posts', { params })
  },

  getById: (id: number) => {
    return request.get<Post>(`/system/posts/${id}`)
  },

  create: (data: Partial<Post>) => {
    return request.post<Post>('/system/posts', data)
  },

  update: (id: number, data: Partial<Post>) => {
    return request.put<Post>(`/system/posts/${id}`, data)
  },

  delete: (id: number) => {
    return request.delete(`/system/posts/${id}`)
  },

  updateStatus: (id: number, status: number) => {
    return request.post(`/system/posts/${id}/status`, { status })
  },
}
