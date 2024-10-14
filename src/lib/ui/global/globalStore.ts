import { writable } from 'svelte/store';

export const isNavigationDrawerOpen = writable<boolean>(true);
