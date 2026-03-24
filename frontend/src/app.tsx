import { LogoutOutlined } from '@ant-design/icons';
import type { Settings as LayoutSettings } from '@ant-design/pro-components';
import { SettingDrawer } from '@ant-design/pro-components';
import type { RequestConfig, RunTimeLayoutConfig } from '@umijs/max';
import { history } from '@umijs/max';
import { request } from '@umijs/max';
import defaultSettings from '../config/defaultSettings';
import '@ant-design/v5-patch-for-react-19';

const isDev = process.env.NODE_ENV === 'development';
const loginPath = '/user/login';

export async function getInitialState(): Promise<{
  settings?: Partial<LayoutSettings>;
  currentUser?: API.CurrentUser;
  loading?: boolean;
  fetchUserInfo?: () => Promise<API.CurrentUser | undefined>;
}> {
  const token = localStorage.getItem('access_token');

  const fetchUserInfo = async () => {
    if (!token) {
      return undefined;
    }
    try {
      const response = await fetch('/api/v1/auth/profile', {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
      });
      const result = await response.json();
      if (result.code === 0) {
        return result.data;
      }
      throw new Error(result.message);
    } catch (_error) {
      localStorage.removeItem('access_token');
      history.push(loginPath);
    }
    return undefined;
  };

  const { location } = history;
  if (location.pathname !== loginPath) {
    const currentUser = await fetchUserInfo();
    return {
      fetchUserInfo,
      currentUser,
      settings: defaultSettings as Partial<LayoutSettings>,
    };
  }
  return {
    fetchUserInfo,
    settings: defaultSettings as Partial<LayoutSettings>,
  };
}

export const layout: RunTimeLayoutConfig = ({
  initialState,
  setInitialState,
}) => {
  return {
    logout: (props) => {
      localStorage.removeItem('access_token');
      history.push(loginPath);
      return undefined;
    },
    avatarProps: {
      title: initialState?.currentUser?.username,
      render: (_, defaultDom) => {
        return (
          <div
            onClick={() => {
              localStorage.removeItem('access_token');
              history.push(loginPath);
            }}
            style={{ cursor: 'pointer', display: 'flex', alignItems: 'center' }}
          >
            {defaultDom}
            <LogoutOutlined style={{ marginLeft: 8 }} />
          </div>
        );
      },
    },
    footerRender: () => (
      <div style={{ textAlign: 'center', padding: '16px' }}>
        RBAC Admin ©{new Date().getFullYear()}
      </div>
    ),
    onPageChange: () => {
      const { location } = history;
      if (!initialState?.currentUser && location.pathname !== loginPath) {
        history.push(loginPath);
      }
    },
    menuHeaderRender: undefined,
    childrenRender: (children) => {
      if (isDev) {
        return (
          <>
            {children}
            <SettingDrawer
              disableUrlParams
              enableDarkTheme
              settings={initialState?.settings}
              onSettingChange={(settings) => {
                setInitialState((preInitialState) => ({
                  ...preInitialState,
                  settings,
                }));
              }}
            />
          </>
        );
      }
      return <>{children}</>;
    },
    ...initialState?.settings,
  };
};

export const request: RequestConfig = {
  baseURL: '/api',
  errorConfig: {
    errorHandler: (error: any) => {
      if (error?.response?.status === 401) {
        localStorage.removeItem('access_token');
        history.push(loginPath);
      }
      throw error;
    },
  },
};
