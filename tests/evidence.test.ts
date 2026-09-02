import { describe, expect, it } from 'vitest';
import { validateEvidence } from '../src/evidence.ts';

describe('validateEvidence', () => {
  it('accepts the documented evidence shape', () => {
    expect(validateEvidence({ run: 'r-1', environment: 'test', recordedAt: '2026-08-27T12:00:00Z', checks: [{ name: 'health', status: 'pass' }] })).toEqual({ valid: true, errors: [], checks: [{ name: 'health', status: 'pass' }] });
  });
  it('makes missing reviewer fields actionable', () => {
    const result = validateEvidence({ checks: [] });
    expect(result.valid).toBe(false);
    expect(result.errors).toContain('Add a non-empty “run” field.');
    expect(result.errors).toContain('Add at least one object to “checks”.');
  });
  it('rejects unsupported statuses, malformed check entries, and every malformed duration type like the CLI', () => {
    const result = validateEvidence({ run: 'r', environment: 'test', recordedAt: '2026-08-27T12:00:00Z', checks: [{ name: 'health', status: 'mystery', durationMs: -1 }] });
    expect(result.valid).toBe(false);
    expect(result.errors).toContain('Check 1 needs a supported status.');
    expect(result.errors).toContain('Check 1 needs a non-negative duration.');

    for (const durationMs of ['fast', false, null]) {
      const malformed = validateEvidence({ run: 'r', environment: 'test', recordedAt: '2026-08-27T12:00:00Z', checks: [{ name: 'health', status: 'pass', durationMs }] });
      expect(malformed.valid).toBe(false);
      expect(malformed.errors).toContain('Check 1 needs a non-negative duration.');
    }

    const mixed = validateEvidence({ run: 'r', environment: 'test', recordedAt: '2026-08-27T12:00:00Z', checks: [{ name: 'health', status: 'pass' }, 'discarded before this repair'] });
    expect(mixed.valid).toBe(false);
    expect(mixed.errors).toContain('Check 2 must be an object.');
    expect(mixed.checks).toHaveLength(1);
  });
});
