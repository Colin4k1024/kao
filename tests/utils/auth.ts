import { test as base, Page } from '@playwright/test';

// Test users - in production, use environment variables
export const TEST_USERS = {
  admin: {
    username: 'admin',
    password: 'admin123',
  },
  testuser2: {
    username: 'testuser2',
    password: 'Test@123456',
  },
} as const;

export type TestUser = keyof typeof TEST_USERS;

// Extended fixture type with auth helpers
export interface AuthenticatedPage {
  authenticatedPage: Page;
  loginAs: (user?: TestUser) => Promise<void>;
}

export const test = base.extend<AuthenticatedPage>({
  authenticatedPage: async ({ page }, use) => {
    // Initialize with the page as-is
    await use(page);
  },

  loginAs: async ({ page }: Page, use) => {
    await use(async (user: TestUser = 'testuser2') => {
      const credentials = TEST_USERS[user];
      
      await page.goto('/login');
      await page.waitForLoadState('domcontentloaded');
      
      const usernameInput = page.getByPlaceholder(/用户名|username/i);
      const passwordInput = page.getByPlaceholder(/密码|password/i);
      
      await usernameInput.fill(credentials.username);
      await passwordInput.fill(credentials.password);
      
      await page.getByRole('button', { name: /登录|Login/i }).click();
      await page.waitForLoadState('networkidle');
      
      // Wait for redirect to dashboard or main page
      await page.waitForTimeout(1000);
    });
  },
});

export { expect } from '@playwright/test';
