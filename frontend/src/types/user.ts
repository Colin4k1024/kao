export interface User {
  id: number
  username: string
  nickname?: string
  avatar?: string
  email?: string
  phone?: string
  status: number
  deptId?: number
  deptName?: string
  roles?: Role[]
  createdAt: string
  updatedAt: string
}

export interface Role {
  id: number
  name: string
  code: string
  description?: string
  status: number
  orderNum?: number
  createdAt: string
  updatedAt: string
}

export interface Department {
  id: number
  name: string
  parentId?: number
  leader?: string
  phone?: string
  email?: string
  status: number
  orderNum?: number
  children?: Department[]
  createdAt: string
  updatedAt: string
}

export interface Post {
  id: number
  name: string
  code: string
  sort: number
  status: number
  remark?: string
  createdAt: string
  updatedAt: string
}

export interface Menu {
  id: number
  name: string
  parentId?: number
  path: string
  component?: string
  icon?: string
  type: 'M' | 'C' | 'B'
  sort: number
  visible: boolean
  status: number
  perms?: string
  remark?: string
  children?: Menu[]
  createdAt: string
  updatedAt: string
}

export interface LoginForm {
  username: string
  password: string
  captcha?: string
  captchaId?: string
}

export interface RegisterForm {
  username: string
  password: string
  confirmPassword: string
  email?: string
  phone?: string
}

export interface ChangePasswordForm {
  oldPassword: string
  newPassword: string
  confirmPassword: string
}

export interface UserQueryParams {
  username?: string
  nickname?: string
  status?: number
  deptId?: number
  phone?: string
  email?: string
}

export interface RoleQueryParams {
  name?: string
  code?: string
  status?: number
}

export interface MenuQueryParams {
  name?: string
  visible?: boolean
  status?: number
}

export interface DeptQueryParams {
  name?: string
  status?: number
  parentId?: number
}
