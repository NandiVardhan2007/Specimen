import React, { useState, useEffect } from 'react';
import { CollectionType, Item, PaletteData, TypographyData, ComponentData, IconData, EffectData, AnimationData, LayoutData } from './types';
import { fetchServerStatus, fetchCollection, fetchSources, searchLibrary, ServerStatus, SourceRecord } from './api';
import { Header } from './components/Header';
import { OverviewView } from './components/OverviewView';
import { PalettesView } from './components/PalettesView';
import { TypographyView } from './components/TypographyView';
import { ComponentsView } from './components/ComponentsView';
import { IconsView } from './components/IconsView';
import { EffectsView } from './components/EffectsView';
import { AnimationsView } from './components/AnimationsView';
import { LayoutsView } from './components/LayoutsView';
import { SourcesView } from './components/SourcesView';
import { CompareView } from './components/CompareView';
import { ExportView } from './components/ExportView';
import { CaptureModal } from './components/CaptureModal';
import { Search } from 'lucide-react';

export const App: React.FC = () => {
  const [activeTab, setActiveTab] = useState<CollectionType>('overview');
  const [serverStatus, setServerStatus] = useState<ServerStatus>({
    status: 'connecting',
    version: '1.0.0',
    library_root: '.',
    counts: {},
    sources_count: 0,
  });

  const [palettes, setPalettes] = useState<Item<PaletteData>[]>([]);
  const [typography, setTypography] = useState<Item<TypographyData>[]>([]);
  const [components, setComponents] = useState<Item<ComponentData>[]>([]);
  const [icons, setIcons] = useState<Item<IconData>[]>([]);
  const [effects, setEffects] = useState<Item<EffectData>[]>([]);
  const [animations, setAnimations] = useState<Item<AnimationData>[]>([]);
  const [layouts, setLayouts] = useState<Item<LayoutData>[]>([]);
  const [sources, setSources] = useState<SourceRecord[]>([]);

  const [searchQuery, setSearchQuery] = useState('');
  const [searchResults, setSearchResults] = useState<any[]>([]);
  const [isSearching, setIsSearching] = useState(false);
  const [isCaptureOpen, setIsCaptureOpen] = useState(false);

  // Load status on mount
  useEffect(() => {
    fetchServerStatus().then(setServerStatus);
  }, []);

  // Fetch real items when tab changes
  useEffect(() => {
    let isMounted = true;
    switch (activeTab) {
      case 'palettes':
        fetchCollection('palettes').then((res) => isMounted && setPalettes(res as any));
        break;
      case 'typography':
        fetchCollection('typography').then((res) => isMounted && setTypography(res as any));
        break;
      case 'components':
        fetchCollection('components').then((res) => isMounted && setComponents(res as any));
        break;
      case 'icons':
        fetchCollection('icons').then((res) => isMounted && setIcons(res as any));
        break;
      case 'effects':
        fetchCollection('effects').then((res) => isMounted && setEffects(res as any));
        break;
      case 'animations':
        fetchCollection('animations').then((res) => isMounted && setAnimations(res as any));
        break;
      case 'layouts':
        fetchCollection('layouts').then((res) => isMounted && setLayouts(res as any));
        break;
      case 'sources':
        fetchSources().then((res) => isMounted && setSources(res));
        break;
      default:
        break;
    }
    return () => {
      isMounted = false;
    };
  }, [activeTab]);

  // Live search debounce
  useEffect(() => {
    if (!searchQuery.trim()) {
      setSearchResults([]);
      setIsSearching(false);
      return;
    }

    setIsSearching(true);
    const handler = setTimeout(() => {
      searchLibrary(searchQuery).then((results) => {
        setSearchResults(results);
        setIsSearching(false);
      });
    }, 200);

    return () => clearTimeout(handler);
  }, [searchQuery]);

  const renderContent = () => {
    // If search query is active, display search hits
    if (searchQuery.trim()) {
      return (
        <div className="space-y-6">
          <div className="pb-4 border-b border-specimen-border flex items-center justify-between">
            <h2 className="text-xl font-bold text-white flex items-center gap-2">
              <Search className="w-5 h-5 text-specimen-accent" />
              <span>Search Results for "{searchQuery}"</span>
            </h2>
            <span className="text-xs text-specimen-muted">
              {isSearching ? 'Searching...' : `Found ${searchResults.length} item(s)`}
            </span>
          </div>

          {searchResults.length === 0 && !isSearching ? (
            <div className="p-12 text-center text-specimen-muted bg-specimen-surface rounded-xl border border-specimen-border">
              No items in library matching '{searchQuery}'.
            </div>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {searchResults.map((hit: any) => (
                <div
                  key={hit.id}
                  className="p-5 rounded-xl border border-specimen-border bg-specimen-surface hover:border-specimen-accent/40 transition"
                >
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-sm font-bold text-white">{hit.name || hit.id}</span>
                    <span className="text-xs px-2 py-0.5 rounded bg-[#0b0d12] text-specimen-accent font-mono">
                      {hit.item_type}
                    </span>
                  </div>
                  <p className="text-xs text-specimen-muted mb-3">{hit.description || hit.id}</p>
                  <div className="flex items-center justify-between text-[11px] text-specimen-muted pt-2 border-t border-specimen-border">
                    <span className="font-mono">Score: {hit.score?.toFixed(1)}</span>
                    <span className="capitalize">{hit.license_status || 'reusable'}</span>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      );
    }

    switch (activeTab) {
      case 'overview':
        return (
          <OverviewView
            status={serverStatus}
            onSelectTab={setActiveTab}
            onOpenCapture={() => setIsCaptureOpen(true)}
          />
        );
      case 'palettes':
        return <PalettesView palettes={palettes} onOpenCapture={() => setIsCaptureOpen(true)} />;
      case 'typography':
        return <TypographyView typographyItems={typography} onOpenCapture={() => setIsCaptureOpen(true)} />;
      case 'components':
        return <ComponentsView components={components} onOpenCapture={() => setIsCaptureOpen(true)} />;
      case 'icons':
        return <IconsView icons={icons} onOpenCapture={() => setIsCaptureOpen(true)} />;
      case 'effects':
        return <EffectsView effects={effects} onOpenCapture={() => setIsCaptureOpen(true)} />;
      case 'animations':
        return <AnimationsView animations={animations} onOpenCapture={() => setIsCaptureOpen(true)} />;
      case 'layouts':
        return <LayoutsView layouts={layouts} onOpenCapture={() => setIsCaptureOpen(true)} />;
      case 'sources':
        return <SourcesView sources={sources} onOpenCapture={() => setIsCaptureOpen(true)} />;
      case 'compare':
        return <CompareView onOpenCapture={() => setIsCaptureOpen(true)} />;
      case 'export':
        return <ExportView />;
      default:
        return null;
    }
  };

  return (
    <div className="min-h-screen bg-[#0b0d12] flex flex-col font-sans">
      <Header
        activeTab={activeTab}
        setActiveTab={setActiveTab}
        onOpenCapture={() => setIsCaptureOpen(true)}
        searchQuery={searchQuery}
        setSearchQuery={setSearchQuery}
      />

      <main className="flex-1 max-w-7xl mx-auto w-full px-4 sm:px-6 lg:px-8 py-8">
        {renderContent()}
      </main>

      <footer className="border-t border-specimen-border py-6 bg-specimen-surface/50 text-center text-xs text-specimen-muted">
        <p>Specimen Studio • Stored as git-friendly plain files • Loopback only 127.0.0.1</p>
      </footer>

      <CaptureModal isOpen={isCaptureOpen} onClose={() => setIsCaptureOpen(false)} />
    </div>
  );
};

export default App;
