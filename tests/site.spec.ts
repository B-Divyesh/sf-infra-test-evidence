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
