import React, { useEffect, useState } from 'react';
import {
  Table,
  Button,
  Space,
  Input,
  Tag,
  Modal,
  Select,
  message,
} from 'antd';
import {
  ReloadOutlined,
  SearchOutlined,
  EyeOutlined,
} from '@ant-design/icons';
import request from '@/lib/api';
import type { PageParams } from '@/types/api';
import type { ColumnsType } from 'antd/es/table';

// Log interface
export interface Log {
  id: number;
  job_id: number;
  job_name: string;
  job_code: string;
  job_group: string;
  execute_status: number;
  execute_message?: string;
  execute_time: string;
  created_at: string;
}

// API service
export const logApi = {
  list(params: PageParams & { job_id?: number; job_name?: string; execute_status?: number }) {
    return request.get<{ list: Log[]; total: number }>(
      '/api/system/jobs/logs',
      { params }
    );
  },
  get(id: number) {
    return request.get<Log>(`/api/system/jobs/logs/${id}`);
  },
  clear(jobId?: number) {
    return request.delete(
      jobId
        ? `/api/system/jobs/logs/clear?job_id=${jobId}`
        : '/api/system/jobs/logs/clear'
    );
  },
};

// Log Page Component
export const JobLogPage: React.FC = () => {
  const [logs, setLogs] = useState<Log[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({
    current: 1,
    pageSize: 10,
    total: 0,
  });
  const [isDetailModalVisible, setIsDetailModalVisible] = useState(false);
  const [selectedLog, setSelectedLog] = useState<Log | null>(null);
  const [searchForm] = Form.useForm();

  const fetchLogs = async () => {
    setLoading(true);
    try {
      const values = searchForm.getFieldsValue();
      const params: PageParams = {
        page: pagination.current,
        pageSize: pagination.pageSize,
        keyword: values.job_name,
        execute_status: values.execute_status,
      };
      const data = await logApi.list(params);
      setLogs(data.list);
      setPagination({ ...pagination, total: data.total });
    } catch (error) {
      message.error('获取任务日志列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchLogs();
  }, [pagination.current, pagination.pageSize]);

  const handleSearch = () => {
    setPagination({ ...pagination, current: 1 });
    fetchLogs();
  };

  const handleReset = () => {
    searchForm.resetFields();
    setPagination({ ...pagination, current: 1 });
    fetchLogs();
  };

  const handleViewDetail = (record: Log) => {
    setSelectedLog(record);
    setIsDetailModalVisible(true);
  };

  const handleClearLogs = async () => {
    try {
      await logApi.clear();
      message.success('日志清除成功');
      fetchLogs();
    } catch (error) {
      message.error('日志清除失败');
    }
  };

  const handleClearJobLogs = async (jobId: number) => {
    try {
      await logApi.clear(jobId);
      message.success('任务日志清除成功');
      fetchLogs();
    } catch (error) {
      message.error('任务日志清除失败');
    }
  };

  const columns: ColumnsType<Log> = [
    {
      title: '日志ID',
      dataIndex: 'id',
      key: 'id',
      width: 100,
    },
    {
      title: '任务名称',
      dataIndex: 'job_name',
      key: 'job_name',
      width: 150,
    },
    {
      title: '任务编码',
      dataIndex: 'job_code',
      key: 'job_code',
      width: 150,
    },
    {
      title: '任务组',
      dataIndex: 'job_group',
      key: 'job_group',
      width: 100,
    },
    {
      title: '执行状态',
      dataIndex: 'execute_status',
      key: 'execute_status',
      width: 100,
      render: (status: number) => {
        if (status === 1) {
          return <Tag color="green">成功</Tag>;
        } else if (status === 2) {
          return <Tag color="orange">执行中</Tag>;
        } else {
          return <Tag color="red">失败</Tag>;
        }
      },
    },
    {
      title: '执行时间',
      dataIndex: 'execute_time',
      key: 'execute_time',
      width: 180,
      render: (date: string) => new Date(date).toLocaleString('zh-CN'),
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      width: 180,
      render: (date: string) => new Date(date).toLocaleString('zh-CN'),
    },
    {
      title: '操作',
      key: 'action',
      width: 150,
      fixed: 'right',
      render: (_, record) => (
        <Space size="small">
          <Button
            type="link"
            icon={<EyeOutlined />}
            onClick={() => handleViewDetail(record)}
          >
            详情
          </Button>
          <Popconfirm
            title="确认清除日志"
            description={`确定要清除任务【${record.job_name}】的日志吗？`}
            onConfirm={() => handleClearJobLogs(record.job_id)}
          >
            <Button type="link" danger>
              清除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ padding: 24 }}>
      <div style={{ marginBottom: 16, display: 'flex', justifyContent: 'space-between' }}>
        <Form form={searchForm} layout="inline" onFinish={handleSearch}>
          <Form.Item label="任务名称" name="job_name">
            <Input placeholder="请输入任务名称" />
          </Form.Item>
          <Form.Item label="执行状态" name="execute_status">
            <Select placeholder="请选择状态">
              <Select.Option value={1}>成功</Select.Option>
              <Select.Option value={2}>执行中</Select.Option>
              <Select.Option value={0}>失败</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item>
            <Space>
              <Button type="primary" icon={<SearchOutlined />} htmlType="submit">
                搜索
              </Button>
              <Button icon={<ReloadOutlined />} onClick={handleReset}>
                重置
              </Button>
            </Space>
          </Form.Item>
        </Form>
        <div>
          <Button danger icon={<ReloadOutlined />} onClick={handleClearLogs}>
            清除所有日志
          </Button>
          <Button icon={<ReloadOutlined />} onClick={fetchLogs} style={{ marginLeft: 8 }}>
            刷新
          </Button>
        </div>
      </div>
      <Table
        columns={columns}
        dataSource={logs}
        loading={loading}
        rowKey="id"
        pagination={{
          current: pagination.current,
          pageSize: pagination.pageSize,
          total: pagination.total,
          showSizeChanger: true,
          showTotal: (total) => `共 ${total} 条`,
          onChange: (page, pageSize) =>
            setPagination({ ...pagination, current: page, pageSize }),
        }}
      />
      <Modal
        title="任务日志详情"
        open={isDetailModalVisible}
        onCancel={() => setIsDetailModalVisible(false)}
        width={700}
        footer={[
          <Button key="close" onClick={() => setIsDetailModalVisible(false)}>
            关闭
          </Button>,
        ]}
      >
        {selectedLog && (
          <div style={{ padding: 16 }}>
            <div style={{ marginBottom: 16 }}>
              <h4>任务信息</h4>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
                <div>
                  <label>任务ID：</label>
                  <span>{selectedLog.job_id}</span>
                </div>
                <div>
                  <label>任务名称：</label>
                  <span>{selectedLog.job_name}</span>
                </div>
                <div>
                  <label>任务编码：</label>
                  <span>{selectedLog.job_code}</span>
                </div>
                <div>
                  <label>任务组：</label>
                  <span>{selectedLog.job_group}</span>
                </div>
              </div>
            </div>
            <div style={{ marginBottom: 16 }}>
              <h4>执行信息</h4>
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px' }}>
                <div>
                  <label>执行状态：</label>
                  <span>
                    {selectedLog.execute_status === 1
                      ? <Tag color="green">成功</Tag>
                      : selectedLog.execute_status === 2
                      ? <Tag color="orange">执行中</Tag>
                      : <Tag color="red">失败</Tag>}
                  </span>
                </div>
                <div>
                  <label>执行时间：</label>
                  <span>{selectedLog.execute_time}</span>
                </div>
                <div style={{ gridColumn: '1 / -1' }}>
                  <label>执行消息：</label>
                  <div
                    style={{
                      marginTop: '8px',
                      padding: '12px',
                      background: '#f5f5f5',
                      borderRadius: '4px',
                      fontFamily: 'monospace',
                      whiteSpace: 'pre-wrap',
                      wordBreak: 'break-all',
                    }}
                  >
                    {selectedLog.execute_message || '无'}
                  </div>
                </div>
              </div>
            </div>
            <div>
              <h4>创建信息</h4>
              <div>
                <label>创建时间：</label>
                <span>{selectedLog.created_at}</span>
              </div>
            </div>
          </div>
        )}
      </Modal>
    </div>
  );
};

export default JobLogPage;
