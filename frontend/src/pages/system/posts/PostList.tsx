import React, { useEffect, useState } from 'react';
import { Table, Button, Space, Input, Tag, Modal, Form, Select, message } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { postService, Post } from '@/services/systemService';

const PostList: React.FC = () => {
  const [posts, setPosts] = useState<Post[]>([]);
  const [loading, setLoading] = useState(false);
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [editingPost, setEditingPost] = useState<Post | null>(null);
  const [form] = Form.useForm();

  const fetchPosts = async () => {
    setLoading(true);
    try {
      const data = await postService.list();
      setPosts(data);
    } catch (error) {
      message.error('获取岗位列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchPosts();
  }, []);

  const handleAdd = () => {
    setEditingPost(null);
    form.resetFields();
    setIsModalVisible(true);
  };

  const handleEdit = (record: Post) => {
    setEditingPost(record);
    form.setFieldsValue(record);
    setIsModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    Modal.confirm({
      title: '确认删除',
      content: '确定要删除该岗位吗？',
      onOk: async () => {
        try {
          await postService.delete(id);
          message.success('删除成功');
          fetchPosts();
        } catch (error) {
          message.error('删除失败');
        }
      },
    });
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingPost) {
        await postService.update(editingPost.id, values);
        message.success('更新成功');
      } else {
        await postService.create(values);
        message.success('创建成功');
      }
      setIsModalVisible(false);
      fetchPosts();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const columns: ColumnsType<Post> = [
    {
      title: '岗位编码',
      dataIndex: 'post_code',
      key: 'post_code',
    },
    {
      title: '岗位名称',
      dataIndex: 'post_name',
      key: 'post_name',
    },
    {
      title: '显示顺序',
      dataIndex: 'display_order',
      key: 'display_order',
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: number) => (
        <Tag color={status === 1 ? 'green' : 'red'}>
          {status === 1 ? '启用' : '禁用'}
        </Tag>
      ),
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record) => (
        <Space size="small">
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
          <Input placeholder="搜索岗位名称" style={{ width: 200 }} />
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
            新增岗位
          </Button>
          <Button icon={<ReloadOutlined />} onClick={fetchPosts}>
            刷新
          </Button>
        </Space>
      </div>
      <Table
        columns={columns}
        dataSource={posts}
        loading={loading}
        rowKey="id"
        pagination={false}
      />
      <Modal
        title={editingPost ? '编辑岗位' : '新增岗位'}
        open={isModalVisible}
        onOk={handleSubmit}
        onCancel={() => setIsModalVisible(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="post_code"
            label="岗位编码"
            rules={[{ required: true, message: '请输入岗位编码' }]}
          >
            <Input disabled={!!editingPost} />
          </Form.Item>
          <Form.Item
            name="post_name"
            label="岗位名称"
            rules={[{ required: true, message: '请输入岗位名称' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item name="display_order" label="显示顺序" initialValue={0}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="status" label="状态" initialValue={1}>
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

export default PostList;
