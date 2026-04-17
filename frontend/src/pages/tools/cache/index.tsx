import React from 'react';
import { Card, Typography, Space, Alert, Row, Col, Statistic, Button, Table, Tag, Progress } from 'antd';
import {
  HddOutlined,
  ReloadOutlined,
  DeleteOutlined,
  ClockCircleOutlined,
  KeyOutlined,
  DatabaseOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';

const { Title, Text } = Typography;

interface CacheKey {
  key: string;
  type: string;
  ttl: number;
  size: string;
  hit_rate: number;
}

const mockCacheKeys: CacheKey[] = [
  { key: 'user:session:admin', type: 'hash', ttl: 3600, size: '1KB', hit_rate: 95 },
  { key: 'user:session:test', type: 'hash', ttl: 3600, size: '1KB', hit_rate: 88 },
  { key: 'menu:tree', type: 'string', ttl: 7200, size: '5KB', hit_rate: 92 },
  { key: 'dict:type:all', type: 'string', ttl: 86400, size: '10KB', hit_rate: 85 },
  { key: 'role:permissions:1', type: 'set', ttl: 3600, size: '2KB', hit_rate: 90 },
  { key: 'dept:tree', type: 'string', ttl: 7200, size: '3KB', hit_rate: 87 },
];

const CacheManagement: React.FC = () => {
  const columns: ColumnsType<CacheKey> = [
    {
      title: '缓存Key',
      dataIndex: 'key',
      key: 'key',
      render: (key) => <Tag icon={<KeyOutlined />}>{key}</Tag>,
    },
    {
      title: '类型',
      dataIndex: 'type',
      key: 'type',
      render: (type) => <Tag color="blue">{type}</Tag>,
    },
    {
      title: 'TTL',
      dataIndex: 'ttl',
      key: 'ttl',
      render: (ttl) => (
        <Space>
          <ClockCircleOutlined />
          {ttl}s
        </Space>
      ),
    },
    {
      title: '大小',
      dataIndex: 'size',
      key: 'size',
    },
    {
      title: '命中率',
      dataIndex: 'hit_rate',
      key: 'hit_rate',
      render: (rate) => (
        <Progress
          percent={rate}
          size="small"
          status={rate > 90 ? 'success' : rate > 70 ? 'normal' : 'exception'}
        />
      ),
    },
    {
      title: '操作',
      key: 'action',
      render: () => (
        <Button type="link" danger size="small" icon={<DeleteOutlined />}>
          删除
        </Button>
      ),
    },
  ];

  return (
    <div style={{ padding: 24 }}>
      <Space direction="vertical" size="large" style={{ width: '100%' }}>
        <div>
          <Title level={4}>
            <HddOutlined /> 缓存管理
          </Title>
          <Text type="secondary">
            系统缓存监控、Redis状态查看、缓存清理等功能。
          </Text>
        </div>

        <Alert
          message="功能提示"
          description="缓存管理功能用于监控系统缓存使用情况，支持查看缓存键、清理过期缓存等操作。"
          type="info"
          showIcon
        />

        <Row gutter={16}>
          <Col span={6}>
            <Card>
              <Statistic
                title="Redis连接状态"
                value="在线"
                prefix={<Tag color="success">●</Tag>}
              />
            </Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic
                title="缓存键数量"
                value={1234}
                prefix={<DatabaseOutlined />}
              />
            </Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic
                title="内存使用"
                value={45}
                suffix="%"
                prefix={<HddOutlined />}
                valueStyle={{ color: '#1890ff' }}
              />
            </Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic
                title="平均命中率"
                value={91}
                suffix="%"
                prefix={<ClockCircleOutlined />}
                valueStyle={{ color: '#52c41a' }}
              />
            </Card>
          </Col>
        </Row>

        <Card
          title="Redis服务信息"
          extra={
            <Space>
              <Button icon={<ReloadOutlined />}>刷新</Button>
              <Button danger icon={<DeleteOutlined />}>
                清理所有缓存
              </Button>
            </Space>
          }
        >
          <Row gutter={16}>
            <Col span={8}>
              <Statistic title="Redis版本" value="7.2.4" />
            </Col>
            <Col span={8}>
              <Statistic title="运行时间" value="15天 6小时" />
            </Col>
            <Col span={8}>
              <Statistic title="连接数" value={25} />
            </Col>
          </Row>
        </Card>

        <Card title="缓存键列表">
          <Table
            columns={columns}
            dataSource={mockCacheKeys}
            rowKey="key"
            pagination={{
              pageSize: 10,
              showSizeChanger: true,
              showTotal: (total) => `共 ${total} 个键`,
            }}
          />
        </Card>
      </Space>
    </div>
  );
};

export default CacheManagement;
