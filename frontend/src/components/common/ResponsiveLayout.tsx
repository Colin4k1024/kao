import React from 'react';
import { Layout } from 'antd';
import useResponsive, { BreakpointKey } from '@/hooks/useResponsive';

const { Header: AntdHeader, Sider: AntdSider, Content: AntdContent, Footer: AntdFooter } = Layout;

// Responsive Header component
export const ResponsiveHeader: React.FC<{
  collapsed?: boolean;
  onCollapse?: (collapsed: boolean) => void;
  children?: React.ReactNode;
}> = ({ collapsed, onCollapse, children }) => {
  const { isMobile } = useResponsive();

  return (
    <AntdHeader
      style={{
        padding: isMobile ? '0 12px' : '0 24px',
        background: '#fff',
        boxShadow: '0 2px 8px rgba(0, 0, 0, 0.1)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
      }}
    >
      <div style={{ display: 'flex', alignItems: 'center', flex: 1 }}>
        {children}
      </div>
    </AntdHeader>
  );
};

// Responsive Sider component
export const ResponsiveSider: React.FC<{
  collapsed?: boolean;
  onCollapse?: (collapsed: boolean) => void;
  breakPoint?: 'xs' | 'sm' | 'md' | 'lg' | 'xl' | 'xxl';
  width?: number | string;
  collapsedWidth?: number | string;
  children?: React.ReactNode;
}> = ({ collapsed, onCollapse, breakPoint, width = 200, collapsedWidth = 64, children }) => {
  const { isMobile, matches } = useResponsive();

  // Determine if sidebar should be hidden on certain screens
  const hidden = breakPoint ? !matches(breakPoint) : false;

  // Determine if collapsed state should be controlled
  const isCollapsed = isMobile ? true : collapsed;

  return (
    <AntdSider
      theme="light"
      width={width}
      collapsedWidth={collapsedWidth}
      collapsed={isCollapsed}
      collapsible
      trigger={null}
      style={{
        height: '100vh',
        overflow: 'auto',
        position: 'fixed',
        left: 0,
        top: 0,
        bottom: 0,
        transition: 'all 0.3s ease',
        zIndex: 100,
        backgroundColor: '#fff',
        borderRight: '1px solid #f0f0f0',
      }}
      className={hidden ? 'hidden-sider' : ''}
    >
      {children}
    </AntdSider>
  );
};

// Responsive Content component
export const ResponsiveContent: React.FC<{
  hasSider?: boolean;
  siderWidth?: number;
  children?: React.ReactNode;
}> = ({ hasSider = false, siderWidth = 200, children }) => {
  const { isMobile } = useResponsive();

  return (
    <AntdContent
      style={{
        margin: hasSider ? (isMobile ? '0 12px' : '0 24px') : '0 auto',
        padding: isMobile ? '12px' : '24px',
        minHeight: '100vh',
        backgroundColor: '#f0f2f5',
        transition: 'margin 0.3s ease',
        marginLeft: hasSider ? (isMobile ? 0 : siderWidth) : 0,
      }}
    >
      {children}
    </AntdContent>
  );
};

// Responsive Layout wrapper
export const ResponsiveLayout: React.FC<{
  header?: React.ReactNode;
  sider?: React.ReactNode;
  content?: React.ReactNode;
  footer?: React.ReactNode;
  hasSider?: boolean;
  siderWidth?: number;
  collapsed?: boolean;
  onCollapse?: (collapsed: boolean) => void;
  breakPoint?: BreakpointKey;
}> = ({
  header,
  sider,
  content,
  footer,
  hasSider = false,
  siderWidth = 200,
  collapsed,
  onCollapse,
  breakPoint,
}) => {
  const { isMobile } = useResponsive();

  return (
    <Layout style={{ minHeight: '100vh' }}>
      {hasSider && (
        <ResponsiveSider
          collapsed={collapsed}
          onCollapse={onCollapse}
          breakPoint={breakPoint}
          width={siderWidth}
          collapsedWidth={isMobile ? 0 : 64}
        >
          {sider}
        </ResponsiveSider>
      )}
      <Layout style={{ marginLeft: hasSider && !isMobile ? siderWidth : 0 }}>
        {header && <ResponsiveHeader>{header}</ResponsiveHeader>}
        <ResponsiveContent hasSider={hasSider} siderWidth={siderWidth}>
          {content}
        </ResponsiveContent>
        {footer && <AntdFooter style={{ textAlign: 'center' }}>{footer}</AntdFooter>}
      </Layout>
    </Layout>
  );
};

// Responsive Container component
export const ResponsiveContainer: React.FC<{
  maxWidth?: number;
  children?: React.ReactNode;
  className?: string;
}> = ({ maxWidth = 1200, children, className }) => {
  const { isMobile } = useResponsive();

  return (
    <div
      className={className}
      style={{
        maxWidth,
        margin: '0 auto',
        padding: isMobile ? '12px' : '24px',
      }}
    >
      {children}
    </div>
  );
};

export default ResponsiveLayout;
