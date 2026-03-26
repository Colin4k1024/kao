import React from 'react';
import { Card, Row, Col, Statistic } from 'antd';
import type { MetricsResponse } from '@/services/api/monitoring';

interface ChartSectionProps {
  metrics: MetricsResponse;
}

const ChartSection: React.FC<ChartSectionProps> = ({ metrics }) => {
  return (
    <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
      <Col span={8}>
        <Card>
          <Statistic
            title="HTTP Requests"
            value={metrics.http_requests_total}
          />
        </Card>
      </Col>
      <Col span={8}>
        <Card>
          <Statistic
            title="Avg Response Time"
            value={(metrics.http_request_duration_seconds_sum / metrics.http_requests_total * 1000).toFixed(2)}
            suffix="ms"
          />
        </Card>
      </Col>
      <Col span={8}>
        <Card>
          <Statistic
            title="CPU Usage"
            value={metrics.cpu_usage_percent}
            suffix="%"
          />
        </Card>
      </Col>
    </Row>
  );
};

export default ChartSection;
