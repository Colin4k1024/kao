# Kao Frontend - React Admin Dashboard

Enterprise admin management system frontend built with React and Ant Design.

## Features

- ✅ TypeScript for type safety
- ✅ React 18 with hooks
- ✅ Ant Design UI components
- ✅ React Router for navigation
- ✅ React Query for data fetching
- ✅ Axios for HTTP requests
- ✅ Responsive design
- ✅ Authentication with JWT
- ✅ RBAC permission control

## Tech Stack

- **Framework**: React 18.2
- **Language**: TypeScript 5.6
- **Build Tool**: Vite 5.4
- **UI Library**: Ant Design 5.21
- **Routing**: React Router DOM 6.20
- **State Management**: React Query 5.60, React Hook Form 7.53
- **HTTP Client**: Axios 1.7
- **Validation**: Zod 3.23
- **Styling**: Tailwind CSS

## Quick Start

### Prerequisites

```bash
# Node.js 18 or later
node --version  # Should be 18+
npm --version   # Should be 9+

# Or use nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
```

### Setup

```bash
# Clone repository
git clone https://github.com/kao-admin/kao.git
cd kao/frontend

# Install dependencies
npm install

# Create environment file
cp .env.example .env
# Edit .env with your API URL

# Start development server
npm run dev
```

The application will start on `http://localhost:3000`.

## Available Scripts

### Development

```bash
# Start development server
npm run dev

# Run linter
npm run lint

# Format code
npm run format

# Run tests
npm test

# Run tests with coverage
npm run test:coverage
```

### Production

```bash
# Build for production
npm run build

# Preview production build
npm run preview

# Type check
npm run type-check
```

### Utilities

```bash
# Clean build artifacts
npm run clean

# Generate API client from OpenAPI spec
npm run generate-api

# Run CI checks
npm run ci
```

## Project Structure

```
frontend/
├── src/
│   ├── main.tsx                    # Entry point
│   ├── App.tsx                     # Root component
│   ├── assets/
│   │   ├── styles/
│   │   │   ├── main.css
│   │   │   └── variables.css
│   │   └── images/
│   ├── components/
│   │   ├── common/                # Common components
│   │   │   ├── Button.tsx
│   │   │   ├── Card.tsx
│   │   │   ├── Input.tsx
│   │   │   └── Table.tsx
│   │   ├── layout/
│   │   │   ├── Header.tsx
│   │   │   ├── Sidebar.tsx
│   │   │   └── Footer.tsx
│   │   └── .../
│   ├── pages/                      # Page components
│   │   ├── Login.tsx
│   │   ├── Dashboard.tsx
│   │   ├── system/
│   │   │   ├── users/
│   │   │   ├── roles/
│   │   │   ├── menus/
│   │   │   └── departments/
│   │   ├── config/
│   │   │   ├── ConfigList.tsx
│   │   │   └── ...
│   │   ├── dictionary/
│   │   │   └── ...
│   │   ├── notice/
│   │   │   └── ...
│   │   ├── job/
│   │   │   └── ...
│   │   ├── monitoring/
│   │   │   └── ...
│   │   └── error/
│   │       ├── 403.tsx
│   │       ├── 404.tsx
│   │       └── 500.tsx
│   ├── services/
│   │   ├── api/                   # API clients
│   │   │   ├── auth.ts
│   │   │   ├── user.ts
│   │   │   ├── role.ts
│   │   │   └── ...
│   │   └── request.ts             # HTTP client setup
│   ├── hooks/                      # Custom hooks
│   │   ├── useAuth.ts
│   │   ├── usePermission.ts
│   │   └── useValidation.ts
│   ├── lib/                        # Utility libraries
│   │   ├── utils.ts
│   │   ├── validator.ts
│   │   └── constants.ts
│   ├── routes/                     # Route configuration
│   │   ├── index.ts
│   │   └── guards.ts
│   ├── store/                      # Global state
│   │   ├── index.ts
│   │   └── reducer.ts
│   └── types/                      # TypeScript types
│       ├── api.d.ts
│       ├── auth.d.ts
│       └── common.d.ts
├── public/
│   ├── index.html
│   └── favicon.ico
├── tests/
│   ├── components/
│   ├── pages/
│   └── utils/
├── vite.config.ts                  # Vite configuration
├── tailwind.config.js              # Tailwind configuration
├── postcss.config.js               # PostCSS configuration
├── tsconfig.json                   # TypeScript configuration
├── eslintrc.js                     # ESLint configuration
├── prettier.config.js              # Prettier configuration
├── package.json
└── README.md                       # This file
```

## Configuration

### Environment Variables

Create `.env` file:

```env
# API Configuration
VITE_API_URL=http://localhost:8080
VITE_API_PREFIX=/api

# Application Configuration
VITE_APP_NAME=Kao Admin
VITE_APP_URL=http://localhost:3000
VITE_APP_VERSION=1.0.0

# Feature Flags
VITE_ENABLE_ANALYTICS=false
VITE_ENABLE_DEBUG=true
```

### Vite Configuration

Edit `vite.config.ts`:

```typescript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  base: '/',
  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
      },
    },
  },
  build: {
    outDir: 'dist',
    sourcemap: true,
  },
})
```

## Development

### Common Patterns

#### API Service Pattern

```typescript
// src/services/api/user.ts
import { request } from '../request'

export interface User {
  id: string
  username: string
  email: string
  status: 'active' | 'disabled'
}

export const userApi = {
  getUsers: (params?: { page?: number; pageSize?: number }) =>
    request.get<User[]>('/api/system/users', { params }),
  
  getUser: (id: string) =>
    request.get<User>(`/api/system/users/${id}`),
  
  createUser: (data: Partial<User>) =>
    request.post<User>('/api/system/users', data),
  
  updateUser: (id: string, data: Partial<User>) =>
    request.put<User>(`/api/system/users/${id}`, data),
  
  deleteUser: (id: string) =>
    request.delete(`/api/system/users/${id}`),
}
```

#### Component Pattern

```typescript
// src/pages/system/users/UserList.tsx
import React from 'react'
import { Table, Button, Space, message } from 'antd'
import { User } from '@/types/api'
import { useQuery } from '@tanstack/react-query'
import { userApi } from '@/services/api/user'

export const UserList: React.FC = () => {
  const { data, isLoading, refetch } = useQuery(['users'], () =>
    userApi.getUsers({ page: 1, pageSize: 10 })
  )

  const handleDelete = async (id: string) => {
    await userApi.deleteUser(id)
    message.success('User deleted')
    refetch()
  }

  return (
    <div>
      <Table
        loading={isLoading}
        dataSource={data?.records || []}
        columns={[
          { title: 'Username', dataIndex: 'username' },
          { title: 'Email', dataIndex: 'email' },
          {
            title: 'Status',
            dataIndex: 'status',
            render: (status: string) => (
              <span>{status === 'active' ? 'Active' : 'Disabled'}</span>
            ),
          },
          {
            title: 'Actions',
            render: (_, record) => (
              <Space>
                <Button onClick={() => handleDelete(record.id)}>Delete</Button>
              </Space>
            ),
          },
        ]}
      />
    </div>
  )
}
```

#### Form Pattern

```typescript
// src/pages/system/users/UserForm.tsx
import React from 'react'
import { Form, Input, Button, Modal } from 'antd'
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'

const schema = z.object({
  username: z.string().min(3).max(50),
  email: z.string().email(),
  password: z.string().min(8),
})

export type FormData = z.infer<typeof schema>

export const UserForm: React.FC<{ isOpen: boolean; onClose: () => void }> = ({
  isOpen,
  onClose,
}) => {
  const [form] = useForm<FormData>()

  const onSubmit = (data: FormData) => {
    console.log(data)
  }

  return (
    <Modal open={isOpen} onClose={onClose}>
      <Form form={form} onSubmit={onSubmit}>
        <Form.Item name="username" label="Username">
          <Input />
        </Form.Item>
        <Form.Item name="email" label="Email">
          <Input />
        </Form.Item>
        <Form.Item name="password" label="Password">
          <Input type="password" />
        </Form.Item>
        <Form.Item>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
        </Form.Item>
      </Form>
    </Modal>
  )
}
```

## Component Library

### Common Components

#### Button

```typescript
import { Button } from 'antd'

<Button type="primary">Primary Button</Button>
<Button type="dashed">Dashed Button</Button>
<Button danger>Danger Button</Button>
```

#### Table

```typescript
import { Table } from 'antd'

<Table
  dataSource={data}
  columns={columns}
  pagination={{ pageSize: 20 }}
/>
```

#### Form

```typescript
import { Form, Input } from 'antd'

<Form>
  <Form.Item label="Username">
    <Input />
  </Form.Item>
</Form>
```

### Layout Components

#### Header

```typescript
import { Header } from '@/components/layout/Header'

<Header title="Kao Admin" />
```

#### Sidebar

```typescript
import { Sidebar } from '@/components/layout/Sidebar'

<Sidebar menu={menuItems} />
```

## API Integration

### Authentication Flow

```typescript
// Login
const login = async (username: string, password: string) => {
  const response = await request.post('/api/auth/login', {
    username,
    password,
  })
  localStorage.setItem('token', response.data.access_token)
}

// Check auth
const checkAuth = async () => {
  const token = localStorage.getItem('token')
  if (!token) return false
  return await request.post('/api/auth/refresh')
}

// Logout
const logout = async () => {
  await request.post('/api/auth/logout')
  localStorage.removeItem('token')
}
```

### Request Interceptor

```typescript
// src/services/request.ts
import axios from 'axios'

const request = axios.create({
  baseURL: import.meta.env.VITE_API_URL,
  timeout: 10000,
})

// Request interceptor
request.interceptors.request.use((config) => {
  const token = localStorage.getItem('token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// Response interceptor
request.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      localStorage.removeItem('token')
      window.location.href = '/login'
    }
    return Promise.reject(error)
  }
)

export { request }
```

## Styling

### CSS Variables

```css
/* src/assets/styles/variables.css */
:root {
  --primary-color: #1890ff;
  --secondary-color: #52c41a;
  --danger-color: #ff4d4f;
  --warning-color: #faad14;
  --info-color: #1890ff;
  --success-color: #52c41a;
  --text-color: #333;
  --bg-color: #fff;
}
```

### Tailwind Configuration

Edit `tailwind.config.js`:

```javascript
module.exports = {
  content: ['./src/**/*.{js,jsx,ts,tsx}'],
  theme: {
    extend: {
      colors: {
        primary: '#1890ff',
        secondary: '#52c41a',
      },
    },
  },
  plugins: [],
}
```

## Testing

### Unit Tests

```typescript
// src/services/api/user.test.ts
import { userApi } from './user'

describe('User API', () => {
  it('should get users', async () => {
    const response = await userApi.getUsers()
    expect(response.data).toBeInstanceOf(Array)
  })
})
```

### Component Tests

```typescript
// src/pages/system/users/UserList.test.tsx
import { render, screen } from '@testing-library/react'
import { UserList } from './UserList'

describe('UserList', () => {
  it('renders user list', () => {
    render(<UserList />)
    expect(screen.getByText('Username')).toBeInTheDocument()
  })
})
```

## Performance Optimization

### Code Splitting

```typescript
import { lazy } from 'react'

const UserList = lazy(() => import('./UserList'))
const UserRole = lazy(() => import('./UserRole'))

<Suspense fallback={<Spin />}>
  <Route path="/users" element={<UserList />} />
  <Route path="/roles" element={<UserRole />} />
</Suspense>
```

### React Query Caching

```typescript
// Enable automatic caching
const { data } = useQuery(['users'], fetchUsers, {
  staleTime: 1000 * 60 * 5, // 5 minutes
  cacheTime: 1000 * 60 * 30, // 30 minutes
})
```

## Browser Support

- Chrome (latest)
- Firefox (latest)
- Safari (latest)
- Edge (latest)

---

## License

MIT License - See LICENSE file for details.

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'feat: add amazing feature'`)
4. Push branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## Support

- GitHub Issues: https://github.com/kao-admin/kao/issues
- Documentation: https://kao-admin.com/docs
- Email: support@kao-admin.com

---

**Version**: 1.0.0  
**Last Updated**: 2024-01-01
