import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, readdirSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const root = process.cwd();

describe('release CLI conversion', () => {
  it('keeps explicitly sensitive opaque values out of every release-package artifact', () => {
    const output = mkdtempSync(join(tmpdir(), 'infra-test-evidence-package-'));
    const consumer = join(output, 'consumer');
    const junit = join(output, 'report.xml');
    const evidence = join(output, 'evidence');
    const sentinel = 'k9M2qV7xL4';
    try {
      execFileSync('cargo', ['package', '--locked', '--allow-dirty'], { cwd: root, encoding: 'utf8' });
      execFileSync('cargo', ['install', '--path', join(root, 'target/package/infra-test-evidence-0.1.0'), '--root', consumer, '--locked'], { cwd: root, encoding: 'utf8' });
      const result = execFileSync(join(consumer, 'bin/infra-test-evidence'), ['--json', '--junit', junit, '--evidence-dir', evidence, 'examples/explicit-sensitive-output.jsonl'], { cwd: root, encoding: 'utf8' });
      expect(JSON.parse(result)).toEqual({ valid: true, checks: 1, errors: [] });

      const generated = [junit, ...readdirSync(evidence, { withFileTypes: true }).filter((entry) => entry.isFile()).map((entry) => join(evidence, entry.name))];
      expect(generated).toHaveLength(3);
      for (const artifact of generated) expect(readFileSync(artifact, 'utf8')).not.toContain(sentinel);
      expect(readFileSync(join(evidence, 'evidence.json'), 'utf8')).toContain('[REDACTED]');
    } finally {
      rmSync(output, { recursive: true, force: true });
    }
  }, 60_000);

  it('writes JUnit and a redacted, self-contained reviewer artifact', () => {
    const output = mkdtempSync(join(tmpdir(), 'infra-test-evidence-'));
    const junit = join(output, 'report.xml');
    const evidence = join(output, 'evidence');
    try {
      const result = execFileSync('cargo', ['run', '--quiet', '--locked', '--', '--json', '--junit', junit, '--evidence-dir', evidence, 'examples/tofu-test.jsonl'], { cwd: root, encoding: 'utf8' });
      expect(JSON.parse(result)).toEqual({ valid: true, checks: 2, errors: [] });
      expect(readFileSync(junit, 'utf8')).toContain('<testsuite');
      const artifact = readFileSync(join(evidence, 'evidence.json'), 'utf8');
      expect(artifact).toContain('resource identifier redacted');
      expect(artifact).not.toContain('never-export-this');
      expect(readFileSync(join(evidence, 'index.html'), 'utf8')).toContain('Test-case inputs');
    } finally {
      rmSync(output, { recursive: true, force: true });
    }
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
