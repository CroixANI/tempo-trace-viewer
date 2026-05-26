import { writable } from 'svelte/store';
import type { TraceView, LogEntryView } from '../types/trace';

export const traceView = writable<TraceView | null>(null);
export const selectedSpanId = writable<string | null>(null);
export const searchQuery = writable<string>('');
export const unmatchedLogs = writable<LogEntryView[]>([]);
export const collapsedSpans = writable<Set<string>>(new Set());
