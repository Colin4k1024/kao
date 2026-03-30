import request from '@/lib/api';
import type { PageParams } from '@/types/api';

// Job interface
export interface Job {
  id: number;
  job_name: string;
  job_code: string;
  job_group: string;
  job_status: number;
  cron_expression: string;
  retry_count: number;
  retry_interval: number;
  timeout: number;
  description?: string;
  created_by?: string;
  created_at: string;
  updated_at: string;
}

// Job log interface
export interface JobLog {
  id: number;
  job_id: number;
  job_name: string;
  job_code: string;
  job_group: string;
  execute_status: number;
  execute_message?: string;
  execute_time: string;
  created_at: string;
}

// API service
export const jobApi = {
  // Job APIs
  list(params: PageParams & { job_name?: string; job_code?: string; job_status?: number }) {
    return request.get<{ list: Job[]; total: number }>(
      '/api/jobs',
      { params }
    );
  },
  get(id: number) {
    return request.get<Job>(`/api/jobs/${id}`);
  },
  create(data: Partial<Job>) {
    return request.post<Job>('/api/jobs', data);
  },
  update(id: number, data: Partial<Job>) {
    return request.put<Job>(`/api/jobs/${id}`, data);
  },
  delete(id: number) {
    return request.delete(`/api/jobs/${id}`);
  },
  schedule(id: number) {
    return request.put(`/api/jobs/${id}/schedule`);
  },
  unschedule(id: number) {
    return request.put(`/api/jobs/${id}/unschedule`);
  },
  runOnce(id: number) {
    return request.post(`/api/jobs/${id}/run`);
  },

  // Job log APIs
  logs(params: PageParams & { job_id?: number; job_name?: string; execute_status?: number }) {
    return request.get<{ list: JobLog[]; total: number }>(
      '/api/jobs/logs',
      { params }
    );
  },
  getLog(id: number) {
    return request.get<JobLog>(`/api/jobs/logs/${id}`);
  },
  clearLog(jobId?: number) {
    return request.delete(
      jobId
        ? `/api/jobs/logs/clear?job_id=${jobId}`
        : '/api/jobs/logs/clear'
    );
  },
};

// Cron validator
export const cronValidator = {
  isValid(cronExpression: string): boolean {
    if (!cronExpression) return false;
    const parts = cronExpression.trim().split(/\s+/);
    if (parts.length !== 6 && parts.length !== 7) {
      return false;
    }
    // Basic validation for cron format
    const validPatterns = [
      /^[0-9]+$/, // Seconds
      /^[0-9]+$/, // Minutes
      /^[0-9]+$/, // Hours
      /^[\d\-,\/\*]*$/, // Day of month
      /^[a-zA-Z\-\*,\/\?]*$/, // Month
      /^[0-6\-\*,\/\?]*$/, // Day of week
      /^[0-9]*$/, // Year (optional)
    ];
    if (parts.length === 6) {
      parts.push('*');
    }
    for (let i = 0; i < parts.length; i++) {
      const pattern = validPatterns[i] || /^[\d\-\*,\/\?]*$/;
      if (!pattern.test(parts[i])) {
        return false;
      }
    }
    return true;
  },

  validateWithMessage(cronExpression: string): { valid: boolean; message?: string } {
    if (!cronExpression) {
      return { valid: false, message: 'Cron表达式不能为空' };
    }
    const parts = cronExpression.trim().split(/\s+/);
    if (parts.length !== 6 && parts.length !== 7) {
      return {
        valid: false,
        message: 'Cron表达式必须由6或7个字段组成（秒 分 时 日 月 周 [年]）',
      };
    }
    const fieldNames = ['秒', '分', '时', '日', '月', '周'];
    if (parts.length === 7) {
      fieldNames.push('年');
    }
    const maxValues = [59, 59, 23, 31, 12, 7, 2099];
    for (let i = 0; i < parts.length; i++) {
      const part = parts[i];
      const max = maxValues[i];
      if (part !== '*' && part !== '?' && !part.includes('-') && !part.includes('/') && !part.includes(',')) {
        const num = parseInt(part, 10);
        if (isNaN(num) || num < 0 || num > max) {
          return {
            valid: false,
            message: `${fieldNames[i]}字段的值必须在0-${max}之间，或为*, ?等特殊字符`,
          };
        }
      }
    }
    return { valid: true };
  },

  getNextRuns(cronExpression: string, count: number = 5): string[] {
    // Simple implementation - in production use a real cron parser
    const result: string[] = [];
    const validation = this.validateWithMessage(cronExpression);
    if (!validation.valid) {
      return result;
    }
    // Generate mock next run times
    const now = new Date();
    for (let i = 0; i < count; i++) {
      const future = new Date(now.getTime() + (i + 1) * 60 * 60 * 1000);
      result.push(future.toLocaleString('zh-CN'));
    }
    return result;
  },
};

export default jobApi;
