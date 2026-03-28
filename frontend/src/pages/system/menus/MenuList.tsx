import React, { useEffect, useState } from 'react';
import { Tree, Button, Space, Card, Modal, Form, Input, Select, message } from 'antd';
import { PlusOutlined, EditOutlined, DeleteOutlined, ReloadOutlined } from '@ant-design/icons';
import { menuService, Menu } from '@/services/systemService';

const MenuList: React.FC = () => {
  const [menus, setMenus] = useState<Menu[]>([]);
  const [loading, setLoading] = useState(false);
  const [isModalVisible, setIsModalVisible] = useState(false);
  const [editingMenu, setEditingMenu] = useState<Menu | null>(null);
  const [form] = Form.useForm();

  const fetchMenus = async () => {
    setLoading(true);
    try {
      const data = await menuService.tree();
      setMenus(data);
    } catch (error) {
      message.error('获取菜单列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchMenus();
  }, []);

  const handleAdd = () => {
    setEditingMenu(null);
    form.resetFields();
    setIsModalVisible(true);
  };

  const handleEdit = (record: Menu) => {
    setEditingMenu(record);
    form.setFieldsValue(record);
    setIsModalVisible(true);
  };

  const handleDelete = async (id: string) => {
    Modal.confirm({
      title: '确认删除',
      content: '确定要删除该菜单吗？',
      onOk: async () => {
        try {
          await menuService.delete(id);
          message.success('删除成功');
          fetchMenus();
        } catch (error) {
          message.error('删除失败');
        }
      },
    });
  };

  const handleSubmit = async () => {
    try {
      const values = await form.validateFields();
      if (editingMenu) {
        await menuService.update(editingMenu.id, values);
        message.success('更新成功');
      } else {
        await menuService.create(values);
        message.success('创建成功');
      }
      setIsModalVisible(false);
      fetchMenus();
    } catch (error) {
      message.error('操作失败');
    }
  };

  const renderTreeNodes = (data: Menu[]): any[] => {
    return data.map((item) => ({
      key: item.id,
      title: (
        <Space>
          {item.icon && <span>{item.icon}</span>}
          <span>{item.menu_name}</span>
          <span style={{ color: '#999', fontSize: 12 }}>
            [{item.menu_type === 'M' ? '目录' : item.menu_type === 'C' ? '菜单' : '按钮'}]
          </span>
          <Button
            type="link"
            size="small"
            icon={<EditOutlined />}
            onClick={(e) => {
              e.stopPropagation();
              handleEdit(item);
            }}
          >
            编辑
          </Button>
          <Button
            type="link"
            size="small"
            danger
            icon={<DeleteOutlined />}
            onClick={(e) => {
              e.stopPropagation();
              handleDelete(item.id);
            }}
          >
            删除
          </Button>
        </Space>
      ),
      children: item.children ? renderTreeNodes(item.children) : undefined,
    }));
  };

  return (
    <div>
      <div style={{ marginBottom: 16 }}>
        <Space>
          <Button type="primary" icon={<PlusOutlined />} onClick={handleAdd}>
            新增菜单
          </Button>
          <Button icon={<ReloadOutlined />} onClick={fetchMenus}>
            刷新
          </Button>
        </Space>
      </div>
      <Card>
        <Tree showLine defaultExpandAll treeData={loading ? [] : renderTreeNodes(menus)} />
      </Card>
      <Modal
        title={editingMenu ? '编辑菜单' : '新增菜单'}
        open={isModalVisible}
        onOk={handleSubmit}
        onCancel={() => setIsModalVisible(false)}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="parent_id" label="父级菜单">
            <Input placeholder="留空表示顶级菜单" />
          </Form.Item>
          <Form.Item
            name="menu_name"
            label="菜单名称"
            rules={[{ required: true, message: '请输入菜单名称' }]}
          >
            <Input />
          </Form.Item>
          <Form.Item
            name="menu_type"
            label="菜单类型"
            rules={[{ required: true, message: '请选择菜单类型' }]}
          >
            <Select>
              <Select.Option value="M">目录</Select.Option>
              <Select.Option value="C">菜单</Select.Option>
              <Select.Option value="F">按钮</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="icon" label="图标">
            <Input placeholder="请输入图标名称" />
          </Form.Item>
          <Form.Item name="route_path" label="路由路径">
            <Input />
          </Form.Item>
          <Form.Item name="component" label="组件路径">
            <Input />
          </Form.Item>
          <Form.Item name="permission" label="权限标识">
            <Input placeholder="如: system:user:add" />
          </Form.Item>
          <Form.Item name="display_order" label="显示顺序" initialValue={0}>
            <Input type="number" />
          </Form.Item>
          <Form.Item name="is_visible" label="是否显示" initialValue="1">
            <Select>
              <Select.Option value="1">显示</Select.Option>
              <Select.Option value="0">隐藏</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="is_cache" label="是否缓存" initialValue="0">
            <Select>
              <Select.Option value="0">不缓存</Select.Option>
              <Select.Option value="1">缓存</Select.Option>
            </Select>
          </Form.Item>
          <Form.Item name="status" label="状态" initialValue={1}>
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

export default MenuList;
