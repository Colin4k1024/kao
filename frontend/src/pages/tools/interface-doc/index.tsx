import React, { useState } from 'react';
import {
  Card,
  Table,
  Tag,
  Space,
  Input,
  Button,
  Typography,
  Descriptions,
  Modal,
  Collapse,
  Tabs,
} from 'antd';
import {
  SearchOutlined,
  ApiOutlined,
  CopyOutlined,
  FieldTimeOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';

const { Title, Text } = Typography;
const { Panel } = Collapse;
const { Search } = Input;

interface ApiEndpoint {
  id: string;
  method: string;
  path: string;
  description: string;
  module: string;
  auth: boolean;
  parameters?: Parameter[];
  response?: string;
}

interface Parameter {
  name: string;
  type: string;
  required: boolean;
  description: string;
  location: 'path' | 'query' | 'body' | 'header';
}

const mockApiDocs: ApiEndpoint[] = [
  {
    id: '1',
    method: 'POST',
    path: '/api/auth/login',
    description: '用户登录接口',
    module: '认证模块',
    auth: false,
    parameters: [
      { name: 'username', type: 'string', required: true, description: '用户名', location: 'body' },
      { name: 'password', type: 'string', required: true, description: '密码', location: 'body' },
    ],
    response: `{
  "code": 200,
  "message": "success",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 3600
  }
}`,
  },
  {
    id: '2',
    method: 'GET',
    path: '/api/system/users',
    description: '获取用户列表',
    module: '系统管理',
    auth: true,
    parameters: [
      { name: 'page', type: 'number', required: false, description: '页码', location: 'query' },
      { name: 'page_size', type: 'number', required: false, description: '每页数量', location: 'query' },
      { name: 'keyword', type: 'string', required: false, description: '搜索关键词', location: 'query' },
    ],
    response: `{
  "code": 200,
  "message": "success",
  "data": {
    "list": [
      {
        "id": "uuid",
        "username": "admin",
        "email": "admin@example.com",
        "status": "ACTIVE"
      }
    ],
    "total": 100,
    "page": 1,
    "page_size": 10
  }
}`,
  },
  {
    id: '3',
    method: 'POST',
    path: '/api/system/users',
    description: '创建用户',
    module: '系统管理',
    auth: true,
    parameters: [
      { name: 'username', type: 'string', required: true, description: '用户名', location: 'body' },
      { name: 'password', type: 'string', required: true, description: '密码', location: 'body' },
      { name: 'email', type: 'string', required: true, description: '邮箱', location: 'body' },
      { name: 'dept_id', type: 'string', required: false, description: '部门ID', location: 'body' },
    ],
    response: `{
  "code": 200,
  "message": "success",
  "data": {
    "id": "uuid"
  }
}`,
  },
  {
    id: '4',
    method: 'PUT',
    path: '/api/system/users/:id',
    description: '更新用户信息',
    module: '系统管理',
    auth: true,
    parameters: [
      { name: 'id', type: 'string', required: true, description: '用户ID', location: 'path' },
      { name: 'email', type: 'string', required: false, description: '邮箱', location: 'body' },
      { name: 'status', type: 'string', required: false, description: '状态', location: 'body' },
    ],
    response: `{
  "code": 200,
  "message": "success"
}`,
  },
  {
    id: '5',
    method: 'DELETE',
    path: '/api/system/users/:id',
    description: '删除用户',
    module: '系统管理',
    auth: true,
    parameters: [
      { name: 'id', type: 'string', required: true, description: '用户ID', location: 'path' },
    ],
    response: `{
  "code": 200,
  "message": "success"
}`,
  },
  {
    id: '6',
    method: 'GET',
    path: '/api/system/departments',
    description: '获取部门列表（树形）',
    module: '系统管理',
    auth: true,
    parameters: [],
    response: `{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "uuid",
      "name": "技术部",
      "parent_id": null,
      "sort_order": 1,
      "children": []
    }
  ]
}`,
  },
  {
    id: '7',
    method: 'GET',
    path: '/api/system/roles',
    description: '获取角色列表',
    module: '系统管理',
    auth: true,
    parameters: [
      { name: 'page', type: 'number', required: false, description: '页码', location: 'query' },
      { name: 'page_size', type: 'number', required: false, description: '每页数量', location: 'query' },
    ],
    response: `{
  "code": 200,
  "message": "success",
  "data": {
    "list": [
      {
        "id": "uuid",
        "role_name": "管理员",
        "role_code": "ADMIN",
        "status": "ACTIVE"
      }
    ],
    "total": 10
  }
}`,
  },
  {
    id: '8',
    method: 'GET',
    path: '/api/system/menus',
    description: '获取菜单列表（树形）',
    module: '系统管理',
    auth: true,
    parameters: [],
    response: `{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "uuid",
      "name": "系统管理",
      "menu_type": "DIRECTORY",
      "route_path": "/system",
      "children": []
    }
  ]
}`,
  },
  {
    id: '9',
    method: 'GET',
    path: '/api/dictionary/types',
    description: '获取字典类型列表',
    module: '字典管理',
    auth: true,
    parameters: [
      { name: 'page', type: 'number', required: false, description: '页码', location: 'query' },
      { name: 'keyword', type: 'string', required: false, description: '搜索关键词', location: 'query' },
    ],
    response: `{
  "code": 200,
  "message": "success",
  "data": {
    "list": [
      {
        "id": "uuid",
        "dict_name": "用户状态",
        "dict_type": "user_status",
        "status": "ACTIVE"
      }
    ],
    "total": 20
  }
}`,
  },
  {
    id: '10',
    method: 'GET',
    path: '/api/monitoring/online/users',
    description: '获取在线用户列表',
    module: '系统监控',
    auth: true,
    parameters: [],
    response: `{
  "code": 200,
  "message": "success",
  "data": {
    "list": [
      {
        "session_id": "uuid",
        "user_id": "uuid",
        "username": "admin",
        "ip_address": "192.168.1.1",
        "last_activity_time": "2024-01-01 12:00:00"
      }
    ],
    "total": 5
  }
}`,
  },
  {
    id: '11',
    method: 'GET',
    path: '/api/monitoring/oper/logs',
    description: '获取操作日志列表',
    module: '系统监控',
    auth: true,
    parameters: [
      { name: 'page', type: 'number', required: false, description: '页码', location: 'query' },
      { name: 'username', type: 'string', required: false, description: '用户名', location: 'query' },
      { name: 'module', type: 'string', required: false, description: '模块', location: 'query' },
    ],
    response: `{
  "code": 200,
  "message": "success",
  "data": {
    "list": [
      {
        "id": "uuid",
        "username": "admin",
        "module": "用户管理",
        "action_type": "UPDATE",
        "path": "/api/system/users/123",
        "status": 1,
        "execution_time": 50,
        "create_time": "2024-01-01 12:00:00"
      }
    ],
    "total": 100
  }
}`,
  },
  {
    id: '12',
    method: 'GET',
    path: '/api/monitoring/login/logs',
    description: '获取登录日志列表',
    module: '系统监控',
    auth: true,
    parameters: [
      { name: 'page', type: 'number', required: false, description: '页码', location: 'query' },
      { name: 'username', type: 'string', required: false, description: '用户名', location: 'query' },
      { name: 'status', type: 'number', required: false, description: '状态(1成功,0失败)', location: 'query' },
    ],
    response: `{
  "code": 200,
  "message": "success",
  "data": {
    "list": [
      {
        "id": "uuid",
        "username": "admin",
        "ip_address": "192.168.1.1",
        "status": 1,
        "message": "登录成功",
        "login_time": "2024-01-01 12:00:00"
      }
    ],
    "total": 200
  }
}`,
  },
];

const getMethodColor = (method: string) => {
  const colors: Record<string, string> = {
    GET: 'green',
    POST: 'blue',
    PUT: 'orange',
    DELETE: 'red',
    PATCH: 'purple',
  };
  return colors[method] || 'default';
};

const InterfaceDoc: React.FC = () => {
  const [searchText, setSearchText] = useState('');
  const [selectedApi, setSelectedApi] = useState<ApiEndpoint | null>(null);
  const [detailModalVisible, setDetailModalVisible] = useState(false);

  const filteredApis = mockApiDocs.filter(
    (api) =>
      api.path.toLowerCase().includes(searchText.toLowerCase()) ||
      api.description.toLowerCase().includes(searchText.toLowerCase()) ||
      api.module.toLowerCase().includes(searchText.toLowerCase())
  );

  const handleViewDetail = (record: ApiEndpoint) => {
    setSelectedApi(record);
    setDetailModalVisible(true);
  };

  const columns: ColumnsType<ApiEndpoint> = [
    {
      title: '方法',
      dataIndex: 'method',
      key: 'method',
      width: 100,
      render: (method) => (
        <Tag color={getMethodColor(method)}>{method}</Tag>
      ),
    },
    {
      title: '路径',
      dataIndex: 'path',
      key: 'path',
      render: (path) => (
        <Text copyable={{ text: path }} style={{ fontFamily: 'monospace' }}>
          {path}
        </Text>
      ),
    },
    {
      title: '描述',
      dataIndex: 'description',
      key: 'description',
    },
    {
      title: '模块',
      dataIndex: 'module',
      key: 'module',
      width: 120,
      render: (module) => <Tag>{module}</Tag>,
    },
    {
      title: '认证',
      dataIndex: 'auth',
      key: 'auth',
      width: 80,
      render: (auth) => (auth ? <Tag color="blue">需要</Tag> : <Tag color="green">无需</Tag>),
    },
    {
      title: '操作',
      key: 'action',
      width: 100,
      render: (_, record) => (
        <Button type="link" onClick={() => handleViewDetail(record)}>
          查看详情
        </Button>
      ),
    },
  ];

  const modules = [...new Set(mockApiDocs.map((api) => api.module))];

  return (
    <div style={{ padding: 24 }}>
      <div style={{ marginBottom: 16 }}>
        <Space direction="vertical" size="large" style={{ width: '100%' }}>
          <div>
            <Title level={4} style={{ marginBottom: 16 }}>
              <ApiOutlined /> 接口文档
            </Title>
            <Text type="secondary">
              基于 OpenAPI 规范的 RESTful API 接口文档，包含所有模块的接口定义、参数说明和返回值结构。
            </Text>
          </div>
          <Search
            placeholder="搜索接口路径、描述或模块..."
            allowClear
            enterButton={<Button type="primary" icon={<SearchOutlined />}>搜索</Button>}
            onChange={(e) => setSearchText(e.target.value)}
            style={{ maxWidth: 400 }}
          />
        </Space>
      </div>

      <Tabs
        defaultActiveKey="all"
        items={[
          { key: 'all', label: `全部 (${mockApiDocs.length})` },
          ...modules.map((m) => ({
            key: m,
            label: `${m} (${mockApiDocs.filter((api) => api.module === m).length})`,
          })),
        ]}
      />

      <Table
        columns={columns}
        dataSource={filteredApis}
        rowKey="id"
        pagination={{
          pageSize: 10,
          showSizeChanger: true,
          showTotal: (total) => `共 ${total} 条`,
        }}
      />

      <Modal
        title={
          <Space>
            {selectedApi && <Tag color={getMethodColor(selectedApi.method)}>{selectedApi.method}</Tag>}
            <span>{selectedApi?.path}</span>
          </Space>
        }
        open={detailModalVisible}
        onCancel={() => setDetailModalVisible(false)}
        width={800}
        footer={null}
      >
        {selectedApi && (
          <div>
            <Descriptions column={2} bordered size="small" style={{ marginBottom: 16 }}>
              <Descriptions.Item label="接口名称">{selectedApi.description}</Descriptions.Item>
              <Descriptions.Item label="所属模块">{selectedApi.module}</Descriptions.Item>
              <Descriptions.Item label="认证方式">
                {selectedApi.auth ? '需要Token' : '无需认证'}
              </Descriptions.Item>
              <Descriptions.Item label="接口ID">{selectedApi.id}</Descriptions.Item>
            </Descriptions>

            <Collapse defaultActiveKey={['params', 'response']}>
              <Panel header="请求参数" key="params">
                {selectedApi.parameters && selectedApi.parameters.length > 0 ? (
                  <table style={{ width: '100%', borderCollapse: 'collapse' }}>
                    <thead>
                      <tr style={{ background: '#f5f5f5' }}>
                        <th style={{ padding: 8, border: '1px solid #ddd', textAlign: 'left' }}>参数名</th>
                        <th style={{ padding: 8, border: '1px solid #ddd', textAlign: 'left' }}>位置</th>
                        <th style={{ padding: 8, border: '1px solid #ddd', textAlign: 'left' }}>类型</th>
                        <th style={{ padding: 8, border: '1px solid #ddd', textAlign: 'left' }}>必填</th>
                        <th style={{ padding: 8, border: '1px solid #ddd', textAlign: 'left' }}>说明</th>
                      </tr>
                    </thead>
                    <tbody>
                      {selectedApi.parameters.map((param, idx) => (
                        <tr key={idx}>
                          <td style={{ padding: 8, border: '1px solid #ddd', fontFamily: 'monospace' }}>
                            {param.name}
                          </td>
                          <td style={{ padding: 8, border: '1px solid #ddd' }}>
                            <Tag>{param.location}</Tag>
                          </td>
                          <td style={{ padding: 8, border: '1px solid #ddd', fontFamily: 'monospace' }}>
                            {param.type}
                          </td>
                          <td style={{ padding: 8, border: '1px solid #ddd' }}>
                            {param.required ? <Tag color="red">是</Tag> : <Tag>否</Tag>}
                          </td>
                          <td style={{ padding: 8, border: '1px solid #ddd' }}>{param.description}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                ) : (
                  <Text type="secondary">此接口无需参数</Text>
                )}
              </Panel>
              <Panel header="响应示例" key="response">
                <div style={{ background: '#f5f5f5', padding: 16, borderRadius: 4 }}>
                  <pre style={{ margin: 0, whiteSpace: 'pre-wrap', wordBreak: 'break-all' }}>
                    {selectedApi.response}
                  </pre>
                </div>
              </Panel>
            </Collapse>
          </div>
        )}
      </Modal>
    </div>
  );
};

export default InterfaceDoc;
