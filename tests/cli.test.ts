import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { afterAll, beforeAll, describe, expect, it } from 'vitest';

const root = process.cwd();
let releaseRoot: string;
let releaseCli: string;

describe('release CLI conversion', () => {
  beforeAll(() => {
    releaseRoot = mkdtempSync(join(tmpdir(), 'infra-test-evidence-release-'));
    execFileSync('cargo', ['package', '--locked', '--allow-dirty'], { cwd: root, encoding: 'utf8' });
    execFileSync('cargo', ['install', '--path', join(root, 'target/package/infra-test-evidence-0.1.0'), '--root', releaseRoot, '--locked'], { cwd: root, encoding: 'utf8' });
    releaseCli = join(releaseRoot, 'bin/infra-test-evidence');
  }, 60_000);

  afterAll(() => rmSync(releaseRoot, { recursive: true, force: true }));

  it('@claim:sensitive-redaction keeps opaque sensitive values out of every release-package artifact', () => {
    const output = mkdtempSync(join(tmpdir(), 'infra-test-evidence-package-'));
    const junit = join(output, 'report.xml');
    const evidence = join(output, 'evidence');
    const sentinel = 'k9M2qV7xL4';
    try {
      const result = execFileSync(releaseCli, ['--json', '--junit', junit, '--evidence-dir', evidence, 'examples/explicit-sensitive-output.jsonl'], { cwd: root, encoding: 'utf8' });
      expect(JSON.parse(result)).toEqual({ valid: true, checks: 1, errors: [] });

      const generated = [junit, ...readdirSync(evidence, { withFileTypes: true }).filter((entry) => entry.isFile()).map((entry) => join(evidence, entry.name))];
      expect(generated).toHaveLength(3);
      for (const artifact of generated) expect(readFileSync(artifact, 'utf8')).not.toContain(sentinel);
      expect(readFileSync(join(evidence, 'evidence.json'), 'utf8')).toContain('[REDACTED]');
    } finally {
      rmSync(output, { recursive: true, force: true });
    }
  }, 60_000);

  it('@claim:cli-demo runs bundled data from the packaged binary in an isolated temporary directory', () => {
    const temporaryRoot = mkdtempSync(join(tmpdir(), 'infra-test-evidence-demo-test-'));
    try {
      const result = execFileSync(releaseCli, ['--demo'], { cwd: temporaryRoot, encoding: 'utf8', env: { ...process.env, TMPDIR: temporaryRoot } });
      const directory = result.match(/^Demo directory: (.+)$/m)?.[1];
      expect(result).toContain('Demo complete: 2 checks converted');
      expect(directory).toBeTruthy();
      expect(directory!.startsWith(temporaryRoot)).toBe(true);
      expect(result).toContain(`Sample input: ${join(directory!, 'tofu-test.jsonl')}`);
      expect(result).toContain(`JUnit report: ${join(directory!, 'report.xml')}`);
      expect(result).toContain(`Reviewer page: ${join(directory!, 'evidence/index.html')}`);
      expect(result).toContain(`Evidence JSON: ${join(directory!, 'evidence/evidence.json')}`);
      expect(readFileSync(join(directory!, 'report.xml'), 'utf8')).toContain('<testsuite');
      expect(JSON.parse(readFileSync(join(directory!, 'evidence/evidence.json'), 'utf8')).testCases).toHaveLength(2);
      expect(readFileSync(join(directory!, 'evidence/index.html'), 'utf8')).toContain('Infrastructure test evidence');
    } finally {
      rmSync(temporaryRoot, { recursive: true, force: true });
    }
  });

  it('@claim:cli-conversion writes JUnit and complete reviewer evidence', () => {
    const output = mkdtempSync(join(tmpdir(), 'infra-test-evidence-'));
    const junit = join(output, 'report.xml');
    const evidence = join(output, 'evidence');
    try {
      const result = execFileSync('cargo', ['run', '--quiet', '--locked', '--', '--json', '--junit', junit, '--evidence-dir', evidence, 'examples/opentofu-real-stream.jsonl'], { cwd: root, encoding: 'utf8' });
      expect(JSON.parse(result)).toEqual({ valid: true, checks: 2, errors: [] });
      expect(readFileSync(junit, 'utf8')).toContain('<testsuite');
      const artifactText = readFileSync(join(evidence, 'evidence.json'), 'utf8');
      const artifact = JSON.parse(artifactText);
      expect(artifact.provenance.sourceKind).toBe('terraform-test-json');
      expect(artifact.provenance.inputSha256).toMatch(/^[a-f0-9]{64}$/);
      expect(artifact.testCases).toHaveLength(2);
      expect(artifact.testCases[0].inputs.length).toBeGreaterThan(0);
      expect(artifact.assertionPaths.length).toBeGreaterThan(0);
      expect(artifact.planSummary.join('\n')).toContain('resource identifier redacted');
      expect(artifactText).not.toContain('never-export-this');
      expect(readFileSync(join(evidence, 'index.html'), 'utf8')).toContain('Test-case inputs');
    } finally {
      rmSync(output, { recursive: true, force: true });
    }
  });

  it('@claim:sensitivity-fail-closed rejects malformed sensitivity metadata before writing artifacts', () => {
    const output = mkdtempSync(join(tmpdir(), 'infra-test-evidence-malformed-'));
    const input = join(output, 'malformed.jsonl');
    const junit = join(output, 'report.xml');
    const evidence = join(output, 'evidence');
    try {
      writeFileSync(input, readFileSync(join(root, 'examples/explicit-sensitive-output.jsonl'), 'utf8').replace('"sensitive":true', '"sensitive":"unknown"'));
      const result = spawnSync(releaseCli, ['--junit', junit, '--evidence-dir', evidence, input], { cwd: root, encoding: 'utf8' });
      expect(result.status).toBe(2);
      expect(result.stderr).toContain('cannot safely interpret explicit sensitive marker');
      expect(existsSync(junit)).toBe(false);
      expect(existsSync(evidence)).toBe(false);
    } finally {
      rmSync(output, { recursive: true, force: true });
    }
  });

  it('@claim:strict-validation validates compact records and returns documented exit codes', () => {
    const valid = spawnSync(releaseCli, ['--json', 'examples/passing-evidence.json'], { cwd: root, encoding: 'utf8' });
    expect(valid.status).toBe(0);
    expect(JSON.parse(valid.stdout)).toEqual({ valid: true, checks: 2, errors: [] });

    const invalid = spawnSync(releaseCli, ['--json', 'examples/does-not-exist.json'], { cwd: root, encoding: 'utf8' });
    expect(invalid.status).toBe(2);
    expect(JSON.parse(invalid.stdout).valid).toBe(false);

    const usage = spawnSync(releaseCli, ['--demo', '--json'], { cwd: root, encoding: 'utf8' });
    expect(usage.status).toBe(64);
    expect(usage.stderr).toContain('--demo cannot be combined');
  });

  it('fails closed for an unreadable input', () => {
    expect(() => execFileSync('cargo', ['run', '--quiet', '--locked', '--', '--json', 'examples/does-not-exist.json'], { cwd: root, encoding: 'utf8' })).toThrow();
  });

  it('keeps real-style sensitive diagnostics out of every reviewer artifact', () => {
    const output = mkdtempSync(join(tmpdir(), 'infra-test-evidence-real-'));
    const evidence = join(output, 'evidence');
    try {
      execFileSync('cargo', ['run', '--quiet', '--locked', '--', '--evidence-dir', evidence, 'examples/opentofu-real-stream.jsonl'], { cwd: root, encoding: 'utf8' });
      const artifact = readFileSync(join(evidence, 'evidence.json'), 'utf8');
      const page = readFileSync(join(evidence, 'index.html'), 'utf8');
      expect(artifact).not.toContain('s3cr3t-sentinel');
      expect(page).not.toContain('s3cr3t-sentinel');
      expect(artifact).toContain('[REDACTED SENSITIVE DIAGNOSTIC]');
      expect(artifact).toContain('var.environment');
    } finally {
      rmSync(output, { recursive: true, force: true });
    }
  });

  it('rejects incomplete output options with a usage error', () => {
    try {
      execFileSync('cargo', ['run', '--quiet', '--locked', '--', '--junit'], { cwd: root, encoding: 'utf8', stdio: 'pipe' });
      throw new Error('expected usage failure');
    } catch (error) {
      expect((error as { status?: number }).status).toBe(64);
    }
  });
});
