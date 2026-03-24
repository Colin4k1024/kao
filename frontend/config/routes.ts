/**
 * @name umi 的路由配置
 * @description RBAC Admin System Routes
 * @doc https://umijs.org/docs/guides/routes
 */
export default [
  {
    path: '/user',
    layout: false,
    routes: [
      {
        name: 'login',
        path: '/user/login',
        component: './user/login',
      },
    ],
  },
  {
    path: '/',
    name: '首页',
    icon: 'home',
    component: './Welcome',
  },
  {
    path: '/system',
    name: '系统管理',
    icon: 'setting',
    routes: [
      {
        path: '/system/users',
        name: '用户管理',
        icon: 'user',
        component: './system/users',
      },
      {
        path: '/system/roles',
        name: '角色管理',
        icon: 'shield',
        component: './system/roles',
      },
      {
        path: '/system/departments',
        name: '部门管理',
        icon: 'cluster',
        component: './system/departments',
      },
      {
        path: '/system/menus',
        name: '菜单管理',
        icon: 'menu',
        component: './system/menus',
      },
    ],
  },
  {
    path: '*',
    layout: false,
    component: './404',
  },
];
