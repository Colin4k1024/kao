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
import request from '@/lib/api';
import type { PageParams } from '@/types/api';

// Dictionary type interface
export interface DictionaryType {
  id: number;
  type_name: string;
  type_code: string;
  status: number;
  description?: string;
  created_at: string;
  updated_at: string;
}

// API service
export const dictionaryTypeApi = {
  list(params: PageParams & { type_name?: string; type_code?: string }) {
    return request.get<{ list: DictionaryType[]; total: number }>(
      '/api/system/dictionary/types',
      { params }
    );
  },
  get(id: number) {
    return request.get<DictionaryType>(`/api/system/dictionary/types/${id}`);
  },
  create(data: Partial<DictionaryType>) {
    return request.post<DictionaryType>('/api/system/dictionary/types', data);
  },
  update(id: number, data: Partial<DictionaryType>) {
    return request.put<DictionaryType>(`/api/system/dictionary/types/${id}`, data);
  },
  delete(id: number) {
    return request.delete(`/api/system/dictionary/types/${id}`);
  },
  enable(id: number) {
    return request.put(`/api/system/dictionary/types/${id}/enable`);
  },
  disable(id: number) {
    return request.put(`/api/system/dictionary/types/${id}/disable`);
  },
};

// Form interface
interface DictionaryTypeForm {
  type_name: string;
  type_code: string;
  status: number;
  description?: string;
}

// Dictionary Type Page Component
export const DictionaryTypePage: React.FC = () => {
  const [types, setTypes] = useState<DictionaryType[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({
    current: 1,
    pageSize: 10,
    total: 0,
  });
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [editingType, setEditingType] = useState<DictionaryType | null>(null);
  const [searchForm] = Form.useForm();
  const [form] = Form.useForm();

  const fetchTypes = async () => {
    setLoading(true);
    try {
      const values = searchForm.getFieldsValue();
      const params: PageParams = {
        page: pagination.current,
        pageSize: pagination.pageSize,
        keyword: values.type_name || values.type_code,
      };
      const data = await dictionaryTypeApi.list(params);
      setTypes(data.list);
      setPagination({ ...pagination, total: data.total });
    } catch (error) {
      message.error('获取字典类型列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTypes();
  }, [pagination.current, pagination.pageSize]);

  const handleSearch = () => {
    setPagination({ ...pagination, current: 1 });
    fetchTypes();
  };

  const handleReset = () => {
    searchForm.resetFields();
    setPagination({ ...pagination, current: 1 });
    fetchTypes();
  };

  const handleAdd = () => {
    setEditingType(null);
    form.resetFields();
    form.setFieldsValue({ status: 1 });
    setIsModalVisible(true);
  };

  const handleEdit = (record: DictionaryType) => {
    setEditingType(record);
    form.setFieldsValue(record);
    setIsModalVisible(true);
  };

  const handleDelete = async (id: number) => {
    try {
      await dictionaryTypeApi.delete(id);
      message.success('删除成功');
      fetchTypes();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleStatusChange = async (id: number, status: number) => {
    try {
      if (status === 1) {
        await dictionaryTypeApi.enable(id);
      } else {
        await dictionaryTypeApi.disable(id);
      }
      message.success('状态更新成功');
      fetchTypes();
    } catch (error) {
      message.error('状态更新失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingType) {
        await dictionaryTypeApi.update(editingType.id, values);
        message.success('更新成功');
      } else {
        await dictionaryTypeApi.create(values);
        message.success('创建成功');
      }
      setIsModalVisible(false);
      fetchTypes();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const columns: ColumnsType<DictionaryType> = [
    {
      title: '类型名称',
      dataIndex: 'type_name',
      key: 'type_name',
      width: 150,
    },
    {
      title: '类型编码',
      dataIndex: 'type_code',
      key: 'type_code',
      width: 150,
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
      ellipsis: true,
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 100,
      render: (status: number) => (
        <Tag color={status === 1 ? 'green' : 'red'}>
          {status === 1 ? '启用' : '禁用'}
        </Tag>
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
      render: (_, record) => (
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
            description="确定要删除该字典类型吗？"
            onConfirm={() => handleDelete(record.id)}
          >
            <Button type="link" danger icon={<DeleteOutlined />}>
              删除
            </Button>
          </Popconfirm>
          <Button
            type="link"
            onClick={() =>
              handleStatusChange(
                record.id,
                record.status === 1 ? 0 : 1
              )
            }
          >
            {record.status === 1 ? '禁用' : '启用'}
          </Button>
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
            <Form.Item label="类型名称" name="type_name">
              <Input placeholder="请输入类型名称" />
            </Form.Item>
            <Form.Item label="类型编码" name="type_code">
              <Input placeholder="请输入类型编码" />
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
              新增类型
            </Button>
            <Button icon={<ReloadOutlined />} onClick={fetchTypes}>
              刷新
            </Button>
          </div>
        </Space>
      </div>
      <Table
        columns={columns}
        dataSource={types}
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
        title={editingType ? '编辑字典类型' : '新增字典类型'}
        open={isModalVisible}
        onOk={handleSubmit}
        onCancel={() => setIsModalVisible(false)}
        width={500}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="type_name"
            label="类型名称"
            rules={[{ required: true, message: '请输入类型名称' }]}
          >
            <Input placeholder="请输入类型名称" />
          </Form.Item>
          <Form.Item
            name="type_code"
            label="类型编码"
            rules={[{ required: true, message: '请输入类型编码' }]}
          >
            <Input placeholder="请输入类型编码" />
          </Form.Item>
          <Form.Item name="description" label="描述">
            <Input.TextArea rows={3} placeholder="请输入描述" />
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
    </div>
  );
};

// Export TypeFormModal as separate component
export const TypeFormModal: React.FC<{
  visible: boolean;
  onCancel: () => void;
  onFinish: () => void;
  initialData?: DictionaryType;
}> = ({ visible, onCancel, onFinish, initialData }) => {
  const [form] = Form.useForm();

  useEffect(() => {
    if (initialData) {
      form.setFieldsValue(initialData);
    } else {
      form.resetFields();
    }
  }, [initialData, form, visible]);

  const handleOk = async () => {
    try {
      await form.validateFields();
      onFinish();
      form.resetFields();
    } catch (error) {
      console.error('Validation failed:', error);
    }
  };

  return (
    <Modal
      title={initialData ? '编辑字典类型' : '新增字典类型'}
      open={visible}
      onOk={handleOk}
      onCancel={onCancel}
      width={500}
    >
      <Form form={form} layout="vertical">
        <Form.Item
          name="type_name"
          label="类型名称"
          rules={[{ required: true, message: '请输入类型名称' }]}
        >
          <Input placeholder="请输入类型名称" />
        </Form.Item>
        <Form.Item
          name="type_code"
          label="类型编码"
          rules={[{ required: true, message: '请输入类型编码' }]}
        >
          <Input placeholder="请输入类型编码" />
        </Form.Item>
        <Form.Item name="description" label="描述">
          <Input.TextArea rows={3} placeholder="请输入描述" />
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
  );
};

export default DictionaryTypePage;
