import React, { useEffect, useState } from 'react';
import { Card, Row, Col, Statistic } from 'antd';
import { UserOutlined, TeamOutlined, SettingOutlined, SafetyOutlined } from '@ant-design/icons';

interface DashboardStats {
  totalUsers: number;
  totalDepartments: number;
  totalRoles: number;
  totalMenus: number;
}

const Dashboard: React.FC = () => {
  const [stats, setStats] = useState<DashboardStats>({
    totalUsers: 0,
    totalDepartments: 0,
    totalRoles: 0,
    totalMenus: 0,
  });

  useEffect(() => {
    // 实际项目中应该从API获取数据
    setStats({
      totalUsers: 100,
      totalDepartments: 20,
      totalRoles: 5,
      totalMenus: 50,
    });
  }, []);

  return (
    <div>
      <h1 style={{ marginBottom: 24 }}>系统概览</h1>
      <Row gutter={16}>
        <Col span={6}>
          <Card>
            <Statistic
              title="用户总数"
              value={stats.totalUsers}
              prefix={<UserOutlined />}
              valueStyle={{ color: '#1890ff' }}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic
              title="部门总数"
              value={stats.totalDepartments}
              prefix={<TeamOutlined />}
              valueStyle={{ color: '#52c41a' }}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic
              title="角色总数"
              value={stats.totalRoles}
              prefix={<SafetyOutlined />}
              valueStyle={{ color: '#faad14' }}
            />
          </Card>
        </Col>
        <Col span={6}>
          <Card>
            <Statistic
              title="菜单总数"
              value={stats.totalMenus}
              prefix={<SettingOutlined />}
              valueStyle={{ color: '#f5222d' }}
            />
          </Card>
        </Col>
      </Row>
    </div>
  );
};

export default Dashboard;
