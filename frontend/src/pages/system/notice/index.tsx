import React, { useEffect, useState } from 'react';
import { Table, Button, Space, Input, Tag, Modal, Form, Select, message, Popconfirm } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, ReloadOutlined, SearchOutlined, EyeOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import request from '@/lib/api';
import type { PageParams } from '@/types/api';

// Backend Notice response interface (snake_case)
interface NoticeResponse {
  id: string;
  notice_title: string;
  notice_type: string;
  notice_content: string | null;
  notice_status: string;
  is_top: string;
  priority: number;
  publish_time: string | null;
  view_count: number;
  publisher_id: string | null;
  publisher_name: string | null;
  created_at: string;
  updated_at: string;
}

// Form interface (camelCase for Ant Design)
interface NoticeFormData {
  notice_title: string;
  notice_type: string;
  notice_content: string;
  notice_status: string;
  is_top: string;
  priority: number;
}

const NoticeList: React.FC = () => {
  const [notices, setNotices] = useState<NoticeResponse[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 10, total: 0 });
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [isPreviewVisible, setIsPreviewVisible] = useState(false);
  const [editingNotice, setEditingNotice] = useState<NoticeResponse | null>(null);
  const [previewNotice, setPreviewNotice] = useState<NoticeResponse | null>(null);
  const [form] = Form.useForm();
  const [searchForm] = Form.useForm();

  const fetchNotices = async () => {
    setLoading(true);
    try {
      const searchValues = searchForm.getFieldsValue();
      const params: PageParams & { notice_title?: string; notice_type?: string } = {
        page: pagination.current,
        pageSize: pagination.pageSize,
        notice_title: searchValues.notice_title,
        notice_type: searchValues.notice_type,
      };
      const data = await request.get<{ items: NoticeResponse[]; total: number }>(
        '/api/system/notice',
        { params }
      );
      setNotices(data.items || []);
      setPagination({ ...pagination, total: data.total || 0 });
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
    form.setFieldsValue({
      notice_type: '1',
      notice_status: '0',
      is_top: 'N',
      priority: 0,
    });
    setIsModalVisible(true);
  };

  const handleEdit = (record: NoticeResponse) => {
    setEditingNotice(record);
    form.setFieldsValue({
      notice_title: record.notice_title,
      notice_type: record.notice_type,
      notice_content: record.notice_content,
      notice_status: record.notice_status,
      is_top: record.is_top,
      priority: record.priority,
    });
    setIsModalVisible(true);
  };

  const handlePreview = (record: NoticeResponse) => {
    setPreviewNotice(record);
    setIsPreviewVisible(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await request.delete(`/api/system/notice/${id}`);
      message.success('删除成功');
      fetchNotices();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingNotice) {
        await request.put(`/api/system/notice/${editingNotice.id}`, values);
        message.success('更新成功');
      } else {
        await request.post('/api/system/notice', values);
        message.success('创建成功');
      }
      setIsModalVisible(false);
      fetchNotices();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const noticeTypeMap: Record<string, string> = { '1': '通知', '2': '公告' };
  const noticeStatusMap: Record<string, string> = { '0': '已发布', '1': '已取消' };

  const columns: ColumnsType<NoticeResponse> = [
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
      render: (type: string) => (
        <Tag color={type === '1' ? 'blue' : 'green'}>
          {noticeTypeMap[type] || type}
        </Tag>
      ),
    },
    {
      title: '状态',
      dataIndex: 'notice_status',
      key: 'notice_status',
      width: 100,
      render: (status: string) => (
        <Tag color={status === '0' ? 'green' : 'red'}>
          {noticeStatusMap[status] || status}
        </Tag>
      ),
    },
    {
      title: '置顶',
      dataIndex: 'is_top',
      key: 'is_top',
      width: 80,
      render: (isTop: string) => (
        <Tag color={isTop === 'Y' ? 'gold' : 'default'}>
          {isTop === 'Y' ? '是' : '否'}
        </Tag>
      ),
    },
    {
      title: '优先级',
      dataIndex: 'priority',
      key: 'priority',
      width: 80,
    },
    {
      title: '浏览次数',
      dataIndex: 'view_count',
      key: 'view_count',
      width: 100,
    },
    {
      title: '发布时间',
      dataIndex: 'publish_time',
      key: 'publish_time',
      width: 180,
      render: (time: string | null) => time ? new Date(time).toLocaleString('zh-CN') : '-',
    },
    {
      title: '操作',
      key: 'action',
      width: 200,
      fixed: 'right' as const,
      render: (_, record) => (
        <Space size="small">
          <Button type="link" icon={<EyeOutlined />} onClick={() => handlePreview(record)}>
            预览
          </Button>
          <Button type="link" icon={<EditOutlined />} onClick={() => handleEdit(record)}>
            编辑
          </Button>
          <Popconfirm
            title="确认删除"
            description="确定要删除该通知/公告吗？"
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
      {/* Search Form */}
      <div style={{ marginBottom: 16 }}>
        <Form form={searchForm} layout="inline" onFinish={handleSearch}>
          <Form.Item name="notice_title" label="标题">
            <Input placeholder="请输入标题" style={{ width: 200 }} />
          </Form.Item>
          <Form.Item name="notice_type" label="类型">
            <Select placeholder="请选择" style={{ width: 120 }} allowClear>
              <Select.Option value="1">通知</Select.Option>
              <Select.Option value="2">公告</Select.Option>
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
      </div>

      {/* Action Buttons */}
      <div style={{ marginBottom: 16, display: 'flex', justifyContent: 'space-between' }}>
        <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
          新增通知/公告
        </Button>
        <Button icon={<ReloadOutlined />} onClick={fetchNotices}>
          刷新
        </Button>
      </div>

      {/* Table */}
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
          onChange: (page, pageSize) => setPagination({ ...pagination, current: page, pageSize }),
        }}
      />

      {/* Edit/Add Modal */}
      <Modal
        title={editingNotice ? '编辑通知/公告' : '新增通知/公告'}
        open={isModalVisible}
        onOk={handleSubmit}
        onCancel={() => setIsModalVisible(false)}
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
          <Form.Item name="notice_type" label="类型" rules={[{ required: true, message: '请选择类型' }]}>
            <Select placeholder="请选择类型">
              <Select.Option value="1">通知</Select.Option>
              <Select.Option value="2">公告</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item
            name="notice_content"
            label="内容"
            rules={[{ required: true, message: '请输入内容' }]}
          >
            <Input.TextArea rows={10} placeholder="请输入内容" />
          </Form.Item>
          <Form.Item name="notice_status" label="状态">
            <Select>
              <Select.Option value="0">已发布</Select.Option>
              <Select.Option value="1">已取消</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="is_top" label="置顶">
            <Select>
              <Select.Option value="Y">是</Select.Option>
              <Select.Option value="N">否</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="priority" label="优先级" initialValue={0}>
            <Input type="number" min={0} max={100} />
          </Form.Item>
        </Form>
      </Modal>

      {/* Preview Modal */}
      <Modal
        title="通知预览"
        open={isPreviewVisible}
        onCancel={() => setIsPreviewVisible(false)}
        footer={null}
        width={700}
      >
        {previewNotice && (
          <div>
            <div style={{ marginBottom: 16 }}>
              <Tag color={previewNotice.notice_type === '1' ? 'blue' : 'green'}>
                {noticeTypeMap[previewNotice.notice_type] || previewNotice.notice_type}
              </Tag>
              <span style={{ marginLeft: 8 }}>状态：</span>
              <Tag color={previewNotice.notice_status === '0' ? 'green' : 'red'}>
                {noticeStatusMap[previewNotice.notice_status] || previewNotice.notice_status}
              </Tag>
            </div>
            <h3 style={{ marginBottom: 16 }}>{previewNotice.notice_title}</h3>
            <div
              style={{
                padding: 16,
                border: '1px solid #f0f0f0',
                borderRadius: 4,
                minHeight: 200,
                backgroundColor: '#fafafa',
              }}
            >
              {previewNotice.notice_content || '无内容'}
            </div>
            <div style={{ marginTop: 16, color: '#999' }}>
              <p>浏览次数：{previewNotice.view_count}</p>
              <p>发布人：{previewNotice.publisher_name || '-'}</p>
              <p>发布时间：{previewNotice.publish_time ? new Date(previewNotice.publish_time).toLocaleString('zh-CN') : '-'}</p>
            </div>
          </div>
        )}
      </Modal>
    </div>
  );
};

export default NoticeList;
