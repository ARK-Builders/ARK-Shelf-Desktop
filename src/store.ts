import { listen } from '@tauri-apps/api/event';
import { writable, type Updater } from 'svelte/store';
import type { LinkInfo, PreviewLoaded, SortMode } from './types';

const sortLinks = (links: LinkInfo[], mode: SortMode) => {
    links.sort((a, b) => {
        switch (mode) {
            case 'normal':
                return a.title.toLowerCase().localeCompare(b.title.toLowerCase());
            case 'date':
                return (
                    (b.created_time?.secs_since_epoch ?? 0) -
                    (a.created_time?.secs_since_epoch ?? 0)
                );
            case 'score':
                return (b.score?.value ?? 0) - (a.score?.value ?? 0);
            default:
                return 0;
        }
    });
    return links;
};

export const createLinksInfos = (defaultMode: SortMode = 'normal') => {
    const mode = writable(defaultMode);
    let currentMode = defaultMode;
    mode.subscribe(m => {
        currentMode = m;
    });

    const { subscribe, update, set } = writable<LinkInfo[]>([]);

    let currentLinks: LinkInfo[] = [];
    subscribe(links => {
        currentLinks = links;
    });

    const updateLinks = (updater: Updater<LinkInfo[]>) => {
        update(currentLinks => {
            const updatedLinks = updater(currentLinks);
            const sortedLinks = sortLinks(updatedLinks, currentMode);
            return sortedLinks;
        });
    };

    listen<PreviewLoaded>('preview_loaded', e => {
        const linkUrl = e.payload.url;
        if (currentLinks.some(link => link.url === linkUrl)) {
            update(links => {
                links.forEach(l => {
                    if (l.url === linkUrl) {
                        l.graph = e.payload.graph;
                    }
                });
                return links;
            });
        }
    });

    return {
        subscribe,
        update: updateLinks,
        setMode: (newMode: SortMode) => {
            mode.set(newMode);
            update(links => {
                const sorted = sortLinks(links, newMode);
                return sorted;
            });
        },
        set: (newLinks: LinkInfo[]) => {
            const sorted = sortLinks(newLinks, currentMode);
            set(sorted);
        },
        mode: { subscribe: mode.subscribe },
    };
};

export const linksInfos = createLinksInfos();
