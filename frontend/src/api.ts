import { Item, CollectionType } from './types';

export interface ServerStatus {
  status: string;
  version: string;
  library_root: string;
  counts: Record<string, number>;
  sources_count: number;
}

export interface SourceRecord {
  host: string;
  timestamp: string;
  path: string;
  has_manifest: boolean;
}

export async function fetchServerStatus(): Promise<ServerStatus> {
  try {
    const res = await fetch('/api/status');
    if (res.ok) {
      return await res.json();
    }
  } catch (err) {
    console.error('Failed to connect to backend server:', err);
  }
  return {
    status: 'offline',
    version: '1.0.0',
    library_root: '.',
    counts: {},
    sources_count: 0,
  };
}

export async function fetchCollection(type: CollectionType): Promise<Item[]> {
  try {
    const res = await fetch(`/api/collections/${type}`);
    if (res.ok) {
      const data = await res.json();
      if (Array.isArray(data)) return data;
    }
  } catch (err) {
    console.error(`Failed to fetch collection ${type}:`, err);
  }
  return [];
}

export async function fetchItem(itemId: string): Promise<Item | null> {
  try {
    const res = await fetch(`/api/items/${encodeURIComponent(itemId)}`);
    if (res.ok) {
      return await res.json();
    }
  } catch (err) {
    console.error(`Failed to fetch item ${itemId}:`, err);
  }
  return null;
}

export async function fetchSources(): Promise<SourceRecord[]> {
  try {
    const res = await fetch('/api/sources');
    if (res.ok) {
      const data = await res.json();
      if (Array.isArray(data)) return data;
    }
  } catch (err) {
    console.error('Failed to fetch sources:', err);
  }
  return [];
}

export async function searchLibrary(query: string): Promise<any[]> {
  try {
    const res = await fetch(`/api/search?q=${encodeURIComponent(query)}`);
    if (res.ok) {
      const data = await res.json();
      if (Array.isArray(data)) return data;
    }
  } catch (err) {
    console.error(`Failed to search library with '${query}':`, err);
  }
  return [];
}

export async function fetchExportTokens(format: string): Promise<string> {
  try {
    const res = await fetch(`/api/export?format=${encodeURIComponent(format)}`);
    if (res.ok) {
      const data = await res.json();
      return data.content || '';
    }
  } catch (err) {
    console.error(`Failed to export tokens as ${format}:`, err);
  }
  return `/* Export failed or server is offline */`;
}

export async function compareItems(idA: string, idB: string): Promise<{ item_a: Item | null; item_b: Item | null }> {
  try {
    const res = await fetch(`/api/compare?a=${encodeURIComponent(idA)}&b=${encodeURIComponent(idB)}`);
    if (res.ok) {
      const data = await res.json();
      return { item_a: data.item_a || null, item_b: data.item_b || null };
    }
  } catch (err) {
    console.error('Failed to compare items:', err);
  }
  return { item_a: null, item_b: null };
}
