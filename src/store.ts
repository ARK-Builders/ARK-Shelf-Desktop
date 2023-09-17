import { writable } from 'svelte/store';
import type { LinkInfo, SortMode } from './types';

export const linksInfos = writable<LinkInfo[]>([]);
export const sortingMode = writable<SortMode>('normal');
