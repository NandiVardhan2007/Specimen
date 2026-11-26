import React from 'react';
import { CollectionType } from '../types';
import {
  Layers,
  Palette,
  Type,
  Component,
  Sparkles,
  Zap,
  Play,
  Layout,
  GitCompare,
  Download,
  PlusCircle,
  Search,
  ShieldCheck,
} from 'lucide-react';

interface HeaderProps {
  activeTab: CollectionType;
  setActiveTab: (tab: CollectionType) => void;
  onOpenCapture: () => void;
  searchQuery: string;
  setSearchQuery: (q: string) => void;
}

export const Header: React.FC<HeaderProps> = ({
  activeTab,
  setActiveTab,
  onOpenCapture,
  searchQuery,
  setSearchQuery,
}) => {
  const navItems: { id: CollectionType; label: string; icon: React.ReactNode }[] = [
    { id: 'overview', label: 'Overview', icon: <Layers className="w-4 h-4" /> },
    { id: 'palettes', label: 'Palettes', icon: <Palette className="w-4 h-4" /> },
    { id: 'typography', label: 'Typography', icon: <Type className="w-4 h-4" /> },
    { id: 'components', label: 'Components', icon: <Component className="w-4 h-4" /> },
    { id: 'icons', label: 'Icons', icon: <Sparkles className="w-4 h-4" /> },
    { id: 'effects', label: 'Effects', icon: <Zap className="w-4 h-4" /> },
    { id: 'animations', label: 'Motion', icon: <Play className="w-4 h-4" /> },
    { id: 'layouts', label: 'Layouts', icon: <Layout className="w-4 h-4" /> },
    { id: 'compare', label: 'Compare', icon: <GitCompare className="w-4 h-4" /> },
    { id: 'export', label: 'Export', icon: <Download className="w-4 h-4" /> },
  ];

  return (
    <header className="border-b border-specimen-border bg-specimen-surface/90 backdrop-blur sticky top-0 z-40">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          {/* Brand */}
          <div className="flex items-center gap-3 cursor-pointer" onClick={() => setActiveTab('overview')}>
            <div className="w-8 h-8 rounded-lg bg-gradient-to-tr from-specimen-accent to-indigo-500 flex items-center justify-center shadow-lg shadow-specimen-accent/20">
              <span className="font-bold text-white text-base">S</span>
            </div>
            <div className="flex items-center gap-2">
              <span className="font-bold text-lg tracking-tight text-white">Specimen</span>
              <span className="text-xs px-2 py-0.5 rounded-full bg-specimen-accent/20 text-specimen-accent font-semibold border border-specimen-accent/30">
                Studio v1.0
              </span>
            </div>
          </div>

          {/* Search bar */}
          <div className="flex-1 max-w-md mx-6">
            <div className="relative">
              <Search className="w-4 h-4 text-specimen-muted absolute left-3 top-1/2 -translate-y-1/2" />
              <input
                type="text"
                placeholder="Search tokens, tags, roles... (e.g. type:palette dark)"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="w-full bg-[#0b0d12] border border-specimen-border rounded-lg pl-9 pr-4 py-1.5 text-sm text-specimen-text placeholder-specimen-muted focus:border-specimen-accent focus:ring-1 focus:ring-specimen-accent outline-none transition"
              />
            </div>
          </div>

          {/* Actions & Loopback status */}
          <div className="flex items-center gap-3">
            <div className="hidden sm:flex items-center gap-1.5 text-xs text-specimen-muted px-2.5 py-1 rounded bg-[#0b0d12] border border-specimen-border">
              <ShieldCheck className="w-3.5 h-3.5 text-specimen-success" />
              <span>127.0.0.1:4000</span>
            </div>
            <button
              onClick={onOpenCapture}
              className="flex items-center gap-1.5 px-3.5 py-1.5 rounded-lg bg-specimen-accent hover:bg-specimen-accent-hover text-white text-sm font-semibold shadow-md shadow-specimen-accent/25 transition active:scale-95"
            >
              <PlusCircle className="w-4 h-4" />
              <span>Capture Site</span>
            </button>
          </div>
        </div>

        {/* Tab Navigation */}
        <nav className="flex space-x-1 overflow-x-auto pb-2 scrollbar-none">
          {navItems.map((item) => {
            const isActive = activeTab === item.id;
            return (
              <button
                key={item.id}
                onClick={() => setActiveTab(item.id)}
                className={`flex items-center gap-2 px-3 py-1.5 rounded-md text-xs font-medium transition whitespace-nowrap ${
                  isActive
                    ? 'bg-specimen-accent text-white shadow-sm'
                    : 'text-specimen-muted hover:text-specimen-text hover:bg-specimen-hover'
                }`}
              >
                {item.icon}
                <span>{item.label}</span>
              </button>
            );
          })}
        </nav>
      </div>
    </header>
  );
};
