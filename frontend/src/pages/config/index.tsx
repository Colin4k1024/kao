import { PlusOutlined } from '@ant-design/icons';
import { ModalForm, ProFormSelect, ProFormText } from '@ant-design/pro-components';
import { ActionType, ProColumns, ProTable } from '@ant-design/pro-components';
import { App, Popconfirm, Tag } from 'antd';
import React, { useRef, useState } from 'react';
import * as api from '@/services/api';
import type { ConfigItem } from '@/services/api/data';

// ConfigItem interface - matches backend ConfigResponse
export interface ConfigItem {
  id: string;
  config_key: string;
  config_name: string;
  config_value: string;
  config_type: string;
  is_encrypt: string;
  status: number;
  remark?: string;
  created_by?: string;
  updated_by?: string;
  created_at: string;
  updated_at: string;
}

const ConfigManagement: React.FC = () => {
  const actionRef = useRef<ActionType>();
  const { message } = App.useApp();
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<ConfigItem | null>(null);

  // Fetch configs from API
  const fetchConfigs = async () => {
    const response = await api.queryConfigs();
    return {
      data: response.data?.items || response.data || [],
      total: response.data?.total || (response.data?.items || response.data || []).length,
      success: true,
    };
  };

  // Handle create/update
  const handleAdd = async (values: Record<string, unknown>) => {
    try {
      if (editingRecord) {
        await api.updateConfig(editingRecord.config_key, values);
        message.success('更新成功');
      } else {
        await api.createConfig(values);
        message.success('创建成功');
      }
      setModalVisible(false);
      actionRef.current?.reload();
    } catch (error: any) {
      message.error(error?.response?.data?.message || '操作失败');
    }
  };

  // Handle delete
  const handleDelete = async (config_key: string) => {
    try {
      await api.deleteConfig(config_key);
      message.success('删除成功');
      actionRef.current?.reload();
    } catch (error: any) {
      message.error(error?.response?.data?.message || '删除失败');
    }
  };

  // Config type mapping
  const getConfigTypeTag = (type: string) => {
    const typeMap: Record<string, { text: string; color: string }> = {
      '0': { text: '自定义', color: 'blue' },
      '1': { text: '系统内置', color: 'green' },
      '2': { text: '加密', color: 'orange' },
    };
    const configType = typeMap[type] || { text: type, color: 'default' };
    return <Tag color={configType.color}>{configType.text}</Tag>;
  };

  // Status mapping
  const getStatusTag = (status: number) => {
    return status === 1 ? (
      <Tag color="success">启用</Tag>
    ) : (
      <Tag color="error">禁用</Tag>
    );
  };

  // Table columns
  const columns: ProColumns<ConfigItem>[] = [
    {
      title: '参数键',
      dataIndex: 'config_key',
      key: 'config_key',
      width: 200,
      copyable: true,
    },
    {
      title: '参数名称',
      dataIndex: 'config_name',
      key: 'config_name',
      width: 150,
      ellipsis: true,
    },
    {
      title: '参数值',
      dataIndex: 'config_value',
      key: 'config_value',
      width: 200,
      ellipsis: true,
      hideInSearch: true,
    },
    {
      title: '参数类型',
      dataIndex: 'config_type',
      key: 'config_type',
      width: 100,
      render: (_, record) => getConfigTypeTag(record.config_type),
      valueType: 'select',
      valueEnum: {
        '0': { text: '自定义', status: 'Default' },
        '1': { text: '系统内置', status: 'Success' },
        '2': { text: '加密', status: 'Processing' },
      },
    },
    {
      title: '加密',
      dataIndex: 'is_encrypt',
      key: 'is_encrypt',
      width: 80,
      hideInSearch: true,
      render: (_, record) => (record.is_encrypt === '1' ? '是' : '否'),
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 80,
      render: (_, record) => getStatusTag(record.status),
      valueType: 'select',
      valueEnum: {
        1: { text: '启用', status: 'Success' },
        0: { text: '禁用', status: 'Error' },
      },
    },
    {
      title: '备注',
      dataIndex: 'remark',
      key: 'remark',
      width: 150,
      ellipsis: true,
      hideInSearch: true,
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      width: 180,
      valueType: 'dateTime',
      hideInSearch: true,
    },
    {
      title: '操作',
      valueType: 'option',
      width: 150,
      fixed: 'right',
      render: (_, record) => [
        <a
          key="edit"
          onClick={() => {
            setEditingRecord(record);
            setModalVisible(true);
          }}
        >
          编辑
        </a>,
        <Popconfirm
          key="delete"
          title="确定删除此参数配置?"
          description="删除后无法恢复，请确认"
          onConfirm={() => handleDelete(record.config_key)}
        >
          <a style={{ color: 'red' }}>删除</a>
        </Popconfirm>,
      ],
    },
  ];

  return (
    <>
      <ProTable<ConfigItem>
        columns={columns}
        actionRef={actionRef}
        request={fetchConfigs}
        rowKey="config_key"
        search={{
          labelWidth: 'auto',
        }}
        pagination={{
          pageSize: 10,
          showSizeChanger: true,
          showQuickJumper: true,
        }}
        toolBarRender={() => [
          <PlusOutlined
            style={{ fontSize: 18, cursor: 'pointer', color: '#1890ff' }}
            key="primary"
            onClick={() => {
              setEditingRecord(null);
              setModalVisible(true);
            }}
          />,
        ]}
      />

      <ModalForm
        title={editingRecord ? '编辑参数配置' : '新建参数配置'}
        open={modalVisible}
        onOpenChange={setModalVisible}
        onFinish={handleAdd}
        initialValues={editingRecord || { config_type: '0', is_encrypt: '0', status: 1 }}
        layout="horizontal"
        modalProps={{
          destroyOnClose: true,
        }}
      >
        <ProFormText
          name="config_key"
          label="参数键"
          placeholder="请输入参数键，如：sys.app.name"
          disabled={!!editingRecord}
          rules={[
            { required: true, message: '请输入参数键' },
            { pattern: /^[a-zA-Z][a-zA-Z0-9_.]+$/, message: '参数键以字母开头，支持字母、数字、下划线和点' },
          ]}
          tooltip="参数键是唯一标识，编辑后不可修改"
        />
        <ProFormText
          name="config_name"
          label="参数名称"
          placeholder="请输入参数名称"
          rules={[{ required: true, message: '请输入参数名称' }]}
        />
        <ProFormText
          name="config_value"
          label="参数值"
          placeholder="请输入参数值"
          rules={[{ required: true, message: '请输入参数值' }]}
          fieldProps={{
            rows: 3,
          }}
        />
        <ProFormSelect
          name="config_type"
          label="参数类型"
          options={[
            { label: '自定义', value: '0' },
            { label: '系统内置', value: '1' },
            { label: '加密', value: '2' },
          ]}
          rules={[{ required: true, message: '请选择参数类型' }]}
        />
        <ProFormSelect
          name="is_encrypt"
          label="是否加密"
          options={[
            { label: '否', value: '0' },
            { label: '是', value: '1' },
          ]}
        />
        <ProFormSelect
          name="status"
          label="状态"
          options={[
            { label: '启用', value: 1 },
            { label: '禁用', value: 0 },
          ]}
          rules={[{ required: true, message: '请选择状态' }]}
        />
        <ProFormText
          name="remark"
          label="备注"
          placeholder="请输入备注信息"
          fieldProps={{
            rows: 2,
          }}
        />
      </ModalForm>
    </>
  );
};

export default ConfigManagement;
