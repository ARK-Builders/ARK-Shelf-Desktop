import { invoke } from '@tauri-apps/api';
import { type LinkInfo, type OpenGraph, type SortMode } from '../types';

export const createLink = async (
  data: Omit<LinkInfo, 'created_timed' | 'score' | 'name'>,
): Promise<LinkInfo | undefined> => {
  try {
    const name = await invoke<string>('create_link', {
      title: data.title,
      desc: data.desc,
      url: data.url,
    });
    const now = new Date();
    const created_time = {
      secs_since_epoch: Math.round(now.getTime() / 1000),
      nanos_since_epoch: 0,
    };
    const newLink = {
      ...data,
      created_time,
      score: 0,
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

export const readCurrentLinks = async (sortingMode: SortMode) => {
  const names = await invoke<string[]>('read_link_list');
  const linkPromises = names.map(async name => {
    const link = await invoke<Omit<LinkInfo, 'score' | 'name'>>('read_link', {
      name,
    });
    return {
      ...link,
      name,
      score: 0,
    };
  });
  const links = await Promise.all(linkPromises);
  links.sort((a, b) => {
    switch (sortingMode) {
      case 'normal':
        return a.title.localeCompare(b.title);
      case 'date':
        return b.created_time?.secs_since_epoch ?? 0 - (a.created_time?.secs_since_epoch ?? 0);
      case 'score':
      default:
        return 0;
    }
  });
  return links;
};
