import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('loads a sample evidence report without console errors', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (message) => { if (message.type() === 'error') errors.push(message.text()); });
  await page.goto('/');
  await page.getByRole('button', { name: 'Load a sample report' }).click();
  await expect(page.getByText('2 checks ready')).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Recorded checks' })).toBeVisible();
  expect(errors).toEqual([]);
});

test('has no serious accessibility violations', async ({ page }) => {
  await page.goto('/');
  // axe's declarations currently target a newer Playwright Page than the
  // factory-pinned runner; their runtime API is compatible.
  const scan = await new AxeBuilder({ page: page as never }).analyze();
  expect(scan.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? '')).map((violation) => violation.id)).toEqual([]);
});

test('publishes policy documents and exposes a visible file focus ring', async ({ page }) => {
  await page.goto('/privacy/');
  await expect(page).toHaveTitle('Privacy — Infra Test Evidence');
  await expect(page.getByRole('heading', { name: 'Privacy' })).toBeVisible();
  await page.goto('/terms/');
  await expect(page.getByRole('heading', { name: 'Terms' })).toBeVisible();
  await page.goto('/');
  await page.locator('#evidence-file').focus();
  await expect(page.locator('.drop-zone')).toHaveCSS('outline-style', 'solid');
});
