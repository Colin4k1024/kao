import { test, expect, chromium } from '@playwright/test';

const BASE_URL = 'http://localhost:3000';

test('Security Monitoring Page', async () => {
  const browser = await chromium.launch({ headless: true });
  const context = await browser.newContext();
  const page = await context.newPage();

  // Login first
  await page.goto(`${BASE_URL}/login`);
  await page.waitForLoadState('networkidle');

  const usernameInput = page.getByPlaceholder(/用户名|username/i);
  const passwordInput = page.getByPlaceholder(/密码|password/i);

  await usernameInput.fill('testuser2');
  await passwordInput.fill('Test@123456');

  await page.getByRole('button', { name: /登录|Login/i }).click();
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(2000);

  console.log('After login URL:', page.url());

  // Navigate to security monitoring
  await page.goto(`${BASE_URL}/monitoring/security`);
  await page.waitForLoadState('networkidle');
  await page.waitForTimeout(3000);

  console.log('Security page URL:', page.url());

  const screenshotPath = 'test-results/smoke-security-monitoring-fixed.png';
  await page.screenshot({
    path: screenshotPath,
    fullPage: true,
  });

  console.log(`Screenshot saved: ${screenshotPath}`);

  // Check page content
  const bodyText = await page.locator('body').innerText();
  console.log('Page contains "安全监控":', bodyText.includes('安全监控') || bodyText.includes('Security'));
  console.log('Page contains "登录":', bodyText.includes('登录'));

  await browser.close();
});
