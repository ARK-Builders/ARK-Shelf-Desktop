import { invoke } from '@tauri-apps/api';
import type { LinkInfo, LinkScoreMap, OpenGraph } from '../types';

export const createLink = async (
    data: Omit<LinkInfo, 'created_timed' | 'score' | 'name'>,
): Promise<LinkInfo | undefined> => {
    try {
        const name = await invoke<string>('create_link', {
            url: data.url,
            metadata: {
                title: data.title,
                description: data.desc,
            },
        });
        const now = new Date();
        const created_time = {
            secs_since_epoch: Math.round(now.getTime() / 1000),
            nanos_since_epoch: 0,
        };
        const newLink = {
            ...data,
            created_time,
            score: undefined,
            name,
        };
        return newLink;
    } catch (_) {
        return undefined;
    }
};

export const getPreview = async (url: string): Promise<OpenGraph | undefined> => {
    try {
        const openGraph = await invoke<OpenGraph>('generate_link_preview', { url });
        return openGraph;
    } catch (e) {
        return;
    }
};

const getScores = async () => {
    try {
        const scores = await invoke<LinkScoreMap[]>('get_scores');
        return scores;
    } catch (_) {
        return;
    }
};

export const readCurrentLinks = async () => {
    const names = await invoke<string[]>('read_link_list');
    const scores = await getScores();
    const linkPromises = names.map(async name => {
        const link = await invoke<Omit<LinkInfo, 'score' | 'name'>>('read_link', {
            name,
        });
        const score = scores?.find(s => s.name === name);
        return {
            ...link,
            name,
            score,
        };
    });

    const links = await Promise.all(linkPromises);
    return links;
};

export const deleteLink = async (name: string): Promise<boolean> => {
    try {
        await invoke('delete_link', { name });
        return true;
    } catch (_) {
        return false;
    }
};

export const addScore = async (name: string) => {
    try {
        const score = await invoke<LinkScoreMap | undefined>('add', { name });
        return score;
    } catch (e) {
        return;
    }
};

export const substractScore = async (name: string) => {
    try {
        const score = await invoke<LinkScoreMap | undefined>('substract', { name });
        return score;
    } catch (e) {
        return;
    }
};

export const createScore = async ({ value, url }: { value: number; url: string }) => {
    try {
        const score = await invoke<LinkScoreMap>('create_score', { url, value });
        return score;
    } catch (_) {
        return;
    }
};

export const debounce = (callback: unknown, wait = 500) => {
    let timeoutId: number;
    return (...args: unknown[]) => {
        window.clearTimeout(timeoutId);
        timeoutId = window.setTimeout(() => {
            if (typeof callback === 'function') {
                callback(...args);
            }
        }, wait);
    };
};
