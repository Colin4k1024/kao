import React from 'react';
import { Card, Tag, Typography, Row, Col, Space } from 'antd';
import type { CardProps } from 'antd';

const { Title, Text } = Typography;

export type StatusType = 'healthy' | 'degraded' | 'unhealthy';

interface StatusCardProps extends CardProps {
  title: string;
  status: StatusType;
  value: string | number;
  details?: { label: string; value: string }[];
  description?: string;
  onClick?: () => void;
}

export const StatusCard: React.FC<StatusCardProps> = ({
  title,
  status,
  value,
  details = [],
  description,
  onClick,
  ...rest
}) => {
  const getStatusColor = () => {
    switch (status) {
      case 'healthy':
        return '#52c41a';
      case 'degraded':
        return '#faad14';
      case 'unhealthy':
        return '#f5222d';
      default:
        return '#8c8c8c';
    }
  };

  const getStatusText = () => {
    switch (status) {
      case 'healthy':
        return '正常';
      case 'degraded':
        return '降级';
      case 'unhealthy':
        return '异常';
      default:
        return '未知';
    }
  };

  return (
    <Card
      hoverable
      onClick={onClick}
      style={{
        borderRadius: '8px',
        boxShadow: '0 2px 8px rgba(0, 0, 0, 0.1)',
        cursor: onClick ? 'pointer' : 'default',
      }}
      {...rest}
    >
      <div style={{ marginBottom: 16 }}>
        <Title level={5} style={{ margin: 0 }}>
          {title}
        </Title>
        {description && (
          <Text type="secondary" style={{ fontSize: '12px' }}>
            {description}
          </Text>
        )}
      </div>
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
        <Space size="large">
          <div>
            <div style={{ fontSize: '24px', fontWeight: '600' }}>
              {value}
            </div>
            <div style={{ marginTop: '4px' }}>
              <Tag color={getStatusColor()}>{getStatusText()}</Tag>
            </div>
          </div>
        </Space>
      </div>
      {details.length > 0 && (
        <div style={{ marginTop: '12px', borderTop: '1px solid #f0f0f0', paddingTop: '12px' }}>
          {details.map((detail, index) => (
            <Row key={index} justify="space-between" style={{ marginBottom: '8px' }}>
              <Col span={10}>
                <Text type="secondary" style={{ fontSize: '12px' }}>
                  {detail.label}
                </Text>
              </Col>
              <Col span={14}>
                <Text style={{ fontSize: '12px', textAlign: 'right', width: '100%', display: 'block' }}>
                  {detail.value}
                </Text>
              </Col>
            </Row>
          ))}
        </div>
      )}
    </Card>
  );
};

export default StatusCard;
