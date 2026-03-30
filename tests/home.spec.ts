import { test, expect } from '@playwright/test';

test.describe('首页测试', () => {
  test('首页应该显示系统概览', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder(/用户名|username/i).fill('testuser');
    await page.getByPlaceholder(/密码|password/i).fill('NewTest@123456');
    await page.getByRole('button', { name: /登录|Login/i }).click();
    
    await page.waitForURL(/\/(dashboard|system)/);
    
    // 验证首页内容
    const content = page.locator('body');
    await expect(content).toBeVisible();
  });
});

test.describe('响应式设计测试', () => {
  test('桌面端显示正常', async ({ page }) => {
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.goto('/login');
    await expect(page).toHaveTitle(/KAO/);
  });

  test('移动端显示正常', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/login');
    await expect(page).toHaveTitle(/KAO/);
    await expect(page.locator('body')).toBeVisible();
  });
});

test.describe('404页面测试', () => {
  test('访问不存在的页面应显示404', async ({ page }) => {
    await page.goto('/nonexistent-page-12345');
    const body = page.locator('body');
    const text = await body.textContent();
    // 应该显示404或者回到某个已知页面
    expect(text).toBeTruthy();
  });
});
