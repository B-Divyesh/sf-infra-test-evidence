import { expect, test } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const origin = 'http://127.0.0.1:4173';

test('@claim:site-demo opens, resets, and exits the in-memory sample from the first screen', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');
  await expect(page.getByText('For infrastructure-module maintainers')).toBeVisible();
  await page.getByRole('link', { name: 'Try it with sample data' }).first().click();
  await expect(page).toHaveURL(/\/demo\/\?demo=1$/);
  await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible();
  await expect(page.getByText('2 checks ready')).toBeVisible();
  const proof = page.getByLabel('Failed OpenTofu test ready for review');
  await expect(proof.getByText('blocks_public_ingress', { exact: true })).toBeVisible();
  await expect(proof.getByText('aws_security_group.web.ingress', { exact: true })).toBeVisible();
  await expect(proof.getByText('[REDACTED]', { exact: true })).toBeVisible();
  await expect(proof.getByText('report.xml', { exact: true })).toBeVisible();
  await expect(proof.getByText(/SHA-256 926eb21/)).toBeVisible();
  await expect(page.getByRole('heading', { name: 'Recorded checks' })).toBeVisible();
  for (const viewport of [{ width: 390, height: 844 }, { width: 1440, height: 900 }]) {
    await page.setViewportSize(viewport);
    for (const text of ['blocks_public_ingress', '[REDACTED]', 'report.xml']) {
      const box = await proof.getByText(text, { exact: true }).first().boundingBox();
      expect(box, `${text} has a bounding box`).not.toBeNull();
      expect(box!.y + box!.height, `${text} is in the ${viewport.width}px first viewport`).toBeLessThanOrEqual(viewport.height);
    }
  }

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
  await expect(proof.getByText('blocks_public_ingress', { exact: true })).toBeVisible();
  await expect(page.getByRole('alert')).toBeHidden();
  await page.getByRole('link', { name: 'Start for real' }).click();
  await expect(page).toHaveURL(/\/$/);
  await expect(page.getByText('Demo — sample data, nothing is saved')).toHaveCount(0);
});

test('@claim:browser-record-import renders compact record details and validation errors', async ({ page }) => {
  await page.goto('/');
  await page.locator('#evidence-file').setInputFiles({
    name: 'passing-evidence.json',
    mimeType: 'application/json',
    buffer: readFileSync('examples/passing-evidence.json'),
  });
  await expect(page.getByText('2 checks ready')).toBeVisible();
  await expect(page.getByText('staging-2026-08-27.1', { exact: true })).toBeVisible();
  await expect(page.getByText('staging', { exact: true })).toBeVisible();
  await expect(page.getByText('HTTP health endpoint')).toBeVisible();
  await expect(page.getByText('Database migration')).toBeVisible();
  await expect(page.getByText('pass', { exact: true })).toHaveCount(2);
  await page.locator('#evidence-file').setInputFiles({
    name: 'invalid-compact-record.json',
    mimeType: 'application/json',
    buffer: Buffer.from(JSON.stringify({ run: '', environment: 'staging', recordedAt: '', checks: [{ name: '', status: 'unknown', durationMs: -1 }] })),
  });
  await expect(page.getByText('Make this record reviewable')).toBeVisible();
  await expect(page.getByText('Add a non-empty “run” field.')).toBeVisible();
  await expect(page.getByText('Check 1 needs a supported status.')).toBeVisible();
});

test('keeps the landing action and all three product facts in a 1440px first viewport', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto('/');
  await expect(page.getByRole('link', { name: 'Try it with sample data' })).toBeVisible();
  for (const fact of ['Runs in your browser', 'No trackers or uploads', 'Free under the MIT License']) {
    const box = await page.getByText(fact, { exact: true }).boundingBox();
    expect(box, `${fact} has a bounding box`).not.toBeNull();
    expect(box!.y + box!.height, `${fact} is in the desktop first viewport`).toBeLessThanOrEqual(900);
  }
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

test('@claim:cli-recording plays the packaged CLI recording with a transcript and reduced-motion fallback', async ({ page, request }) => {
  const recording = await request.get('/cli-demo.cast');
  expect(recording.status()).toBe(200);
  expect(await recording.text()).toContain('Demo complete: 2 checks converted');

  await page.goto('/');
  const figure = page.getByRole('figure', { name: /See the CLI create JUnit, JSON, and HTML/ });
  const output = page.locator('#recording-output');
  const transcript = page.locator('.recording-transcript');
  await expect(figure).toBeVisible();
  await expect(page.getByRole('button', { name: 'Replay recording' })).toBeVisible({ timeout: 5_000 });
  await expect(output).toContainText('JUnit report:');
  await expect(output).toContainText('Evidence JSON:');
  await expect(output).toContainText('Reviewer page:');
  await transcript.getByText('Read the recording transcript').click();
  await expect(transcript).toContainText('evidence/evidence.json');

  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.reload();
  await expect(page.getByRole('button', { name: 'Show recording without motion' })).toBeVisible();
  await expect(page.locator('#recording-status')).toHaveText(/complete recording is shown without animation/);
  await expect(output).toContainText('Demo complete: 2 checks converted');
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
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('Review sample test evidence');
  await page.goto('/privacy/');
  await expect(page).toHaveTitle('Privacy — Infra Test Evidence');
  await expect(page.getByRole('heading', { name: 'Privacy', exact: true })).toBeVisible();
  await page.goto('/terms/');
  await expect(page).toHaveTitle('Terms — Infra Test Evidence');
  await expect(page.getByRole('heading', { name: 'Terms', exact: true })).toBeVisible();
  await page.goto('/404.html');
  await expect(page).toHaveTitle('Page not found — Infra Test Evidence');
  await expect(page.getByRole('heading', { level: 1 })).toHaveText('This evidence page was not found');
  await expect(page.locator('link[rel="canonical"]')).toHaveAttribute('href', 'https://infra-test-evidence.sociobot.in/404.html');
  await expect(page.locator('meta[property="og:title"]')).toHaveAttribute('content', 'Page not found — Infra Test Evidence');
  await expect(page.locator('meta[name="twitter:card"]')).toHaveAttribute('content', 'summary_large_image');
  expect(errors).toEqual([]);
});

test('moves focus and announces the destination after internal forward and back navigation', async ({ page }) => {
  await page.goto('/');
  await page.getByRole('link', { name: 'Demo' }).click();
  await expect(page.getByRole('heading', { level: 1 })).toBeFocused();
  await expect(page.locator('#route-announcement')).toHaveText('Demo — Infra Test Evidence');
  await page.goBack();
  await expect(page.getByRole('heading', { level: 1 })).toBeFocused();
  await expect(page.locator('#route-announcement')).toHaveText('Infra Test Evidence — review test runs locally');
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
      expect(scan.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? '')).map((violation) => `${violation.id}: ${violation.nodes.map((node) => node.target.join(' ')).join(', ')}`), `${route} in ${colorScheme} mode`).toEqual([]);
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
