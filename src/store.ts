import { writable } from 'svelte/store';
import type { LinkInfo, LinkScoreMap } from './types';

export const linksInfos = writable<LinkInfo[]>([]);
export const scores = writable<LinkScoreMap[]>([]);
