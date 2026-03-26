import React, { useEffect, useState } from 'react';
import { Table, Button, Space, Input, Tag, Modal, Form, Select, TextArea, InputNumber, message } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { dictionaryDataApi, dictionaryTypeApi } from '@/services/api/dictionary';
import type { DictionaryData, DictionaryType } from '@/services/api/dictionary';

const DictionaryDataList: React.FC = () => {
  const [datas, setDatas] = useState<DictionaryData[]>([]);
  const [types, setTypes] = useState<DictionaryType[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 10, total: 0 });
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [editingData, setEditingData] = useState<DictionaryData | null>(null);
  const [selectedDictType, setSelectedDictType] = useState<string>('');
  const [form] = Form.useForm();

  const fetchTypes = async () => {
    try {
      const data = await dictionaryTypeApi.listAll();
      setTypes(data);
    } catch (error) {
      message.error('获取字典类型列表失败');
    }
  };

  const fetchDatas = async () => {
    setLoading(true);
    try {
      const params: { page?: number; pageSize?: number; dictType?: string } = {
        page: pagination.current,
        pageSize: pagination.pageSize,
      };
      if (selectedDictType) {
        params.dictType = selectedDictType;
      }
      const data = await dictionaryDataApi.list(params);
      setDatas(data.list);
      setPagination({ ...pagination, total: data.total });
    } catch (error) {
      message.error('获取字典数据列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTypes();
  }, []);

  useEffect(() => {
    if (selectedDictType) {
      fetchDatas();
    }
  }, [selectedDictType, pagination.current, pagination.pageSize]);

  const handleAdd = () => {
    setEditingData(null);
    form.resetFields();
    setIsModalVisible(true);
  };

  const handleEdit = (record: DictionaryData) => {
    setEditingData(record);
    form.setFieldsValue(record);
    setIsModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    Modal.confirm({
      title: '确认删除',
      content: '确定要删除该字典数据吗？',
      onOk: async () => {
        try {
          await dictionaryDataApi.delete(id);
          message.success('删除成功');
          fetchDatas();
        } catch (error) {
          message.error('删除失败');
        }
      },
    });
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
      fetchDatas();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const columns: ColumnsType<DictionaryData> = [
    {
      title: '字典标签',
      dataIndex: 'dictLabel',
      key: 'dictLabel',
      width: 120,
    },
    {
      title: '字典值',
      dataIndex: 'dictValue',
      key: 'dictValue',
      width: 120,
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
          {status === 1 ? '正常' : '停用'}
        </Tag>
      ),
    },
    {
      title: '默认',
      dataIndex: 'isDefault',
      key: 'isDefault',
      width: 80,
      render: (isDefault: string) => (
        <Tag color={isDefault === 'Y' ? 'blue' : 'default'}>
          {isDefault === 'Y' ? '是' : '否'}
        </Tag>
      ),
    },
    {
      title: '备注',
      dataIndex: 'remark',
      key: 'remark',
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
          <Select
            placeholder="选择字典类型"
            style={{ width: 200 }}
            value={selectedDictType}
            onChange={(value) => {
              setSelectedDictType(value);
              setPagination({ ...pagination, current: 1 });
            }}
          >
            {types.map((type) => (
              <Select.Option key={type.dictType} value={type.dictType}>
                {type.dictName} ({type.dictType})
              </Select.Option>
            ))}
          </Select>
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
            新增字典数据
          </Button>
          <Button icon={<ReloadOutlined />} onClick={fetchDatas}>
            刷新
          </Button>
        </Space>
      </div>
      <Table
        columns={columns}
        dataSource={datas}
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
        title={editingData ? '编辑字典数据' : '新增字典数据'}
        open={isModalVisible}
        onOk={handleSubmit}
        onCancel={() => setIsModalVisible(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="dictLabel"
            label="字典标签"
            rules={[{ required: true, message: '请输入字典标签' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item
            name="dictValue"
            label="字典值"
            rules={[{ required: true, message: '请输入字典值' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item name="dictSort" label="排序" initialValue={0}>
            <InputNumber min={0} />
          </Form.Item>
          <Form.Item name="dictType" label="字典类型" rules={[{ required: true, message: '请输入字典类型' }]}>
            <Input />
          </Form.Item>
          <Form.Item name="status" label="状态" initialValue={1}>
            <Select>
              <Select.Option value={1}>正常</Select.Option>
              <Select.Option value={0}>停用</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="isDefault" label="默认" initialValue="N">
            <Select>
              <Select.Option value="Y">是</Select.Option>
              <Select.Option value="N">否</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <TextArea autoSize={{ minRows: 4, maxRows: 6 }} />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default DictionaryDataList;
