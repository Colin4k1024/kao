import React from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import Login from '@/pages/Login';
import MainLayout from '@/pages/layout/MainLayout';
import Dashboard from '@/pages/Dashboard';
import UserList from '@/pages/system/users/UserList';
import DepartmentList from '@/pages/system/departments/DepartmentList';
import RoleList from '@/pages/system/roles/RoleList';
import MenuList from '@/pages/system/menus/MenuList';
import PostList from '@/pages/system/posts/PostList';
import DictionaryTypePage from '@/pages/system/dictionary/type/index';
import DictionaryDataPage from '@/pages/system/dictionary/data/index';
import ConfigPage from '@/pages/system/config/index';
import NoticePage from '@/pages/system/notice/index';
import JobPage from '@/pages/job/index';
import JobLogPage from '@/pages/job/log/index';
import SecurityMonitoring from '@/pages/monitoring/security';
import OnlineUser from '@/pages/monitoring/online-user';
import OperationLog from '@/pages/monitoring/operation-log';
import LoginLog from '@/pages/monitoring/login-log';

const AppRoutes: React.FC = () => {
  const isAuthenticated = !!localStorage.getItem('access_token');

  return (
    <BrowserRouter>
      <Routes>
        <Route path="/login" element={<Login />} />
        <Route
          path="/"
          element={isAuthenticated ? <MainLayout /> : <Navigate to="/login" />}
        >
          <Route index element={<Navigate to="/dashboard" />} />
          <Route path="dashboard" element={<Dashboard />} />
          <Route path="system">
            <Route path="users" element={<UserList />} />
            <Route path="departments" element={<DepartmentList />} />
            <Route path="roles" element={<RoleList />} />
            <Route path="menus" element={<MenuList />} />
            <Route path="posts" element={<PostList />} />
          </Route>
          <Route path="dictionary">
            <Route path="type" element={<DictionaryTypePage />} />
            <Route path="data" element={<DictionaryDataPage />} />
          </Route>
          <Route path="config" element={<ConfigPage />} />
          <Route path="notice" element={<NoticePage />} />
          <Route path="job">
            <Route path="" element={<JobPage />} />
            <Route path="log" element={<JobLogPage />} />
          </Route>
          <Route path="monitoring">
            <Route path="security" element={<SecurityMonitoring />} />
            <Route path="online-user" element={<OnlineUser />} />
            <Route path="operation-log" element={<OperationLog />} />
            <Route path="login-log" element={<LoginLog />} />
          </Route>
        </Route>
      </Routes>
    </BrowserRouter>
  );
};

export default AppRoutes;
