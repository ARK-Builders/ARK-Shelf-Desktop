import { writable } from "svelte/store";
import type { LinkScoreMap, LinkInfo } from "./types";

export const linksInfos = writable<LinkInfo[]>([])
export const scores = writable<LinkScoreMap[]>([])