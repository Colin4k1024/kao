// Ant Design Pro login stubs
import request from '@/lib/api';

/** 发送验证码 POST /api/login/captcha */
export async function getFakeCaptcha(
  params: {
    phone?: string;
  } = {},
  options?: { [key: string]: any },
) {
  return request.get<API.FakeCaptcha>('/api/login/captcha', { params, ...options });
}
