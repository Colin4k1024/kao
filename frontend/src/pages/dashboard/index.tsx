import { useState, useEffect } from 'react';
import {
  Card,
  Statistic,
  Row,
  Col,
  Table,
  Tag,
  Button,
} from 'antd';
import {
  ReloadOutlined,
} from '@ant-design/icons';
import type { ColumnType } from 'antd/es/table';

import {
  fetchMetrics,
  fetchHealthCheck,
  fetchOperationLogs,
  fetchLoginLogs,
  fetchOnlineUsers,
} from '@/services/api/monitoring';
import type { MetricsResponse, HealthCheckResponse } from '@/services/api/monitoring';

const DashboardPage = () => {
  const [metrics, setMetrics] = useState<MetricsResponse | null>(null);
  const [health, setHealth] = useState<HealthCheckResponse | null>(null);
  const [operationLogs, setOperationLogs] = useState<any[]>([]);
  const [loginLogs, setLoginLogs] = useState<any[]>([]);
  const [onlineUsers, setOnlineUsers] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);

  // Load data
  const loadData = async () => {
    setLoading(true);
    try {
      const [metricsData, healthData, onlineRes, operRes, loginRes] = await Promise.all([
        fetchMetrics(),
        fetchHealthCheck(),
        fetchOnlineUsers(),
        fetchOperationLogs(),
        fetchLoginLogs(),
      ]);
      setMetrics(metricsData);
      setHealth(healthData);
      setOnlineUsers(onlineRes.list || []);
      setOperationLogs(operRes.list || []);
      setLoginLogs(loginRes.list || []);
    } catch (error) {
      console.error('Failed to load data:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  const handleRefresh = () => {
    loadData();
  };

  const columns: ColumnType<any>[] = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
    },
    {
      title: 'Username',
      dataIndex: 'username',
      key: 'username',
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: number) =>
        status === 1 ? (
          <Tag color="success">Success</Tag>
        ) : (
          <Tag color="error">Failed</Tag>
        ),
    },
    {
      title: 'Create Time',
      dataIndex: 'create_time',
      key: 'create_time',
    },
  ];

  return (
    <div style={{ padding: 24 }}>
      <div
        style={{
          marginBottom: 24,
          display: 'flex',
          justifyContent: 'space-between',
          alignItems: 'center',
        }}
      >
        <h1 style={{ margin: 0 }}>System Monitoring Dashboard</h1>
        <Button
          type="primary"
          icon={<ReloadOutlined />}
          onClick={handleRefresh}
          loading={loading}
        >
          Refresh
        </Button>
      </div>

      {/* Metrics Section */}
      {metrics && (
        <Card title="System Metrics" style={{ marginBottom: 24 }}>
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
        <Card title="Health Check" style={{ marginBottom: 24 }}>
          <Row gutter={[16, 16]}>
            <Col span={8}>
              <Card>
                <Statistic
                  title="Overall Status"
                  value={health.status}
                  valueStyle={{
                    color:
                      health.status === 'healthy'
                        ? '#52c41a'
                        : health.status === 'degraded'
                        ? '#faad14'
                        : '#f5222d',
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
      <Table
        columns={columns}
        dataSource={[...operationLogs.slice(0, 5), ...loginLogs.slice(0, 5)]}
        pagination={{
          pageSize: 5,
          showSizeChanger: true,
        }}
        rowKey={(record: any) => record.id || Math.random()}
        style={{ marginBottom: 24 }}
      />
    </div>
  );
};

export default DashboardPage;
