import { PlusOutlined } from '@ant-design/icons';
import { ModalForm, ProFormSelect, ProFormText } from '@ant-design/pro-components';
import { ActionType, ProColumns, ProTable } from '@ant-design/pro-components';
import { App, Popconfirm, Tag } from 'antd';
import React, { useRef, useState } from 'react';
import * as api from '@/services/api';
import type { MenuItem } from '@/services/api/data';

const MenuManagement: React.FC = () => {
  const actionRef = useRef<ActionType>();
  const { message } = App.useApp();
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<MenuItem | null>(null);

  const handleAdd = async (values: Record<string, unknown>) => {
    try {
      if (editingRecord) {
        await api.updateMenu(editingRecord.id, values);
        message.success('更新成功');
      } else {
        await api.createMenu(values);
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
      await api.deleteMenu(id);
      message.success('删除成功');
      actionRef.current?.reload();
    } catch (error: any) {
      message.error(error?.response?.data?.message || '删除失败');
    }
  };

  const columns: ProColumns<MenuItem>[] = [
    {
      title: '菜单名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '菜单类型',
      dataIndex: 'menu_type',
      key: 'menu_type',
      render: (_, record) => {
        const typeMap: Record<string, { text: string; color: string }> = {
          DIRECTORY: { text: '目录', color: 'blue' },
          MENU: { text: '菜单', color: 'green' },
          BUTTON: { text: '按钮', color: 'orange' },
        };
        const type = typeMap[record.menu_type] || { text: record.menu_type, color: 'default' };
        return <Tag color={type.color}>{type.text}</Tag>;
      },
    },
    {
      title: '路由路径',
      dataIndex: 'route_path',
      key: 'route_path',
    },
    {
      title: '组件路径',
      dataIndex: 'component',
      key: 'component',
    },
    {
      title: '权限标识',
      dataIndex: 'permission',
      key: 'permission',
    },
    {
      title: '图标',
      dataIndex: 'icon',
      key: 'icon',
    },
    {
      title: '排序',
      dataIndex: 'sort_order',
      key: 'sort_order',
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
        <Popconfirm key="delete" title="确定删除此菜单?" onConfirm={() => handleDelete(record.id)}>
          <a style={{ color: 'red' }}>删除</a>
        </Popconfirm>,
      ],
    },
  ];

  return (
    <>
      <ProTable<MenuItem>
        columns={columns}
        actionRef={actionRef}
        request={async () => {
          const response = await api.queryMenus();
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
        title={editingRecord ? '编辑菜单' : '新建菜单'}
        open={modalVisible}
        onOpenChange={setModalVisible}
        onFinish={handleAdd}
        initialValues={editingRecord || { menu_type: 'MENU', status: 'ACTIVE', visible: true }}
      >
        <ProFormText
          name="name"
          label="菜单名称"
          rules={[{ required: true, message: '请输入菜单名称' }]}
        />
        <ProFormSelect
          name="menu_type"
          label="菜单类型"
          options={[
            { label: '目录', value: 'DIRECTORY' },
            { label: '菜单', value: 'MENU' },
            { label: '按钮', value: 'BUTTON' },
          ]}
        />
        <ProFormText name="route_path" label="路由路径" />
        <ProFormText name="component" label="组件路径" />
        <ProFormText name="permission" label="权限标识" />
        <ProFormText name="icon" label="图标" />
        <ProFormText name="sort_order" label="排序" />
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

export default MenuManagement;
