import { execFileSync, spawnSync } from 'node:child_process';
import { chmodSync, existsSync, mkdtempSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { afterAll, beforeAll, describe, expect, it } from 'vitest';

const root = process.cwd();
let releaseRoot: string;
let releaseCli: string;

function hasBalancedXmlTags(xml: string): boolean {
  const tags = [...xml.matchAll(/<\/?([A-Za-z_:][\w:.-]*)(?:\s[^>]*)?\/?\s*>/g)].map((match) => match[0]);
  const stack: string[] = [];
  for (const tag of tags) {
    if (tag.startsWith('</')) {
      if (stack.pop() !== tag.slice(2, -1)) return false;
    } else if (!tag.endsWith('/>') && !tag.startsWith('<?') && !tag.startsWith('<!')) {
      const name = /^<([^\s/>]+)/.exec(tag)?.[1];
      if (name) stack.push(name);
    }
  }
  return stack.length === 0;
}

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

  it('@claim:resource-identifier-redaction removes AWS ARNs and EC2 identifiers from every release-package artifact', () => {
    const sandbox = mkdtempSync(join(tmpdir(), 'infra-test-evidence-resource-identifiers-'));
    try {
      for (const input of ['tests/fixtures/verification-9-resource-identifiers.jsonl', 'tests/fixtures/verification-9-legacy-identifiers.json']) {
        const output = mkdtempSync(join(sandbox, 'output-'));
        const junit = join(output, 'report.xml');
        const evidence = join(output, 'evidence');
        const result = spawnSync(releaseCli, ['--json', '--junit', junit, '--evidence-dir', evidence, input], { cwd: root, encoding: 'utf8' });
        expect(result.status, `${input}: ${result.stderr}`).toBe(0);
        expect(JSON.parse(result.stdout)).toEqual({ valid: true, checks: 1, errors: [] });
        const generated = [junit, ...readdirSync(evidence, { withFileTypes: true }).filter((entry) => entry.isFile()).map((entry) => join(evidence, entry.name))];
        for (const artifact of generated) {
          const contents = readFileSync(artifact, 'utf8');
          for (const identifier of ['arn:aws', 'i-0abc123', 'aws_instance.web']) expect(contents).not.toContain(identifier);
        }
        expect(readFileSync(join(evidence, 'evidence.json'), 'utf8')).toContain('[REDACTED]');
      }
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it('@claim:cross-provider-resource-redaction removes AWS network, Azure resource, and GCP instance identifiers from every release-package artifact', () => {
    const output = mkdtempSync(join(tmpdir(), 'infra-test-evidence-cross-provider-identifiers-'));
    const junit = join(output, 'report.xml');
    const evidence = join(output, 'evidence');
    const identifiers = [
      'subnet-0123456789abcdef0',
      'sg-0123456789abcdef0',
      '/subscriptions/11111111-2222-3333-4444-555555555555/resourceGroups/prod/providers/Microsoft.Compute/virtualMachines/api-01',
      'projects/acme-prod/zones/us-central1-a/instances/api-01',
    ];
    try {
      const result = spawnSync(releaseCli, ['--json', '--junit', junit, '--evidence-dir', evidence, 'tests/fixtures/verification-10-cross-provider-identifiers.jsonl'], { cwd: root, encoding: 'utf8' });
      expect(result.status, result.stderr).toBe(0);
      expect(JSON.parse(result.stdout)).toEqual({ valid: true, checks: 1, errors: [] });

      const artifacts = [junit, join(evidence, 'evidence.json'), join(evidence, 'index.html')];
      for (const artifact of artifacts) {
        const contents = readFileSync(artifact, 'utf8');
        for (const identifier of identifiers) expect(contents).not.toContain(identifier);
        expect(contents).toContain('[REDACTED]');
      }
    } finally {
      rmSync(output, { recursive: true, force: true });
    }
  });

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
      const artifact = JSON.parse(readFileSync(join(directory!, 'evidence/evidence.json'), 'utf8'));
      expect(artifact.testCases).toHaveLength(2);
      expect(artifact.assertionPaths).toEqual(['aws_security_group.web.ingress', 'aws_security_group.web.ingress']);
      expect(artifact.testCases.map((testCase: { assertionPaths: string[] }) => testCase.assertionPaths)).toEqual([['aws_security_group.web.ingress'], ['aws_security_group.web.ingress']]);
      expect(JSON.parse(readFileSync(join(root, 'public/demo-evidence.json'), 'utf8'))).toEqual(artifact);
      expect(readFileSync(join(directory!, 'evidence/index.html'), 'utf8')).toContain('Infrastructure test evidence');
      const recorded = readFileSync(join(root, 'public/cli-demo.cast'), 'utf8').trim().split('\n').slice(1)
        .map((line) => (JSON.parse(line) as [number, string, string])[2]).join('').replaceAll('\r\n', '\n').trim();
      const captured = `$ infra-test-evidence --demo\n${result.replaceAll(directory!, '/tmp/infra-test-evidence-demo-…')}`.trim();
      expect(recorded).toBe(captured);
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
      const junitXml = readFileSync(junit, 'utf8');
      expect(junitXml).toContain('<testsuite');
      expect(hasBalancedXmlTags(junitXml)).toBe(true);
      expect(junitXml).toContain('name="requires_production"');
      expect(junitXml).toContain('name="protects_sensitive_value"');
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

  it('@claim:malformed-duration-types rejects string and boolean duration values instead of treating them as missing', () => {
    const sandbox = mkdtempSync(join(tmpdir(), 'infra-test-evidence-duration-types-'));
    const booleanDuration = join(sandbox, 'duration-false.json');
    try {
      writeFileSync(booleanDuration, JSON.stringify({ run: 'r', environment: 'e', recordedAt: 'x', checks: [{ name: 'x', status: 'pass', durationMs: false }] }));
      for (const input of ['tests/fixtures/verification-9-duration-string.json', booleanDuration, 'tests/fixtures/verification-9-elapsed-string.jsonl']) {
        const result = spawnSync(releaseCli, ['--json', input], { cwd: root, encoding: 'utf8' });
        expect(result.status, input).toBe(2);
        const response = JSON.parse(result.stdout);
        expect(response).toMatchObject({ valid: false, checks: 0 });
        expect(response.errors.join('\n')).toContain('must be a non-negative finite number');
      }
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it('@claim:help-options lists every accepted CLI option and succeeds', () => {
    const result = spawnSync(releaseCli, ['--help'], { cwd: root, encoding: 'utf8' });
    expect(result.status).toBe(0);
    for (const option of ['--demo', '--json', '--junit <report.xml>', '--evidence-dir <dir>', '-h, --help']) expect(result.stdout).toContain(option);
  });

  it('@claim:json-validation-output prints one parseable validation shape for compact and event-stream inputs', () => {
    for (const input of ['examples/passing-evidence.json', 'examples/tofu-test.jsonl']) {
      const result = spawnSync(releaseCli, ['--json', input], { cwd: root, encoding: 'utf8' });
      expect(result.status, input).toBe(0);
      expect(JSON.parse(result.stdout), input).toEqual({ valid: true, checks: 2, errors: [] });
    }
  });

  it('@claim:event-stream-validation rejects incomplete, repeated, late, unsupported, and negative stream results', () => {
    const sandbox = mkdtempSync(join(tmpdir(), 'infra-test-evidence-stream-validation-'));
    const run = { type: 'test_run', '@testfile': 'tests/main.tftest.hcl', '@testrun': 'main', test_run: { status: 'pass', elapsed: 0.25 } };
    const summary = { type: 'test_summary', test_summary: { status: 'pass' } };
    const cases = [
      { name: 'missing-summary', events: [run], error: 'event stream ended without a final test_summary' },
      { name: 'repeated-summary', events: [run, summary, summary], error: 'event stream contains more than one test_summary' },
      { name: 'late-summary', events: [run, summary, { type: 'log', message: 'late output' }], error: 'test_summary must be the final event' },
      { name: 'unsupported-summary', events: [run, { type: 'test_summary', test_summary: { status: 'unknown' } }], error: 'test_summary has an unsupported status unknown' },
      { name: 'unsupported-run', events: [{ ...run, test_run: { status: 'unknown', elapsed: 0.25 } }, { ...summary, test_summary: { status: 'fail' } }], error: 'test_run has an unsupported status unknown' },
      { name: 'negative-duration', events: [{ ...run, test_run: { status: 'pass', elapsed: -0.25 } }, summary], error: 'test_run elapsed must be a non-negative finite number' },
      { name: 'string-duration', events: [{ ...run, test_run: { status: 'pass', elapsed: 'minus one' } }, summary], error: 'test_run elapsed must be a non-negative finite number' },
    ];
    try {
      for (const item of cases) {
        const input = join(sandbox, `${item.name}.jsonl`);
        const junit = join(sandbox, `${item.name}.xml`);
        const evidence = join(sandbox, `${item.name}-evidence`);
        writeFileSync(input, item.events.map((event) => JSON.stringify(event)).join('\n'));
        const result = spawnSync(releaseCli, ['--json', '--junit', junit, '--evidence-dir', evidence, input], { cwd: sandbox, encoding: 'utf8' });
        expect(result.status, item.name).toBe(2);
        expect(JSON.parse(result.stdout).errors.join('\n'), item.name).toContain(item.error);
        expect(existsSync(junit), item.name).toBe(false);
        expect(existsSync(evidence), item.name).toBe(false);
      }
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it('@claim:summary-consistency rejects a skipped summary after a failed completed test run', () => {
    const result = spawnSync(releaseCli, ['--json', 'tests/fixtures/verification-9-skipped-summary.jsonl'], { cwd: root, encoding: 'utf8' });
    expect(result.status).toBe(2);
    expect(JSON.parse(result.stdout)).toEqual({ valid: false, checks: 0, errors: ['test_summary status does not match completed test_run results'] });
  });

  it('@claim:run-correlation keeps interleaved plans and assertions with their test run', () => {
    const sandbox = mkdtempSync(join(tmpdir(), 'infra-test-evidence-correlation-'));
    const input = join(sandbox, 'interleaved.jsonl');
    const evidence = join(sandbox, 'evidence');
    const events = [
      { type: 'test_plan', '@testfile': 'tests/main.tftest.hcl', '@testrun': 'alpha', test_plan: { variables: { target: { value: 'alpha-input' } }, outputs: { result: { value: 'alpha-output' } }, resource_changes: [{ change: { actions: ['create'], after: { note: 'alpha-change' } } }] } },
      { type: 'test_plan', '@testfile': 'tests/main.tftest.hcl', '@testrun': 'beta', test_plan: { variables: { target: { value: 'beta-input' } }, outputs: { result: { value: 'beta-output' } }, resource_changes: [{ change: { actions: ['update'], after: { note: 'beta-change' } } }] } },
      { type: 'diagnostic', '@testfile': 'tests/main.tftest.hcl', '@testrun': 'beta', diagnostic: { detail: 'beta assertion failed', snippet: { values: [{ traversal: 'var.beta' }] } } },
      { type: 'diagnostic', '@testfile': 'tests/main.tftest.hcl', '@testrun': 'alpha', diagnostic: { detail: 'alpha assertion failed', snippet: { values: [{ traversal: 'var.alpha' }] } } },
      { type: 'test_run', '@testfile': 'tests/main.tftest.hcl', '@testrun': 'alpha', test_run: { status: 'fail', elapsed: 0.1 } },
      { type: 'test_run', '@testfile': 'tests/main.tftest.hcl', '@testrun': 'beta', test_run: { status: 'fail', elapsed: 0.2 } },
      { type: 'test_summary', test_summary: { status: 'fail' } },
    ];
    try {
      writeFileSync(input, events.map((event) => JSON.stringify(event)).join('\n'));
      const result = spawnSync(releaseCli, ['--json', '--evidence-dir', evidence, input], { cwd: sandbox, encoding: 'utf8' });
      expect(result.status, result.stderr).toBe(0);
      const artifact = JSON.parse(readFileSync(join(evidence, 'evidence.json'), 'utf8'));
      const alpha = artifact.testCases.find((testCase: { name: string }) => testCase.name === 'alpha');
      const beta = artifact.testCases.find((testCase: { name: string }) => testCase.name === 'beta');
      expect(JSON.stringify(alpha)).toContain('alpha-input');
      expect(JSON.stringify(alpha)).toContain('alpha-output');
      expect(JSON.stringify(alpha)).toContain('alpha-change');
      expect(alpha.assertionPaths).toEqual(['var.alpha']);
      expect(alpha.failure).toBe('alpha assertion failed');
      expect(JSON.stringify(alpha)).not.toContain('beta-');
      expect(JSON.stringify(beta)).toContain('beta-input');
      expect(JSON.stringify(beta)).toContain('beta-output');
      expect(JSON.stringify(beta)).toContain('beta-change');
      expect(beta.assertionPaths).toEqual(['var.beta']);
      expect(beta.failure).toBe('beta assertion failed');
      expect(JSON.stringify(beta)).not.toContain('alpha-');
      expect(readFileSync(join(evidence, 'index.html'), 'utf8')).toContain('planSummary:c.planSummary');
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it('@claim:conversion-only never launches infrastructure tools or opens a network socket', () => {
    if (process.platform !== 'linux') throw new Error('The conversion-only claim sandbox requires Linux syscall interposition.');
    const sandbox = mkdtempSync(join(tmpdir(), 'infra-test-evidence-isolation-'));
    const guard = join(sandbox, 'no-external-effects.so');
    const log = join(sandbox, 'blocked-effect.log');
    const fakeBin = join(sandbox, 'bin');
    const junit = join(sandbox, 'report.xml');
    const evidence = join(sandbox, 'evidence');
    try {
      execFileSync('cc', ['-shared', '-fPIC', '-Wall', '-Werror', '-o', guard, join(root, 'tests/fixtures/no-external-effects.c')], { cwd: root, encoding: 'utf8' });
      mkdirSync(fakeBin);
      for (const command of ['tofu', 'terraform']) {
        const executable = join(fakeBin, command);
        writeFileSync(executable, '#!/bin/sh\nprintf "%s\\n" "infrastructure command launched" >> "$ITE_SIDE_EFFECT_LOG"\nexit 191\n');
        chmodSync(executable, 0o755);
      }

      const result = spawnSync(releaseCli, ['--json', '--junit', junit, '--evidence-dir', evidence, 'examples/opentofu-real-stream.jsonl'], {
        cwd: root,
        encoding: 'utf8',
        env: {
          ...process.env,
          PATH: fakeBin,
          LD_PRELOAD: guard,
          ITE_SIDE_EFFECT_LOG: log,
        },
      });

      expect(result.status, result.stderr).toBe(0);
      expect(JSON.parse(result.stdout)).toEqual({ valid: true, checks: 2, errors: [] });
      expect(existsSync(junit)).toBe(true);
      expect(existsSync(join(evidence, 'evidence.json'))).toBe(true);
      expect(existsSync(join(evidence, 'index.html'))).toBe(true);
      expect(existsSync(log)).toBe(false);
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it('@claim:requested-path-writes writes conversion artifacts only to named paths', () => {
    if (process.platform !== 'linux') throw new Error('The requested-path claim sandbox requires Linux syscall interposition.');
    const sandbox = mkdtempSync(join(tmpdir(), 'infra-test-evidence-requested-paths-'));
    const guard = join(sandbox, 'requested-writes-only.so');
    const input = join(sandbox, 'input.jsonl');
    const junit = join(sandbox, 'report.xml');
    const evidence = join(sandbox, 'evidence');
    try {
      writeFileSync(input, readFileSync(join(root, 'examples/tofu-test.jsonl'), 'utf8'));
      execFileSync('cc', ['-shared', '-fPIC', '-Wall', '-Werror', '-o', guard, join(root, 'tests/fixtures/requested-writes-only.c'), '-ldl'], { cwd: root, encoding: 'utf8' });
      const result = spawnSync(releaseCli, ['--json', '--junit', junit, '--evidence-dir', evidence, input], {
        cwd: sandbox,
        encoding: 'utf8',
        env: {
          ...process.env,
          TMPDIR: sandbox,
          LD_PRELOAD: guard,
          ITE_ALLOWED_JUNIT: junit,
          ITE_ALLOWED_EVIDENCE_DIR: evidence,
        },
      });
      expect(result.status, result.stderr).toBe(0);
      expect(result.stderr).toBe('');
      expect(JSON.parse(result.stdout)).toEqual({ valid: true, checks: 2, errors: [] });
      expect(readdirSync(sandbox).sort()).toEqual(['evidence', 'input.jsonl', 'report.xml', 'requested-writes-only.so']);
      expect(readdirSync(evidence).sort()).toEqual(['evidence.json', 'index.html']);
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  it('fails closed for an unreadable input', () => {
    expect(() => execFileSync('cargo', ['run', '--quiet', '--locked', '--', '--json', 'examples/does-not-exist.json'], { cwd: root, encoding: 'utf8' })).toThrow();
  });

  it('@claim:sensitive-diagnostics keeps sensitive diagnostics out of every packaged reviewer artifact', () => {
    const output = mkdtempSync(join(tmpdir(), 'infra-test-evidence-real-'));
    const evidence = join(output, 'evidence');
    try {
      execFileSync(releaseCli, ['--evidence-dir', evidence, 'examples/opentofu-real-stream.jsonl'], { cwd: root, encoding: 'utf8' });
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
