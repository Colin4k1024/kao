import { Page, Locator } from '@playwright/test';

export class LoginPage {
  readonly page: Page;
  readonly usernameInput: Locator;
  readonly passwordInput: Locator;
  readonly submitButton: Locator;

  constructor(page: Page) {
    this.page = page;
    this.usernameInput = page.getByPlaceholder(/用户名|username/i);
    this.passwordInput = page.getByPlaceholder(/密码|password/i);
    this.submitButton = page.getByRole('button', { name: /登录|Login/i });
  }

  async goto() {
    await this.page.goto('/login');
    await this.page.waitForLoadState('domcontentloaded');
  }

  async login(username: string, password: string) {
    await this.usernameInput.fill(username);
    await this.passwordInput.fill(password);
    await this.submitButton.click();
    await this.page.waitForLoadState('networkidle');
    await this.page.waitForTimeout(1000);
  }

  async expectToBeLoggedIn() {
    await this.page.waitForURL(/\/(dashboard|system)/);
  }
}

export class DashboardPage {
  readonly page: Page;
  readonly sidebar: Locator;

  constructor(page: Page) {
    this.page = page;
    this.sidebar = page.locator('nav, aside, [class*="sidebar"]').first();
  }

  async goto() {
    await this.page.goto('/dashboard');
    await this.page.waitForLoadState('domcontentloaded');
  }

  async isVisible() {
    await this.page.waitForLoadState('networkidle');
    return this.sidebar.isVisible().catch(() => false);
  }
}
