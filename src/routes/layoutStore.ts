import { writable } from 'svelte/store';

export interface ToastProperties {
  id: string;
  message: string;
  type: 'info' | 'warn' | 'success' | 'error' | 'wait';
}

export const toasts = writable<ToastProperties[]>([]);
