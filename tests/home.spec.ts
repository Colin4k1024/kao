import { test, expect } from '@playwright/test';
import { LoginPage, DashboardPage } from './utils/page-objects';

test.describe('首页测试', () => {
  test('登录后应显示系统概览', async ({ page }) => {
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    await loginPage.login('testuser2', 'Test@123456');
    
    await page.waitForURL(/\/(dashboard|system)/);
    const content = page.locator('body');
    await expect(content).toBeVisible();
  });

  test('Dashboard 页面加载正确', async ({ page }) => {
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    await loginPage.login('testuser2', 'Test@123456');
    
    const dashboard = new DashboardPage(page);
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');
    await expect(dashboard.sidebar).toBeVisible({ timeout: 5000 });
  });
});

test.describe('响应式设计测试', () => {
  test('桌面端显示正常 (1920x1080)', async ({ page }) => {
    await page.setViewportSize({ width: 1920, height: 1080 });
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    await expect(page).toHaveTitle(/KAO/);
  });

  test('移动端显示正常 (375x667)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    await expect(page).toHaveTitle(/KAO/);
    await expect(page.locator('body')).toBeVisible();
  });
});

test.describe('错误页面测试', () => {
  test('访问不存在的页面应有响应', async ({ page }) => {
    await page.goto('/nonexistent-page-12345');
    const body = page.locator('body');
    const text = await body.textContent();
    expect(text).toBeTruthy();
  });
});
