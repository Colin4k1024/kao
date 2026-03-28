import React, { useEffect, useState } from 'react';
import { Table, Button, Space, Input, Tag, Modal, Form, Select, message } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { dictionaryTypeApi } from '@/services/api/dictionary';
import type { DictionaryType } from '@/services/api/dictionary';

const DictionaryTypeList: React.FC = () => {
  const [types, setTypes] = useState<DictionaryType[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 10, total: 0 });
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [editingType, setEditingType] = useState<DictionaryType | null>(null);
  const [form] = Form.useForm();
  const [searchText, setSearchText] = useState('');

  const fetchTypes = async () => {
    setLoading(true);
    try {
      const data = await dictionaryTypeApi.list({
        page: pagination.current,
        pageSize: pagination.pageSize,
        dictName: searchText,
        dictType: searchText,
      });
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

  const handleAdd = () => {
    setEditingType(null);
    form.resetFields();
    setIsModalVisible(true);
  };

  const handleEdit = (record: DictionaryType) => {
    setEditingType(record);
    form.setFieldsValue(record);
    setIsModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    Modal.confirm({
      title: '确认删除',
      content: '确定要删除该字典类型吗？',
      onOk: async () => {
        try {
          await dictionaryTypeApi.delete(id);
          message.success('删除成功');
          fetchTypes();
        } catch (error) {
          message.error('删除失败');
        }
      },
    });
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
      title: '字典名称',
      dataIndex: 'dictName',
      key: 'dictName',
      width: 150,
      render: (text: string) => <span>{text}</span>,
    },
    {
      title: '字典类型',
      dataIndex: 'dictType',
      key: 'dictType',
      width: 150,
      render: (text: string) => <span>{text}</span>,
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 100,
      render: (status: number) => (
        <Tag color={status === 1 ? 'green' : 'red'}>
          {status === 1 ? '正常' : '停用'}
        </Tag>
      ),
    },
    {
      title: '备注',
      dataIndex: 'remark',
      key: 'remark',
    },
    {
      title: '创建时间',
      dataIndex: 'createdAt',
      key: 'createdAt',
      width: 180,
      render: (text: string) => <span>{text}</span>,
    },
    {
      title: '操作',
      key: 'action',
      width: 150,
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
          <Input
            placeholder="搜索字典名称或类型"
            style={{ width: 200 }}
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            onPressEnter={fetchTypes}
          />
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
            新增字典类型
          </Button>
          <Button icon={<ReloadOutlined />} onClick={fetchTypes}>
            刷新
          </Button>
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
          onChange: (page, pageSize) => setPagination({ ...pagination, current: page, pageSize }),
        }}
      />
      <Modal
        title={editingType ? '编辑字典类型' : '新增字典类型'}
        open={isModalVisible}
        onOk={handleSubmit}
        onCancel={() => setIsModalVisible(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="dictName"
            label="字典名称"
            rules={[{ required: true, message: '请输入字典名称' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item
            name="dictType"
            label="字典类型"
            rules={[{ required: true, message: '请输入字典类型' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item name="status" label="状态" initialValue={1}>
            <Select>
              <Select.Option value={1}>正常</Select.Option>
              <Select.Option value={0}>停用</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea autoSize={{ minRows: 4, maxRows: 6 }} />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default DictionaryTypeList;
