import React from 'react';
import { Card, Typography, Space, Alert, Table, Tag, Button, Descriptions } from 'antd';
import {
  DatabaseOutlined,
  ReloadOutlined,
  SyncOutlined,
  TableOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';

const { Title, Text } = Typography;

interface TableInfo {
  table_name: string;
  table_comment: string;
  row_count: number;
  data_size: string;
  index_size: string;
  engine: string;
}

const mockTables: TableInfo[] = [
  {
    table_name: 'sys_user',
    table_comment: '用户表',
    row_count: 100,
    data_size: '10MB',
    index_size: '5MB',
    engine: 'InnoDB',
  },
  {
    table_name: 'sys_department',
    table_comment: '部门表',
    row_count: 50,
    data_size: '2MB',
    index_size: '1MB',
    engine: 'InnoDB',
  },
  {
    table_name: 'sys_role',
    table_comment: '角色表',
    row_count: 20,
    data_size: '1MB',
    index_size: '0.5MB',
    engine: 'InnoDB',
  },
  {
    table_name: 'sys_menu',
    table_comment: '菜单表',
    row_count: 200,
    data_size: '5MB',
    index_size: '2MB',
    engine: 'InnoDB',
  },
  {
    table_name: 'sys_job',
    table_comment: '定时任务表',
    row_count: 30,
    data_size: '1MB',
    index_size: '0.5MB',
    engine: 'InnoDB',
  },
  {
    table_name: 'sys_oper_log',
    table_comment: '操作日志表',
    row_count: 5000,
    data_size: '100MB',
    index_size: '50MB',
    engine: 'InnoDB',
  },
];

const DatabaseManagement: React.FC = () => {
  const columns: ColumnsType<TableInfo> = [
    {
      title: '表名',
      dataIndex: 'table_name',
      key: 'table_name',
      render: (name) => <Tag icon={<TableOutlined />}>{name}</Tag>,
    },
    {
      title: '注释',
      dataIndex: 'table_comment',
      key: 'table_comment',
    },
    {
      title: '行数',
      dataIndex: 'row_count',
      key: 'row_count',
      sorter: (a, b) => a.row_count - b.row_count,
    },
    {
      title: '数据大小',
      dataIndex: 'data_size',
      key: 'data_size',
    },
    {
      title: '索引大小',
      dataIndex: 'index_size',
      key: 'index_size',
    },
    {
      title: '引擎',
      dataIndex: 'engine',
      key: 'engine',
      render: (engine) => <Tag color="blue">{engine}</Tag>,
    },
    {
      title: '操作',
      key: 'action',
      render: () => (
        <Button type="link" size="small">
          查看结构
        </Button>
      ),
    },
  ];

  return (
    <div style={{ padding: 24 }}>
      <Space direction="vertical" size="large" style={{ width: '100%' }}>
        <div>
          <Title level={4}>
            <DatabaseOutlined /> 数据库管理
          </Title>
          <Text type="secondary">
            数据库连接管理、表结构查看、数据预览等功能。
          </Text>
        </div>

        <Alert
          message="功能提示"
          description="数据库管理功能需要数据库管理员权限，普通用户仅可查看表结构信息。"
          type="info"
          showIcon
        />

        <Card title="数据库连接信息">
          <Descriptions column={3}>
            <Descriptions.Item label="数据库类型">PostgreSQL</Descriptions.Item>
            <Descriptions.Item label="数据库版本">16.2</Descriptions.Item>
            <Descriptions.Item label="连接状态">
              <Tag color="success">已连接</Tag>
            </Descriptions.Item>
            <Descriptions.Item label="主机地址">localhost:5432</Descriptions.Item>
            <Descriptions.Item label="当前数据库">kao_db</Descriptions.Item>
            <Descriptions.Item label="字符编码">UTF8</Descriptions.Item>
          </Descriptions>
        </Card>

        <Card
          title="数据表列表"
          extra={
            <Space>
              <Button icon={<ReloadOutlined />}>刷新</Button>
              <Button icon={<SyncOutlined />}>同步结构</Button>
            </Space>
          }
        >
          <Table
            columns={columns}
            dataSource={mockTables}
            rowKey="table_name"
            pagination={{
              pageSize: 10,
              showSizeChanger: true,
              showTotal: (total) => `共 ${total} 个表`,
            }}
          />
        </Card>
      </Space>
    </div>
  );
};

export default DatabaseManagement;
