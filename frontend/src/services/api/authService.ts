import api from '@/lib/api'
import type { ApiResponse, LoginParams, LoginResult, UserInfo } from '@/types/api'

export const authApi = {
  login: (params: LoginParams) => {
    return api.post<LoginResult>('/auth/login', params)
  },

  logout: () => {
    return api.post('/auth/logout')
  },

  getUserInfo: () => {
    return api.get<UserInfo>('/auth/userinfo')
  },

  updatePassword: (oldPassword: string, newPassword: string) => {
    return api.post('/auth/password', { oldPassword, newPassword })
  },
}
