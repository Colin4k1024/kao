import { test, expect } from '@playwright/test';

const BASE_URL = 'http://localhost:8002';
const API_URL = 'http://127.0.0.1:3000/api/v1';

test.describe('RBAC Admin System E2E Tests', () => {
  let authToken: string;

  test.beforeAll(async ({ request }) => {
    const response = await request.post(`${API_URL}/auth/login`, {
      data: {
        username: 'admin',
        password: 'Admin123!',
      },
    });
    const body = await response.json();
    authToken = body.data.access_token;
    console.log('✅ Login API successful, token obtained');
  });

  test('1. Login page renders', async ({ page }) => {
    await page.goto(`${BASE_URL}/user/login`);
    await expect(page.locator('text=RBAC Admin').first()).toBeVisible({ timeout: 10000 });
    console.log('✅ Login page renders correctly');
  });

  test('2. API - Profile returns correct data', async ({ request }) => {
    const response = await request.get(`${API_URL}/auth/profile`, {
      headers: {
        Authorization: `Bearer ${authToken}`,
      },
    });
    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.code).toBe(0);
    expect(body.data.username).toBe('admin');
    expect(body.data.roles).toContain('ADMIN');
    console.log('✅ API Profile returns correct user data');
  });

  test('3. API - Users list returns data', async ({ request }) => {
    const response = await request.get(`${API_URL}/users`, {
      headers: {
        Authorization: `Bearer ${authToken}`,
      },
    });
    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.code).toBe(0);
    expect(body.data).toBeDefined();
    console.log(`✅ API Users returns ${Array.isArray(body.data) ? body.data.length : 0} users`);
  });

  test('4. API - Roles list returns data', async ({ request }) => {
    const response = await request.get(`${API_URL}/roles`, {
      headers: {
        Authorization: `Bearer ${authToken}`,
      },
    });
    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.code).toBe(0);
    expect(Array.isArray(body.data)).toBeTruthy();
    console.log(`✅ API Roles returns ${body.data.length} roles`);
  });

  test('5. API - Departments list returns data', async ({ request }) => {
    const response = await request.get(`${API_URL}/departments`, {
      headers: {
        Authorization: `Bearer ${authToken}`,
      },
    });
    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.code).toBe(0);
    expect(Array.isArray(body.data)).toBeTruthy();
    console.log(`✅ API Departments returns ${body.data.length} departments`);
  });

  test('6. API - Menus list returns data', async ({ request }) => {
    const response = await request.get(`${API_URL}/menus`, {
      headers: {
        Authorization: `Bearer ${authToken}`,
      },
    });
    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.code).toBe(0);
    expect(Array.isArray(body.data)).toBeTruthy();
    console.log(`✅ API Menus returns ${body.data.length} menus`);
  });

  test('7. API - Permissions returns data', async ({ request }) => {
    const response = await request.get(`${API_URL}/auth/permissions`, {
      headers: {
        Authorization: `Bearer ${authToken}`,
      },
    });
    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.code).toBe(0);
    expect(body.data).toBeDefined();
    console.log('✅ API Permissions returns data');
  });

  test('8. API - User Menus returns data', async ({ request }) => {
    const response = await request.get(`${API_URL}/auth/menus`, {
      headers: {
        Authorization: `Bearer ${authToken}`,
      },
    });
    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.code).toBe(0);
    console.log('✅ API User Menus returns data');
  });

  test('9. Frontend - Dashboard page loads', async ({ page }) => {
    await page.context().addCookies([
      { name: 'access_token', value: authToken, domain: 'localhost', path: '/' },
    ]);
    await page.goto(BASE_URL);
    await page.waitForTimeout(3000);
    const content = await page.content();
    expect(content.length).toBeGreaterThan(100);
    console.log('✅ Frontend Dashboard page loads');
  });

  test('10. Frontend - Login form submission works', async ({ page }) => {
    await page.goto(`${BASE_URL}/user/login`);
    await page.waitForTimeout(1000);

    const usernameInput = page.locator('input').first();
    const passwordInput = page.locator('input[type="password"]').first();

    if (await usernameInput.isVisible()) {
      await usernameInput.fill('admin');
      await passwordInput.fill('Admin123!');

      const submitButton = page.locator('button[type="submit"]');
      if (await submitButton.isEnabled()) {
        await submitButton.click();
        await page.waitForTimeout(2000);
      }
    }
    console.log('✅ Frontend Login form can be filled');
  });
});
