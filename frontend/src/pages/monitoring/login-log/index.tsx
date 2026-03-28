import { useState, useEffect } from 'react';
import {
  Card,
  Table,
  Tag,
  Space,
  Button,
 	message,
  Form,
  Input,
  Select,
  DatePicker,
  Modal,
} from 'antd';
import { DownloadOutlined, ReloadOutlined, EyeOutlined } from '@ant-design/icons';
import type { TableRowSelection } from 'antd/es/table/interface';

import {
  fetchLoginLogs,
  LoginLog,
} from '@/services/api/monitoring';

const LoginLogList = () => {
  const [logs, setLogs] = useState<LoginLog[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);
  const [searchForm] = Form.useForm();
  
  const [viewModalOpen, setViewModalOpen] = useState(false);
  const [selectedLog, setSelectedLog] = useState<LoginLog | null>(null);
  const [viewForm] = Form.useForm();

  const loadData = async (filters?: any) => {
    setLoading(true);
    try {
      const params: any = {
        page: 1,
        page_size: 10,
      };
      
      if (filters) {
        Object.assign(params, filters);
      }
      
      const data = await fetchLoginLogs(params);
      setLogs(data.list);
      setTotal(data.total);
    } catch (error) {
      console.error('Failed to load login logs:', error);
      message.error('Failed to load login logs');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();
  }, []);

  const handleSearch = (values: any) => {
    loadData(values);
  };

  const handleReset = () => {
    searchForm.resetFields();
    loadData();
  };

  const handleView = (record: LoginLog) => {
    setSelectedLog(record);
    viewForm.setFieldsValue(record);
    setViewModalOpen(true);
  };

  const columns = [
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
      title: 'IP Address',
      dataIndex: 'ip_address',
      key: 'ip_address',
    },
    {
      title: 'User Agent',
      dataIndex: 'user_agent',
      key: 'user_agent',
      ellipsis: true,
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (status: number) => (
        <Tag color={status === 1 ? 'success' : 'error'}>
          {status === 1 ? 'Success' : 'Failed'}
        </Tag>
      ),
    },
    {
      title: 'Message',
      dataIndex: 'message',
      key: 'message',
    },
    {
      title: 'Login Time',
      dataIndex: 'login_time',
      key: 'login_time',
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_: any, record: LoginLog) => (
        <Space>
          <Button
            type="link"
            icon={<EyeOutlined />}
            onClick={() => handleView(record)}
          >
            View
          </Button>
        </Space>
      ),
    },
  ];

  const rowSelection: TableRowSelection<LoginLog> = {
    selectedRowKeys,
    onChange: (selectedRowKeys: React.Key[]) => {
      setSelectedRowKeys(selectedRowKeys);
    },
  };

  return (
    <Card
      title="Login Logs"
      extra={
        <Space>
          <Button
            type="primary"
            icon={<ReloadOutlined />}
            onClick={() => loadData()}
            loading={loading}
          >
            Refresh
          </Button>
          <Button icon={<DownloadOutlined />}>
            Export
          </Button>
        </Space>
      }
    >
      {/* Search Form */}
      <Form
        form={searchForm}
        layout="inline"
        onFinish={handleSearch}
        style={{ marginBottom: 16 }}
      >
        <Form.Item name="username" label="Username">
          <Input placeholder="Username" />
        </Form.Item>
        <Form.Item name="ip_address" label="IP Address">
          <Input placeholder="IP Address" />
        </Form.Item>
        <Form.Item name="status" label="Status">
          <Select placeholder="Status" allowClear>
            <Select.Option value={1}>Success</Select.Option>
            <Select.Option value={0}>Failed</Select.Option>
          </Select>
        </Form.Item>
        <Form.Item name="create_time_range" label="Time Range">
          <DatePicker.RangePicker format="YYYY-MM-DD HH:mm:ss" />
        </Form.Item>
        <Form.Item>
          <Space>
            <Button type="primary" htmlType="submit">
              Search
            </Button>
            <Button htmlType="button" onClick={handleReset}>
              Reset
            </Button>
          </Space>
        </Form.Item>
      </Form>

      {/* Table */}
      <Table
        columns={columns}
        dataSource={logs}
        rowSelection={rowSelection}
        pagination={{
          total,
          showSizeChanger: true,
          showQuickJumper: true,
          showTotal: (total: number) => `Total ${total} items`,
        }}
        rowKey="id"
        loading={loading}
      />

      {/* View Modal */}
      <Modal
        title="Login Log Detail"
        open={viewModalOpen}
        onCancel={() => setViewModalOpen(false)}
        footer={null}
        width={700}
      >
        {selectedLog && (
          <Form form={viewForm} layout="vertical">
            <Form.Item label="ID">{selectedLog.id}</Form.Item>
            <Form.Item label="Username">{selectedLog.username}</Form.Item>
            <Form.Item label="IP Address">{selectedLog.ip_address}</Form.Item>
            <Form.Item label="User Agent">
              {selectedLog.user_agent || '-'}
            </Form.Item>
            <Form.Item label="Status">
              <Tag color={selectedLog.status === 1 ? 'success' : 'error'}>
                {selectedLog.status === 1 ? 'Success' : 'Failed'}
              </Tag>
            </Form.Item>
            <Form.Item label="Message">{selectedLog.message}</Form.Item>
            <Form.Item label="Login Time">{selectedLog.login_time}</Form.Item>
            <Form.Item label="Create Time">{selectedLog.create_time}</Form.Item>
          </Form>
        )}
      </Modal>
    </Card>
  );
};

export default LoginLogList;
