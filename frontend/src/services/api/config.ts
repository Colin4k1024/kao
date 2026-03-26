import request from '@/lib/api';
import type { PageParams } from '@/types/api';

// Config interface
export interface Config {
  id: number;
  config_key: string;
  config_name: string;
  config_value: string;
  config_type: number;
  status: number;
  description?: string;
  created_at: string;
  updated_at: string;
}

// API service
export const configApi = {
  list(params: PageParams & { config_key?: string; config_name?: string }) {
    return request.get<{ list: Config[]; total: number }>(
      '/api/system/config',
      { params }
    );
  },
  get(id: number) {
    return request.get<Config>(`/api/system/config/${id}`);
  },
  create(data: Partial<Config>) {
    return request.post<Config>('/api/system/config', data);
  },
  update(id: number, data: Partial<Config>) {
    return request.put<Config>(`/api/system/config/${id}`, data);
  },
  delete(id: number) {
    return request.delete(`/api/system/config/${id}`);
  },
  preview(data: Partial<Config>) {
    return request.post<Config>('/api/system/config/preview', data);
  },
};

export default configApi;
