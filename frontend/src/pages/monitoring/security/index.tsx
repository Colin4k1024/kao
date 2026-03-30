import React, { useState, useEffect } from 'react';
import {
  Card,
  Tabs,
  Table,
  Tag,
  Button,
  Space,
  Row,
  Col,
  Statistic,
  Alert,
  Descriptions,
  message,
  Spin,
  Empty,
  Modal,
  Tooltip,
} from 'antd';
import {
  SafetyCertificateOutlined,
  ReloadOutlined,
  CheckCircleOutlined,
  WarningOutlined,
  ExclamationCircleOutlined,
  LockOutlined,
  UnlockOutlined,
  SafetyOutlined,
  EyeOutlined,
} from '@ant-design/icons';
import type { TableColumnsType } from 'antd';

import {
  fetchSecurityScan,
  fetchSecurityScanByType,
  fetchSecurityEvents,
  fetchPasswordHealth,
  SecurityScanResult,
  SecurityCheck,
  SecurityEventsResponse,
  PasswordHealth,
  SecurityScanType,
  LockedAccount,
  FailedLoginAttempt,
  SuspiciousInput,
  PermissionDeniedEvent,
  BruteForceDetection,
} from '@/services/api/security';

const { TabPane } = Tabs;

const SecurityMonitoring: React.FC = () => {
  // Overall scan state
  const [scanLoading, setScanLoading] = useState(false);
  const [scanResult, setScanResult] = useState<SecurityScanResult | null>(null);
  const [lastScanTime, setLastScanTime] = useState<string>('');

  // Scan type tabs state
  const [activeTab, setActiveTab] = useState<string>('configuration');
  const [typeScanLoading, setTypeScanLoading] = useState(false);
  const [typeScanResult, setTypeScanResult] = useState<SecurityScanResult | null>(null);

  // Security events state
  const [eventsLoading, setEventsLoading] = useState(false);
  const [securityEvents, setSecurityEvents] = useState<SecurityEventsResponse | null>(null);

  // Password health state
  const [passwordHealthLoading, setPasswordHealthLoading] = useState(false);
  const [passwordHealth, setPasswordHealth] = useState<PasswordHealth[]>([]);
  const [selectedUserId, setSelectedUserId] = useState<string>('');
  const [passwordHealthModal, setPasswordHealthModal] = useState(false);
  const [currentPasswordHealth, setCurrentPasswordHealth] = useState<PasswordHealth | null>(null);

  // Load overall security scan
  const loadSecurityScan = async () => {
    setScanLoading(true);
    try {
      const data = await fetchSecurityScan();
      setScanResult(data);
      setLastScanTime(new Date().toLocaleString());
    } catch (error) {
      console.error('Failed to load security scan:', error);
      message.error('Failed to load security scan');
    } finally {
      setScanLoading(false);
    }
  };

  // Load scan by type
  const loadTypeScan = async (type: SecurityScanType) => {
    setTypeScanLoading(true);
    try {
      const data = await fetchSecurityScanByType(type);
      setTypeScanResult(data);
    } catch (error) {
      console.error(`Failed to load ${type} scan:`, error);
      message.error(`Failed to load ${type} scan`);
    } finally {
      setTypeScanLoading(false);
    }
  };

  // Load security events
  const loadSecurityEvents = async () => {
    setEventsLoading(true);
    try {
      const data = await fetchSecurityEvents();
      setSecurityEvents(data);
    } catch (error) {
      console.error('Failed to load security events:', error);
      message.error('Failed to load security events');
    } finally {
      setEventsLoading(false);
    }
  };

  // Load all password health (mock - would need user list API in real scenario)
  const loadPasswordHealth = async () => {
    setPasswordHealthLoading(true);
    try {
      // In a real scenario, we would fetch a list of users first
      // For now, we'll show how the API works with a sample user
      // The actual implementation would iterate through active users
      setPasswordHealth([]);
    } catch (error) {
      console.error('Failed to load password health:', error);
      message.error('Failed to load password health');
    } finally {
      setPasswordHealthLoading(false);
    }
  };

  // Initial load
  useEffect(() => {
    loadSecurityScan();
    loadTypeScan('configuration');
    loadSecurityEvents();
  }, []);

  // Handle tab change
  const handleTabChange = (key: string) => {
    setActiveTab(key);
    if (key === 'configuration' || key === 'input-validation' || key === 'authentication' || key === 'authorization') {
      loadTypeScan(key as SecurityScanType);
    } else if (key === 'events') {
      loadSecurityEvents();
    } else if (key === 'password') {
      loadPasswordHealth();
    }
  };

  // Get status color
  const getStatusColor = (status: string) => {
    switch (status) {
      case 'pass':
      case 'healthy':
        return 'success';
      case 'warning':
      case 'expiring_soon':
        return 'warning';
      case 'fail':
      case 'critical':
      case 'expired':
      case 'force_change':
        return 'error';
      default:
        return 'default';
    }
  };

  // Get status icon
  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'pass':
      case 'healthy':
        return <CheckCircleOutlined />;
      case 'warning':
        return <WarningOutlined />;
      case 'fail':
      case 'critical':
        return <ExclamationCircleOutlined />;
      default:
        return null;
    }
  };

  // Render check item
  const renderCheckItem = (check: SecurityCheck) => (
    <Card
      key={check.name}
      size="small"
      style={{ marginBottom: 8 }}
    >
      <Descriptions column={2} size="small">
        <Descriptions.Item label="Check">
          <Tag color={getStatusColor(check.status)}>
            {getStatusIcon(check.status)} {check.name}
          </Tag>
        </Descriptions.Item>
        <Descriptions.Item label="Status">
          <Tag color={getStatusColor(check.status)}>
            {check.status.toUpperCase()}
          </Tag>
        </Descriptions.Item>
        <Descriptions.Item label="Details" span={2}>
          {check.details}
        </Descriptions.Item>
      </Descriptions>
    </Card>
  );

  // Render scan summary cards
  const renderSummaryCards = () => {
    if (!scanResult) return null;

    const { summary, status } = scanResult;

    return (
      <Row gutter={16} style={{ marginBottom: 24 }}>
        <Col span={6}>
          <Card>
            <Statistic
              title="Total Checks"
              value={summary.total_checks}
              prefix={<SafetyCertificateOutlined />}
              valueStyle={{ color: '#1890ff' }}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic
              title="Passed"
              value={summary.passed_checks}
              prefix={<CheckCircleOutlined />}
              valueStyle={{ color: '#52c41a' }}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic
              title="Warnings"
              value={summary.warning_checks}
              prefix={<WarningOutlined />}
              valueStyle={{ color: '#faad14' }}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic
              title="Failed"
              value={summary.failed_checks}
              prefix={<ExclamationCircleOutlined />}
              valueStyle={{ color: '#ff4d4f' }}
            />
          </Card>
        </Col>
      </Row>
    );
  };

  // Render scan tab content
  const renderScanTabContent = () => {
    if (typeScanLoading) {
      return (
        <div style={{ textAlign: 'center', padding: 40 }}>
          <Spin size="large" />
        </div>
      );
    }

    if (!typeScanResult) {
      return <Empty description="No scan data available" />;
    }

    const { checks, summary, status } = typeScanResult;

    return (
      <div>
        {status !== 'healthy' && (
          <Alert
            message={`Security ${status === 'critical' ? 'Critical' : 'Warning'}`}
            description={`Found ${summary.failed_checks} failed and ${summary.warning_checks} warning checks`}
            type={status === 'critical' ? 'error' : 'warning'}
            showIcon
            style={{ marginBottom: 16 }}
          />
        )}

        {checks.length === 0 ? (
          <Empty description="No checks available" />
        ) : (
          checks.map(renderCheckItem)
        )}
      </div>
    );
  };

  // Locked accounts columns
  const lockedAccountsColumns: TableColumnsType<LockedAccount> = [
    {
      title: 'Username',
      dataIndex: 'username',
      key: 'username',
    },
    {
      title: 'Locked Until',
      dataIndex: 'locked_until',
      key: 'locked_until',
    },
    {
      title: 'Reason',
      dataIndex: 'reason',
      key: 'reason',
      render: (reason) => reason || '-',
    },
  ];

  // Failed login attempts columns
  const failedAttemptsColumns: TableColumnsType<FailedLoginAttempt> = [
    {
      title: 'Username',
      dataIndex: 'username',
      key: 'username',
    },
    {
      title: 'IP Address',
      dataIndex: 'ip_address',
      key: 'ip_address',
    },
    {
      title: 'Attempt Count',
      dataIndex: 'attempt_count',
      key: 'attempt_count',
      render: (count) => <Tag color={count > 3 ? 'error' : 'warning'}>{count}</Tag>,
    },
    {
      title: 'Last Attempt',
      dataIndex: 'last_attempt',
      key: 'last_attempt',
    },
  ];

  // Brute force columns
  const bruteForceColumns: TableColumnsType<BruteForceDetection> = [
    {
      title: 'IP Address',
      dataIndex: 'ip_address',
      key: 'ip_address',
    },
    {
      title: 'Attempt Count',
      dataIndex: 'attempt_count',
      key: 'attempt_count',
    },
    {
      title: 'Status',
      dataIndex: 'is_blocked',
      key: 'is_blocked',
      render: (isBlocked) => (
        <Tag color={isBlocked ? 'error' : 'success'}>
          {isBlocked ? <LockOutlined /> : <UnlockOutlined />} {isBlocked ? 'Blocked' : 'Active'}
        </Tag>
      ),
    },
    {
      title: 'Blocked Until',
      dataIndex: 'blocked_until',
      key: 'blocked_until',
      render: (until) => until || '-',
    },
  ];

  // Suspicious input columns
  const suspiciousInputColumns: TableColumnsType<SuspiciousInput> = [
    {
      title: 'Username',
      dataIndex: 'username',
      key: 'username',
      render: (username) => username || '-',
    },
    {
      title: 'IP Address',
      dataIndex: 'ip_address',
      key: 'ip_address',
    },
    {
      title: 'Event Type',
      dataIndex: 'event_type',
      key: 'event_type',
      render: (type) => <Tag color="warning">{type}</Tag>,
    },
    {
      title: 'Created At',
      dataIndex: 'created_at',
      key: 'created_at',
    },
    {
      title: 'Details',
      dataIndex: 'details',
      key: 'details',
      render: (details) => (
        <Tooltip title={JSON.stringify(details, null, 2)}>
          <Button type="link" icon={<EyeOutlined />}>
            View
          </Button>
        </Tooltip>
      ),
    },
  ];

  // Permission denied columns
  const permissionDeniedColumns: TableColumnsType<PermissionDeniedEvent> = [
    {
      title: 'Username',
      dataIndex: 'username',
      key: 'username',
      render: (username) => username || '-',
    },
    {
      title: 'IP Address',
      dataIndex: 'ip_address',
      key: 'ip_address',
    },
    {
      title: 'Event Type',
      dataIndex: 'event_type',
      key: 'event_type',
    },
    {
      title: 'Created At',
      dataIndex: 'created_at',
      key: 'created_at',
    },
  ];

  // Render events tab content
  const renderEventsTabContent = () => {
    if (eventsLoading) {
      return (
        <div style={{ textAlign: 'center', padding: 40 }}>
          <Spin size="large" />
        </div>
      );
    }

    if (!securityEvents) {
      return <Empty description="No security events available" />;
    }

    const { summary, locked_accounts, recent_failed_attempts, brute_force_detection, suspicious_inputs, permission_denied_events } = securityEvents;

    return (
      <div>
        {/* Summary */}
        <Row gutter={16} style={{ marginBottom: 24 }}>
          <Col span={6}>
            <Card>
              <Statistic
                title="Total Events"
                value={summary.total_events}
                prefix={<SafetyOutlined />}
              />
            </Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic
                title="Permission Denied"
                value={summary.permission_denied_count}
                prefix={<LockOutlined />}
                valueStyle={{ color: summary.permission_denied_count > 0 ? '#ff4d4f' : '#52c41a' }}
              />
            </Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic
                title="Suspicious Input"
                value={summary.suspicious_input_count}
                prefix={<WarningOutlined />}
                valueStyle={{ color: summary.suspicious_input_count > 0 ? '#faad14' : '#52c41a' }}
              />
            </Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic
                title="Brute Force Attempts"
                value={summary.brute_force_attempts}
                prefix={<ExclamationCircleOutlined />}
                valueStyle={{ color: summary.brute_force_attempts > 0 ? '#ff4d4f' : '#52c41a' }}
              />
            </Card>
          </Col>
        </Row>

        {/* Locked Accounts */}
        <Card title="Locked Accounts" size="small" style={{ marginBottom: 16 }}>
          {locked_accounts.length === 0 ? (
            <Empty description="No locked accounts" image={Empty.PRESENTED_IMAGE_SIMPLE} />
          ) : (
            <Table
              columns={lockedAccountsColumns}
              dataSource={locked_accounts}
              rowKey="user_id"
              pagination={false}
              size="small"
            />
          )}
        </Card>

        {/* Failed Login Attempts */}
        <Card title="Recent Failed Login Attempts" size="small" style={{ marginBottom: 16 }}>
          {recent_failed_attempts.length === 0 ? (
            <Empty description="No failed login attempts" image={Empty.PRESENTED_IMAGE_SIMPLE} />
          ) : (
            <Table
              columns={failedAttemptsColumns}
              dataSource={recent_failed_attempts}
              rowKey="last_attempt"
              pagination={false}
              size="small"
            />
          )}
        </Card>

        {/* Brute Force Detection */}
        <Card title="Brute Force Detection" size="small" style={{ marginBottom: 16 }}>
          {brute_force_detection.length === 0 ? (
            <Empty description="No brute force detection events" image={Empty.PRESENTED_IMAGE_SIMPLE} />
          ) : (
            <Table
              columns={bruteForceColumns}
              dataSource={brute_force_detection}
              rowKey="ip_address"
              pagination={false}
              size="small"
            />
          )}
        </Card>

        {/* Suspicious Inputs */}
        <Card title="Suspicious Inputs" size="small" style={{ marginBottom: 16 }}>
          {suspicious_inputs.length === 0 ? (
            <Empty description="No suspicious inputs detected" image={Empty.PRESENTED_IMAGE_SIMPLE} />
          ) : (
            <Table
              columns={suspiciousInputColumns}
              dataSource={suspicious_inputs}
              rowKey="id"
              pagination={false}
              size="small"
            />
          )}
        </Card>

        {/* Permission Denied Events */}
        <Card title="Permission Denied Events" size="small">
          {permission_denied_events.length === 0 ? (
            <Empty description="No permission denied events" image={Empty.PRESENTED_IMAGE_SIMPLE} />
          ) : (
            <Table
              columns={permissionDeniedColumns}
              dataSource={permission_denied_events}
              rowKey="id"
              pagination={false}
              size="small"
            />
          )}
        </Card>
      </div>
    );
  };

  // Password health columns
  const passwordHealthColumns: TableColumnsType<PasswordHealth> = [
    {
      title: 'Username',
      dataIndex: 'username',
      key: 'username',
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status) => (
        <Tag color={getStatusColor(status)}>
          {status.replace('_', ' ').toUpperCase()}
        </Tag>
      ),
    },
    {
      title: 'Days Remaining',
      dataIndex: 'days_remaining',
      key: 'days_remaining',
      render: (days) => days ?? '-',
    },
    {
      title: 'Expires At',
      dataIndex: 'expires_at',
      key: 'expires_at',
      render: (expires) => expires || '-',
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_, record) => (
        <Button
          type="link"
          icon={<EyeOutlined />}
          onClick={() => {
            setCurrentPasswordHealth(record);
            setPasswordHealthModal(true);
          }}
        >
          View Details
        </Button>
      ),
    },
  ];

  // Render password health tab content
  const renderPasswordHealthTabContent = () => {
    if (passwordHealthLoading) {
      return (
        <div style={{ textAlign: 'center', padding: 40 }}>
          <Spin size="large" />
        </div>
      );
    }

    return (
      <div>
        <Alert
          message="Password Health Monitoring"
          description="Monitor password expiration status and force password changes for accounts requiring attention."
          type="info"
          showIcon
          style={{ marginBottom: 16 }}
        />

        <Card
          title="Password Health Status"
          extra={
            <Space>
              <Button
                icon={<ReloadOutlined />}
                onClick={loadPasswordHealth}
                loading={passwordHealthLoading}
              >
                Refresh
              </Button>
            </Space>
          }
        >
          {passwordHealth.length === 0 ? (
            <Empty description="No password health data available. Enter a user ID to check individual password health." />
          ) : (
            <Table
              columns={passwordHealthColumns}
              dataSource={passwordHealth}
              rowKey="user_id"
              pagination={{ pageSize: 10 }}
            />
          )}
        </Card>
      </div>
    );
  };

  return (
    <div>
      {/* Header */}
      <Card
        title="Security Monitoring"
        extra={
          <Space>
            <span style={{ color: '#666', fontSize: 12 }}>
              Last scan: {lastScanTime || 'Never'}
            </span>
            <Button
              type="primary"
              icon={<SafetyCertificateOutlined />}
              onClick={loadSecurityScan}
              loading={scanLoading}
            >
              Full Scan
            </Button>
          </Space>
        }
      >
        {renderSummaryCards()}
      </Card>

      {/* Tabs */}
      <Card style={{ marginTop: 16 }}>
        <Tabs activeKey={activeTab} onChange={handleTabChange}>
          <TabPane tab="Configuration Security" key="configuration">
            {renderScanTabContent()}
          </TabPane>
          <TabPane tab="Input Validation" key="input-validation">
            {renderScanTabContent()}
          </TabPane>
          <TabPane tab="Authentication Security" key="authentication">
            {renderScanTabContent()}
          </TabPane>
          <TabPane tab="Authorization Security" key="authorization">
            {renderScanTabContent()}
          </TabPane>
          <TabPane tab="Security Events" key="events">
            {renderEventsTabContent()}
          </TabPane>
          <TabPane tab="Password Health" key="password">
            {renderPasswordHealthTabContent()}
          </TabPane>
        </Tabs>
      </Card>

      {/* Password Health Detail Modal */}
      <Modal
        title="Password Health Details"
        open={passwordHealthModal}
        onCancel={() => setPasswordHealthModal(false)}
        footer={null}
        width={500}
      >
        {currentPasswordHealth && (
          <Descriptions column={1} bordered size="small">
            <Descriptions.Item label="User ID">{currentPasswordHealth.user_id}</Descriptions.Item>
            <Descriptions.Item label="Username">{currentPasswordHealth.username}</Descriptions.Item>
            <Descriptions.Item label="Status">
              <Tag color={getStatusColor(currentPasswordHealth.status)}>
                {currentPasswordHealth.status.replace('_', ' ').toUpperCase()}
              </Tag>
            </Descriptions.Item>
            <Descriptions.Item label="Days Remaining">
              {currentPasswordHealth.days_remaining ?? 'N/A'}
            </Descriptions.Item>
            <Descriptions.Item label="Expires At">
              {currentPasswordHealth.expires_at || 'N/A'}
            </Descriptions.Item>
          </Descriptions>
        )}
      </Modal>
    </div>
  );
};

export default SecurityMonitoring;
