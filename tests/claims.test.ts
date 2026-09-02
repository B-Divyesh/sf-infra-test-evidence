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

  it('builds the production site before every browser claim run', () => {
    const config = readFileSync('playwright.config.ts', 'utf8');
    expect(config).toContain("command: 'npm run build:site && npm run preview'");
  });

  it('ships an asciinema recording with the packaged demo output contract', () => {
    const recording = readFileSync('public/cli-demo.cast', 'utf8').trim().split('\n');
    const header = JSON.parse(recording[0]) as { version: number; title: string };
    const transcript = recording.slice(1).map((line) => (JSON.parse(line) as [number, string, string])[2]).join('');
    expect(header).toEqual(expect.objectContaining({ version: 2, title: 'Packaged infra-test-evidence 0.1.0 --demo' }));
    expect(transcript).toContain('$ infra-test-evidence --demo');
    expect(transcript).toContain('Demo complete: 2 checks converted');
    expect(transcript).toContain('JUnit report:');
    expect(transcript).toContain('/report.xml');
    expect(transcript).toContain('Reviewer page:');
    expect(transcript).toContain('/evidence/index.html');
    expect(transcript).toContain('Evidence JSON:');
    expect(transcript).toContain('/evidence/evidence.json');
  });

  it('keeps reviewed copy specific, short, and consistent about compact records and output files', () => {
    const landing = readFileSync('index.html', 'utf8');
    const readme = readFileSync('README.md', 'utf8');
    const errorPage = readFileSync('404.html', 'utf8');
    expect(landing).toContain('See the CLI create JUnit, JSON, and HTML');
    expect(landing).toContain('Generated files');
    expect(landing).toContain('three output files: a JUnit report, evidence JSON, and a reviewer page');
    expect(landing).toContain('Create a JUnit report, evidence JSON, and a reviewer page.');
    expect(landing).toContain('compact record');
    expect(landing).toContain('The local reader also opens the compact JSON record shown below:');
    expect(landing).not.toContain('compact form');
    expect(landing).not.toContain('compact portable record');
    expect(landing).not.toContain('earlier workflows');
    expect(landing).not.toContain('reviewer JSON');
    expect(landing).not.toContain('static HTML page');
    expect(landing).not.toContain('Small by design');
    expect(readme).toContain('`report.xml` contains the converted checks in JUnit XML.');
    expect(readme).toContain('It also redacts values marked by');
    expect(readme).toContain('The CLI also redacts values in fields named `id`, `id_*`, `*_id`, `identifier`,');
    expect(readme).toContain('The reviewer page records each test’s inputs,');
    expect(readme).toContain('assertion path, failure, and redacted plan changes. It also records the input');
    expect(readme).not.toContain('assertion paths where emitted by the runner');
    expect(readme).not.toContain('source provenance');
    expect(readme).toContain('The converter rejects an event stream without one final supported summary.');
    expect(readme).toMatch(/This keeps unmarked values in the same\s+diagnostic out of every output file\./);
    expect(readme).not.toContain('restrictive browser response policies');
    expect(readme).not.toContain('portable workflows');
    expect(errorPage).toContain('404 · PAGE NOT FOUND');
    expect(errorPage).not.toContain('NOT IN THE LEDGER');
  });

  it('@claim:mit-license keeps package metadata and the shipped license on MIT', () => {
    const cargo = readFileSync('Cargo.toml', 'utf8');
    const license = readFileSync('LICENSE', 'utf8');
    expect(cargo).toMatch(/^license = "MIT"$/m);
    expect(license).toContain('Permission is hereby granted, free of charge');
    expect(license).toContain('THE SOFTWARE IS PROVIDED "AS IS"');
  });
});
