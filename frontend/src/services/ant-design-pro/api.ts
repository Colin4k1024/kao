// Ant Design Pro API stubs - these are auto-generated and not fully used in Vite setup
import request from '@/lib/api';

// These functions are stubs for the ant-design-pro components
// The actual API calls go through our services/api/* modules

/** 获取当前的用户 GET /api/currentUser */
export async function currentUser(options?: { [key: string]: any }) {
  return request.get<{ data: API.CurrentUser }>('/api/currentUser', options as any);
}

/** 退出登录接口 POST /api/login/outLogin */
export async function outLogin(options?: { [key: string]: any }) {
  return request.post<Record<string, any>>('/api/login/outLogin', undefined, options as any);
}

/** 登录接口 POST /api/login/account */
export async function login(body: API.LoginParams, options?: { [key: string]: any }) {
  return request.post<API.LoginResult>('/api/login/account', body, options as any);
}

/** 此处后端没有提供注释 GET /api/notices */
export async function getNotices(options?: { [key: string]: any }) {
  return request.get<API.NoticeIconList>('/api/notices', options as any);
}

/** 获取规则列表 GET /api/rule */
export async function rule(
  params: {
    current?: number;
    pageSize?: number;
  } = {},
  options?: { [key: string]: any },
) {
  return request.get<API.RuleList>('/api/rule', { params, ...options });
}

/** 更新规则 PUT /api/rule */
export async function updateRule(options?: { [key: string]: any }) {
  return request.post<API.RuleListItem>('/api/rule', { method: 'update', ...options });
}

/** 新建规则 POST /api/rule */
export async function addRule(options?: { [key: string]: any }) {
  return request.post<API.RuleListItem>('/api/rule', { method: 'post', ...options });
}

/** 删除规则 DELETE /api/rule */
export async function removeRule(options?: { [key: string]: any }) {
  return request.post<Record<string, any>>('/api/rule', { method: 'delete', ...options });
}
