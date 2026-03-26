import React from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import Login from '@/pages/Login';
import MainLayout from '@/pages/layout/MainLayout';
import Dashboard from '@/pages/Dashboard';
import UserList from '@/pages/system/users/UserList';
import DepartmentList from '@/pages/system/departments/DepartmentList';

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
          </Route>
        </Route>
      </Routes>
    </BrowserRouter>
  );
};

export default AppRoutes;
