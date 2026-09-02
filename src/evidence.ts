export type Check = { name?: unknown; status?: unknown; durationMs?: unknown };
export type Evidence = { run?: unknown; environment?: unknown; recordedAt?: unknown; checks?: unknown };
export type Validation = { valid: boolean; errors: string[]; checks: Check[] };

export function validateEvidence(input: unknown): Validation {
  if (!input || typeof input !== 'object' || Array.isArray(input)) return { valid: false, errors: ['The file must contain one JSON object.'], checks: [] };
  const record = input as Evidence;
  const errors: string[] = [];
  for (const field of ['run', 'environment', 'recordedAt'] as const) {
    if (typeof record[field] !== 'string' || record[field].trim() === '') errors.push('Add a non-empty “' + field + '” field.');
  }
  const rawChecks = Array.isArray(record.checks) ? record.checks : [];
  if (!rawChecks.length) errors.push('Add at least one object to “checks”.');
  const checks: Check[] = [];
  const supported = new Set(['pass', 'fail', 'error', 'skip']);
  rawChecks.forEach((item, index) => {
    const number = index + 1;
    if (!item || typeof item !== 'object' || Array.isArray(item)) {
      errors.push('Check ' + number + ' must be an object.');
      return;
    }
    const check = item as Check;
    checks.push(check);
    if (typeof check.name !== 'string' || !check.name.trim()) errors.push('Check ' + number + ' needs a name.');
    if (typeof check.status !== 'string' || !supported.has(check.status.trim().toLowerCase())) errors.push('Check ' + number + ' needs a supported status.');
    if (check.durationMs !== undefined && (typeof check.durationMs !== 'number' || !Number.isFinite(check.durationMs) || check.durationMs < 0)) errors.push('Check ' + number + ' needs a non-negative duration.');
  });
  return { valid: errors.length === 0, errors, checks };
}
