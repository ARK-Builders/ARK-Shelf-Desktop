import { writable } from 'svelte/store';
import type { LinkInfo } from './types';

export const linksInfos = writable<LinkInfo[]>([]);
