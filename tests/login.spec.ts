import { test, expect } from '@playwright/test';
import { LoginPage } from './utils/page-objects';

test.describe('登录页面测试', () => {
  test('页面加载成功', async ({ page }) => {
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    await expect(page).toHaveTitle(/KAO/);
  });

  test('登录表单存在且可交互', async ({ page }) => {
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    
    await expect(loginPage.usernameInput).toBeVisible();
    await expect(loginPage.passwordInput).toBeVisible();
    await expect(loginPage.submitButton).toBeVisible();
  });

  test('用户名输入框可正常输入', async ({ page }) => {
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    
    await loginPage.usernameInput.fill('admin');
    await expect(loginPage.usernameInput).toHaveValue('admin');
  });

  test('密码输入框可正常输入', async ({ page }) => {
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    
    await loginPage.passwordInput.fill('admin123');
    await expect(loginPage.passwordInput).toHaveValue('admin123');
  });

  test('登录成功应跳转到仪表盘', async ({ page }) => {
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    
    await loginPage.login('testuser2', 'Test@123456');
    await loginPage.expectToBeLoggedIn();
  });

  test('登录后侧边栏菜单可见', async ({ page }) => {
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    
    await loginPage.login('testuser2', 'Test@123456');
    
    // 验证侧边栏存在
    const sidebar = page.locator('nav, aside, [class*="sidebar"]').first();
    await expect(sidebar).toBeVisible({ timeout: 5000 });
  });
});
