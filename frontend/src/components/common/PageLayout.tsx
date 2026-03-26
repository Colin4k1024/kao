import React from 'react';
import { Spin, Card, Typography } from 'antd';
import ErrorBoundary from './ErrorBoundary';
import EmptyState from './EmptyState';
import useResponsive from '@/hooks/useResponsive';

const { Header, Content, Footer } = Card;

export interface PageLayoutProps {
  title?: React.ReactNode;
  description?: React.ReactNode;
  extra?: React.ReactNode;
  actions?: React.ReactNode;
  loading?: boolean;
  error?: Error | null;
  isEmpty?: boolean;
  onEmptyAction?: () => void;
  emptyTitle?: string;
  emptyDescription?: string;
  children?: React.ReactNode;
  headerStyle?: React.CSSProperties;
  contentStyle?: React.CSSProperties;
  footer?: React.ReactNode;
}

export const PageLayout: React.FC<PageLayoutProps> = ({
  title,
  description,
  extra,
  actions,
  loading,
  error,
  isEmpty,
  onEmptyAction,
  emptyTitle = '暂无数据',
  emptyDescription = '当前页面没有数据',
  children,
  headerStyle,
  contentStyle,
  footer,
}) => {
  const { isMobile } = useResponsive();

  // Show loading state
  if (loading) {
    return (
      <div
        style={{
          padding: isMobile ? '12px' : '24px',
          minHeight: '100vh',
          backgroundColor: '#f0f2f5',
        }}
      >
        <Spin
          size="large"
          tip="Loading..."
          style={{
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            height: '100%',
          }}
        />
      </div>
    );
  }

  // Show error state using ErrorBoundary
  if (error) {
    return (
      <ErrorBoundary
        fallback={
          <div
            style={{
              padding: isMobile ? '12px' : '24px',
              minHeight: '100vh',
              backgroundColor: '#f0f2f5',
            }}
          >
            <div style={{ maxWidth: 800, margin: '0 auto' }}>
              <h2 style={{ color: '#ff4d4f' }}>发生错误</h2>
              <p>{error.message}</p>
            </div>
          </div>
        }
      >
        <div style={{ display: 'none' }} />
      </ErrorBoundary>
    );
  }

  // Show empty state
  if (isEmpty) {
    return (
      <div
        style={{
          padding: isMobile ? '12px' : '24px',
          minHeight: '100vh',
          backgroundColor: '#f0f2f5',
        }}
      >
        <EmptyState
          title={emptyTitle}
          description={emptyDescription}
          button={
            onEmptyAction
              ? {
                  text: '操作按钮',
                  onClick: onEmptyAction,
                }
              : undefined
          }
        />
      </div>
    );
  }

  // Normal layout
  return (
    <div
      style={{
        padding: isMobile ? '12px' : '24px',
        minHeight: '100vh',
        backgroundColor: '#f0f2f5',
      }}
    >
      <Card
        style={{
          borderRadius: 8,
          boxShadow: '0 2px 8px rgba(0, 0, 0, 0.1)',
        }}
      >
        {(title || extra || actions) && (
          <Header
            style={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              paddingBottom: 16,
              borderBottom: '1px solid #f0f0f0',
              ...headerStyle,
            }}
          >
            <div style={{ flex: 1 }}>
              {title && (
                <div style={{ marginBottom: 8 }}>
                  <Typography.Title
                    level={4}
                    style={{ margin: 0 }}
                  >
                    {title}
                  </Typography.Title>
                </div>
              )}
              {description && (
                <Typography.Text type="secondary">
                  {description}
                </Typography.Text>
              )}
            </div>
            {(extra || actions) && (
              <div style={{ display: 'flex', gap: 8 }}>
                {extra}
                {actions}
              </div>
            )}
          </Header>
        )}
        <Content
          style={{
            padding: 16,
            ...contentStyle,
          }}
        >
          {children}
        </Content>
        {footer && (
          <Footer
            style={{
              borderTop: '1px solid #f0f0f0',
              paddingTop: 16,
              textAlign: 'right',
            }}
          >
            {footer}
          </Footer>
        )}
      </Card>
    </div>
  );
};

export default PageLayout;
