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
} from 'antd';
import {
  DownloadOutlined,
  ReloadOutlined,
  EyeOutlined,
  DeleteOutlined,
} from '@ant-design/icons';
import type { TableRowSelection } from 'antd/es/table/interface';

import {
  fetchOperationLogs,
  deleteOperationLog,
  OperationLog,
} from '@/services/api/monitoring';

const OperationLogList = () => {
  const [logs, setLogs] = useState<OperationLog[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(false);
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);
  const [searchForm] = Form.useForm();
  
  const [viewModalOpen, setViewModalOpen] = useState(false);
  const [selectedLog, setSelectedLog] = useState<OperationLog | null>(null);
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
      
      const data = await fetchOperationLogs(params);
      setLogs(data.list);
      setTotal(data.total);
    } catch (error) {
      console.error('Failed to load operation logs:', error);
      message.error('Failed to load operation logs');
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

  const handleDelete = async (id: string) => {
    try {
      await deleteOperationLog(id);
      message.success('Operation log deleted successfully');
      loadData();
    } catch (error) {
      message.error('Failed to delete operation log');
    }
  };

  const handleView = (record: OperationLog) => {
    setSelectedLog(record);
    viewForm.setFieldsValue(record);
    setViewModalOpen(true);
  };

  const handleDeleteSelected = async () => {
    if (selectedRowKeys.length === 0) {
      message.warning('Please select logs to delete');
      return;
    }

    try {
      await Promise.all(
        selectedRowKeys.map((key) => deleteOperationLog(key as string))
      );
      message.success('Selected operation logs deleted successfully');
      setSelectedRowKeys([]);
      loadData();
    } catch (error) {
      message.error('Failed to delete selected operation logs');
    }
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
      title: 'Module',
      dataIndex: 'module',
      key: 'module',
    },
    {
      title: 'Action',
      dataIndex: 'action_type',
      key: 'action_type',
    },
    {
      title: 'Method',
      dataIndex: 'method',
      key: 'method',
    },
    {
      title: 'Path',
      dataIndex: 'path',
      key: 'path',
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
      title: 'Execution Time',
      dataIndex: 'execution_time',
      key: 'execution_time',
      render: (time: number) => `${time}ms`,
    },
    {
      title: 'Create Time',
      dataIndex: 'create_time',
      key: 'create_time',
    },
    {
      title: 'Actions',
      key: 'actions',
      render: (_: any, record: OperationLog) => (
        <Space>
          <Button
            type="link"
            icon={<EyeOutlined />}
            onClick={() => handleView(record)}
          >
            View
          </Button>
          <Button
            type="link"
            danger
            icon={<DeleteOutlined />}
            onClick={() => handleDelete(record.id)}
          >
            Delete
          </Button>
        </Space>
      ),
    },
  ];

  const rowSelection: TableRowSelection<OperationLog> = {
    selectedRowKeys,
    onChange: (selectedRowKeys: React.Key[]) => {
      setSelectedRowKeys(selectedRowKeys);
    },
  };

  return (
    <Card
      title="Operation Logs"
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
            Delete Selected
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
        <Form.Item name="module" label="Module">
          <Input placeholder="Module" />
        </Form.Item>
        <Form.Item name="action_type" label="Action Type">
          <Input placeholder="Action Type" />
        </Form.Item>
        <Form.Item name="status" label="Status">
          <Select placeholder="Status" allowClear>
            <Select.Option value={1}>Success</Select.Option>
            <Select.Option value={0}>Failed</Select.Option>
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
        title="Operation Log Detail"
        open={viewModalOpen}
        onCancel={() => setViewModalOpen(false)}
        footer={null}
        width={700}
      >
        {selectedLog && (
          <Form form={viewForm} layout="vertical">
            <Form.Item label="ID">{selectedLog.id}</Form.Item>
            <Form.Item label="Username">{selectedLog.username}</Form.Item>
            <Form.Item label="Module">{selectedLog.module}</Form.Item>
            <Form.Item label="Action">{selectedLog.action_type}</Form.Item>
            <Form.Item label="Method">{selectedLog.method}</Form.Item>
            <Form.Item label="Path">{selectedLog.path}</Form.Item>
            <Form.Item label="Request Method">{selectedLog.request_method}</Form.Item>
            <Form.Item label="Response Code">{selectedLog.response_code}</Form.Item>
            <Form.Item label="Execution Time">
              {selectedLog.execution_time}ms
            </Form.Item>
            <Form.Item label="IP Address">{selectedLog.ip_address}</Form.Item>
            <Form.Item label="Request Params">
              {selectedLog.request_params || '-'}
            </Form.Item>
            <Form.Item label="Response Message">
              {selectedLog.response_message || '-'}
            </Form.Item>
            <Form.Item label="Create Time">{selectedLog.create_time}</Form.Item>
          </Form>
        )}
      </Modal>
    </Card>
  );
};

export default OperationLogList;
