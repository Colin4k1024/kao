import { PlusOutlined } from '@ant-design/icons';
import { ModalForm, ProFormSelect, ProFormText } from '@ant-design/pro-components';
import { ActionType, ProColumns, ProTable } from '@ant-design/pro-components';
import { App, Popconfirm, Tag } from 'antd';
import React, { useRef, useState } from 'react';
import * as api from '@/services/api';
import type { RoleItem } from '@/services/api/data';

const RoleManagement: React.FC = () => {
  const actionRef = useRef<ActionType>();
  const { message } = App.useApp();
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<RoleItem | null>(null);

  const handleAdd = async (values: Record<string, unknown>) => {
    try {
      if (editingRecord) {
        await api.updateRole(editingRecord.id, values);
        message.success('更新成功');
      } else {
        await api.createRole(values);
        message.success('创建成功');
      }
      setModalVisible(false);
      actionRef.current?.reload();
    } catch (error: any) {
      message.error(error?.response?.data?.message || '操作失败');
    }
  };

  const handleDelete = async (id: string) => {
    try {
      await api.deleteRole(id);
      message.success('删除成功');
      actionRef.current?.reload();
    } catch (error: any) {
      message.error(error?.response?.data?.message || '删除失败');
    }
  };

  const columns: ProColumns<RoleItem>[] = [
    {
      title: '角色代码',
      dataIndex: 'code',
      key: 'code',
    },
    {
      title: '角色名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
    },
    {
      title: '数据范围',
      dataIndex: 'data_scope',
      key: 'data_scope',
      render: (_, record) => {
        const scopeMap: Record<string, { text: string; color: string }> = {
          ALL: { text: '全部数据', color: 'green' },
          CUSTOM: { text: '自定义', color: 'blue' },
          DEPT: { text: '本部门', color: 'orange' },
          DEPT_AND_CHILD: { text: '本部门及子部门', color: 'purple' },
          SELF: { text: '仅本人', color: 'red' },
        };
        const scope = scopeMap[record.data_scope] || { text: record.data_scope, color: 'default' };
        return <Tag color={scope.color}>{scope.text}</Tag>;
      },
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      valueType: 'select',
      valueEnum: {
        ACTIVE: { text: '正常', status: 'Success' },
        DISABLED: { text: '禁用', status: 'Error' },
      },
    },
    {
      title: '系统角色',
      dataIndex: 'is_system',
      key: 'is_system',
      render: (_, record) => (record.is_system ? '是' : '否'),
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      valueType: 'dateTime',
    },
    {
      title: '操作',
      valueType: 'option',
      render: (_, record) =>
        record.is_system ? (
          <span style={{ color: '#999' }}>系统角色不可操作</span>
        ) : (
          <>
            <a
              key="edit"
              onClick={() => {
                setEditingRecord(record);
                setModalVisible(true);
              }}
            >
              编辑
            </a>
            <Popconfirm key="delete" title="确定删除此角色?" onConfirm={() => handleDelete(record.id)}>
              <a style={{ color: 'red' }}>删除</a>
            </Popconfirm>
          </>
        ),
    },
  ];

  return (
    <>
      <ProTable<RoleItem>
        columns={columns}
        actionRef={actionRef}
        request={async () => {
          const response = await api.queryRoles();
          return {
            data: response.data || [],
            total: (response.data || []).length,
            success: true,
          };
        }}
        rowKey="id"
        search={false}
        pagination={false}
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
        title={editingRecord ? '编辑角色' : '新建角色'}
        open={modalVisible}
        onOpenChange={setModalVisible}
        onFinish={handleAdd}
        initialValues={editingRecord || { data_scope: 'DEPT', status: 'ACTIVE' }}
      >
        <ProFormText
          name="code"
          label="角色代码"
          disabled={!!editingRecord}
          rules={[{ required: true, message: '请输入角色代码' }]}
        />
        <ProFormText
          name="name"
          label="角色名称"
          rules={[{ required: true, message: '请输入角色名称' }]}
        />
        <ProFormText name="description" label="描述" />
        <ProFormSelect
          name="data_scope"
          label="数据范围"
          options={[
            { label: '全部数据', value: 'ALL' },
            { label: '自定义', value: 'CUSTOM' },
            { label: '本部门', value: 'DEPT' },
            { label: '本部门及子部门', value: 'DEPT_AND_CHILD' },
            { label: '仅本人', value: 'SELF' },
          ]}
        />
        <ProFormSelect
          name="status"
          label="状态"
          options={[
            { label: '正常', value: 'ACTIVE' },
            { label: '禁用', value: 'DISABLED' },
          ]}
        />
      </ModalForm>
    </>
  );
};

export default RoleManagement;
