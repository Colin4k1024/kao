import React from 'react';
import { Outlet, Link, useNavigate } from 'react-router-dom';
import { Layout, Menu, Avatar, Dropdown, Space } from 'antd';
import {
  DashboardOutlined,
  UserOutlined,
  TeamOutlined,
  SettingOutlined,
  LogoutOutlined,
  DatabaseOutlined,
  NotificationOutlined,
  SafetyOutlined,
  FileTextOutlined,
  LoginOutlined,
} from '@ant-design/icons';

const { Header, Sider, Content } = Layout;

const MainLayout: React.FC = () => {
  const navigate = useNavigate();
  const userInfo = JSON.parse(localStorage.getItem('user') || '{}');

  const menuItems = [
    {
      key: '/dashboard',
      icon: <DashboardOutlined />,
      label: <Link to="/dashboard">首页</Link>,
    },
    {
      key: '/system',
      icon: <SettingOutlined />,
      label: '系统管理',
      children: [
        {
          key: '/system/users',
          icon: <UserOutlined />,
          label: <Link to="/system/users">用户管理</Link>,
        },
        {
          key: '/system/departments',
          icon: <TeamOutlined />,
          label: <Link to="/system/departments">部门管理</Link>,
        },
        {
          key: '/system/roles',
          icon: <TeamOutlined />,
          label: <Link to="/system/roles">角色管理</Link>,
        },
        {
          key: '/system/menus',
          icon: <TeamOutlined />,
          label: <Link to="/system/menus">菜单管理</Link>,
        },
        {
          key: '/dictionary/type',
          icon: <DatabaseOutlined />,
          label: <Link to="/dictionary/type">字典管理</Link>,
        },
        {
          key: '/config',
          icon: <SettingOutlined />,
          label: <Link to="/config">参数配置</Link>,
        },
        {
          key: '/notice',
          icon: <NotificationOutlined />,
          label: <Link to="/notice">通知公告</Link>,
        },
      ],
    },
    {
      key: '/monitoring',
      icon: <SafetyOutlined />,
      label: '系统监控',
      children: [
        {
          key: '/monitoring/security',
          icon: <SafetyOutlined />,
          label: <Link to="/monitoring/security">安全监控</Link>,
        },
        {
          key: '/monitoring/online-user',
          icon: <UserOutlined />,
          label: <Link to="/monitoring/online-user">在线用户</Link>,
        },
        {
          key: '/monitoring/operation-log',
          icon: <FileTextOutlined />,
          label: <Link to="/monitoring/operation-log">操作日志</Link>,
        },
        {
          key: '/monitoring/login-log',
          icon: <LoginOutlined />,
          label: <Link to="/monitoring/login-log">登录日志</Link>,
        },
      ],
    },
  ];

  const userMenuItems = [
    {
      key: 'profile',
      icon: <UserOutlined />,
      label: '个人信息',
    },
    {
      type: 'divider' as const,
    },
    {
      key: 'logout',
      icon: <LogoutOutlined />,
      label: '退出登录',
      danger: true,
    },
  ];

  const handleMenuClick = (e: { key: string }) => {
    if (e.key === 'logout') {
      localStorage.removeItem('access_token');
      localStorage.removeItem('refresh_token');
      localStorage.removeItem('user');
      navigate('/login');
    }
  };

  return (
    <Layout style={{ minHeight: '100vh' }}>
      <Sider theme="dark" width={240}>
        <div style={{ 
          height: 64, 
          display: 'flex', 
          alignItems: 'center', 
          justifyContent: 'center',
          color: 'white',
          fontSize: 20,
          fontWeight: 'bold',
          borderBottom: '1px solid rgba(255,255,255,0.1)'
        }}>
          后台管理系统
        </div>
        <Menu 
          theme="dark" 
          mode="inline" 
          defaultSelectedKeys={['/dashboard']}
          items={menuItems}
        />
      </Sider>
      <Layout>
        <Header style={{ 
          background: '#fff', 
          padding: '0 24px', 
          display: 'flex', 
          justifyContent: 'flex-end',
          alignItems: 'center',
          borderBottom: '1px solid #f0f0f0'
        }}>
          <Space size="middle">
            <Dropdown 
              menu={{ 
                items: userMenuItems, 
                onClick: handleMenuClick 
              }}
              placement="bottomRight"
            >
              <Space style={{ cursor: 'pointer' }}>
                <Avatar style={{ backgroundColor: '#1890ff' }}>
                  {userInfo.nickname || userInfo.username?.charAt(0).toUpperCase()}
                </Avatar>
                <span>{userInfo.nickname || userInfo.username}</span>
              </Space>
            </Dropdown>
          </Space>
        </Header>
        <Content style={{ margin: '24px 16px', padding: 24, background: '#fff' }}>
          <Outlet />
        </Content>
      </Layout>
    </Layout>
  );
};

export default MainLayout;
