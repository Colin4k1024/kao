import React from 'react';
import { Card, Row, Col, Typography, Space } from 'antd';
import {
  ApiOutlined,
  DatabaseOutlined,
  HddOutlined,
  FileOutlined,
  RightOutlined,
} from '@ant-design/icons';
import { useNavigate } from 'react-router-dom';

const { Title, Text } = Typography;

interface ToolCardProps {
  title: string;
  description: string;
  icon: React.ReactNode;
  path: string;
}

const ToolCard: React.FC<ToolCardProps> = ({ title, description, icon, path }) => {
  const navigate = useNavigate();

  return (
    <Card
      hoverable
      onClick={() => navigate(path)}
      style={{ height: '100%', cursor: 'pointer' }}
      styles={{
        body: { padding: 24 }
      }}
    >
      <Space direction="vertical" size="large" style={{ width: '100%' }}>
        <div style={{ fontSize: 48, color: '#1890ff', textAlign: 'center' }}>
          {icon}
        </div>
        <div>
          <Title level={5} style={{ marginBottom: 8 }}>{title}</Title>
          <Text type="secondary">{description}</Text>
        </div>
        <div style={{ textAlign: 'right', color: '#1890ff' }}>
          <RightOutlined />
        </div>
      </Space>
    </Card>
  );
};

const ToolsIndex: React.FC = () => {
  const tools = [
    {
      title: '接口文档',
      description: '查看和管理API接口文档，包含接口列表、参数说明、返回值结构等',
      icon: <ApiOutlined />,
      path: '/tools/interface-doc',
    },
    {
      title: '数据库管理',
      description: '数据库连接管理、表结构查看、数据预览等功能',
      icon: <DatabaseOutlined />,
      path: '/tools/database',
    },
    {
      title: '缓存管理',
      description: '系统缓存监控、Redis状态查看、缓存清理等功能',
      icon: <HddOutlined />,
      path: '/tools/cache',
    },
    {
      title: '文件管理',
      description: '系统文件浏览、文件上传、下载、预览等功能',
      icon: <FileOutlined />,
      path: '/tools/file',
    },
  ];

  return (
    <div style={{ padding: 24 }}>
      <Title level={4} style={{ marginBottom: 24 }}>系统工具</Title>
      <Row gutter={[16, 16]}>
        {tools.map((tool) => (
          <Col xs={24} sm={12} md={8} lg={6} key={tool.path}>
            <ToolCard {...tool} />
          </Col>
        ))}
      </Row>
    </div>
  );
};

export default ToolsIndex;
