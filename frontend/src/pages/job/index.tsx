import React, { useEffect, useState } from 'react';
import {
  Table,
  Button,
  Space,
  Input,
  Tag,
  Modal,
  Form,
  message,
  Popconfirm,
  Select,
} from 'antd';
import {
  PlusOutlined,
  EditOutlined,
  DeleteOutlined,
  ReloadOutlined,
  SearchOutlined,
} from '@ant-design/icons';
import type { ColumnType, ColumnsType } from 'antd/es/table';
import { Switch, Modal, Text, InputNumber } from 'antd';
import request from '@/lib/api';
import type { PageParams } from '@/types/api';
import { jobApi, Job, cronValidator } from '@/services/api/job';

// Job Page Component
export const JobPage: React.FC = () => {
  const [jobs, setJobs] = useState<Job[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({
    current: 1,
    pageSize: 10,
    total: 0,
  });
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [editingJob, setEditingJob] = useState<Job | null>(null);
  const [searchForm] = Form.useForm();
  const [form] = Form.useForm();

  const fetchJobs = async () => {
    setLoading(true);
    try {
      const values = searchForm.getFieldsValue();
      const params: PageParams = {
        page: pagination.current,
        pageSize: pagination.pageSize,
        keyword: values.job_name || values.job_code,
      };
      const data = await jobApi.list(params);
      setJobs(data.list);
      setPagination({ ...pagination, total: data.total });
    } catch (error) {
      message.error('获取定时任务列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchJobs();
  }, [pagination.current, pagination.pageSize]);

  const handleSearch = () => {
    setPagination({ ...pagination, current: 1 });
    fetchJobs();
  };

  const handleReset = () => {
    searchForm.resetFields();
    setPagination({ ...pagination, current: 1 });
    fetchJobs();
  };

  const handleAdd = () => {
    setEditingJob(null);
    form.resetFields();
    form.setFieldsValue({
      job_group: 'DEFAULT',
      job_status: 0,
      retry_count: 0,
      retry_interval: 60,
      timeout: 300,
    });
    setIsModalVisible(true);
  };

  const handleEdit = (record: Job) => {
    setEditingJob(record);
    form.setFieldsValue(record);
    setIsModalVisible(true);
  };

  const handleDelete = async (id: number) => {
    Modal.confirm({
      title: '确认删除',
      content: '确定要删除该定时任务吗？这将停止任务执行。',
      onOk: async () => {
        try {
          await jobApi.delete(id);
          message.success('删除成功');
          fetchJobs();
        } catch (error) {
          message.error('删除失败');
        }
      },
    });
  };

  const handleStatusChange = async (id: number, status: number) => {
    try {
      if (status === 1) {
        await jobApi.schedule(id);
      } else {
        await jobApi.unschedule(id);
      }
      message.success('状态更新成功');
      fetchJobs();
    } catch (error) {
      message.error('状态更新失败');
    }
  };

  const handleRunOnce = async (id: number) => {
    try {
      await jobApi.runOnce(id);
      message.success('任务执行成功');
    } catch (error) {
      message.error('任务执行失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingJob) {
        await jobApi.update(editingJob.id, values);
        message.success('更新成功');
      } else {
        await jobApi.create(values);
        message.success('创建成功');
      }
      setIsModalVisible(false);
      fetchJobs();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const validateCronExpression = (_: any, value: string) => {
    if (!value) {
      return Promise.resolve();
    }
    const validation = cronValidator.validateWithMessage(value);
    if (validation.valid) {
      // Show next run times
      const nextRuns = cronValidator.getNextRuns(value, 3);
      form.setFieldsValue({ next_run_times: nextRuns.join(', ') });
      return Promise.resolve();
    }
    return Promise.reject(new Error(validation.message || 'Cron表达式格式错误'));
  };

  const columns: ColumnsType<Job> = [
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
      title: 'Cron表达式',
      dataIndex: 'cron_expression',
      key: 'cron_expression',
      width: 180,
    },
    {
      title: '状态',
      dataIndex: 'job_status',
      key: 'job_status',
      width: 100,
      render: (status: number, record: Job) => (
        <Switch
          checked={status === 1}
          onChange={(checked) => handleStatusChange(record.id, checked ? 1 : 0)}
          size="small"
        />
      ),
    },
    {
      title: '重试次数',
      dataIndex: 'retry_count',
      key: 'retry_count',
      width: 100,
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
      width: 250,
      fixed: 'right',
      render: (_a: any, record: Job) => (
        <Space size="small">
          <Button
            type="link"
            icon={<PlusOutlined />}
            onClick={() => handleRunOnce(record.id)}
          >
            执行一次
          </Button>
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title="确认删除"
            description="确定要删除该定时任务吗？"
            onConfirm={() => handleDelete(record.id)}
          >
            <Button type="link" danger icon={<DeleteOutlined />}>
              删除
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  const nextRunTimes = Form.useWatch('next_run_times', form);

  return (
    <div style={{ padding: 24 }}>
      <div style={{ marginBottom: 16 }}>
        <Space direction="vertical" style={{ width: '100%' }}>
          <Form
            form={searchForm}
            layout="inline"
            onFinish={handleSearch}
            initialValues={{ status: undefined }}
          >
            <Form.Item label="任务名称" name="job_name">
              <Input placeholder="请输入任务名称" />
            </Form.Item>
            <Form.Item label="任务编码" name="job_code">
              <Input placeholder="请输入任务编码" />
            </Form.Item>
            <Form.Item>
              <Space>
                <Button
                  type="primary"
                  icon={<SearchOutlined />}
                  htmlType="submit"
                >
                  搜索
                </Button>
                <Button icon={<ReloadOutlined />} onClick={handleReset}>
                  重置
                </Button>
              </Space>
            </Form.Item>
          </Form>
          <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <Button
              type="primary"
              icon={<PlusOutlined />}
              onClick={handleAdd}
            >
              新增任务
            </Button>
            <Button icon={<ReloadOutlined />} onClick={fetchJobs}>
              刷新
            </Button>
          </div>
        </Space>
      </div>
      <Table
        columns={columns}
        dataSource={jobs}
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
        title={editingJob ? '编辑定时任务' : '新增定时任务'}
        open={isModalVisible}
        onOk={handleSubmit}
        onCancel={() => {
          setIsModalVisible(false);
          form.resetFields();
        }}
        width={700}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="job_name"
            label="任务名称"
            rules={[{ required: true, message: '请输入任务名称' }]}
          >
            <Input placeholder="请输入任务名称" />
          </Form.Item>
          <Form.Item
            name="job_code"
            label="任务编码"
            rules={[{ required: true, message: '请输入任务编码' }]}
          >
            <Input placeholder="请输入任务编码，如：sys.job.test" />
          </Form.Item>
          <Form.Item
            name="job_group"
            label="任务组"
            rules={[{ required: true, message: '请选择任务组' }]}
          >
            <Select placeholder="请选择任务组">
              <Select.Option value="DEFAULT">默认组</Select.Option>
              <Select.Option value="SYSTEM">系统组</Select.Option>
              <Select.Option value="BUSINESS">业务组</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item
            name="cron_expression"
            label="Cron表达式"
            rules={[
              { required: true, message: '请输入Cron表达式' },
              { validator: validateCronExpression },
            ]}
            extra={
              nextRunTimes ? (
                <div style={{ marginTop: '8px' }}>
                  <Tag color="blue">下次运行时间：{nextRunTimes}</Tag>
                </div>
              ) : (
                <div style={{ marginTop: '8px' }}>
                  <Text type="secondary">支持标准Cron表达式，例如：0 0 12 * * ? (每天中午12点触发)</Text>
                </div>
              )
            }
          >
            <Input placeholder="0 0 12 * * ?" />
          </Form.Item>
          <Form.Item
            name="job_status"
            label="状态"
            rules={[{ required: true, message: '请选择状态' }]}
          >
            <Select>
              <Select.Option value={0}>停止</Select.Option>
              <Select.Option value={1}>运行中</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item
            name="retry_count"
            label="重试次数"
            rules={[{ required: true, message: '请输入重试次数' }]}
            initialValue={0}
          >
            <InputNumber min={0} max={10} placeholder="0-10" style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item
            name="retry_interval"
            label="重试间隔（秒）"
            rules={[{ required: true, message: '请输入重试间隔' }]}
            initialValue={60}
          >
            <InputNumber min={1} max={3600} placeholder="1-3600" style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item
            name="timeout"
            label="超时时间（秒）"
            rules={[{ required: true, message: '请输入超时时间' }]}
            initialValue={300}
          >
            <InputNumber min={1} max={3600} placeholder="1-3600" style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="description" label="描述">
            <Input.TextArea rows={3} placeholder="请输入描述" />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default JobPage;
