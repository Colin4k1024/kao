import React from 'react';
import { Empty, Button, Space } from 'antd';

export interface EmptyStateProps {
  title?: string;
  description?: string;
  button?: {
    text: string;
    onClick: () => void;
    type?: 'default' | 'primary' | 'dashed';
  };
  image?: 'simple' | 'predefined' | React.ReactNode;
  style?: React.CSSProperties;
}

export const EmptyState: React.FC<EmptyStateProps> = ({
  title = '暂无数据',
  description = '当前페이지에 데이터가 없습니다.',
  button,
  image = 'simple',
  style,
}) => {
  return (
    <div
      style={{
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        padding: '48px 24px',
        backgroundColor: '#fafafa',
        borderRadius: 8,
        ...style,
      }}
    >
      <Empty
        image={image === 'simple' ? Empty.PRESENTED_IMAGE_SIMPLE : image === 'predefined' ? Empty.PRESENTED_IMAGE_DEFAULT : image}
        description={
          <Space direction="vertical">
            <span>{title}</span>
            <span style={{ color: '#8c8c8c' }}>{description}</span>
          </Space>
        }
      />
      {button && (
        <Button
          type={button.type || 'primary'}
          style={{ marginTop: 16 }}
          onClick={button.onClick}
        >
          {button.text}
        </Button>
      )}
    </div>
  );
};

export default EmptyState;
