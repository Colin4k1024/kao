export interface ApiResponse<T = any> {
  code: number
  message: string
  data: T
}

export interface PageResult<T> {
  list: T[]
  total: number
  page: number
  pageSize: number
}

export interface PageParams {
  page?: number
  pageSize?: number
  keyword?: string
}

export interface ListParams extends PageParams {
  orderBy?: string
  orderDir?: 'asc' | 'desc'
}

export interface IdParam {
  id: number | string
}

export interface StatusParam extends IdParam {
  status: number
}

export interface BatchDeleteParam {
  ids: (number | string)[]
}

export interface LoginParams {
  username: string
  password: string
}

export interface LoginResult {
  access_token: string
  refresh_token: string
  token_type: string
  expires_in: number
  user: UserInfo
}

export interface UserInfo {
  id: number
  username: string
  nickname?: string
  avatar?: string
  email?: string
  phone?: string
  roles: string[]
  permissions: string[]
}

export interface MenuItem {
  id: number
  name: string
  path: string
  component?: string
  icon?: string
  parentId?: number
  orderNum?: number
  type: 'M' | 'C' | 'B'
  visible?: boolean
  status?: number
  children?: MenuItem[]
}
