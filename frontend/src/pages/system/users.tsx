import { PlusOutlined } from '@ant-design/icons';
import { ModalForm, ProFormSelect, ProFormText } from '@ant-design/pro-components';
import { ActionType, ProColumns, ProTable } from '@ant-design/pro-components';
import { App, Button, Popconfirm } from 'antd';
import React, { useRef, useState } from 'react';
import * as api from '@/services/api';
import type { TableListItem } from '@/services/api/data';

const UserManagement: React.FC = () => {
  const actionRef = useRef<ActionType>();
  const { message } = App.useApp();
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<TableListItem | null>(null);

  const handleAdd = async (values: Partial<TableListItem>) => {
    try {
      if (editingRecord) {
        await api.updateUser(editingRecord.id, values);
        message.success('更新成功');
      } else {
        await api.createUser(values);
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
      await api.deleteUser(id);
      message.success('删除成功');
      actionRef.current?.reload();
    } catch (error: any) {
      message.error(error?.response?.data?.message || '删除失败');
    }
  };

  const columns: ProColumns<TableListItem>[] = [
    {
      title: '用户名',
      dataIndex: 'username',
      key: 'username',
    },
    {
      title: '显示名称',
      dataIndex: 'display_name',
      key: 'display_name',
    },
    {
      title: '邮箱',
      dataIndex: 'email',
      key: 'email',
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
      title: '超级管理员',
      dataIndex: 'is_super_admin',
      key: 'is_super_admin',
      valueType: 'select',
      render: (_, record) => (record.is_super_admin ? '是' : '否'),
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
          title="确定删除此用户?"
          onConfirm={() => handleDelete(record.id)}
        >
          <a style={{ color: 'red' }}>删除</a>
        </Popconfirm>,
      ],
    },
  ];

  return (
    <>
      <ProTable<TableListItem>
        columns={columns}
        actionRef={actionRef}
        request={async (params) => {
          const response = await api.queryUsers({
            page: params.current,
            pageSize: params.pageSize,
          });
          return {
            data: response.data?.items || response.data || [],
            total: response.data?.total || 0,
            success: true,
          };
        }}
        rowKey="id"
        search={false}
        pagination={{ pageSize: 10 }}
        toolBarRender={() => [
          <Button
            type="primary"
            key="primary"
            onClick={() => {
              setEditingRecord(null);
              setModalVisible(true);
            }}
          >
            <PlusOutlined /> 新建用户
          </Button>,
        ]}
      />

      <ModalForm
        title={editingRecord ? '编辑用户' : '新建用户'}
        open={modalVisible}
        onOpenChange={setModalVisible}
        onFinish={handleAdd}
        initialValues={editingRecord || {}}
      >
        <ProFormText
          name="username"
          label="用户名"
          disabled={!!editingRecord}
          rules={[{ required: true, message: '请输入用户名' }]}
        />
        {!editingRecord && (
          <ProFormText.Password
            name="password"
            label="密码"
            rules={[{ required: true, message: '请输入密码' }]}
          />
        )}
        <ProFormText name="display_name" label="显示名称" />
        <ProFormText name="email" label="邮箱" />
        <ProFormText name="phone" label="手机号" />
        <ProFormSelect
          name="status"
          label="状态"
          initialValue="ACTIVE"
          options={[
            { label: '正常', value: 'ACTIVE' },
            { label: '禁用', value: 'DISABLED' },
          ]}
        />
      </ModalForm>
    </>
  );
};

export default UserManagement;
