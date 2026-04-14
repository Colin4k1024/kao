import { test, expect, Page, chromium } from '@playwright/test';
import { LoginPage, DashboardPage } from './utils/page-objects';

const BASE_URL = process.env.FRONTEND_URL || 'http://localhost:3000';

interface PageConfig {
  path: string;
  name: string;
}

const PAGES: PageConfig[] = [
  { path: '/dashboard', name: 'Dashboard' },
  { path: '/system/users', name: 'User Management' },
  { path: '/system/departments', name: 'Department Management' },
  { path: '/system/roles', name: 'Role Management' },
  { path: '/system/menus', name: 'Menu Management' },
  { path: '/system/posts', name: 'Post Management' },
  { path: '/dictionary/type', name: 'Dictionary Type' },
  { path: '/dictionary/data', name: 'Dictionary Data' },
  { path: '/config', name: 'Config' },
  { path: '/notice', name: 'Notice' },
  { path: '/job', name: 'Job Scheduler' },
  { path: '/job/log', name: 'Job Log' },
  { path: '/monitoring/security', name: 'Security Monitoring' },
  { path: '/monitoring/online-user', name: 'Online Users' },
  { path: '/monitoring/operation-log', name: 'Operation Log' },
  { path: '/monitoring/login-log', name: 'Login Log' },
];

/**
 * Helper function to login before tests
 */
async function login(page: Page): Promise<void> {
  const loginPage = new LoginPage(page);
  await loginPage.goto();
  await loginPage.login('testuser2', 'Test@123456');
}

/**
 * Helper to sanitize filename
 */
function sanitizeFilename(name: string): string {
  return name.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '');
}

test.describe('Smoke Test - All Pages', () => {
  test.beforeEach(async ({ page }) => {
    await login(page);
  });

  test('Login successful and redirect to dashboard', async ({ page }) => {
    await expect(page).toHaveURL(/\/(dashboard|system)/);
  });

  test('Dashboard page loads correctly', async ({ page }) => {
    const dashboard = new DashboardPage(page);
    await page.goto(`${BASE_URL}/dashboard`);
    await page.waitForLoadState('networkidle');
    await expect(page.locator('body')).toBeVisible();
  });

  test('Take screenshot of all pages', async ({ page }) => {
    // Login screenshot
    await page.screenshot({ 
      path: 'test-results/smoke-00-login.png', 
      fullPage: true 
    });

    // Navigate to each page and take screenshot
    for (const { path, name } of PAGES) {
      const sanitizedName = sanitizeFilename(name);
      
      try {
        await page.goto(`${BASE_URL}${path}`, { timeout: 15000 });
        await page.waitForLoadState('domcontentloaded');
        await page.waitForTimeout(1000);

        const screenshotPath = `test-results/smoke-${sanitizedName}.png`;
        await page.screenshot({
          path: screenshotPath,
          fullPage: true,
        });
      } catch (e) {
        // Take error screenshot on failure
        await page.screenshot({
          path: `test-results/smoke-${sanitizedName}-error.png`,
          fullPage: true,
        });
        throw e;
      }
    }
  });
});

test.describe('Individual Page Tests', () => {
  test('User Management page loads', async ({ page }) => {
    await login(page);
    await page.goto(`${BASE_URL}/system/users`);
    await page.waitForLoadState('networkidle');
    await expect(page.locator('body')).toBeVisible();
  });

  test('Department Management page loads', async ({ page }) => {
    await login(page);
    await page.goto(`${BASE_URL}/system/departments`);
    await page.waitForLoadState('networkidle');
    await expect(page.locator('body')).toBeVisible();
  });

  test('Role Management page loads', async ({ page }) => {
    await login(page);
    await login(page);
    await page.goto(`${BASE_URL}/system/roles`);
    await page.waitForLoadState('networkidle');
    await expect(page.locator('body')).toBeVisible();
  });

  test('Menu Management page loads', async ({ page }) => {
    await login(page);
    await page.goto(`${BASE_URL}/system/menus`);
    await page.waitForLoadState('networkidle');
    await expect(page.locator('body')).toBeVisible();
  });

  test('Post Management page loads', async ({ page }) => {
    await login(page);
    await page.goto(`${BASE_URL}/system/posts`);
    await page.waitForLoadState('networkidle');
    await expect(page.locator('body')).toBeVisible();
  });
});

test.describe('Security Monitoring Tests', () => {
  test('Security Monitoring page loads correctly', async ({ page }) => {
    await login(page);
    await page.goto(`${BASE_URL}/monitoring/security`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);
    
    const screenshotPath = 'test-results/smoke-security-monitoring.png';
    await page.screenshot({ path: screenshotPath, fullPage: true });
    
    await expect(page.locator('body')).toBeVisible();
  });

  test('Online Users page loads correctly', async ({ page }) => {
    await login(page);
    await page.goto(`${BASE_URL}/monitoring/online-user`);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);
    
    const screenshotPath = 'test-results/smoke-online-users.png';
    await page.screenshot({ path: screenshotPath, fullPage: true });
    
    await expect(page.locator('body')).toBeVisible();
  });
});
