import request from '@/lib/api'
import type { ApiResponse, PageResult, ListParams } from '@/types/api'
import type { User, Role, Department, Menu, Post, UserQueryParams, RoleQueryParams, DeptQueryParams, MenuQueryParams } from '@/types/user'

export const userApi = {
  list: (params: ListParams & UserQueryParams) => {
    return request.get<PageResult<User>>('/api/v1/users', { params })
  },

  getById: (id: number) => {
    return request.get<User>(`/api/v1/users/${id}`)
  },

  create: (data: Partial<User>) => {
    return request.post<User>('/api/v1/users', data)
  },

  update: (id: number, data: Partial<User>) => {
    return request.put<User>(`/api/v1/users/${id}`, data)
  },

  delete: (id: number) => {
    return request.delete(`/api/v1/users/${id}`)
  },

  batchDelete: (ids: number[]) => {
    return request.post('/api/v1/users/batch-delete', { ids })
  },

  updateStatus: (id: number, status: number) => {
    return request.post(`/api/v1/users/${id}/status`, { status })
  },

  resetPassword: (id: number) => {
    return request.post(`/api/v1/users/${id}/reset-password`)
  },
}

export const roleApi = {
  list: (params: ListParams & RoleQueryParams) => {
    return request.get<PageResult<Role>>('/api/v1/roles', { params })
  },

  getById: (id: number) => {
    return request.get<Role>(`/api/v1/roles/${id}`)
  },

  create: (data: Partial<Role>) => {
    return request.post<Role>('/api/v1/roles', data)
  },

  update: (id: number, data: Partial<Role>) => {
    return request.put<Role>(`/api/v1/roles/${id}`, data)
  },

  delete: (id: number) => {
    return request.delete(`/api/v1/roles/${id}`)
  },

  updateStatus: (id: number, status: number) => {
    return request.post(`/api/v1/roles/${id}/status`, { status })
  },

  getMenuTree: (id: number) => {
    return request.get<Menu[]>(`/api/v1/roles/${id}/menus`)
  },

  assignMenus: (id: number, menuIds: number[]) => {
    return request.post(`/api/v1/roles/${id}/menus`, { menuIds })
  },
}

export const deptApi = {
  list: (params: DeptQueryParams) => {
    return request.get<Department[]>('/api/v1/departments', { params })
  },

  getById: (id: number) => {
    return request.get<Department>(`/api/v1/departments/${id}`)
  },

  create: (data: Partial<Department>) => {
    return request.post<Department>('/api/v1/departments', data)
  },

  update: (id: number, data: Partial<Department>) => {
    return request.put<Department>(`/api/v1/departments/${id}`, data)
  },

  delete: (id: number) => {
    return request.delete(`/api/v1/departments/${id}`)
  },

  updateStatus: (id: number, status: number) => {
    return request.post(`/api/v1/departments/${id}/status`, { status })
  },
}

export const menuApi = {
  list: (params: MenuQueryParams) => {
    return request.get<Menu[]>('/api/v1/menus', { params })
  },

  getById: (id: number) => {
    return request.get<Menu>(`/api/v1/menus/${id}`)
  },

  create: (data: Partial<Menu>) => {
    return request.post<Menu>('/api/v1/menus', data)
  },

  update: (id: number, data: Partial<Menu>) => {
    return request.put<Menu>(`/api/v1/menus/${id}`, data)
  },

  delete: (id: number) => {
    return request.delete(`/api/v1/menus/${id}`)
  },

  updateStatus: (id: number, status: number) => {
    return request.post(`/api/v1/menus/${id}/status`, { status })
  },

  getMenuTree: () => {
    return request.get<Menu[]>('/api/v1/menus/tree')
  },
}

export const postApi = {
  list: (params: ListParams) => {
    return request.get<PageResult<Post>>('/api/v1/posts', { params })
  },

  getById: (id: number) => {
    return request.get<Post>(`/api/v1/posts/${id}`)
  },

  create: (data: Partial<Post>) => {
    return request.post<Post>('/api/v1/posts', data)
  },

  update: (id: number, data: Partial<Post>) => {
    return request.put<Post>(`/api/v1/posts/${id}`, data)
  },

  delete: (id: number) => {
    return request.delete(`/api/v1/posts/${id}`)
  },

  updateStatus: (id: number, status: number) => {
    return request.post(`/api/v1/posts/${id}/status`, { status })
  },
}
