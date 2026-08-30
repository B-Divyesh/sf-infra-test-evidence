import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

type Claim = { id: string; claim: string; where: string; test: string; sandbox: string };

describe('release claims contract', () => {
  it('lists every claim with one tagged regression test', () => {
    const claims = JSON.parse(readFileSync('.factory/claims.json', 'utf8')) as Claim[];
    const testSource = readdirSync('tests')
      .filter((file) => file.endsWith('.ts'))
      .map((file) => readFileSync(join('tests', file), 'utf8'))
      .join('\n');

    expect(claims.length).toBeGreaterThan(0);
    expect(new Set(claims.map(({ id }) => id)).size).toBe(claims.length);
    for (const claim of claims) {
      expect(claim.id).toMatch(/^[a-z0-9]+(?:-[a-z0-9]+)*$/);
      expect(claim.claim.trim()).not.toBe('');
      expect(claim.where.trim()).not.toBe('');
      expect(claim.sandbox.trim()).not.toBe('');
      expect(claim.test).toContain(`@claim:${claim.id}`);
      expect(testSource.split(`@claim:${claim.id}`).length - 1).toBe(1);
    }
  });

  it('publishes exact static routes for the demo and 404 response', () => {
    const config = JSON.parse(readFileSync('public/staticwebapp.config.json', 'utf8')) as {
      routes: Array<{ route: string; redirect?: string; statusCode?: number }>;
      responseOverrides: Record<string, { rewrite: string; statusCode: number }>;
    };
    expect(config.routes).toContainEqual({ route: '/demo', redirect: '/demo/', statusCode: 301 });
    expect(config.responseOverrides['404']).toEqual({ rewrite: '/404.html', statusCode: 404 });
    expect(readFileSync('demo/index.html', 'utf8')).toContain('<title>Demo — Infra Test Evidence</title>');
    expect(readFileSync('404.html', 'utf8')).toContain('<title>Page not found — Infra Test Evidence</title>');
  });

  it('@claim:mit-license keeps package metadata and the shipped license on MIT', () => {
    const cargo = readFileSync('Cargo.toml', 'utf8');
    const license = readFileSync('LICENSE', 'utf8');
    expect(cargo).toMatch(/^license = "MIT"$/m);
    expect(license).toContain('Permission is hereby granted, free of charge');
    expect(license).toContain('THE SOFTWARE IS PROVIDED "AS IS"');
  });
});
