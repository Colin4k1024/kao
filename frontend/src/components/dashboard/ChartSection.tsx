import React from 'react';
import { Card } from 'antd';
import type { CardProps } from 'antd';

export type ChartType = 'line' | 'bar' | 'progress';

interface ChartSectionProps extends CardProps {
  title: string;
  type?: ChartType;
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
        <h3 style={{ margin: 0, fontSize: '16px' }}>
          {title}
        </h3>
        {description && (
          <div style={{ fontSize: '12px', color: '#8c8c8c' }}>
            {description}
          </div>
        )}
      </div>
      {type === 'line' && (
        <div style={{ textAlign: 'center', padding: '40px 0' }}>
          <div style={{ fontSize: '24px', color: color, fontWeight: '600' }}>
            {data.length > 0 ? data.map((d) => d[yField]).reduce((a, b) => a + b, 0) : 0}
          </div>
          <div style={{ marginTop: '8px', fontSize: '12px', color: '#8c8c8c' }}>
            Line Chart
          </div>
        </div>
      )}
      {type === 'bar' && (
        <div style={{ textAlign: 'center', padding: '40px 0' }}>
          <div style={{ fontSize: '24px', color: color, fontWeight: '600' }}>
            {data.length > 0 ? data.map((d) => d[yField]).reduce((a, b) => a + b, 0) : 0}
          </div>
          <div style={{ marginTop: '8px', fontSize: '12px', color: '#8c8c8c' }}>
            Bar Chart
          </div>
        </div>
      )}
      {type === 'progress' && (
        <div style={{ textAlign: 'center', padding: '40px 0' }}>
          <div
            style={{
              fontSize: '24px',
              fontWeight: '600',
              color,
            }}
          >
            {progressValue}%
          </div>
          <div style={{ marginTop: '8px', fontSize: '12px', color: '#8c8c8c' }}>
            Progress
          </div>
        </div>
      )}
    </Card>
  );
};

export default ChartSection;
