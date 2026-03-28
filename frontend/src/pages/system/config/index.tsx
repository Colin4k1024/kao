import React, { useEffect, useState } from 'react';
import { Table, Button, Space, Input, Tag, Modal, Form, Select, message } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, ReloadOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { configApi } from '@/services/api/dictionary';
import type { Config } from '@/services/api/dictionary';

const ConfigList: React.FC = () => {
  const [configs, setConfigs] = useState<Config[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 10, total: 0 });
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [editingConfig, setEditingConfig] = useState<Config | null>(null);
  const [form] = Form.useForm();

  const fetchConfigs = async () => {
    setLoading(true);
    try {
      const data = await configApi.list({
        page: pagination.current,
        pageSize: pagination.pageSize,
      });
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

  const handleAdd = () => {
    setEditingConfig(null);
    form.resetFields();
    setIsModalVisible(true);
  };

  const handleEdit = (record: Config) => {
    setEditingConfig(record);
    form.setFieldsValue(record);
    setIsModalVisible(true);
  };

  const handleDelete = async (configKey: string) => {
    Modal.confirm({
      title: '确认删除',
      content: '确定要删除该参数配置吗？',
      onOk: async () => {
        try {
          await configApi.delete(configKey);
          message.success('删除成功');
          fetchConfigs();
        } catch (error) {
          message.error('删除失败');
        }
      },
    });
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingConfig) {
        await configApi.update(editingConfig.configKey, values);
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

  const columns: ColumnsType<Config> = [
    {
      title: '参数名称',
      dataIndex: 'configName',
      key: 'configName',
      width: 150,
    },
    {
      title: '参数键名',
      dataIndex: 'configKey',
      key: 'configKey',
      width: 150,
    },
    {
      title: '参数值',
      dataIndex: 'configValue',
      key: 'configValue',
      ellipsis: true,
    },
    {
      title: '系统内置',
      dataIndex: 'configType',
      key: 'configType',
      width: 100,
      render: (configType: string) => (
        <Tag color={configType === 'Y' ? 'gold' : 'blue'}>
          {configType === 'Y' ? '是' : '否'}
        </Tag>
      ),
    },
    {
      title: '是否加密',
      dataIndex: 'isEncrypt',
      key: 'isEncrypt',
      width: 100,
      render: (isEncrypt: string) => (
        <Tag color={isEncrypt === 'Y' ? 'red' : 'default'}>
          {isEncrypt === 'Y' ? '是' : '否'}
        </Tag>
      ),
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
      title: '操作',
      key: 'action',
      width: 150,
      render: (_, record) => (
        <Space size="small">
          <Button type="link" icon={<EditOutlined />} onClick={() => handleEdit(record)}>
            编辑
          </Button>
          <Button type="link" danger icon={<DeleteOutlined />} onClick={() => handleDelete(record.configKey)}>
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
            新增参数
          </Button>
          <Button icon={<ReloadOutlined />} onClick={fetchConfigs}>
            刷新
          </Button>
        </Space>
      </div>
      <Table
        columns={columns}
        dataSource={configs}
        loading={loading}
        rowKey="configKey"
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
        title={editingConfig ? '编辑参数' : '新增参数'}
        open={isModalVisible}
        onOk={handleSubmit}
        onCancel={() => setIsModalVisible(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="configName"
            label="参数名称"
            rules={[{ required: true, message: '请输入参数名称' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item
            name="configKey"
            label="参数键名"
            rules={[{ required: true, message: '请输入参数键名' }]}
          >
            <Input disabled={!!editingConfig} />
          </Form.Item>
          <Form.Item
            name="configValue"
            label="参数值"
            rules={[{ required: true, message: '请输入参数值' }]}
          >
            <Input.TextArea rows={6} />
          </Form.Item>
          <Form.Item name="configType" label="系统内置" initialValue="N">
            <Select>
              <Select.Option value="Y">是</Select.Option>
              <Select.Option value="N">否</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="isEncrypt" label="是否加密" initialValue="N">
            <Select>
              <Select.Option value="Y">是</Select.Option>
              <Select.Option value="N">否</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="status" label="状态" initialValue={1}>
            <Select>
              <Select.Option value={1}>正常</Select.Option>
              <Select.Option value={0}>停用</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea rows={4} />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
};

export default ConfigList;
