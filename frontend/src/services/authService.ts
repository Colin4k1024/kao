import request from '@/lib/api';
import { LoginRequest, LoginResponse, RegisterRequest, RefreshTokenRequest, User } from '@/types/auth';

export const authService = {
  login(data: LoginRequest) {
    return request.post<LoginResponse>('/api/auth/login', data);
  },
  
  register(data: RegisterRequest) {
    return request.post<User>('/api/auth/register', data);
  },
  
  logout() {
    return request.post('/api/auth/logout');
  },
  
  refreshToken(data: RefreshTokenRequest) {
    return request.post<LoginResponse>('/api/auth/refresh', data);
  },
  
  getCurrentUser() {
    return request.get<User>('/api/auth/me');
  },
};

export default authService;
