import { ReloadOutlined } from '@ant-design/icons';
import { ProColumns, ProTable } from '@ant-design/pro-components';
import { ActionType } from '@ant-design/pro-components';
import { App, Popconfirm, Tag, Tooltip, Button } from 'antd';
import React, { useRef } from 'react';
import * as api from '@/services/api/monitoring';
import type { OnlineUser } from '@/services/api/monitoring';

const OnlineUserList: React.FC = () => {
  const actionRef = useRef<ActionType>();
  const { message } = App.useApp();

  const handleForceLogout = async (record: OnlineUser) => {
    try {
      await api.forceLogout(record.session_id, record.user_id, '管理员手动踢出');
      message.success('强制下线成功');
      actionRef.current?.reload();
    } catch (error: any) {
      message.error(error?.response?.data?.message || '强制下线失败');
    }
  };

  const columns: ProColumns<OnlineUser>[] = [
    {
      title: '用户名',
      dataIndex: 'username',
      key: 'username',
      width: 120,
    },
    {
      title: '部门',
      dataIndex: 'dept_name',
      key: 'dept_name',
      width: 120,
      hideInSearch: true,
    },
    {
      title: 'IP地址',
      dataIndex: 'ip_address',
      key: 'ip_address',
      width: 140,
    },
    {
      title: '用户代理',
      dataIndex: 'user_agent',
      key: 'user_agent',
      ellipsis: true,
      hideInSearch: true,
      hideInTable: true,
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 100,
      render: (_, record) => (
        <Tag color={record.status === 1 ? 'success' : 'default'}>
          {record.status === 1 ? '在线' : '已下线'}
        </Tag>
      ),
    },
    {
      title: '最后活动',
      dataIndex: 'last_activity_time',
      key: 'last_activity_time',
      width: 180,
      valueType: 'dateTime',
      hideInSearch: true,
    },
    {
      title: '过期时间',
      dataIndex: 'expire_time',
      key: 'expire_time',
      width: 180,
      valueType: 'dateTime',
      hideInSearch: true,
    },
    {
      title: '操作',
      valueType: 'option',
      width: 100,
      render: (_, record) => (
        <Tooltip title="强制下线">
          <Popconfirm
            title="确定强制下线此用户?"
            onConfirm={() => handleForceLogout(record)}
            okText="确定"
            cancelText="取消"
          >
            <Button danger size="small" icon={<ReloadOutlined />} />
          </Popconfirm>
        </Tooltip>
      ),
    },
  ];

  return (
    <ProTable<OnlineUser>
      columns={columns}
      actionRef={actionRef}
      request={async (params) => {
        const response = await api.fetchOnlineUsers();
        let data = response.list || [];
        if (params.username) {
          data = data.filter(u => u.username.includes(params.username));
        }
        return {
          data,
          total: response.total || 0,
          success: true,
        };
      }}
      rowKey="session_id"
      search={false}
      pagination={{ pageSize: 10 }}
      toolBarRender={() => []}
    />
  );
};

export default OnlineUserList;