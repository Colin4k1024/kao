import request from '@/lib/api';
import type { PageParams } from '@/types/api';

// Notice interface
export interface Notice {
  id: number;
  notice_title: string;
  notice_content: string;
  notice_type: number;
  status: number;
  created_by?: string;
  created_at: string;
  updated_at: string;
}

// API service
export const noticeApi = {
  list(params: PageParams & { notice_title?: string; notice_type?: number }) {
    return request.get<{ list: Notice[]; total: number }>(
      '/api/system/notice',
      { params }
    );
  },
  get(id: number) {
    return request.get<Notice>(`/api/system/notice/${id}`);
  },
  create(data: Partial<Notice>) {
    return request.post<Notice>('/api/system/notice', data);
  },
  update(id: number, data: Partial<Notice>) {
    return request.put<Notice>(`/api/system/notice/${id}`, data);
  },
  delete(id: number) {
    return request.delete(`/api/system/notice/${id}`);
  },
  publish(id: number) {
    return request.put(`/api/system/notice/${id}/publish`);
  },
  unpublish(id: number) {
    return request.put(`/api/system/notice/${id}/unpublish`);
  },
};

export default noticeApi;
