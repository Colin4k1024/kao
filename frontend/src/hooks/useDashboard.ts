import { useState, useEffect } from 'react';
import request from '@/lib/api';

// Dashboard data interface
export interface DashboardData {
  totalUsers: number;
  totalDepartments: number;
  totalRoles: number;
  totalMenus: number;
  onlineUsers: number;
  systemStatus: 'healthy' | 'degraded' | 'unhealthy';
  cpuUsage: number;
  memoryUsage: number;
  diskUsage: number;
  lastLoginUsers: Array<{
    username: string;
    nickname: string;
    loginTime: string;
    ip: string;
  }>;
  recentJobs: Array<{
    jobName: string;
    status: string;
    executeTime?: string;
  }>;
}

// Dashboard hook
export const useDashboard = () => {
  const [data, setData] = useState<DashboardData>({
    totalUsers: 0,
    totalDepartments: 0,
    totalRoles: 0,
    totalMenus: 0,
    onlineUsers: 0,
    systemStatus: 'healthy',
    cpuUsage: 0,
    memoryUsage: 0,
    diskUsage: 0,
    lastLoginUsers: [],
    recentJobs: [],
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchData = async () => {
    setLoading(true);
    setError(null);
    try {
      // Fetch system metrics
      const metricsResponse = await request.get<{
        totalUsers: number;
        totalDepartments: number;
        totalRoles: number;
        totalMenus: number;
        onlineUsers: number;
        cpuUsage: number;
        memoryUsage: number;
        diskUsage: number;
      }>('/metrics');

      // Fetch system health status
      const healthResponse = await request.get<{
        status: 'healthy' | 'degraded' | 'unhealthy';
        components: Array<{
          name: string;
          status: 'healthy' | 'degraded' | 'unhealthy';
          message?: string;
        }>;
      }>('/api/system/health');

      // Fetch recent login users
      const loginResponse = await request.get<{ list: any[] }>(
        '/api/system/login/logs?page=1&pageSize=5&orderBy=created_at&orderDir=desc'
      );

      // Fetch recent job logs
      const jobResponse = await request.get<{ list: any[] }>(
        '/api/system/jobs/logs?page=1&pageSize=5'
      );

      // Format recent login users
      const lastLoginUsers = (loginResponse.list || []).slice(0, 5).map((user: any) => ({
        username: user.username || '未知',
        nickname: user.nickname || user.username || '未知',
        loginTime: user.created_at || new Date().toISOString(),
        ip: user.ip || '未知',
      }));

      // Format recent jobs
      const recentJobs = (jobResponse.list || []).slice(0, 5).map((job: any) => ({
        jobName: job.job_name || '未知任务',
        status: job.execute_status === 1 ? '成功' : job.execute_status === 2 ? '执行中' : '失败',
        executeTime: job.execute_time,
      }));

      setData({
        totalUsers: metricsResponse.totalUsers || 0,
        totalDepartments: metricsResponse.totalDepartments || 0,
        totalRoles: metricsResponse.totalRoles || 0,
        totalMenus: metricsResponse.totalMenus || 0,
        onlineUsers: metricsResponse.onlineUsers || 0,
        systemStatus: healthResponse.status || 'healthy',
        cpuUsage: metricsResponse.cpuUsage || 0,
        memoryUsage: metricsResponse.memoryUsage || 0,
        diskUsage: metricsResponse.diskUsage || 0,
        lastLoginUsers,
        recentJobs,
      });
    } catch (err) {
      setError(err instanceof Error ? err.message : '获取数据失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const refresh = () => {
    fetchData();
  };

  return {
    data,
    loading,
    error,
    refresh,
  };
};

export default useDashboard;
