import { PlusOutlined } from '@ant-design/icons';
import { ModalForm, ProFormSelect, ProFormText } from '@ant-design/pro-components';
import { ActionType, ProColumns, ProTable } from '@ant-design/pro-components';
import { App, Popconfirm, Tree } from 'antd';
import React, { useRef, useState } from 'react';
import * as api from '@/services/api';
import type { DepartmentItem } from '@/services/api/data';

const DepartmentManagement: React.FC = () => {
  const actionRef = useRef<ActionType>();
  const { message } = App.useApp();
  const [modalVisible, setModalVisible] = useState(false);
  const [editingRecord, setEditingRecord] = useState<DepartmentItem | null>(null);
  const [treeData, setTreeData] = useState<DepartmentItem[]>([]);

  const handleAdd = async (values: Record<string, unknown>) => {
    try {
      if (editingRecord) {
        await api.updateDepartment(editingRecord.id, values);
        message.success('更新成功');
      } else {
        await api.createDepartment(values);
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
      await api.deleteDepartment(id);
      message.success('删除成功');
      actionRef.current?.reload();
    } catch (error: any) {
      message.error(error?.response?.data?.message || '删除失败');
    }
  };

  const columns: ProColumns<DepartmentItem>[] = [
    {
      title: '部门名称',
      dataIndex: 'name',
      key: 'name',
    },
    {
      title: '部门编码',
      dataIndex: 'code',
      key: 'code',
    },
    {
      title: '负责人',
      dataIndex: 'leader',
      key: 'leader',
    },
    {
      title: '联系电话',
      dataIndex: 'phone',
      key: 'phone',
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
        <Popconfirm key="delete" title="确定删除此部门?" onConfirm={() => handleDelete(record.id)}>
          <a style={{ color: 'red' }}>删除</a>
        </Popconfirm>,
      ],
    },
  ];

  return (
    <>
      <ProTable<DepartmentItem>
        columns={columns}
        actionRef={actionRef}
        request={async () => {
          const response = await api.queryDepartments();
          setTreeData(response.data || []);
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
        title={editingRecord ? '编辑部门' : '新建部门'}
        open={modalVisible}
        onOpenChange={setModalVisible}
        onFinish={handleAdd}
        initialValues={editingRecord || { status: 'ACTIVE' }}
      >
        <ProFormText
          name="code"
          label="部门编码"
          disabled={!!editingRecord}
          rules={[{ required: true, message: '请输入部门编码' }]}
        />
        <ProFormText
          name="name"
          label="部门名称"
          rules={[{ required: true, message: '请输入部门名称' }]}
        />
        <ProFormText name="leader" label="负责人" />
        <ProFormText name="phone" label="联系电话" />
        <ProFormText name="email" label="邮箱" />
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

export default DepartmentManagement;
