import React, { useEffect, useState } from 'react';
import { Table, Button, Space, Input, Tag, Modal, Form, Select, Tree, message } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, ReloadOutlined, KeyOutlined } from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';
import { roleService, menuService, Role, Menu } from '@/services/systemService';

const RoleList: React.FC = () => {
  const [roles, setRoles] = useState<Role[]>([]);
  const [loading, setLoading] = useState(false);
  const [pagination, setPagination] = useState({ current: 1, pageSize: 10, total: 0 });
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [isMenuModalVisible, setIsMenuModalVisible] = useState(false);
  const [editingRole, setEditingRole] = useState<Role | null>(null);
  const [menuTree, setMenuTree] = useState<Menu[]>([]);
  const [selectedMenuIds, setSelectedMenuIds] = useState<string[]>([]);
  const [form] = Form.useForm();

  const fetchRoles = async () => {
    setLoading(true);
    try {
      const data = await roleService.list({ page: pagination.current, pageSize: pagination.pageSize });
      setRoles(data);
      setPagination({ ...pagination, total: data.length });
    } catch (error) {
      message.error('获取角色列表失败');
    } finally {
      setLoading(false);
    }
  };

  const fetchMenuTree = async () => {
    try {
      const data = await menuService.tree();
      setMenuTree(data);
    } catch (error) {
      message.error('获取菜单树失败');
    }
  };

  useEffect(() => {
    fetchRoles();
  }, [pagination.current, pagination.pageSize]);

  const handleAdd = () => {
    setEditingRole(null);
    form.resetFields();
    setIsModalVisible(true);
  };

  const handleEdit = (record: Role) => {
    setEditingRole(record);
    form.setFieldsValue(record);
    setIsModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    Modal.confirm({
      title: '确认删除',
      content: '确定要删除该角色吗？',
      onOk: async () => {
        try {
          await roleService.delete(id);
          message.success('删除成功');
          fetchRoles();
        } catch (error) {
          message.error('删除失败');
        }
      },
    });
  };

  const handleAssignMenus = async (record: Role) => {
    setEditingRole(record);
    setSelectedMenuIds(record.menu_ids || []);
    await fetchMenuTree();
    setIsMenuModalVisible(true);
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingRole) {
        await roleService.update(editingRole.id, values);
        message.success('更新成功');
      } else {
        await roleService.create(values);
        message.success('创建成功');
      }
      setIsModalVisible(false);
      fetchRoles();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const handleMenuSubmit = async () => {
    try {
      await roleService.assignMenus(editingRole!.id, selectedMenuIds);
      message.success('菜单分配成功');
      setIsMenuModalVisible(false);
      fetchRoles();
    } catch (error) {
      message.error('菜单分配失败');
    }
  };

  const columns: ColumnsType<Role> = [
    {
      title: '角色名称',
      dataIndex: 'role_name',
      key: 'role_name',
    },
    {
      title: '角色编码',
      dataIndex: 'role_code',
      key: 'role_code',
    },
    {
      title: '角色类型',
      dataIndex: 'role_type',
      key: 'role_type',
      render: (type: number) => (
        <Tag color={type === 1 ? 'blue' : 'default'}>
          {type === 1 ? '系统角色' : '自定义角色'}
        </Tag>
      ),
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
      title: '备注',
      dataIndex: 'remark',
      key: 'remark',
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record) => (
        <Space size="small">
          <Button type="link" icon={<EditOutlined />} onClick={() => handleEdit(record)}>
            编辑
          </Button>
          <Button type="link" icon={<KeyOutlined />} onClick={() => handleAssignMenus(record)}>
            分配菜单
          </Button>
          <Button type="link" danger icon={<DeleteOutlined />} onClick={() => handleDelete(record.id)}>
            删除
          </Button>
        </Space>
      ),
    },
  ];

  const menuColumns: ColumnsType<Menu> = [
    { title: '菜单名称', dataIndex: 'menu_name', key: 'menu_name' },
    { title: '菜单类型', dataIndex: 'menu_type', key: 'menu_type' },
    { title: '权限标识', dataIndex: 'permission', key: 'permission' },
  ];

  return (
    <div>
      <div style={{ marginBottom: 16 }}>
        <Space>
          <Input placeholder="搜索角色名称" style={{ width: 200 }} />
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
            新增角色
          </Button>
          <Button icon={<ReloadOutlined />} onClick={fetchRoles}>
            刷新
          </Button>
        </Space>
      </div>
      <Table
        columns={columns}
        dataSource={roles}
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
        title={editingRole ? '编辑角色' : '新增角色'}
        open={isModalVisible}
        onOk={handleSubmit}
        onCancel={() => setIsModalVisible(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item
            name="role_name"
            label="角色名称"
            rules={[{ required: true, message: '请输入角色名称' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item
            name="role_code"
            label="角色编码"
            rules={[{ required: true, message: '请输入角色编码' }]}
          >
            <Input disabled={!!editingRole} />
          </Form.Item>
          <Form.Item name="role_type" label="角色类型" initialValue={0}>
            <Select>
              <Select.Option value={0}>自定义角色</Select.Option>
              <Select.Option value={1}>系统角色</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="status" label="状态" initialValue={1}>
            <Select>
              <Select.Option value={1}>启用</Select.Option>
              <Select.Option value={0}>禁用</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="data_scope" label="数据范围" initialValue={1}>
            <Select>
              <Select.Option value={1}>全部数据</Select.Option>
              <Select.Option value={2}>本部门数据</Select.Option>
              <Select.Option value={3}>本部门及以下数据</Select.Option>
              <Select.Option value={4}>自定义数据</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="remark" label="备注">
            <Input.TextArea />
          </Form.Item>
        </Form>
      </Modal>
      <Modal
        title="分配菜单权限"
        open={isMenuModalVisible}
        onOk={handleMenuSubmit}
        onCancel={() => setIsMenuModalVisible(false)}
        width={800}
      >
        <p>为角色 "{editingRole?.role_name}" 分配菜单权限</p>
        <Table
          columns={menuColumns}
          dataSource={menuTree}
          rowKey="id"
          pagination={false}
          size="small"
        />
      </Modal>
    </div>
  );
};

export default RoleList;
