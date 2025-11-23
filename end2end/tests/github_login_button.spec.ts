import { test, expect, Page } from "@playwright/test";

/**
 * GitHubLoginButton E2E Tests
 *
 * These tests verify the GitHubLoginButton component behavior:
 * - TC-009: Authenticated users see "ダッシュボードへ" button
 * - TC-010: Unauthenticated users see "GitHub でログイン" button
 * - TC-011: Button display changes when auth status changes
 *
 * Related Documentation:
 * - Issue: https://github.com/otomatty/continuum/issues/13
 * - Spec: docs/01_issues/open/2025_11/20251123_01_authentication-state-management.md
 */

// Helper function to create a valid session cookie value
function createSessionCookieValue(userId: string, expiresInSeconds: number = 3600): string {
  const expiresAt = Math.floor(Date.now() / 1000) + expiresInSeconds;
  return JSON.stringify({
    user_id: userId,
    access_token: "test_access_token",
    expires_at: expiresAt,
  });
}

// Helper function to set authenticated session cookie
async function setAuthenticatedSession(
  page: Page,
  userId: string = "test_user"
): Promise<void> {
  const cookieValue = createSessionCookieValue(userId);
  await page.context().addCookies([
    {
      name: "session",
      value: cookieValue,
      domain: "localhost",
      path: "/",
      httpOnly: true,
      secure: false,
      sameSite: "Lax",
    },
  ]);
}

// Helper function to clear session cookie
async function clearSession(page: Page): Promise<void> {
  await page.context().addCookies([
    {
      name: "session",
      value: "",
      domain: "localhost",
      path: "/",
      httpOnly: true,
      secure: false,
      sameSite: "Lax",
      expires: Math.floor(Date.now() / 1000) - 1, // Expired
    },
  ]);
}

test.describe("GitHubLoginButton Component", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to homepage where GitHubLoginButton is displayed
    await page.goto("http://localhost:3000/");
  });

  // TC-010: 未認証の場合、「GitHubでログイン」ボタンが表示される
  test("TC-010: displays login button for unauthenticated users", async ({
    page,
  }) => {
    // Given: User is not authenticated (no session cookie)
    // This is the default state after page.goto()

    // When: Page loads
    // Then: "GitHub でログイン" button should be visible
    const loginButton = page.getByRole("button", { name: /GitHub.*ログイン/i });
    await expect(loginButton).toBeVisible();

    // And: Button should contain GitHub icon (SVG)
    const githubIcon = loginButton.locator("svg");
    await expect(githubIcon).toBeVisible();

    // And: Clicking button should redirect to /auth/login
    // Note: We can't test actual redirect in E2E without mocking, but we can verify the button exists
    await expect(loginButton).toBeEnabled();
  });

  // TC-009: 認証済みの場合、「ダッシュボードへ」ボタンが表示される
  test("TC-009: displays dashboard button for authenticated users", async ({
    page,
  }) => {
    // Given: User is authenticated (has valid session cookie)
    await setAuthenticatedSession(page, "test_user_123");

    // When: Page loads/reloads with authenticated session
    await page.reload();
    await page.waitForLoadState("networkidle");

    // Then: "ダッシュボードへ" button should be visible
    const dashboardButton = page.getByRole("button", {
      name: /ダッシュボードへ/i,
    });
    await expect(dashboardButton).toBeVisible();

    // And: "GitHub でログイン" button should NOT be visible
    const loginButton = page.getByRole("button", { name: /GitHub.*ログイン/i });
    await expect(loginButton).not.toBeVisible();

    // And: Button should be enabled
    await expect(dashboardButton).toBeEnabled();
  });

  // TC-011: 認証状態が変更された場合、ボタンの表示が切り替わる
  test("TC-011: button display changes when auth status changes", async ({
    page,
  }) => {
    // Given: Initial state is unauthenticated
    // (No session cookie set)

    // When: Page loads
    await page.waitForLoadState("networkidle");

    // Then: Login button should be visible
    let loginButton = page.getByRole("button", { name: /GitHub.*ログイン/i });
    await expect(loginButton).toBeVisible();

    // When: Authentication status changes to authenticated
    await setAuthenticatedSession(page, "test_user_456");
    await page.reload();
    await page.waitForLoadState("networkidle");

    // Then: Dashboard button should be visible
    const dashboardButton = page.getByRole("button", {
      name: /ダッシュボードへ/i,
    });
    await expect(dashboardButton).toBeVisible();

    // And: Login button should NOT be visible
    loginButton = page.getByRole("button", { name: /GitHub.*ログイン/i });
    await expect(loginButton).not.toBeVisible();

    // When: Authentication status changes back to unauthenticated
    await clearSession(page);
    await page.reload();
    await page.waitForLoadState("networkidle");

    // Then: Login button should be visible again
    loginButton = page.getByRole("button", { name: /GitHub.*ログイン/i });
    await expect(loginButton).toBeVisible();

    // And: Dashboard button should NOT be visible
    const dashboardButtonAfterLogout = page.getByRole("button", {
      name: /ダッシュボードへ/i,
    });
    await expect(dashboardButtonAfterLogout).not.toBeVisible();
  });

  // Additional test: Verify button appears in multiple locations
  test("button appears in header and CTA sections", async ({ page }) => {
    // Given: User is not authenticated
    await page.waitForLoadState("networkidle");

    // When: Page loads
    // Then: Login button should appear in header
    const headerButton = page
      .locator("header")
      .getByRole("button", { name: /GitHub.*ログイン/i });
    await expect(headerButton).toBeVisible();

    // And: Login button should appear in CTA sections (scroll down to find)
    await page.evaluate(() => window.scrollTo(0, document.body.scrollHeight));
    await page.waitForTimeout(500); // Wait for scroll

    // Note: Multiple buttons with same text might exist, that's expected
    const allLoginButtons = page.getByRole("button", {
      name: /GitHub.*ログイン/i,
    });
    const count = await allLoginButtons.count();
    expect(count).toBeGreaterThan(0);
  });

  // Additional test: Verify expired session cookie is treated as unauthenticated
  test("expired session cookie is treated as unauthenticated", async ({
    page,
  }) => {
    // Given: User has expired session cookie
    const expiredCookieValue = createSessionCookieValue("test_user", -3600); // Expired 1 hour ago
    await page.context().addCookies([
      {
        name: "session",
        value: expiredCookieValue,
        domain: "localhost",
        path: "/",
        httpOnly: true,
        secure: false,
        sameSite: "Lax",
      },
    ]);

    // When: Page loads
    await page.reload();
    await page.waitForLoadState("networkidle");

    // Then: Login button should be visible (not dashboard button)
    const loginButton = page.getByRole("button", { name: /GitHub.*ログイン/i });
    await expect(loginButton).toBeVisible();

    const dashboardButton = page.getByRole("button", {
      name: /ダッシュボードへ/i,
    });
    await expect(dashboardButton).not.toBeVisible();
  });
});

