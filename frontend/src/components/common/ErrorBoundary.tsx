import React from 'react';
import { Card, Typography, Button, Divider } from 'antd';
import { ExclamationCircleOutlined } from '@ant-design/icons';

const { Title, Text } = Typography;

export interface ErrorBoundaryProps {
  children: React.ReactNode;
  fallback?: React.ReactNode;
  onError?: (error: Error) => void;
  onReset?: () => void;
}

export interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
  errorInfo: React.ErrorInfo | null;
}

export class ErrorBoundary extends React.Component<
  ErrorBoundaryProps,
  ErrorBoundaryState
> {
  public state: ErrorBoundaryState = {
    hasError: false,
    error: null,
    errorInfo: null,
  };

  public static getDerivedStateFromError(error: Error): Partial<ErrorBoundaryState> {
    return {
      hasError: true,
      error,
    };
  }

  public componentDidCatch(error: Error, errorInfo: React.ErrorInfo): void {
    console.error('ErrorBoundary caught an error:', error, errorInfo);
    
    if (this.props.onError) {
      this.props.onError(error);
    }

    this.setState({
      error,
      errorInfo,
    });
  }

  private handleReset = (): void => {
    this.setState({
      hasError: false,
      error: null,
      errorInfo: null,
    });
    if (this.props.onReset) {
      this.props.onReset();
    }
  };

  public render() {
    const { hasError, error, errorInfo } = this.state;

    if (hasError) {
      // You can render any custom fallback UI
      if (this.props.fallback) {
        return this.props.fallback;
      }

      return (
        <Card
          bordered={false}
          style={{
            margin: 24,
            borderRadius: 8,
            borderColor: '#ff4d4f',
          }}
        >
          <ExclamationCircleOutlined
            style={{
              fontSize: 48,
              color: '#ff4d4f',
              marginBottom: 16,
            }}
          />
          <Title level={3} style={{ color: '#ff4d4f', marginBottom: 16 }}>
            抱歉，系统出错了
          </Title>
          <Text type="secondary" style={{ marginBottom: 16, display: 'block' }}>
            错误信息：
          </Text>
          <div
            style={{
              backgroundColor: '#fff',
              padding: 16,
              borderRadius: 4,
              fontFamily: 'monospace',
              marginBottom: 16,
              maxHeight: 200,
              overflow: 'auto',
            }}
          >
            {error?.message}
          </div>
          {errorInfo && (
            <div
              style={{
                backgroundColor: '#f5f5f5',
                padding: 16,
                borderRadius: 4,
                fontFamily: 'monospace',
                marginBottom: 16,
                fontSize: 12,
                maxHeight: 300,
                overflow: 'auto',
              }}
            >
              <pre>{errorInfo.componentStack}</pre>
            </div>
          )}
          <Button type="primary" onClick={this.handleReset}>
            刷新页面
          </Button>
        </Card>
      );
    }

    return this.props.children;
  }
}

// Error fallback component for render prop
export const ErrorFallback: React.FC<{
  error: Error;
  onReset: () => void;
}> = ({ error, onReset }) => {
  return (
    <Card
      bordered={false}
      style={{
        margin: 24,
        borderRadius: 8,
        borderColor: '#ff4d4f',
      }}
    >
      <ExclamationCircleOutlined
        style={{
          fontSize: 48,
          color: '#ff4d4f',
          marginBottom: 16,
        }}
      />
      <Title level={3} style={{ color: '#ff4d4f', marginBottom: 16 }}>
        抱歉，系统出错了
      </Title>
      <Text type="secondary" style={{ marginBottom: 16, display: 'block' }}>
        错误信息：
      </Text>
      <div
        style={{
          backgroundColor: '#fff',
          padding: 16,
          borderRadius: 4,
          fontFamily: 'monospace',
          marginBottom: 16,
        }}
      >
        {error.message}
      </div>
      <Button type="primary" onClick={onReset}>
        刷新页面
      </Button>
    </Card>
  );
};

export default ErrorBoundary;
