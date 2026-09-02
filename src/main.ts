import './style.css';
import { validateEvidence, type Check, type Evidence } from './evidence.ts';

const sample: Evidence = { run: 'staging-2026-08-27.1', environment: 'staging', recordedAt: '2026-08-27T12:00:00Z', checks: [{ name: 'HTTP health endpoint', status: 'pass', durationMs: 148 }, { name: 'Database migration', status: 'pass', durationMs: 734 }] };
const fileInput = document.querySelector<HTMLInputElement>('#evidence-file')!;
const result = document.querySelector<HTMLElement>('#result')!;
const error = document.querySelector<HTMLElement>('#file-error')!;
const status = document.querySelector<HTMLOutputElement>('#status')!;
const dropZone = document.querySelector<HTMLElement>('.drop-zone')!;
const resetDemo = document.querySelector<HTMLButtonElement>('#reset-demo');
const recordingOutput = document.querySelector<HTMLElement>('#recording-output code');
const recordingControl = document.querySelector<HTMLButtonElement>('#recording-control');
const recordingStatus = document.querySelector<HTMLElement>('#recording-status');

function text(value: unknown, fallback = 'Not recorded'): string { return typeof value === 'string' && value.trim() ? value : fallback; }
function escapeHtml(value: string): string { const node = document.createElement('span'); node.textContent = value; return node.innerHTML; }
function render(record: Evidence): void {
  const checked = validateEvidence(record);
  error.hidden = true;
  status.textContent = checked.valid ? `${checked.checks.length} checks ready` : 'Needs attention';
  status.className = `status ${checked.valid ? 'status-ok' : 'status-error'}`;
  const rows = checked.checks.map((check: Check) => `<li><span>${escapeHtml(text(check.name, 'Unnamed check'))}</span><b class="check-${escapeHtml(text(check.status, 'unknown').toLowerCase())}">${escapeHtml(text(check.status, 'unknown'))}</b><small>${typeof check.durationMs === 'number' ? `${check.durationMs} ms` : 'No duration'}</small></li>`).join('');
  result.className = 'result';
  result.innerHTML = `<div class="record-summary"><span>Run <strong>${escapeHtml(text(record.run))}</strong></span><span>Environment <strong>${escapeHtml(text(record.environment))}</strong></span><span>Recorded <strong>${escapeHtml(text(record.recordedAt))}</strong></span></div>${checked.valid ? `<h3>Recorded checks</h3><ul class="checks">${rows}</ul>` : `<h3>Make this record reviewable</h3><ul class="problems">${checked.errors.map((issue) => `<li>${escapeHtml(issue)}</li>`).join('')}</ul>`}`;
}
function showFileError(message: string): void { error.textContent = message; error.hidden = false; status.textContent = 'Could not read file'; status.className = 'status status-error'; }
async function readFile(file: File): Promise<void> { try { render(JSON.parse(await file.text()) as Evidence); } catch { showFileError('That file is not valid JSON. Choose an exported evidence record or correct the file and try again.'); } }
fileInput.addEventListener('change', () => { const file = fileInput.files?.[0]; if (file) void readFile(file); });
dropZone.addEventListener('dragover', (event) => { event.preventDefault(); dropZone.classList.add('dragging'); });
dropZone.addEventListener('dragleave', () => dropZone.classList.remove('dragging'));
dropZone.addEventListener('drop', (event) => { event.preventDefault(); dropZone.classList.remove('dragging'); const file = event.dataTransfer?.files[0]; if (file) void readFile(file); });
resetDemo?.addEventListener('click', () => render(sample));
if (document.body.dataset.mode === 'demo') render(sample);

type CastEvent = [number, 'o', string];

async function prepareRecording(): Promise<void> {
  if (!recordingOutput || !recordingControl || !recordingStatus) return;
  try {
    const response = await fetch('/cli-demo.cast');
    if (!response.ok) throw new Error('recording unavailable');
    const lines = (await response.text()).trim().split('\n');
    const events = lines.slice(1).map((line) => JSON.parse(line) as CastEvent).filter((event) => event[1] === 'o');
    const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)');
    let position = 0;
    let timer: number | undefined;
    let playing = false;

    const finish = (message: string): void => {
      recordingOutput.textContent = events.map((event) => event[2]).join('');
      position = events.length;
      playing = false;
      recordingControl.textContent = reducedMotion.matches ? 'Show recording without motion' : 'Replay recording';
      recordingStatus.textContent = message;
    };
    const schedule = (): void => {
      if (!playing) return;
      if (position >= events.length) {
        finish('Recording complete. JUnit, reviewer HTML, and evidence JSON paths are shown.');
        return;
      }
      const previousTime = position === 0 ? 0 : events[position - 1][0];
      const delay = Math.max(80, (events[position][0] - previousTime) * 700);
      timer = window.setTimeout(() => {
        recordingOutput.textContent += events[position][2];
        position += 1;
        schedule();
      }, delay);
    };
    const start = (): void => {
      if (reducedMotion.matches) {
        finish('Reduced motion is on. The complete recording is shown without animation.');
        return;
      }
      if (position >= events.length) {
        position = 0;
        recordingOutput.textContent = '';
      }
      playing = true;
      recordingControl.textContent = 'Pause recording';
      recordingStatus.textContent = 'Recording playing';
      schedule();
    };
    recordingControl.addEventListener('click', () => {
      if (reducedMotion.matches) {
        finish('Reduced motion is on. The complete recording is shown without animation.');
      } else if (playing) {
        window.clearTimeout(timer);
        playing = false;
        recordingControl.textContent = 'Play recording';
        recordingStatus.textContent = 'Recording paused';
      } else {
        start();
      }
    });
    reducedMotion.addEventListener('change', () => {
      window.clearTimeout(timer);
      if (reducedMotion.matches) finish('Reduced motion is on. The complete recording is shown without animation.');
      else start();
    });
    recordingOutput.textContent = '';
    start();
  } catch {
    recordingControl.disabled = true;
    recordingControl.textContent = 'Recording unavailable';
    recordingStatus.textContent = 'The visual recording could not load. The full transcript follows.';
  }
}

void prepareRecording();
