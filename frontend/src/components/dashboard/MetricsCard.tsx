import React from 'react';
import { Card } from 'antd';
import type { CardProps } from 'antd';

interface MetricsCardProps extends CardProps {
  title: string;
  value: number | string;
  status?: 'success' | 'warning' | 'error' | 'default';
  trend?: 'up' | 'down' | 'neutral';
  description?: string;
}

export const MetricsCard: React.FC<MetricsCardProps> = ({
  title,
  value,
  status = 'default',
  trend,
  description,
  ...rest
}) => {
  const getStatusColor = () => {
    switch (status) {
      case 'success':
        return '#52c41a';
      case 'warning':
        return '#faad14';
      case 'error':
        return '#f5222d';
      default:
        return '#1890ff';
    }
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
        <div
          style={{
            fontSize: '16px',
            fontWeight: '500',
            color: status === 'default' ? '#1890ff' : getStatusColor(),
          }}
        >
          {title}
        </div>
      </div>
      <div style={{ display: 'flex', alignItems: 'baseline', gap: '8px' }}>
        <div
          style={{
            fontSize: '24px',
            fontWeight: '600',
            color: getStatusColor(),
          }}
        >
          {value}
        </div>
      </div>
      {trend && (
        <div style={{ marginTop: '8px' }}>
          <div
            style={{
              color: trend === 'up' ? '#52c41a' : trend === 'down' ? '#f5222d' : '#8c8c8c',
              fontSize: '12px',
            }}
          >
            {trend === 'up' && '↑'}
            {trend === 'down' && '↓'}
            {trend === 'neutral' && '—'}
          </div>
        </div>
      )}
      {description && (
        <div style={{ marginTop: '8px', fontSize: '12px', color: '#8c8c8c' }}>
          {description}
        </div>
      )}
    </Card>
  );
};

export default MetricsCard;
