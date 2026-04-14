import { ReloadOutlined, EyeOutlined, DeleteOutlined } from '@ant-design/icons';
import { ProColumns, ProTable } from '@ant-design/pro-components';
import { ActionType } from '@ant-design/pro-components';
import { App, Popconfirm } from 'antd';
import React, { useRef } from 'react';
import * as api from '@/services/api/monitoring';
import type { OperationLog } from '@/services/api/monitoring';

const OperationLogList: React.FC = () => {
  const actionRef = useRef<ActionType>();
  const { message } = App.useApp();

  const handleDelete = async (id: string) => {
    try {
      await api.deleteOperationLog(id);
      message.success('删除成功');
      actionRef.current?.reload();
    } catch (error: any) {
      message.error(error?.response?.data?.message || '删除失败');
    }
  };

  const columns: ProColumns<OperationLog>[] = [
    {
      title: '用户名',
      dataIndex: 'username',
      key: 'username',
      width: 120,
    },
    {
      title: '模块',
      dataIndex: 'module',
      key: 'module',
      width: 120,
    },
    {
      title: '操作类型',
      dataIndex: 'action_type',
      key: 'action_type',
      width: 100,
    },
    {
      title: '请求方法',
      dataIndex: 'request_method',
      key: 'request_method',
      width: 80,
    },
    {
      title: '请求路径',
      dataIndex: 'path',
      key: 'path',
      ellipsis: true,
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
      title: '执行时间',
      dataIndex: 'execution_time',
      key: 'execution_time',
      width: 100,
      render: (_: any, record: OperationLog) => `${record.execution_time}ms`,
    },
    {
      title: 'IP地址',
      dataIndex: 'ip_address',
      key: 'ip_address',
      width: 140,
    },
    {
      title: '操作时间',
      dataIndex: 'create_time',
      key: 'create_time',
      width: 180,
      valueType: 'dateTime',
    },
    {
      title: '操作',
      valueType: 'option',
      width: 120,
      render: (_, record) => [
        <Popconfirm
          key="delete"
          title="确定删除此日志?"
          onConfirm={() => handleDelete(record.id)}
        >
          <a style={{ color: 'red' }}>删除</a>
        </Popconfirm>,
      ],
    },
  ];

  return (
    <ProTable<OperationLog>
      columns={columns}
      actionRef={actionRef}
      request={async (params) => {
        const response = await api.fetchOperationLogs({
          page: params.current,
          page_size: params.pageSize,
          username: params.username,
          module: params.module,
          action_type: params.action_type,
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

export default OperationLogList;