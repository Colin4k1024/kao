import { api } from '@/lib/api'
import type { ApiResponse, LoginParams, LoginResult, UserInfo } from '@/types/api'

export const authApi = {
  login: (params: LoginParams) => {
    return api.post('/api/v1/login', params).then((res: any) => {
      // res is the axios response, res.data is { code, message, data }
      const response = res.data as { code: number; message: string; data: LoginResult }
      return { code: response.code, data: response.data } as ApiResponse<LoginResult>
    })
  },

  logout: () => {
    return api.post('/api/v1/logout')
  },

  getUserInfo: () => {
    return api.get<UserInfo>('/api/v1/profile')
  },

  updatePassword: (oldPassword: string, newPassword: string) => {
    return api.post('/api/v1/change-password', { oldPassword, newPassword })
  },
}
