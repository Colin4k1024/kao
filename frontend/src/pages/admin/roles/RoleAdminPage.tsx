import { PlusOutlined, EditOutlined, DeleteOutlined, ReloadOutlined } from '@ant-design/icons';
import { ModalForm, ProFormSelect, ProFormText, ProFormTextArea } from '@ant-design/pro-components';
import { ActionType, ProColumns, ProTable } from '@ant-design/pro-components';
import { App, Popconfirm, Tag, Tree, message } from 'antd';
import React, { useRef, useState } from 'react';
import * as api from '@/services/api';
import type { RoleItem } from '@/services/api/data';
import type { DataNode } from 'antd/es/tree';

// RoleItem matches backend RoleResponse
// API: GET /v1/roles -> { items: RoleItem[], total: number }
//       POST /v1/roles -> RoleItem
//       PUT  /v1/roles/:id -> RoleItem
//       DELETE /v1/roles/:id

// data_scope options (matches backend model.rs)
const DATA_SCOPE_OPTIONS = [
  { label: '全部数据 (ALL)', value: 'ALL' },
  { label: '自定义数据 (CUSTOM)', value: 'CUSTOM' },
  { label: '本部门数据 (DEPT)', value: 'DEPT' },
  { label: '本部门及以下数据 (DEPT_AND_CHILD)', value: 'DEPT_AND_CHILD' },
  { label: '仅本人数据 (SELF)', value: 'SELF' },
];

// status options
const STATUS_OPTIONS = [
  { label: '启用', value: '1' },
  { label: '禁用', value: '0' },
];

// Data scope tag color
const getDataScopeTag = (scope: string) => {
  const map: Record<string, { text: string; color: string }> = {
    ALL: { text: '全部数据', color: 'blue' },
    CUSTOM: { text: '自定义', color: 'orange' },
    DEPT: { text: '本部门', color: 'cyan' },
    DEPT_AND_CHILD: { text: '本部门及子级', color: 'green' },
    SELF: { text: '仅本人', color: 'default' },
  };
  const item = map[scope] || { text: scope, color: 'default' };
  return <Tag color={item.color}>{item.text}</Tag>;
};

const RoleAdminPage: React.FC = () => {
  const actionRef = useRef<ActionType>();
  const { message: antMessage } = App.useApp();
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRole, setEditingRole] = useState<RoleItem | null>(null);
  const [menuModalVisible, setMenuModalVisible] = useState(false);
  const [menuTree, setMenuTree] = useState<DataNode[]>([]);
  const [selectedMenuKeys, setSelectedMenuKeys] = useState<string[]>([]);
  const [menuLoading, setMenuLoading] = useState(false);

  // Fetch roles
  const fetchRoles = async () => {
    const response = await api.queryRoles() as any;
    const raw = response.data;
    const items = Array.isArray(raw) ? raw : (raw?.items || []);
    return {
      data: items,
      total: items.length,
      success: true,
    };
  };

  // Fetch menu tree for permission assignment
  const fetchMenuTree = async () => {
    setMenuLoading(true);
    try {
      const data = await api.queryMenus();
      const rawMenus = data.data || data || [];
      // Transform to DataNode tree
      const buildTree = (menus: any[]): DataNode[] =>
        menus.map((m) => ({
          title: m.menu_name || m.name,
          key: m.id,
          children: m.children ? buildTree(m.children) : undefined,
        }));
      setMenuTree(buildTree(rawMenus));
    } catch (error) {
      antMessage.error('获取菜单树失败');
    } finally {
      setMenuLoading(false);
    }
  };

  // Open menu assignment modal
  const handleAssignMenus = async (record: RoleItem) => {
    setEditingRole(record);
    await fetchMenuTree();
    // Load current role's menu_ids if available
    setSelectedMenuKeys([]);
    setMenuModalVisible(true);
  };

  // Handle create/update
  const handleSubmit = async (values: Record<string, unknown>) => {
    try {
      if (editingRole) {
        await api.updateRole(editingRole.id, values);
        antMessage.success('更新成功');
      } else {
        await api.createRole(values);
        antMessage.success('创建成功');
      }
      setModalVisible(false);
      actionRef.current?.reload();
    } catch (error: any) {
      antMessage.error(error?.response?.data?.message || error?.message || '操作失败');
    }
  };

  // Handle delete
  const handleDelete = async (id: string) => {
    try {
      await api.deleteRole(id);
      antMessage.success('删除成功');
      actionRef.current?.reload();
    } catch (error: any) {
      antMessage.error(error?.response?.data?.message || '删除失败');
    }
  };

  // Handle menu assignment
  const handleMenuSubmit = async () => {
    if (!editingRole) return;
    try {
      // The backend uses menu_ids in UpdateRoleRequest
      await api.updateRole(editingRole.id, { menu_ids: selectedMenuKeys });
      antMessage.success('菜单权限分配成功');
      setMenuModalVisible(false);
    } catch (error: any) {
      antMessage.error(error?.response?.data?.message || '分配失败');
    }
  };

  // Table columns
  const columns: ProColumns<RoleItem>[] = [
    {
      title: '角色编码',
      dataIndex: 'code',
      key: 'code',
      width: 160,
      copyable: true,
      fixed: 'left',
    },
    {
      title: '角色名称',
      dataIndex: 'name',
      key: 'name',
      width: 160,
    },
    {
      title: '数据范围',
      dataIndex: 'data_scope',
      key: 'data_scope',
      width: 140,
      render: (_, record) => getDataScopeTag(record.data_scope),
      valueType: 'select',
      valueEnum: {
        ALL: { text: '全部数据', status: 'Default' },
        CUSTOM: { text: '自定义', status: 'Default' },
        DEPT: { text: '本部门', status: 'Default' },
        DEPT_AND_CHILD: { text: '本部门及子级', status: 'Default' },
        SELF: { text: '仅本人', status: 'Default' },
      },
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 90,
      render: (_, record) => (
        <Tag color={record.status === '1' ? 'success' : 'error'}>
          {record.status === '1' ? '启用' : '禁用'}
        </Tag>
      ),
      valueType: 'select',
      valueEnum: {
        '1': { text: '启用', status: 'Success' },
        '0': { text: '禁用', status: 'Error' },
      },
    },
    {
      title: '系统角色',
      dataIndex: 'is_system',
      key: 'is_system',
      width: 100,
      render: (_, record) => (
        record.is_system ? <Tag color="blue">是</Tag> : <Tag>否</Tag>
      ),
      hideInSearch: true,
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
      width: 200,
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
      sorter: true,
    },
    {
      title: '操作',
      valueType: 'option',
      width: 200,
      fixed: 'right',
      render: (_, record) => [
        <a
          key="edit"
          onClick={() => {
            setEditingRole(record);
            setModalVisible(true);
          }}
        >
          <EditOutlined /> 编辑
        </a>,
        <a
          key="menus"
          onClick={() => handleAssignMenus(record)}
        >
          分配菜单
        </a>,
        <Popconfirm
          key="delete"
          title="确定删除该角色？"
          description="删除后无法恢复，请确认"
          onConfirm={() => handleDelete(record.id)}
          disabled={record.is_system === true}
        >
          <a style={{ color: record.is_system ? '#ccc' : 'red', cursor: record.is_system ? 'not-allowed' : 'pointer', pointerEvents: record.is_system ? 'none' : 'auto' }}>
            <DeleteOutlined /> 删除
          </a>
        </Popconfirm>,
      ],
    },
  ];

  return (
    <>
      <ProTable<RoleItem>
        columns={columns}
        actionRef={actionRef}
        request={fetchRoles}
        rowKey="id"
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
              setEditingRole(null);
              setModalVisible(true);
            }}
          />,
        ]}
        scroll={{ x: 1200 }}
      />

      {/* Create / Edit Modal */}
      <ModalForm
        title={editingRole ? '编辑角色' : '新建角色'}
        open={modalVisible}
        onOpenChange={setModalVisible}
        onFinish={handleSubmit}
        initialValues={{
          ...editingRole,
          status: editingRole?.status ?? '1',
          data_scope: editingRole?.data_scope ?? 'ALL',
        }}
        layout="horizontal"
        modalProps={{
          destroyOnClose: true,
        }}
      >
        <ProFormText
          name="code"
          label="角色编码"
          placeholder="如: ROLE_ADMIN"
          rules={[
            { required: true, message: '请输入角色编码' },
            { pattern: /^[A-Z_]+$/, message: '角色编码必须是大写字母和下划线' },
          ]}
          disabled={!!editingRole}
          tooltip={editingRole ? '角色编码不可修改' : '唯一标识，创建后不可修改'}
        />
        <ProFormText
          name="name"
          label="角色名称"
          placeholder="请输入角色名称"
          rules={[{ required: true, message: '请输入角色名称' }]}
        />
        <ProFormTextArea
          name="description"
          label="描述"
          placeholder="请输入角色描述"
          fieldProps={{ rows: 2 }}
        />
        <ProFormSelect
          name="data_scope"
          label="数据范围"
          options={DATA_SCOPE_OPTIONS}
          rules={[{ required: true, message: '请选择数据范围' }]}
        />
        <ProFormSelect
          name="status"
          label="状态"
          options={STATUS_OPTIONS}
          rules={[{ required: true, message: '请选择状态' }]}
        />
      </ModalForm>

      {/* Menu Assignment Modal */}
      <ModalForm
        title={`分配菜单权限 - ${editingRole?.name || ''}`}
        open={menuModalVisible}
        onOpenChange={setMenuModalVisible}
        onFinish={handleMenuSubmit}
        layout="horizontal"
        modalProps={{
          destroyOnClose: true,
          width: 600,
        }}
        submitter={{
          searchConfig: { submitText: '确认分配', resetText: '取消' },
        }}
      >
        <Tree
          checkable
          selectable={false}
          treeData={menuTree}
          checkedKeys={selectedMenuKeys}
          onCheck={(checked) => {
            const keys = (checked as string[] || []);
            setSelectedMenuKeys(keys);
          }}
          style={{ maxHeight: 400, overflowY: 'auto', border: '1px solid #d9d9d9', padding: 8, borderRadius: 4 }}
        />
      </ModalForm>
    </>
  );
};

export default RoleAdminPage;
