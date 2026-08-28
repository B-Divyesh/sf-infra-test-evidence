import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';

const root = process.cwd();

describe('release CLI conversion', () => {
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

  it('rejects incomplete output options with a usage error', () => {
    try {
      execFileSync('cargo', ['run', '--quiet', '--locked', '--', '--junit'], { cwd: root, encoding: 'utf8', stdio: 'pipe' });
      throw new Error('expected usage failure');
    } catch (error) {
      expect((error as { status?: number }).status).toBe(64);
    }
  });
});
