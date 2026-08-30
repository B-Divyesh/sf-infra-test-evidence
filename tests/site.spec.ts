import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const origin = 'http://127.0.0.1:4173';

test('@claim:site-demo opens, resets, and exits the in-memory sample from the first screen', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByText('For infrastructure-module maintainers')).toBeVisible();
  await page.getByRole('link', { name: 'Try it with sample data' }).first().click();
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await expect(page.getByText('2 checks ready')).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Recorded checks' })).toBeVisible();

  await page.locator('#evidence-file').setInputFiles({
    name: 'one-check.json',
    mimeType: 'application/json',
    buffer: Buffer.from(JSON.stringify({ run: 'changed', environment: 'demo', recordedAt: '2026-08-30T00:00:00Z', checks: [{ name: 'Policy check', status: 'pass' }] })),
  });
  await expect(page.getByText('1 checks ready')).toBeVisible();
  await page.locator('#evidence-file').setInputFiles({ name: 'broken.json', mimeType: 'application/json', buffer: Buffer.from('{') });
  await expect(page.getByRole('alert')).toContainText('not valid JSON');
  await page.getByRole('button', { name: 'Reset demo' }).click();
  await expect(page.getByText('2 checks ready')).toBeVisible();
  await expect(page.getByRole('alert')).toBeHidden();
  await page.getByRole('link', { name: 'Start for real' }).click();
  await expect(page).toHaveURL(/\/$/);
  await expect(page.getByText('Demo — sample data, nothing is saved')).toHaveCount(0);
});

test('@claim:reader-private keeps local evidence off the network and out of browser storage', async ({ browser }) => {
  const context = await browser.newContext({ baseURL: origin });
  const page = await context.newPage();
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));
  try {
    await page.goto('/demo/');
    await page.locator('#evidence-file').setInputFiles({
      name: 'private.json',
      mimeType: 'application/json',
      buffer: Buffer.from(JSON.stringify({ run: 'private-run', environment: 'local', recordedAt: '2026-08-30T00:00:00Z', checks: [{ name: 'Private check', status: 'pass' }] })),
    });
    await expect(page.getByText('1 checks ready')).toBeVisible();
    expect(requests.length).toBeGreaterThan(0);
    expect(requests.every((url) => new URL(url).origin === origin)).toBe(true);
    expect(await context.cookies()).toEqual([]);
    expect(await page.evaluate(async () => ({
      local: Object.keys(localStorage),
      session: Object.keys(sessionStorage),
      databases: 'databases' in indexedDB ? (await indexedDB.databases()).map(({ name }) => name) : [],
    }))).toEqual({ local: [], session: [], databases: [] });
  } finally {
    await context.close();
  }
});

test('publishes distinct demo, policy, discovery, and error documents', async ({ page, request }) => {
  const errors: string[] = [];
  page.on('console', (message) => { if (message.type() === 'error') errors.push(message.text()); });
  page.on('pageerror', (error) => errors.push(error.message));
  const robots = await request.get('/robots.txt');
  expect(robots.status()).toBe(200);
  expect(await robots.text()).toContain('Sitemap: https://infra-test-evidence.sociobot.in/sitemap.xml');
  const sitemap = await request.get('/sitemap.xml');
  expect(sitemap.status()).toBe(200);
  expect(await sitemap.text()).toContain('/demo/');

  await page.goto('/demo/');
  await expect(page).toHaveTitle('Demo — Infra Test Evidence');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Inspect sample infrastructure test evidence');
  await page.goto('/privacy/');
  await expect(page).toHaveTitle('Privacy — Infra Test Evidence');
  await expect(page.getByRole('heading', { name: 'Privacy', exact: true })).toBeVisible();
  await page.goto('/terms/');
  await expect(page).toHaveTitle('Terms — Infra Test Evidence');
  await expect(page.getByRole('heading', { name: 'Terms', exact: true })).toBeVisible();
  await page.goto('/404.html');
  await expect(page).toHaveTitle('Page not found — Infra Test Evidence');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('This evidence page was not found');
  expect(errors).toEqual([]);
});

test('supports keyboard navigation, visible focus, reduced motion, and 200% text', async ({ page }) => {
  await page.goto('/');
  await page.keyboard.press('Tab');
  await expect(page.getByRole('link', { name: 'Skip to evidence reader' })).toBeFocused();
  await page.keyboard.press('Enter');
  await expect(page).toHaveURL(/#main$/);
  await page.locator('#evidence-file').focus();
  await expect(page.locator('.drop-zone')).toHaveCSS('outline-style', 'solid');
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth)).toBe(true);

  await page.emulateMedia({ reducedMotion: 'reduce' });
  const duration = await page.locator('.drop-zone').evaluate((element) => getComputedStyle(element).transitionDuration);
  expect(Number.parseFloat(duration)).toBeLessThanOrEqual(0.00001);

  const undersizedTargets = await page.locator('a, button').evaluateAll((elements) => elements
    .filter((element) => {
      const style = getComputedStyle(element);
      return style.visibility !== 'hidden' && style.display !== 'none';
    })
    .map((element) => ({ label: element.textContent?.trim(), box: element.getBoundingClientRect().toJSON() }))
    .filter(({ box }) => box.width < 44 || box.height < 44));
  expect(undersizedTargets).toEqual([]);

  await page.goto('/demo/');
  const undersizedDemoTargets = await page.locator('a, button').evaluateAll((elements) => elements
    .filter((element) => {
      const style = getComputedStyle(element);
      return style.visibility !== 'hidden' && style.display !== 'none';
    })
    .map((element) => ({ label: element.textContent?.trim(), box: element.getBoundingClientRect().toJSON() }))
    .filter(({ box }) => box.width < 44 || box.height < 44));
  expect(undersizedDemoTargets).toEqual([]);

  await page.evaluate(() => { document.documentElement.style.fontSize = '200%'; });
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= document.documentElement.clientWidth)).toBe(true);
});

test('accessibility: root, demo, policy, and error pages have no serious findings', async ({ page }) => {
  for (const colorScheme of ['light', 'dark'] as const) {
    await page.emulateMedia({ colorScheme });
    for (const route of ['/', '/demo/', '/privacy/', '/terms/', '/404.html']) {
      await page.goto(route);
      await expect(page.locator('html')).toHaveAttribute('lang', 'en');
      await expect(page.locator('main')).toHaveCount(1);
      await expect(page.locator('h1')).toHaveCount(1);
      const scan = await new AxeBuilder({ page: page as never }).analyze();
      expect(scan.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? '')).map((violation) => violation.id), `${route} in ${colorScheme} mode`).toEqual([]);
    }
  }
});

test('@claim:artifact-private generated reviewer evidence works from disk without network requests', async ({ page }) => {
  const output = mkdtempSync(join(tmpdir(), 'infra-test-evidence-artifact-'));
  const networkRequests: string[] = [];
  page.on('request', (request) => {
    if (/^https?:/.test(request.url())) networkRequests.push(request.url());
  });
  try {
    execFileSync('cargo', ['run', '--quiet', '--locked', '--', '--evidence-dir', output, 'examples/opentofu-real-stream.jsonl'], { encoding: 'utf8' });
    await page.goto(pathToFileURL(join(output, 'index.html')).href);
    await expect(page.locator('pre[tabindex="0"]')).toHaveCount(6);
    await page.locator('#provenance').focus();
    await expect(page.locator('#provenance')).toBeFocused();
    const scan = await new AxeBuilder({ page: page as never }).analyze();
    expect(scan.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? '')).map((violation) => violation.id)).toEqual([]);
    expect(networkRequests).toEqual([]);
  } finally {
    rmSync(output, { recursive: true, force: true });
  }
});
