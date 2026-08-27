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
});
