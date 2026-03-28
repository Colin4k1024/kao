import React, { useEffect, useState } from 'react';
import { Table, Button, Space, Input, Tag, Modal, Form, Select, InputNumber, message } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { noticeApi } from '@/services/api/dictionary';
import type { Notice } from '@/services/api/dictionary';

const NoticeList: React.FC = () => {
  const [notices, setNotices] = useState<Notice[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 10, total: 0 });
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [editingNotice, setEditingNotice] = useState<Notice | null>(null);
  const [form] = Form.useForm();

  const fetchNotices = async () => {
    setLoading(true);
    try {
      const data = await noticeApi.list({
        page: pagination.current,
        pageSize: pagination.pageSize,
      });
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

  const handleAdd = () => {
    setEditingNotice(null);
    form.resetFields();
    setIsModalVisible(true);
  };

  const handleEdit = (record: Notice) => {
    setEditingNotice(record);
    form.setFieldsValue(record);
    setIsModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    Modal.confirm({
      title: '确认删除',
      content: '确定要删除该通知/公告吗？',
      onOk: async () => {
        try {
          await noticeApi.delete(id);
          message.success('删除成功');
          fetchNotices();
        } catch (error) {
          message.error('删除失败');
        }
      },
    });
  };

  const handleView = async (id: string) => {
    try {
      await noticeApi.incrementView(id);
      message.success('已记录浏览次数');
    } catch (error) {
      message.error('记录浏览次数失败');
    }
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
      dataIndex: 'noticeTitle',
      key: 'noticeTitle',
      width: 200,
    },
    {
      title: '类型',
      dataIndex: 'noticeType',
      key: 'noticeType',
      width: 100,
      render: (noticeType: string) => {
        const typeMap: Record<string, string> = { '1': '通知', '2': '公告' };
        return <Tag color={noticeType === '1' ? 'blue' : 'green'}>{typeMap[noticeType] || noticeType}</Tag>;
      },
    },
    {
      title: '状态',
      dataIndex: 'noticeStatus',
      key: 'noticeStatus',
      width: 100,
      render: (noticeStatus: string) => {
        const statusMap: Record<string, string> = { '0': '已发布', '1': '已取消' };
        return <Tag color={statusMap[noticeStatus] === '已发布' ? 'green' : 'red'}>
          {statusMap[noticeStatus] || noticeStatus}
        </Tag>;
      },
    },
    {
      title: '置顶',
      dataIndex: 'isTop',
      key: 'isTop',
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
      dataIndex: 'viewCount',
      key: 'viewCount',
      width: 100,
    },
    {
      title: '操作',
      key: 'action',
      width: 200,
      render: (_, record) => (
        <Space size="small">
          <Button type="link" onClick={() => handleView(record.id)}>
            浏览
          </Button>
          <Button type="link" icon={<EditOutlined />} onClick={() => handleEdit(record)}>
            编辑
          </Button>
          <Button type="link" danger icon={<DeleteOutlined />} onClick={() => handleDelete(record.id)}>
            删除
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <div style={{ marginBottom: 16 }}>
        <Space>
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
            新增通知/公告
          </Button>
          <Button icon={<ReloadOutlined />} onClick={fetchNotices}>
            刷新
          </Button>
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
          onChange: (page, pageSize) => setPagination({ ...pagination, current: page, pageSize }),
        }}
      />
      <Modal
        title={editingNotice ? '编辑通知/公告' : '新增通知/公告'}
        open={isModalVisible}
        onOk={handleSubmit}
        onCancel={() => setIsModalVisible(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="noticeTitle"
            label="标题"
            rules={[{ required: true, message: '请输入标题' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item name="noticeType" label="类型" initialValue="1">
            <Select>
              <Select.Option value="1">通知</Select.Option>
              <Select.Option value="2">公告</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item
            name="noticeContent"
            label="内容"
            rules={[{ required: true, message: '请输入内容' }]}
          >
            <Input.TextArea rows={10} />
          </Form.Item>
          <Form.Item name="noticeStatus" label="状态" initialValue="0">
            <Select>
              <Select.Option value="0">已发布</Select.Option>
              <Select.Option value="1">已取消</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="isTop" label="置顶" initialValue="N">
            <Select>
              <Select.Option value="Y">是</Select.Option>
              <Select.Option value="N">否</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="priority" label="优先级" initialValue={0}>
            <InputNumber min={0} max={100} />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default NoticeList;
