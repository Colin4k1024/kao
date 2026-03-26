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
  Modal,
  Tooltip,
  Popconfirm,
} from 'antd';
import {
  DownloadOutlined,
  ReloadOutlined,
} from '@ant-design/icons';
import type { TableRowSelection } from 'antd/es/table/interface';

import {
  fetchOnlineUsers,
  OnlineUser,
  forceLogout,
} from '@/services/api/monitoring';

const OnlineUserList = () => {
  const [users, setUsers] = useState<OnlineUser[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);
  const [searchForm] = Form.useForm();

  const [forceLogoutModal, setForceLogoutModal] = useState(false);
  const [userToLogout, setUserToLogout] = useState<{ session_id: string; user_id: string } | null>(null);
  const [logoutReason, setLogoutReason] = useState('');

  const loadData = async () => {
    setLoading(true);
    try {
      const data = await fetchOnlineUsers();
      setUsers(data.list);
      setTotal(data.total);
    } catch (error) {
      console.error('Failed to load online users:', error);
      message.error('Failed to load online users');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadData();

    // Auto-refresh every 15 seconds
    const interval = setInterval(() => {
      loadData();
    }, 15000);

    return () => clearInterval(interval);
  }, []);

  const handleSearch = (values: any) => {
    // In future, add search API
    console.log('Search values:', values);
  };

  const handleReset = () => {
    searchForm.resetFields();
    loadData();
  };

  const handleForceLogout = (record: OnlineUser) => {
    setUserToLogout({
      session_id: record.session_id,
      user_id: record.user_id,
    });
    setLogoutReason('');
    setForceLogoutModal(true);
  };

  const handleConfirmLogout = async () => {
    if (!userToLogout) return;

    try {
      await forceLogout(
        userToLogout.session_id,
        userToLogout.user_id,
        logoutReason || 'Manual logout by admin'
      );
      message.success('User logged out successfully');
      setForceLogoutModal(false);
      setUserToLogout(null);
      loadData();
    } catch (error) {
      message.error('Failed to force logout user');
    }
  };

  const handleDeleteSelected = async () => {
    if (selectedRowKeys.length === 0) {
      message.warning('Please select users to remove');
      return;
    }

    try {
      await Promise.all(
        selectedRowKeys.map((key) => {
          const user = users.find((u) => u.session_id === key);
          if (user) {
            return forceLogout(user.session_id, user.user_id, 'Removed from active session list');
          }
          return Promise.resolve();
        })
      );
      message.success('Selected users removed from session list');
      setSelectedRowKeys([]);
      loadData();
    } catch (error) {
      message.error('Failed to remove selected users');
    }
  };

  const columns: Table<OnlineUser>['columns'] = [
    {
      title: 'Session ID',
      dataIndex: 'session_id',
      key: 'session_id',
      width: 200,
    },
    {
      title: 'Username',
      dataIndex: 'username',
      key: 'username',
    },
    {
      title: 'Department',
      dataIndex: 'dept_name',
      key: 'dept_name',
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
        <Tag color={status === 1 ? 'success' : 'default'}>
          {status === 1 ? 'Active' : 'Force Logged Out'}
        </Tag>
      ),
    },
    {
      title: 'Last Activity',
      dataIndex: 'last_activity_time',
      key: 'last_activity_time',
    },
    {
      title: 'Expire Time',
      dataIndex: 'expire_time',
      key: 'expire_time',
    },
    {
      title: 'Actions',
      key: 'actions',
      width: 120,
      render: (_: any, record: OnlineUser) => (
        <Tooltip title="Force Logout">
          <Popconfirm
            title="Are you sure to force logout this user?"
            onConfirm={() => handleForceLogout(record)}
            okText="Yes"
            cancelText="No"
          >
            <Button danger size="small" icon={<ReloadOutlined />} />
          </Popconfirm>
        </Tooltip>
      ),
    },
  ];

  const rowSelection: TableRowSelection<OnlineUser> = {
    selectedRowKeys,
    onChange: (selectedRowKeys: React.Key[]) => {
      setSelectedRowKeys(selectedRowKeys);
    },
  };

  return (
    <Card
      title="Online Users"
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
          <Button
            danger
            onClick={handleDeleteSelected}
            disabled={selectedRowKeys.length === 0}
          >
            Remove Selected
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
            <Select.Option value={1}>Active</Select.Option>
            <Select.Option value={0}>Force Logged Out</Select.Option>
          </Select>
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
        dataSource={users}
        rowSelection={rowSelection}
        pagination={{
          total,
          showSizeChanger: true,
          showQuickJumper: true,
          showTotal: (total: number) => `Total ${total} users`,
        }}
        rowKey="session_id"
        loading={loading}
      />

      {/* Force Logout Modal */}
      <Modal
        title="Force Logout User"
        open={forceLogoutModal}
        onCancel={() => {
          setForceLogoutModal(false);
          setUserToLogout(null);
        }}
        onOk={handleConfirmLogout}
        okText="Force Logout"
        cancelText="Cancel"
        okButtonProps={{ danger: true }}
      >
        <Form form={searchForm} layout="vertical">
          <Form.Item label="Username">
            {userToLogout
              ? users.find((u) => u.session_id === userToLogout.session_id)?.username
              : '-'}
          </Form.Item>
          <Form.Item label="Session ID">
            {userToLogout?.session_id}
          </Form.Item>
          <Form.Item label="Reason">
            <Input.TextArea
              value={logoutReason}
              onChange={(e) => setLogoutReason(e.target.value)}
              placeholder="Enter logout reason"
              rows={3}
            />
          </Form.Item>
        </Form>
      </Modal>
    </Card>
  );
};

export default OnlineUserList;
