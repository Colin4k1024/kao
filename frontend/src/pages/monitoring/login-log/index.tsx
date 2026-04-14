import { ProColumns, ProTable } from '@ant-design/pro-components';
import { ActionType } from '@ant-design/pro-components';
import { App } from 'antd';
import React, { useRef } from 'react';
import * as api from '@/services/api/monitoring';
import type { LoginLog } from '@/services/api/monitoring';

const LoginLogList: React.FC = () => {
  const actionRef = useRef<ActionType>();
  const { message } = App.useApp();

  const columns: ProColumns<LoginLog>[] = [
    {
      title: '用户名',
      dataIndex: 'username',
      key: 'username',
      width: 120,
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
      hideInTable: true,
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      width: 80,
      render: (_, record) => (
        <span style={{ color: record.status === 1 ? '#52c41a' : '#ff4d4f' }}>
          {record.status === 1 ? '成功' : '失败'}
        </span>
      ),
    },
    {
      title: '消息',
      dataIndex: 'message',
      key: 'message',
      ellipsis: true,
    },
    {
      title: '登录时间',
      dataIndex: 'login_time',
      key: 'login_time',
      width: 180,
      valueType: 'dateTime',
    },
  ];

  return (
    <ProTable<LoginLog>
      columns={columns}
      actionRef={actionRef}
      request={async (params) => {
        const response = await api.fetchLoginLogs({
          page: params.current,
          page_size: params.pageSize,
          username: params.username,
        });
        return {
          data: response.list || [],
          total: response.total || 0,
          success: true,
        };
      }}
      rowKey="id"
      search={false}
      pagination={{ pageSize: 10 }}
      toolBarRender={() => []}
    />
  );
};

export default LoginLogList;