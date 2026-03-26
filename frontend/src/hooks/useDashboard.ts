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
    systemStatus: 'healthy' as const,
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

      // Format data
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
        lastLoginUsers: [],
        recentJobs: [],
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
