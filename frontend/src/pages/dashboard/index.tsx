import { useState, useEffect } from 'react';
import {
  Card,
  Statistic,
  Row,
  Col,
  Table,
  Tag,
  Space,
  Button,
  message,
} from 'antd';
import {
  DownloadOutlined,
  ReloadOutlined,
  EyeOutlined,
  DeleteOutlined,
} from '@ant-design/icons';

import {
  fetchMetrics,
  fetchHealthCheck,
  OperationLog,
  fetchOperationLogs,
  deleteOperationLog,
  LoginLog,
  fetchLoginLogs,
  OnlineUser,
  fetchOnlineUsers,
  forceLogout,
} from '@/services/api/monitoring';
import type { MetricsResponse, HealthCheckResponse } from '@/services/api/monitoring';

const DashboardPage = () => {
  const [metrics, setMetrics] = useState<MetricsResponse | null>(null);
  const [health, setHealth] = useState<HealthCheckResponse | null>(null);
  const [operationLogs, setOperationLogs] = useState<OperationLog[]>([]);
  const [operationLogsTotal, setOperationLogsTotal] = useState(0);
  const [loginLogs, setLoginLogs] = useState<LoginLog[]>([]);
  const [loginLogsTotal, setLoginLogsTotal] = useState(0);
  const [onlineUsers, setOnlineUsers] = useState<OnlineUser[]>([]);
  
  const [loading, setLoading] = useState(false);
  
  // Load metrics
  const loadMetrics = async () => {
    try {
      const data = await fetchMetrics();
      setMetrics(data);
    } catch (error) {
      console.error('Failed to load metrics:', error);
    }
  };

  // Load health check
  const loadHealthCheck = async () => {
    try {
      const data = await fetchHealthCheck();
      setHealth(data);
    } catch (error) {
      console.error('Failed to load health check:', error);
    }
  };

  // Load metrics data
  const loadData = async () => {
    setLoading(true);
    try {
      const [metricsData, healthData, onlineRes, operRes] = await Promise.all([
        fetchMetrics(),
        fetchHealthCheck(),
        fetchOnlineUsers(),
        fetchOperationLogs(),
      ]);
      setMetrics(metricsData);
      setHealth(healthData);
      setOnlineUsers(onlineRes.list || []);
      setOperationLogs(operRes.list || []);
    } catch (error) {
      console.error('Failed to load data:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  // Refresh handler
  const handleRefresh = () => {
    loadData();
  };

  // Render
  return (
    <div className="monitoring-dashboard">
      <div className="dashboard-header">
        <h1>System Monitoring Dashboard</h1>
        <Button
          type="primary"
          icon={<RefreshOutlined />}
          onClick={loadData}
          loading={loading}
        >
          Refresh
        </Button>
      </div>

      {/* Metrics Section */}
      {metrics && (
        <Card title="System Metrics" className="metrics-card">
          <Row gutter={[16, 16]}>
            <Col span={8}>
              <Card>
                <Statistic
                  title="HTTP Requests"
                  value={metrics.http_requests_total}
                />
              </Card>
            </Col>
            <Col span={8}>
              <Card>
                <Statistic
                  title="Avg Response Time"
                  value={(metrics.http_request_duration_seconds_sum / metrics.http_requests_total * 1000).toFixed(2)}
                  suffix="ms"
                />
              </Card>
            </Col>
            <Col span={8}>
              <Card>
                <Statistic
                  title="CPU Usage"
                  value={metrics.cpu_usage_percent}
                  suffix="%"
                />
              </Card>
            </Col>
          </Row>
          <Row gutter={[16, 16]}>
            <Col span={12}>
              <Card>
                <Statistic
                  title="Database Connections"
                  value={metrics.database_connections_total}
                />
              </Card>
            </Col>
            <Col span={12}>
              <Card>
                <Statistic
                  title="Memory Usage"
                  value={`${(metrics.memory_used_bytes / 1024 / 1024).toFixed(2)} MB / ${(metrics.memory_total_bytes / 1024 / 1024).toFixed(2)} MB`}
                />
              </Card>
            </Col>
          </Row>
        </Card>
      )}

      {/* Health Check Section */}
      {health && (
        <Card title="Health Check" className="health-card">
          <Row gutter={[16, 16]}>
            <Col span={8}>
              <Card>
                <Statistic
                  title="Overall Status"
                  value={health.status}
                  valueStyle={{
                    color: health.status === 'healthy' ? '#52c41a' : health.status === 'degraded' ? '#faad14' : '#f5222d',
                  }}
                />
              </Card>
            </Col>
            <Col span={16}>
              <Card>
                <h3>Component Status:</h3>
                <ul>
                  <li>
                    <strong>Database:</strong> {health.checks.database}
                  </li>
                  {health.checks.redis && (
                    <li>
                      <strong>Redis:</strong> {health.checks.redis}
                    </li>
                  )}
                  {health.checks.job_scheduler && (
                    <li>
                      <strong>Job Scheduler:</strong> {health.checks.job_scheduler}
                    </li>
                  )}
                </ul>
              </Card>
            </Col>
          </Row>
        </Card>
      )}

      {/* Tabs */}
      <Tabs type="card">
        <TabPane tab="Operation Logs" key="operation">
          <Card>
            <Table
              columns={operationLogColumns}
              dataSource={operationLogs}
              pagination={{
                total: operationLogsTotal,
                showSizeChanger: true,
                showQuickJumper: true,
              }}
              rowKey="id"
            />
          </Card>
        </TabPane>

        <TabPane tab="Login Logs" key="login">
          <Card>
            <Table
              columns={loginLogColumns}
              dataSource={loginLogs}
              pagination={{
                total: loginLogsTotal,
                showSizeChanger: true,
                showQuickJumper: true,
              }}
              rowKey="id"
            />
          </Card>
        </TabPane>

        <TabPane tab="Online Users" key="online">
          <Card>
            <Table
              columns={onlineUserColumns}
              dataSource={onlineUsers}
              pagination={{
                total: onlineUsers.length,
                showSizeChanger: true,
                showQuickJumper: true,
              }}
              rowKey="session_id"
            />
          </Card>
        </TabPane>
      </Tabs>

      {/* Operation Log Detail Modal */}
      <Modal
        title="Operation Log Detail"
        open={operationLogModalOpen}
        onCancel={() => setOperationLogModalOpen(false)}
        footer={null}
        width={600}
      >
        {selectedOperationLog && (
          <div>
            <Form form={operationLogForm} layout="vertical">
              <Form.Item label="ID">
                {selectedOperationLog.id}
              </Form.Item>
              <Form.Item label="Username">
                {selectedOperationLog.username}
              </Form.Item>
              <Form.Item label="Module">
                {selectedOperationLog.module}
              </Form.Item>
              <Form.Item label="Action">
                {selectedOperationLog.action_type}
              </Form.Item>
              <Form.Item label="Method">
                {selectedOperationLog.method}
              </Form.Item>
              <Form.Item label="Path">
                {selectedOperationLog.path}
              </Form.Item>
              <Form.Item label="Request Method">
                {selectedOperationLog.request_method}
              </Form.Item>
              <Form.Item label="Response Code">
                {selectedOperationLog.response_code}
              </Form.Item>
              <Form.Item label="Execution Time">
                {selectedOperationLog.execution_time}ms
              </Form.Item>
              <Form.Item label="IP Address">
                {selectedOperationLog.ip_address}
              </Form.Item>
              <Form.Item label="Request Params">
                {selectedOperationLog.request_params || '-'}
              </Form.Item>
              <Form.Item label="Response Message">
                {selectedOperationLog.response_message || '-'}
              </Form.Item>
              <Form.Item label="Create Time">
                {selectedOperationLog.create_time}
              </Form.Item>
            </Form>
          </div>
        )}
      </Modal>
    </div>
  );
};

export default MonitoringDashboard;
