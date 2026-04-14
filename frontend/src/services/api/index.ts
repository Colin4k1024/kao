const BASE_URL = '/api';

function getHeaders(): HeadersInit {
  const token = localStorage.getItem('access_token');
  const headers: HeadersInit = {
    'Content-Type': 'application/json',
  };
  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }
  return headers;
}

async function request<T>(url: string, options: RequestInit = {}): Promise<T> {
  const response = await fetch(`${BASE_URL}${url}`, {
    ...options,
    headers: {
      ...getHeaders(),
      ...options.headers,
    },
  });

  const result = await response.json();

  if (result.code !== 0 && result.code !== undefined) {
    throw new Error(result.message || 'Request failed');
  }

  return result;
}

export async function login(data: { username: string; password: string }) {
  const response = await request<{ data: { access_token: string } }>('/v1/auth/login', {
    method: 'POST',
    body: JSON.stringify(data),
  });
  return response.data;
}

export async function getCurrentUser() {
  const response = await request<{ data: any }>('/v1/auth/profile');
  return response.data;
}

export async function getCurrentPermissions() {
  const response = await request<{ data: any }>('/v1/auth/permissions');
  return response.data;
}

export async function getCurrentMenus() {
  const response = await request<{ data: any }>('/v1/auth/menus');
  return response.data;
}

export async function queryUsers(params?: { page?: number; pageSize?: number }) {
  const searchParams = params ? `?page=${params.page || 1}&pageSize=${params.pageSize || 10}` : '';
  const response = await request<{ data: any[]; total: number }>(`/v1/users${searchParams}`);
  return response;
}

export async function createUser(data: Record<string, unknown>) {
  const response = await request<{ data: any }>('/v1/users', {
    method: 'POST',
    body: JSON.stringify(data),
  });
  return response;
}

export async function updateUser(id: string, data: Record<string, unknown>) {
  const response = await request<{ data: any }>(`/v1/users/${id}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  });
  return response;
}

export async function deleteUser(id: string) {
  const response = await request<{ data: any }>(`/v1/users/${id}`, {
    method: 'DELETE',
  });
  return response;
}

export async function queryRoles() {
  const response = await request<{ data: any[] }>('/v1/roles');
  return response;
}

export async function createRole(data: Record<string, unknown>) {
  const response = await request<{ data: any }>('/v1/roles', {
    method: 'POST',
    body: JSON.stringify(data),
  });
  return response;
}

export async function updateRole(id: string, data: Record<string, unknown>) {
  const response = await request<{ data: any }>(`/v1/roles/${id}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  });
  return response;
}

export async function deleteRole(id: string) {
  const response = await request<{ data: any }>(`/v1/roles/${id}`, {
    method: 'DELETE',
  });
  return response;
}

export async function queryDepartments() {
  const response = await request<{ data: any[] }>('/v1/departments');
  return response;
}

export async function createDepartment(data: Record<string, unknown>) {
  const response = await request<{ data: any }>('/v1/departments', {
    method: 'POST',
    body: JSON.stringify(data),
  });
  return response;
}

export async function updateDepartment(id: string, data: Record<string, unknown>) {
  const response = await request<{ data: any }>(`/v1/departments/${id}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  });
  return response;
}

export async function deleteDepartment(id: string) {
  const response = await request<{ data: any }>(`/v1/departments/${id}`, {
    method: 'DELETE',
  });
  return response;
}

export async function queryMenus() {
  const response = await request<{ data: any[] }>('/v1/menus');
  return response;
}

export async function createMenu(data: Record<string, unknown>) {
  const response = await request<{ data: any }>('/v1/menus', {
    method: 'POST',
    body: JSON.stringify(data),
  });
  return response;
}

export async function updateMenu(id: string, data: Record<string, unknown>) {
  const response = await request<{ data: any }>(`/v1/menus/${id}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  });
  return response;
}

export async function deleteMenu(id: string) {
  const response = await request<{ data: any }>(`/v1/menus/${id}`, {
    method: 'DELETE',
  });
  return response;
}

// Config APIs
export async function queryConfigs(params?: { page?: number; pageSize?: number; config_key?: string; config_name?: string }) {
  const searchParams = params
    ? `?page=${params.page || 1}&pageSize=${params.pageSize || 10}${params.config_key ? `&config_key=${params.config_key}` : ''}${params.config_name ? `&config_name=${params.config_name}` : ''}`
    : '';
  const response = await request<{ data: { items: any[]; total: number } }>(`/config${searchParams}`);
  return response;
}

export async function createConfig(data: Record<string, unknown>) {
  const response = await request<{ data: any }>('/config', {
    method: 'POST',
    body: JSON.stringify(data),
  });
  return response;
}

export async function updateConfig(config_key: string, data: Record<string, unknown>) {
  const response = await request<{ data: any }>(`/config/${config_key}`, {
    method: 'PUT',
    body: JSON.stringify(data),
  });
  return response;
}

export async function deleteConfig(config_key: string) {
  const response = await request<{ data: any }>(`/config/${config_key}`, {
    method: 'DELETE',
  });
  return response;
}
