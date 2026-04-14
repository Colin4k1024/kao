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
import type { ColumnsType } from 'antd/es/table';
import {
  PlusOutlined,
  EditOutlined,
  DeleteOutlined,
  ReloadOutlined,
  SearchOutlined,
} from '@ant-design/icons';
import { dictionaryDataApi, dictionaryTypeApi, DictionaryData, DictionaryType } from '@/services/api/dictionary';

// Dictionary Data Page Component
export const DictionaryDataPage: React.FC = () => {
  const [dataList, setDataList] = useState<DictionaryData[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({
    current: 1,
    pageSize: 10,
    total: 0,
  });
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [editingData, setEditingData] = useState<DictionaryData | null>(null);
  const [dictionaryTypes, setDictionaryTypes] = useState<DictionaryType[]>([]);
  const [searchForm] = Form.useForm();
  const [form] = Form.useForm();

  // Fetch dictionary types for dropdown
  const fetchDictionaryTypes = async () => {
    try {
      const data = await dictionaryTypeApi.listAll();
      setDictionaryTypes(data);
    } catch (error) {
      message.error('获取字典类型列表失败');
    }
  };

  const fetchDataList = async () => {
    setLoading(true);
    try {
      const values = searchForm.getFieldsValue();
      const params = {
        page: pagination.current,
        pageSize: pagination.pageSize,
        dictType: values.dictType,
      };
      const data = await dictionaryDataApi.list(params);
      setDataList(data.list);
      setPagination((prev) => ({ ...prev, total: data.total }));
    } catch (error) {
      message.error('获取字典数据列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchDictionaryTypes();
    fetchDataList();
  }, [pagination.current, pagination.pageSize]);

  const handleSearch = () => {
    setPagination((prev) => ({ ...prev, current: 1 }));
    fetchDataList();
  };

  const handleReset = () => {
    searchForm.resetFields();
    setPagination((prev) => ({ ...prev, current: 1 }));
    fetchDataList();
  };

  const handleAdd = () => {
    setEditingData(null);
    form.resetFields();
    form.setFieldsValue({ status: 1 });
    setIsModalVisible(true);
  };

  const handleEdit = (record: DictionaryData) => {
    setEditingData(record);
    form.setFieldsValue(record);
    setIsModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    try {
      await dictionaryDataApi.delete(id);
      message.success('删除成功');
      fetchDataList();
    } catch (error) {
      message.error('删除失败');
    }
  };

  const handleStatusChange = async (id: string, status: number) => {
    try {
      if (status === 1) {
        await dictionaryDataApi.enable(id);
      } else {
        await dictionaryDataApi.disable(id);
      }
      message.success('状态更新成功');
      fetchDataList();
    } catch (error) {
      message.error('状态更新失败');
    }
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingData) {
        await dictionaryDataApi.update(editingData.id, values);
        message.success('更新成功');
      } else {
        await dictionaryDataApi.create(values);
        message.success('创建成功');
      }
      setIsModalVisible(false);
      fetchDataList();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const columns: ColumnsType<DictionaryData> = [
    {
      title: '字典标签',
      dataIndex: 'dictLabel',
      key: 'dictLabel',
      width: 150,
    },
    {
      title: '字典值',
      dataIndex: 'dictValue',
      key: 'dictValue',
      width: 150,
    },
    {
      title: '字典类型',
      dataIndex: 'dictType',
      key: 'dictType',
      width: 150,
    },
    {
      title: '排序',
      dataIndex: 'dictSort',
      key: 'dictSort',
      width: 80,
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
      dataIndex: 'createdAt',
      key: 'createdAt',
      width: 180,
      render: (date: string) => new Date(date).toLocaleString('zh-CN'),
    },
    {
      title: '操作',
      key: 'action',
      width: 180,
      fixed: 'right',
      render: (_: unknown, record: DictionaryData) => (
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
            description="确定要删除该字典数据吗？"
            onConfirm={() => handleDelete(record.id)}
          >
            <Button type="link" danger icon={<DeleteOutlined />}>
              删除
            </Button>
          </Popconfirm>
          <Button
            type="link"
            onClick={() =>
              handleStatusChange(record.id, record.status === 1 ? 0 : 1)
            }
          >
            {record.status === 1 ? '禁用' : '启用'}
          </Button>
        </Space>
      ),
    },
  ];

  const dictionaryTypeOptions = React.useMemo(() => {
    return dictionaryTypes.map((type) => ({
      value: type.dictType,
      label: type.dictName,
    }));
  }, [dictionaryTypes]);

  return (
    <div style={{ padding: 24 }}>
      <div style={{ marginBottom: 16 }}>
        <Space direction="vertical" style={{ width: '100%' }}>
          <Form
            form={searchForm}
            layout="inline"
            onFinish={handleSearch}
          >
            <Form.Item label="字典类型" name="dictType">
              <Select
                placeholder="请选择字典类型"
                options={dictionaryTypeOptions}
                style={{ width: 200 }}
                allowClear
              />
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
              新增数据
            </Button>
            <Button icon={<ReloadOutlined />} onClick={fetchDataList}>
              刷新
            </Button>
          </div>
        </Space>
      </div>
      <Table
        columns={columns}
        dataSource={dataList}
        loading={loading}
        rowKey="id"
        pagination={{
          current: pagination.current,
          pageSize: pagination.pageSize,
          total: pagination.total,
          showSizeChanger: true,
          showTotal: (total) => `共 ${total} 条`,
          onChange: (page, pageSize) =>
            setPagination((prev) => ({ ...prev, current: page, pageSize })),
        }}
      />
      <Modal
        title={editingData ? '编辑字典数据' : '新增字典数据'}
        open={isModalVisible}
        onOk={handleSubmit}
        onCancel={() => setIsModalVisible(false)}
        width={500}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="dictLabel"
            label="字典标签"
            rules={[{ required: true, message: '请输入字典标签' }]}
          >
            <Input placeholder="请输入字典标签" />
          </Form.Item>
          <Form.Item
            name="dictValue"
            label="字典值"
            rules={[{ required: true, message: '请输入字典值' }]}
          >
            <Input placeholder="请输入字典值" />
          </Form.Item>
          <Form.Item
            name="dictType"
            label="字典类型"
            rules={[{ required: true, message: '请选择字典类型' }]}
          >
            <Select
              placeholder="请选择字典类型"
              options={dictionaryTypeOptions}
            />
          </Form.Item>
          <Form.Item
            name="dictSort"
            label="排序"
            rules={[{ required: true, message: '请输入排序值' }]}
          >
            <Input type="number" placeholder="请输入排序值" />
          </Form.Item>
          <Form.Item name="remark" label="描述">
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

export default DictionaryDataPage;