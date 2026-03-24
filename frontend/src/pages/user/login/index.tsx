import { LockOutlined, UserOutlined } from '@ant-design/icons';
import { LoginForm, ProFormText } from '@ant-design/pro-components';
import { Helmet, SelectLang } from '@umijs/max';
import { App } from 'antd';
import React, { useState } from 'react';
import { Footer } from '@/components';
import * as api from '@/services/api';
import Settings from '../../../../config/defaultSettings';

const Login: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const { message } = App.useApp();

  const handleSubmit = async (values: { username: string; password: string }) => {
    setLoading(true);
    try {
      const response = await api.login(values);
      localStorage.setItem('access_token', response.access_token);
      message.success('登录成功！');
      window.location.href = '/';
    } catch (error: any) {
      message.error(error?.response?.data?.message || error?.message || '登录失败，请重试！');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div
      style={{
        height: '100vh',
        display: 'flex',
        flexDirection: 'column',
        background: `url(https://mdn.alipayobjects.com/yuyan_qk0oxh/afts/img/V-_oS6r-i7wAAAAAAAAAAAAAFl94AQBr) no-repeat center/cover`,
      }}
    >
      <Helmet>
        <title>
          登录 - {Settings.title}
        </title>
      </Helmet>

      <div style={{ position: 'fixed', right: 16, top: 16 }}>
        <SelectLang />
      </div>

      <div
        style={{
          flex: 1,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        <LoginForm
          logo={<img alt="logo" src="/logo.svg" style={{ width: 50, height: 50 }} />}
          title="RBAC Admin"
          subTitle="基于角色的权限管理系统"
          loading={loading}
          onFinish={async (values) => {
            await handleSubmit(values as { username: string; password: string });
          }}
        >
          <ProFormText
            name="username"
            fieldProps={{
              size: 'large',
              prefix: <UserOutlined />,
            }}
            placeholder="用户名: admin"
            rules={[
              {
                required: true,
                message: '请输入用户名！',
              },
            ]}
            initialValue="admin"
          />
          <ProFormText.Password
            name="password"
            fieldProps={{
              size: 'large',
              prefix: <LockOutlined />,
            }}
            placeholder="密码: Admin123!"
            rules={[
              {
                required: true,
                message: '请输入密码！',
              },
            ]}
            initialValue="Admin123!"
          />
        </LoginForm>
      </div>

      <Footer />
    </div>
  );
};

export default Login;
