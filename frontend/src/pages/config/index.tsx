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
  Switch,
  Alert,
} from 'antd';
import {
  PlusOutlined,
  EditOutlined,
  DeleteOutlined,
  ReloadOutlined,
  SearchOutlined,
  EyeOutlined,
} from '@ant-design/icons';
import request from '@/lib/api';
import type { PageParams } from '@/types/api';

// Config interface
export interface Config {
  id: number;
  config_key: string;
  config_name: string;
  config_value: string;
  config_type: number;
  status: number;
  description?: string;
  created_at: string;
  updated_at: string;
}

// API service
export const configApi = {
  list(params: PageParams & { config_key?: string; config_name?: string }) {
    return request.get<{ list: Config[]; total: number }>(
      '/api/system/config',
      { params }
    );
  },
  get(id: number) {
    return request.get<Config>(`/api/system/config/${id}`);
  },
  create(data: Partial<Config>) {
    return request.post<Config>('/api/system/config', data);
  },
  update(id: number, data: Partial<Config>) {
    return request.put<Config>(`/api/system/config/${id}`, data);
  },
  delete(id: number) {
    return request.delete(`/api/system/config/${id}`);
  },
  preview(data: Partial<Config>) {
    return request.post<Config>('/api/system/config/preview', data);
  },
};

// Config Page Component
export const ConfigPage: React.FC = () => {
  const [configs, setConfigs] = useState<Config[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({
    current: 1,
    pageSize: 10,
    total: 0,
  });
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [isPreviewModalVisible, setIsPreviewModalVisible] = useState(false);
  const [editingConfig, setEditingConfig] = useState<Config | null>(null);
  const [previewConfig, setPreviewConfig] = useState<Config | null>(null);
  const [searchForm] = Form.useForm();
  const [form] = Form.useForm();

  const fetchConfigs = async () => {
    setLoading(true);
    try {
      const values = searchForm.getFieldsValue();
      const params: PageParams = {
        page: pagination.current,
        pageSize: pagination.pageSize,
        keyword: values.config_key || values.config_name,
      };
      const data = await configApi.list(params);
      setConfigs(data.list);
      setPagination({ ...pagination, total: data.total });
    } catch (error) {
      message.error('获取参数配置列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchConfigs();
  }, [pagination.current, pagination.pageSize]);

  const handleSearch = () => {
    setPagination({ ...pagination, current: 1 });
    fetchConfigs();
  };

  const handleReset = () => {
    searchForm.resetFields();
    setPagination({ ...pagination, current: 1 });
    fetchConfigs();
  };

  const handleAdd = () => {
    setEditingConfig(null);
    form.resetFields();
    form.setFieldsValue({ config_type: 1, status: 1 });
    setIsModalVisible(true);
  };

  const handleEdit = (record: Config) => {
    setEditingConfig(record);
    form.setFieldsValue(record);
    setIsModalVisible(true);
  };

  const handleDelete = async (id: number) => {
    try {
      await configApi.delete(id);
      message.success('删除成功');
      fetchConfigs();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleStatusChange = async (id: number, status: number) => {
    try {
      await configApi.update(id, { status });
      message.success('状态更新成功');
      fetchConfigs();
    } catch (error) {
      message.error('状态更新失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingConfig) {
        await configApi.update(editingConfig.id, values);
        message.success('更新成功');
      } else {
        await configApi.create(values);
        message.success('创建成功');
      }
      setIsModalVisible(false);
      fetchConfigs();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const handlePreview = () => {
    form.validateFields().then((values) => {
      configApi.preview(values).then((result) => {
        setPreviewConfig(result);
        setIsModalVisible(false);
        setIsPreviewModalVisible(true);
      }).catch((err) => {
        message.error('预览失败');
      });
    });
  };

  const columns = [
    {
      title: '参数键',
      dataIndex: 'config_key',
      key: 'config_key',
      width: 150,
    },
    {
      title: '参数名',
      dataIndex: 'config_name',
      key: 'config_name',
      width: 150,
    },
    {
      title: '参数值',
      dataIndex: 'config_value',
      key: 'config_value',
      ellipsis: true,
      render: (value: string, record: Config) => (
        <Button
          type="link"
          icon={<EyeOutlined />}
          onClick={() => {
            setPreviewConfig({ ...record, config_value: value });
            setIsPreviewModalVisible(true);
          }}
        >
          查看
        </Button>
      ),
    },
    {
      title: '参数类型',
      dataIndex: 'config_type',
      key: 'config_type',
      width: 100,
      render: (type: number) => (
        <Tag color={type === 1 ? 'green' : 'blue'}>
          {type === 1 ? '系统内置' : '自定义'}
        </Tag>
      ),
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 100,
      render: (status: number, record: Config) => (
        <Switch
          checked={status === 1}
          onChange={(checked) => handleStatusChange(record.id, checked ? 1 : 0)}
          size="small"
        />
      ),
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
      width: 180,
      fixed: 'right',
      render: (_: any, record: Config) => (
        <Space size="small">
          <Button
            type="link"
            icon={<EditOutlined />}
            onClick={() => handleEdit(record)}
          >
            编辑
          </Button>
          <Popconfirm
            title="确认删除"
            description="确定要删除该参数配置吗？"
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
            <Form.Item label="参数键" name="config_key">
              <Input placeholder="请输入参数键" />
            </Form.Item>
            <Form.Item label="参数名" name="config_name">
              <Input placeholder="请输入参数名" />
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
              新增参数
            </Button>
            <Button icon={<ReloadOutlined />} onClick={fetchConfigs}>
              刷新
            </Button>
          </div>
        </Space>
      </div>
      <Table
        columns={columns}
        dataSource={configs}
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
      {/* Edit/Add Modal */}
      <Modal
        title={editingConfig ? '编辑参数配置' : '新增参数配置'}
        open={isModalVisible}
        onOk={handleSubmit}
        onCancel={() => {
          setIsModalVisible(false);
          form.resetFields();
        }}
        width={600}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="config_key"
            label="参数键"
            rules={[{ required: true, message: '请输入参数键' }]}
          >
            <Input placeholder="请输入参数键，如：sys.test.key" />
          </Form.Item>
          <Form.Item
            name="config_name"
            label="参数名"
            rules={[{ required: true, message: '请输入参数名' }]}
          >
            <Input placeholder="请输入参数名" />
          </Form.Item>
          <Form.Item
            name="config_value"
            label="参数值"
            rules={[{ required: true, message: '请输入参数值' }]}
          >
            <Input.TextArea rows={5} placeholder="请输入参数值" />
          </Form.Item>
          <Form.Item
            name="config_type"
            label="参数类型"
            rules={[{ required: true, message: '请选择参数类型' }]}
          >
            <Select>
              <Select.Option value={1}>系统内置</Select.Option>
              <Select.Option value={0}>自定义</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="description" label="描述">
            <Input.TextArea rows={2} placeholder="请输入描述" />
          </Form.Item>
          <Form.Item
            name="status"
            label="状态"
            rules={[{ required: true, message: '请选择状态' }]}
          >
            <Select>
              <Select.Option value={1}>启用</Select.Option>
              <Select.Option value={0}>禁用</Select.Option>
            </Select>
          </Form.Item>
        </Form>
      </Modal>
      {/* Preview Modal */}
      <Modal
        title="参数预览"
        open={isPreviewModalVisible}
        onCancel={() => setIsPreviewModalVisible(false)}
        width={600}
        footer={null}
      >
        {previewConfig && (
          <>
            <Alert
              message="参数预览"
              description={`参数键：${previewConfig.config_key}
参数名：${previewConfig.config_name}
参数值：
${previewConfig.config_value}
描述：${previewConfig.description || '无'}`}
              type="info"
              showIcon
            />
            <div style={{ marginTop: 16, textAlign: 'right' }}>
              <Button
                type="primary"
                onClick={() => {
                  const text = `参数键：${previewConfig.config_key}
参数名：${previewConfig.config_name}
参数值：
${previewConfig.config_value}
描述：${previewConfig.description || '无'}`;
                  navigator.clipboard.writeText(text);
                  message.success('已复制到剪贴板');
                }}
              >
                复制
              </Button>
            </div>
          </>
        )}
      </Modal>
    </div>
  );
};

export default ConfigPage;
