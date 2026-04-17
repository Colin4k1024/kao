import React from 'react';
import { Card, Typography, Space, Alert, Table, Tag, Button, Upload, Progress, Row, Col, Statistic } from 'antd';
import {
  FileOutlined,
  FolderOutlined,
  UploadOutlined,
  DownloadOutlined,
  DeleteOutlined,
  EyeOutlined,
  ReloadOutlined,
  PictureOutlined,
  FilePdfOutlined,
  FileWordOutlined,
  FileExcelOutlined,
} from '@ant-design/icons';
import type { ColumnsType } from 'antd/es/table';

const { Title, Text } = Typography;

interface FileItem {
  id: string;
  name: string;
  type: 'file' | 'folder';
  size?: string;
  mime_type?: string;
  created_at: string;
  updated_at: string;
  uploader: string;
}

const mockFiles: FileItem[] = [
  {
    id: '1',
    name: 'documents',
    type: 'folder',
    created_at: '2024-01-01 10:00:00',
    updated_at: '2024-01-15 14:30:00',
    uploader: 'admin',
  },
  {
    id: '2',
    name: '头像.jpg',
    type: 'file',
    size: '256KB',
    mime_type: 'image/jpeg',
    created_at: '2024-01-05 09:00:00',
    updated_at: '2024-01-05 09:00:00',
    uploader: 'admin',
  },
  {
    id: '3',
    name: '报告.pdf',
    type: 'file',
    size: '1.5MB',
    mime_type: 'application/pdf',
    created_at: '2024-01-10 11:00:00',
    updated_at: '2024-01-10 11:00:00',
    uploader: 'testuser',
  },
  {
    id: '4',
    name: '数据导入模板.xlsx',
    type: 'file',
    size: '50KB',
    mime_type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    created_at: '2024-01-12 15:00:00',
    updated_at: '2024-01-12 15:00:00',
    uploader: 'admin',
  },
  {
    id: '5',
    name: '需求文档.docx',
    type: 'file',
    size: '200KB',
    mime_type: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    created_at: '2024-01-14 10:00:00',
    updated_at: '2024-01-14 10:00:00',
    uploader: 'manager',
  },
];

const getFileIcon = (mimeType?: string) => {
  if (!mimeType) return <FileOutlined />;
  if (mimeType.startsWith('image/')) return <PictureOutlined />;
  if (mimeType.includes('pdf')) return <FilePdfOutlined />;
  if (mimeType.includes('word') || mimeType.includes('document')) return <FileWordOutlined />;
  if (mimeType.includes('excel') || mimeType.includes('spreadsheet')) return <FileExcelOutlined />;
  return <FileOutlined />;
};

const FileManagement: React.FC = () => {
  const columns: ColumnsType<FileItem> = [
    {
      title: '名称',
      dataIndex: 'name',
      key: 'name',
      render: (name, record) => (
        <Space>
          {record.type === 'folder' ? <FolderOutlined style={{ color: '#1890ff' }} /> : getFileIcon(record.mime_type)}
          <span>{name}</span>
        </Space>
      ),
    },
    {
      title: '类型',
      dataIndex: 'type',
      key: 'type',
      render: (type) => (
        <Tag color={type === 'folder' ? 'blue' : 'default'}>
          {type === 'folder' ? '文件夹' : '文件'}
        </Tag>
      ),
    },
    {
      title: '大小',
      dataIndex: 'size',
      key: 'size',
      render: (size) => size || '-',
    },
    {
      title: '上传者',
      dataIndex: 'uploader',
      key: 'uploader',
    },
    {
      title: '更新时间',
      dataIndex: 'updated_at',
      key: 'updated_at',
    },
    {
      title: '操作',
      key: 'action',
      render: () => (
        <Space>
          <Button type="link" size="small" icon={<EyeOutlined />}>
            预览
          </Button>
          <Button type="link" size="small" icon={<DownloadOutlined />}>
            下载
          </Button>
          <Button type="link" danger size="small" icon={<DeleteOutlined />}>
            删除
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ padding: 24 }}>
      <Space direction="vertical" size="large" style={{ width: '100%' }}>
        <div>
          <Title level={4}>
            <FileOutlined /> 文件管理
          </Title>
          <Text type="secondary">
            系统文件浏览、文件上传、下载、预览等功能。
          </Text>
        </div>

        <Alert
          message="功能提示"
          description="文件管理功能支持上传、下载、预览和删除文件。请注意上传文件大小限制为10MB。"
          type="info"
          showIcon
        />

        <Row gutter={16}>
          <Col span={8}>
            <Card>
              <Statistic title="存储使用" value={256} suffix="MB / 1GB" />
              <Progress percent={25.6} size="small" status="active" />
            </Card>
          </Col>
          <Col span={8}>
            <Card>
              <Statistic title="文件数量" value={156} />
            </Card>
          </Col>
          <Col span={8}>
            <Card>
              <Statistic title="文件夹数量" value={12} />
            </Card>
          </Col>
        </Row>

        <Card
          title="文件列表"
          extra={
            <Space>
              <Button icon={<ReloadOutlined />}>刷新</Button>
              <Upload showUploadList={false}>
                <Button type="primary" icon={<UploadOutlined />}>
                  上传文件
                </Button>
              </Upload>
            </Space>
          }
        >
          <Table
            columns={columns}
            dataSource={mockFiles}
            rowKey="id"
            pagination={{
              pageSize: 10,
              showSizeChanger: true,
              showTotal: (total) => `共 ${total} 个文件/文件夹`,
            }}
          />
        </Card>
      </Space>
    </div>
  );
};

export default FileManagement;
