import React from 'react';
import { Card, Typography, Row, Col } from 'antd';
import { Line, Bar, Progress } from '@ant-design/plots';
import type { CardProps } from 'antd';

const { Title, Text } = Typography;

interface ChartSectionProps extends CardProps {
  title: string;
  type?: 'line' | 'bar' | 'progress';
  data?: any[];
  xField?: string;
  yField?: string;
  color?: string;
  progressValue?: number;
  description?: string;
}

export const ChartSection: React.FC<ChartSectionProps> = ({
  title,
  type = 'line',
  data = [],
  xField = 'name',
  yField = 'value',
  color = '#1890ff',
  progressValue = 0,
  description,
  ...rest
}) => {
  const config = {
    data,
    xField,
    yField,
    color,
    animation: {
      appear: {
        animation: 'position-update',
        duration: 500,
      },
    },
  };

  return (
    <Card
      hoverable
      style={{
        borderRadius: '8px',
        boxShadow: '0 2px 8px rgba(0, 0, 0, 0.1)',
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
      {type === 'line' && (
        <Line {...config} />
      )}
      {type === 'bar' && (
        <Bar {...config} />
      )}
      {type === 'progress' && (
        <Row justify="center" align="middle" style={{ height: '100%' }}>
          <Col>
            <Progress
              type="circle"
              percent={progressValue}
              format={(percent) => `${percent}%`}
              width={120}
              strokeWidth={8}
              strokeColor={color}
            />
          </Col>
        </Row>
      )}
    </Card>
  );
};

export default ChartSection;
