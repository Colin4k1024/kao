import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
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
    rollupOptions: {
      output: {
        manualChunks: {
          // React core libraries
          react: ['react', 'react-dom', 'react-router-dom'],
          // Ant Design
          'antd': [
            'antd',
            '@ant-design/icons',
          ],
          // Charts
          'charts': ['ant-design-charts', '@antv/g2', '@antv/dataset'],
          // Utils
          'utils': ['axios', 'zod', 'lodash-es'],
          // Services
          'services': [
            '@/services/api/*',
          ],
        },
      },
    },
    chunkSizeWarningLimit: 800,
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true,
      },
    },
    sourcemap: false,
  },
})
