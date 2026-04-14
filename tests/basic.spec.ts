import { test, expect } from '@playwright/test';
import { LoginPage } from './utils/page-objects';

test.describe('基本功能测试', () => {
  test('前端首页可以访问', async ({ page }) => {
    await page.goto('/');
    const body = page.locator('body');
    await expect(body).toBeVisible();
  });

  test('登录页面可以访问', async ({ page }) => {
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    await page.waitForLoadState('networkidle');
    const body = page.locator('body');
    await expect(body).toBeVisible();
  });

  test('页面标题正确', async ({ page }) => {
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    await expect(page).toHaveTitle(/KAO/i);
  });

  test('可以输入用户名和密码', async ({ page }) => {
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    await page.waitForLoadState('networkidle');
    
    await loginPage.usernameInput.fill('admin');
    await loginPage.passwordInput.fill('admin123');
    
    await expect(loginPage.usernameInput).toHaveValue('admin');
    await expect(loginPage.passwordInput).toHaveValue('admin123');
  });
});

test.describe('响应式设计测试', () => {
  test('桌面端显示正常 (1920x1080)', async ({ page }) => {
    await page.setViewportSize({ width: 1920, height: 1080 });
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    await expect(page).toHaveTitle(/KAO/);
  });

  test('平板显示正常 (768x1024)', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    await expect(page).toHaveTitle(/KAO/);
  });

  test('移动端显示正常 (375x667)', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    const loginPage = new LoginPage(page);
    await loginPage.goto();
    await expect(page).toHaveTitle(/KAO/);
  });
});

test.describe('404页面测试', () => {
  test('访问不存在的页面应有响应', async ({ page }) => {
    await page.goto('/nonexistent-page-12345');
    const body = page.locator('body');
    const text = await body.textContent();
    expect(text).toBeTruthy();
  });
});
