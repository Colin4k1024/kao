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
  Select,
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

// Notice interface
export interface Notice {
  id: number;
  notice_title: string;
  notice_content: string;
  notice_type: number;
  status: number;
  created_by?: string;
  created_at: string;
  updated_at: string;
}

// API service
export const noticeApi = {
  list(params: PageParams & { notice_title?: string; notice_type?: number }) {
    return request.get<{ list: Notice[]; total: number }>(
      '/api/system/notice',
      { params }
    );
  },
  get(id: number) {
    return request.get<Notice>(`/api/system/notice/${id}`);
  },
  create(data: Partial<Notice>) {
    return request.post<Notice>('/api/system/notice', data);
  },
  update(id: number, data: Partial<Notice>) {
    return request.put<Notice>(`/api/system/notice/${id}`, data);
  },
  delete(id: number) {
    return request.delete(`/api/system/notice/${id}`);
  },
  publish(id: number) {
    return request.put(`/api/system/notice/${id}/publish`);
  },
  unpublish(id: number) {
    return request.put(`/api/system/notice/${id}/unpublish`);
  },
};

// Form interface
interface NoticeForm {
  notice_title: string;
  notice_content: string;
  notice_type: number;
  status: number;
}

// Notice Page Component
export const NoticePage: React.FC = () => {
  const [notices, setNotices] = useState<Notice[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({
    current: 1,
    pageSize: 10,
    total: 0,
  });
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [isPreviewModalVisible, setIsPreviewModalVisible] = useState(false);
  const [editingNotice, setEditingNotice] = useState<Notice | null>(null);
  const [previewNotice, setPreviewNotice] = useState<Notice | null>(null);
  const [searchForm] = Form.useForm();
  const [form] = Form.useForm();

  const fetchNotices = async () => {
    setLoading(true);
    try {
      const values = searchForm.getFieldsValue();
      const params: PageParams = {
        page: pagination.current,
        pageSize: pagination.pageSize,
        keyword: values.notice_title,
        notice_type: values.notice_type,
      };
      const data = await noticeApi.list(params);
      setNotices(data.list);
      setPagination({ ...pagination, total: data.total });
    } catch (error) {
      message.error('获取通知公告列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchNotices();
  }, [pagination.current, pagination.pageSize]);

  const handleSearch = () => {
    setPagination({ ...pagination, current: 1 });
    fetchNotices();
  };

  const handleReset = () => {
    searchForm.resetFields();
    setPagination({ ...pagination, current: 1 });
    fetchNotices();
  };

  const handleAdd = () => {
    setEditingNotice(null);
    form.resetFields();
    form.setFieldsValue({ notice_type: 1, status: 1 });
    setIsModalVisible(true);
  };

  const handleEdit = (record: Notice) => {
    setEditingNotice(record);
    form.setFieldsValue(record);
    setIsModalVisible(true);
  };

  const handleDelete = async (id: number) => {
    try {
      await noticeApi.delete(id);
      message.success('删除成功');
      fetchNotices();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleStatusChange = async (id: number, status: number) => {
    try {
      if (status === 1) {
        await noticeApi.publish(id);
      } else {
        await noticeApi.unpublish(id);
      }
      message.success('状态更新成功');
      fetchNotices();
    } catch (error) {
      message.error('状态更新失败');
    }
  };

  const handlePreview = (record: Notice) => {
    setPreviewNotice(record);
    setIsPreviewModalVisible(true);
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingNotice) {
        await noticeApi.update(editingNotice.id, values);
        message.success('更新成功');
      } else {
        await noticeApi.create(values);
        message.success('创建成功');
      }
      setIsModalVisible(false);
      fetchNotices();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const columns: ColumnsType<Notice> = [
    {
      title: '标题',
      dataIndex: 'notice_title',
      key: 'notice_title',
      width: 200,
      ellipsis: true,
    },
    {
      title: '类型',
      dataIndex: 'notice_type',
      key: 'notice_type',
      width: 100,
      render: (type: number) => (
        <Tag color={type === 1 ? 'green' : 'blue'}>
          {type === 1 ? '通知' : '公告'}
        </Tag>
      ),
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 100,
      render: (status: number, record: Notice) => (
        <Switch
          checked={status === 1}
          onChange={(checked) => handleStatusChange(record.id, checked ? 1 : 0)}
          size="small"
        />
      ),
    },
    {
      title: '创建者',
      dataIndex: 'created_by',
      key: 'created_by',
      width: 120,
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
            icon={<EyeOutlined />}
            onClick={() => handlePreview(record)}
          >
            预览
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
            description="确定要删除该通知公告吗？"
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
            <Form.Item label="标题" name="notice_title">
              <Input placeholder="请输入标题" />
            </Form.Item>
            <Form.Item label="类型" name="notice_type">
              <Select placeholder="请选择类型">
                <Select.Option value={1}>通知</Select.Option>
                <Select.Option value={0}>公告</Select.Option>
              </Select>
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
              发布通知
            </Button>
            <Button icon={<ReloadOutlined />} onClick={fetchNotices}>
              刷新
            </Button>
          </div>
        </Space>
      </div>
      <Table
        columns={columns}
        dataSource={notices}
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
        title={editingNotice ? '编辑通知公告' : '新增通知公告'}
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
            name="notice_title"
            label="标题"
            rules={[{ required: true, message: '请输入标题' }]}
          >
            <Input placeholder="请输入标题" />
          </Form.Item>
          <Form.Item
            name="notice_type"
            label="类型"
            rules={[{ required: true, message: '请选择类型' }]}
          >
            <Select placeholder="请选择类型">
              <Select.Option value={1}>通知</Select.Option>
              <Select.Option value={0}>公告</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item
            name="notice_content"
            label="内容"
            rules={[{ required: true, message: '请输入内容' }]}
          >
            <Input.TextArea rows={10} placeholder="请输入内容" />
          </Form.Item>
          <Form.Item
            name="status"
            label="状态"
            rules={[{ required: true, message: '请选择状态' }]}
          >
            <Select>
              <Select.Option value={1}>已发布</Select.Option>
              <Select.Option value={0}>未发布</Select.Option>
            </Select>
          </Form.Item>
        </Form>
      </Modal>
      {/* Preview Modal */}
      <Modal
        title="通知预览"
        open={isPreviewModalVisible}
        onCancel={() => setIsPreviewModalVisible(false)}
        width={700}
        footer={null}
      >
        {previewNotice && (
          <>
            <Alert
              message={
                <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                  <span>
                    标题：{previewNotice.notice_title}
                  </span>
                  <Tag color={previewNotice.notice_type === 1 ? 'green' : 'blue'}>
                    {previewNotice.notice_type === 1 ? '通知' : '公告'}
                  </Tag>
                </div>
              }
              type="info"
              showIcon
            />
            <div
              style={{
                marginTop: 16,
                padding: 16,
                border: '1px solid #f0f0f0',
                borderRadius: '4px',
                maxHeight: '400px',
                overflow: 'auto',
              }}
            >
              {previewNotice.notice_content}
            </div>
            <div style={{ marginTop: 16, textAlign: 'right' }}>
              <Button
                type="primary"
                onClick={() => {
                  const div = document.createElement('div');
                  div.textContent = `标题：${previewNotice.notice_title}
类型：${previewNotice.notice_type === 1 ? '通知' : '公告'}
内容：
${previewNotice.notice_content}`;
                  navigator.clipboard.writeText(div.textContent);
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

export default NoticePage;
