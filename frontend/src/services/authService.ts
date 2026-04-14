import request from '@/lib/api';
import { LoginRequest, LoginResponse, RegisterRequest, RefreshTokenRequest, User } from '@/types/auth';

export const authService = {
  login(data: LoginRequest) {
    return request.post<LoginResponse>('/api/v1/login', data);
  },

  register(data: RegisterRequest) {
    return request.post<User>('/api/v1/register', data);
  },

  logout() {
    return request.post('/api/v1/logout');
  },

  refreshToken(data: RefreshTokenRequest) {
    return request.post<LoginResponse>('/api/v1/refresh', data);
  },

  getCurrentUser() {
    return request.get<User>('/api/v1/profile');
  },
};

export default authService;
