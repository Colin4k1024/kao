import { test, expect } from '@playwright/test';

test.describe('基本功能测试', () => {
  test('前端页面可以访问', async ({ page }) => {
    await page.goto('/');
    const body = page.locator('body');
    await expect(body).toBeVisible();
  });

  test('登录页面可以访问', async ({ page }) => {
    await page.goto('/login');
    await page.waitForLoadState('networkidle');
    const body = page.locator('body');
    await expect(body).toBeVisible();
  });

  test('页面标题正确', async ({ page }) => {
    await page.goto('/login');
    await expect(page).toHaveTitle(/KAO/i);
  });
});

test.describe('登录功能测试', () => {
  test('可以输入用户名和密码', async ({ page }) => {
    await page.goto('/login');
    await page.waitForLoadState('networkidle');
    
    // 查找输入框并输入内容
    const usernameInput = page.locator('input').first();
    const passwordInput = page.locator('input').nth(1);
    
    if (await usernameInput.isVisible()) {
      await usernameInput.fill('admin');
    }
    if (await passwordInput.isVisible()) {
      await passwordInput.fill('admin123');
    }
  });
});
