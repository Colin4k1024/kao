import axios from 'axios';
import type { AxiosResponse } from 'axios';

// Base URL for monitoring API
const BASE_URL = '/api/system/monitor';

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
    
    // Parse Prometheus format - simplified parser
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
    
    // Simple mock for now
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
    const response = await axios.get<OperationLogListResponse>(
      `${BASE_URL}/oper/logs`,
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
    await axios.post(`${BASE_URL}/oper/logs`, data);
  } catch (error) {
    console.error('Failed to create operation log:', error);
    throw error;
  }
}

export async function getOperationLog(id: string): Promise<OperationLog> {
  try {
    const response = await axios.get<OperationLog>(`${BASE_URL}/oper/logs/${id}`);
    return response.data;
  } catch (error) {
    console.error('Failed to get operation log:', error);
    throw error;
  }
}

export async function deleteOperationLog(id: string): Promise<void> {
  try {
    await axios.delete(`${BASE_URL}/oper/logs/${id}`);
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
    const response = await axios.get<LoginLogListResponse>(
      `${BASE_URL}/login/logs`,
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
    const response = await axios.get<LoginLog>(`${BASE_URL}/login/logs/${id}`);
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
    const response = await axios.get<OnlineUsersResponse>(
      `${BASE_URL}/online/users`
    );
    return response.data;
  } catch (error) {
    console.error('Failed to fetch online users:', error);
    throw error;
  }
}

export async function forceLogout(sessionId: string, userId: string, reason?: string): Promise<void> {
  try {
    await axios.post(`${BASE_URL}/online/users/force-logout`, {
      session_id: sessionId,
      user_id: userId,
      reason,
    });
  } catch (error) {
    console.error('Failed to force logout:', error);
    throw error;
  }
}