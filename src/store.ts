import { writable, type Updater } from 'svelte/store';
import type { LinkInfo, SortMode } from './types';

const sortLinks = (links: LinkInfo[], mode: SortMode) => {
    links.sort((a, b) => {
        switch (mode) {
            case 'normal':
                return a.title.localeCompare(b.title);
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
    let mode = defaultMode;

    const { subscribe, update, set } = writable<LinkInfo[]>([]);

    const updateLinks = (updater: Updater<LinkInfo[]>) => {
        update(currentLinks => {
            const updatedLinks = updater(currentLinks);
            const sortedLinks = sortLinks(updatedLinks, mode);
            return sortedLinks;
        });
    };

    return {
        subscribe,
        update: updateLinks,
        setMode: (newMode: SortMode) => {
            mode = newMode;
            update(links => {
                const sorted = sortLinks(links, mode);
                return sorted;
            });
        },
        set: (newLinks: LinkInfo[]) => {
            const sorted = sortLinks(newLinks, mode);
            set(sorted);
        },
    };
};

export const linksInfos = createLinksInfos();
