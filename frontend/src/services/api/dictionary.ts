import request from '@/lib/api';
import type { PageParams } from '@/types/api';

// Dictionary type interface
export interface DictionaryType {
  id: string;
  dictName: string;
  dictType: string;
  status: number;
  remark: string | null;
  createdAt: string;
  updatedAt: string;
}

// Dictionary data interface
export interface DictionaryData {
  id: string;
  dictSort: number;
  dictLabel: string;
  dictValue: string;
  dictType: string;
  cssClass: string | null;
  listClass: string | null;
  isDefault: string;
  status: number;
  remark: string | null;
  createdAt: string;
  updatedAt: string;
}

// Dictionary Type APIs
export const dictionaryTypeApi = {
  list(params: PageParams & { dictName?: string; dictType?: string }) {
    return request.get<{ list: DictionaryType[]; total: number }>(
      '/api/system/dictionary/types',
      { params }
    );
  },
  get(id: string) {
    return request.get<DictionaryType>(`/api/system/dictionary/types/${id}`);
  },
  create(data: Partial<DictionaryType>) {
    return request.post<DictionaryType>('/api/system/dictionary/types', data);
  },
  update(id: string, data: Partial<DictionaryType>) {
    return request.put<DictionaryType>(`/api/system/dictionary/types/${id}`, data);
  },
  delete(id: string) {
    return request.delete(`/api/system/dictionary/types/${id}`);
  },
  enable(id: string) {
    return request.put(`/api/system/dictionary/types/${id}/enable`);
  },
  disable(id: string) {
    return request.put(`/api/system/dictionary/types/${id}/disable`);
  },
  listAll() {
    return request.get<DictionaryType[]>('/api/system/dictionary/types/list-all');
  },
};

// Dictionary Data APIs
export const dictionaryDataApi = {
  list(params: PageParams & { dictType?: string }) {
    return request.get<{ list: DictionaryData[]; total: number }>(
      '/api/system/dictionary/data',
      { params }
    );
  },
  listByType(dictType: string) {
    return request.get<{ list: DictionaryData[]; total: number }>(
      `/api/system/dictionary/data/type/${dictType}`
    );
  },
  get(id: string) {
    return request.get<DictionaryData>(`/api/system/dictionary/data/${id}`);
  },
  create(data: Partial<DictionaryData>) {
    return request.post<DictionaryData>('/api/system/dictionary/data', data);
  },
  update(id: string, data: Partial<DictionaryData>) {
    return request.put<DictionaryData>(`/api/system/dictionary/data/${id}`, data);
  },
  delete(id: string) {
    return request.delete(`/api/system/dictionary/data/${id}`);
  },
  enable(id: string) {
    return request.put(`/api/system/dictionary/data/${id}/enable`);
  },
  disable(id: string) {
    return request.put(`/api/system/dictionary/data/${id}/disable`);
  },
  listAllByType(dictType: string) {
    return request.get<DictionaryData[]>(`/api/system/dictionary/data/type/${dictType}`);
  },
};

// Config APIs
export interface Config {
  id: string;
  configName: string;
  configKey: string;
  configValue: string;
  configType: string;
  isEncrypt: string;
  status: number;
  remark: string | null;
  createdAt: string;
  updatedAt: string;
}

export const configApi = {
  list(params: PageParams & { configKey?: string; configType?: string }) {
    return request.get<{ list: Config[]; total: number }>(
      '/api/system/config',
      { params }
    );
  },
  get(configKey: string) {
    return request.get<Config>(`/api/system/config/${configKey}`);
  },
  create(data: Partial<Config>) {
    return request.post<Config>('/api/system/config', data);
  },
  update(configKey: string, data: Partial<Config>) {
    return request.put<Config>(`/api/system/config/${configKey}`, data);
  },
  delete(configKey: string) {
    return request.delete(`/api/system/config/${configKey}`);
  },
};

// Notice APIs
export interface Notice {
  id: string;
  noticeTitle: string;
  noticeType: string;
  noticeContent: string | null;
  noticeStatus: string;
  isTop: string;
  priority: number;
  publishTime: string | null;
  viewCount: number;
  publisherId: string | null;
  publisherName: string | null;
  createdAt: string;
  updatedAt: string;
}

export const noticeApi = {
  list(params: PageParams & { noticeTitle?: string; noticeType?: string }) {
    return request.get<{ list: Notice[]; total: number }>(
      '/api/system/notice',
      { params }
    );
  },
  get(id: string) {
    return request.get<Notice>(`/api/system/notice/${id}`);
  },
  create(data: Partial<Notice>) {
    return request.post<Notice>('/api/system/notice', data);
  },
  update(id: string, data: Partial<Notice>) {
    return request.put<Notice>(`/api/system/notice/${id}`, data);
  },
  delete(id: string) {
    return request.delete(`/api/system/notice/${id}`);
  },
  incrementView(id: string) {
    return request.post<{ viewCount: number }>(`/api/system/notice/${id}/view`);
  },
};

export default {
  dictionaryType: dictionaryTypeApi,
  dictionaryData: dictionaryDataApi,
  config: configApi,
  notice: noticeApi,
};
