import React from 'react';
import { Spin, theme, ConfigProvider, Flex } from 'antd';
import { LoadingOutlined } from '@ant-design/icons';
import useResponsive from '@/hooks/useResponsive';

// Loading component with various sizes
export interface LoadingProps {
  spinning?: boolean;
  tip?: string;
  size?: 'small' | 'default' | 'large';
  fullscreen?: boolean;
  children?: React.ReactNode;
  delay?: number;
}

export const Loading: React.FC<LoadingProps> = ({
  spinning = true,
  tip = 'Loading...',
  size = 'default',
  fullscreen = false,
  children,
  delay = 0,
}) => {
  const { token } = theme.useToken();
  const { isMobile } = useResponsive();

  const antIcon = (
    <LoadingOutlined style={{ fontSize: size === 'small' ? 14 : size === 'large' ? 32 : 24 }} spin />
  );

  if (fullscreen) {
    return (
      <ConfigProvider theme={{ token: { fontFamily: token.fontFamily } }}>
        <Spin
          spinning={spinning}
          tip={tip}
          size={size}
          delay={delay}
          style={{
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            minHeight: '100vh',
          }}
        />
      </ConfigProvider>
    );
  }

  if (spinning) {
    return (
      <Spin
        spinning={spinning}
        tip={tip}
        size={size}
        indicator={antIcon}
        style={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          minHeight: isMobile ? '200px' : '400px',
        }}
      >
        {children}
      </Spin>
    );
  }

  return <>{children}</>;
};

// Full page loading component
export const FullPageLoading: React.FC<{
  tip?: string;
  size?: 'small' | 'default' | 'large';
}> = ({ tip = 'Loading...', size = 'default' }) => {
  const { isMobile } = useResponsive();

  return (
    <Flex
      vertical
      align="center"
      justify="center"
      style={{
        minHeight: '100vh',
        backgroundColor: '#f0f2f5',
      }}
    >
      <Spin
        size={size}
        tip={tip}
        style={{ fontSize: isMobile ? '12px' : '16px' }}
      />
    </Flex>
  );
};

// Inline loading component
export const InlineLoading: React.FC<{
  spinning?: boolean;
  tip?: string;
  size?: 'small' | 'default' | 'large';
  delay?: number;
}> = ({ spinning = true, tip = '', size = 'small', delay = 0 }) => {
  return (
    <Spin
      spinning={spinning}
      tip={tip}
      size={size}
      delay={delay}
      style={{ display: 'inline-block' }}
    />
  );
};

// Suspense fallback
export const SuspenseFallback: React.FC<{
  tip?: string;
  size?: 'small' | 'default' | 'large';
}> = ({ tip = 'Loading...', size = 'default' }) => {
  return (
    <FullPageLoading tip={tip} size={size} />
  );
};

export default Loading;
