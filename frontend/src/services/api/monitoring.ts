import axios from 'axios';
import type { AxiosResponse } from 'axios';

// Base URL for monitoring API
const BASE_URL = '/api/monitoring';

// Auth-enabled axios instance for monitoring endpoints that require authentication
const authApi = axios.create({
  baseURL: BASE_URL,
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add auth interceptor
authApi.interceptors.request.use(
  (config) => {
    const token = localStorage.getItem('access_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

// Metrics API
export interface MetricsResponse {
  http_requests_total: number;
  http_request_duration_seconds_sum: number;
  http_request_duration_seconds_bucket: Array<[string, number]>;
  database_connections_active: number;
  database_connections_idle: number;
  database_connections_total: number;
  cpu_usage_percent: number;
  memory_used_bytes: number;
  memory_total_bytes: number;
  timestamp: string;
}

export async function fetchMetrics(): Promise<MetricsResponse> {
  try {
    const response: AxiosResponse<string> = await axios.get(`${BASE_URL}/metrics`, {
      responseType: 'text',
      headers: {
        'Content-Type': 'text/plain; charset=utf-8',
      },
    });

    const text = response.data;
    const metrics: MetricsResponse = {
      http_requests_total: 0,
      http_request_duration_seconds_sum: 0,
      http_request_duration_seconds_bucket: [],
      database_connections_active: 0,
      database_connections_idle: 0,
      database_connections_total: 0,
      cpu_usage_percent: 0,
      memory_used_bytes: 0,
      memory_total_bytes: 0,
      timestamp: new Date().toISOString(),
    };

    // Parse Prometheus text format
    // Lines can be: comments (#), help lines, type lines, or metric lines
    const lines = text.split('\n');

    for (const line of lines) {
      const trimmed = line.trim();
      if (!trimmed || trimmed.startsWith('#')) continue;

      // Metric line format: metric_name{labels} value
      const match = trimmed.match(/^([a-zA-Z_][a-zA-Z0-9_]*)(?:\{[^}]*\})?\s+([0-9.eE+-]+)/);
      if (!match) continue;

      const [, metricName, valueStr] = match;
      const value = parseFloat(valueStr);

      switch (metricName) {
        case 'http_requests_total':
          metrics.http_requests_total = value;
          break;
        case 'http_request_duration_seconds_sum':
          metrics.http_request_duration_seconds_sum = value;
          break;
        case 'http_request_duration_seconds_bucket':
          // Parse bucket with le (less than or equal) label
          const leMatch = trimmed.match(/le="([^"]+)"/);
          if (leMatch) {
            metrics.http_request_duration_seconds_bucket.push([leMatch[1], value]);
          }
          break;
        case 'database_connections_active':
          metrics.database_connections_active = value;
          break;
        case 'database_connections_idle':
          metrics.database_connections_idle = value;
          break;
        case 'database_connections_total':
          metrics.database_connections_total = value;
          break;
        case 'process_cpu_seconds_total':
          // Convert to percentage (assuming 1 minute window)
          metrics.cpu_usage_percent = value * 100;
          break;
        case 'process_resident_memory_bytes':
          metrics.memory_used_bytes = value;
          break;
        case 'node_memory_MemTotal_bytes':
          metrics.memory_total_bytes = value;
          break;
      }
    }

    return metrics;
  } catch (error) {
    console.error('Failed to fetch metrics:', error);
    throw error;
  }
}

// Health check API
export interface HealthCheckResponse {
  status: 'healthy' | 'degraded' | 'unhealthy';
  checks: {
    database: 'ok' | 'error' | 'warning';
    redis?: 'ok' | 'error' | 'warning';
    job_scheduler?: 'ok' | 'error' | 'warning';
  };
  timestamp: string;
}

export async function fetchHealthCheck(): Promise<HealthCheckResponse> {
  try {
    const response = await axios.get<HealthCheckResponse>(`${BASE_URL}/health`);
    return response.data;
  } catch (error) {
    console.error('Failed to fetch health check:', error);
    throw error;
  }
}

// Operation log API
export interface OperationLog {
  id: string;
  user_id: string;
  username: string;
  module: string;
  action_type: string;
  method: string;
  path: string;
  request_method: string;
  request_params?: string;
  response_code: number;
  response_message?: string;
  execution_time: number;
  ip_address: string;
  user_agent?: string;
  status: number;
  create_time: string;
}

export interface OperationLogListResponse {
  list: OperationLog[];
  total: number;
  page: number;
  page_size: number;
}

export interface OperationLogQueryParams {
  user_id?: string;
  username?: string;
  module?: string;
  action_type?: string;
  status?: number;
  start_time?: string;
  end_time?: string;
  page?: number;
  page_size?: number;
}

export async function fetchOperationLogs(
  params?: OperationLogQueryParams
): Promise<OperationLogListResponse> {
  try {
    const response = await authApi.get<OperationLogListResponse>(
      `/oper/logs`,
      { params }
    );
    return response.data;
  } catch (error) {
    console.error('Failed to fetch operation logs:', error);
    throw error;
  }
}

export async function createOperationLog(data: Partial<OperationLog>): Promise<void> {
  try {
    await authApi.post(`/oper/logs`, data);
  } catch (error) {
    console.error('Failed to create operation log:', error);
    throw error;
  }
}

export async function getOperationLog(id: string): Promise<OperationLog> {
  try {
    const response = await authApi.get<OperationLog>(`/oper/logs/${id}`);
    return response.data;
  } catch (error) {
    console.error('Failed to get operation log:', error);
    throw error;
  }
}

export async function deleteOperationLog(id: string): Promise<void> {
  try {
    await authApi.delete(`/oper/logs/${id}`);
  } catch (error) {
    console.error('Failed to delete operation log:', error);
    throw error;
  }
}

// Login log API
export interface LoginLog {
  id: string;
  user_id: string;
  username: string;
  ip_address: string;
  user_agent?: string;
  status: number; // 1=success, 0=failure
  message?: string;
  login_time: string;
  create_time: string;
}

export interface LoginLogListResponse {
  list: LoginLog[];
  total: number;
  page: number;
  page_size: number;
}

export interface LoginLogQueryParams {
  user_id?: string;
  username?: string;
  status?: number;
  start_time?: string;
  end_time?: string;
  page?: number;
  page_size?: number;
}

export async function fetchLoginLogs(
  params?: LoginLogQueryParams
): Promise<LoginLogListResponse> {
  try {
    const response = await authApi.get<LoginLogListResponse>(
      `/login/logs`,
      { params }
    );
    return response.data;
  } catch (error) {
    console.error('Failed to fetch login logs:', error);
    throw error;
  }
}

export async function getLoginLog(id: string): Promise<LoginLog> {
  try {
    const response = await authApi.get<LoginLog>(`/login/logs/${id}`);
    return response.data;
  } catch (error) {
    console.error('Failed to get login log:', error);
    throw error;
  }
}

// Online user API
export interface OnlineUser {
  session_id: string;
  user_id: string;
  username: string;
  dept_name?: string;
  ip_address: string;
  user_agent?: string;
  login_time: string;
  last_activity_time: string;
  expire_time: string;
  status: number; // 0: force logged out, 1: active
}

export interface OnlineUsersResponse {
  list: OnlineUser[];
  total: number;
}

export async function fetchOnlineUsers(): Promise<OnlineUsersResponse> {
  try {
    const response = await authApi.get<OnlineUsersResponse>(
      `/online/users`
    );
    return response.data;
  } catch (error) {
    console.error('Failed to fetch online users:', error);
    throw error;
  }
}

export async function forceLogout(sessionId: string, userId: string, reason?: string): Promise<void> {
  try {
    await authApi.post(`/online/users/force-logout`, {
      session_id: sessionId,
      user_id: userId,
      reason,
    });
  } catch (error) {
    console.error('Failed to force logout:', error);
    throw error;
  }
}