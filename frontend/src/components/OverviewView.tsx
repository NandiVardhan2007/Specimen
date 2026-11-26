import React from 'react';
import { CollectionType } from '../types';
import { ServerStatus } from '../api';
import {
  Palette,
  Type,
  Component,
  Sparkles,
  Zap,
  Play,
  Layout,
  ArrowRight,
  Shield,
  FileCode,
  Globe,
  Database,
} from 'lucide-react';

interface OverviewViewProps {
  status: ServerStatus;
  onSelectTab: (tab: CollectionType) => void;
  onOpenCapture: () => void;
}

export const OverviewView: React.FC<OverviewViewProps> = ({ status, onSelectTab, onOpenCapture }) => {
  const counts = status.counts || {};
  const totalTokens = Object.values(counts).reduce((a, b) => a + b, 0);

  const collectionCards = [
    { type: 'palettes' as CollectionType, title: 'Color Palettes', count: counts['palettes'] || 0, desc: 'OKLCH color space values, WCAG contrast pairs, and scheme roles.', icon: <Palette className="w-5 h-5 text-purple-400" /> },
    { type: 'typography' as CollectionType, title: 'Typography Scales', count: counts['typography'] || 0, desc: 'Loaded font families, font weights, and modular type scale ratios.', icon: <Type className="w-5 h-5 text-blue-400" /> },
    { type: 'components' as CollectionType, title: 'Component States', count: counts['components'] || 0, desc: 'Buttons, inputs, cards across hover/focus/active interaction states.', icon: <Component className="w-5 h-5 text-indigo-400" /> },
    { type: 'icons' as CollectionType, title: 'Vector Icons', count: counts['icons'] || 0, desc: 'Sanitized SVGs, SHA-256 geometry fingerprints, and matched icon sets.', icon: <Sparkles className="w-5 h-5 text-amber-400" /> },
    { type: 'effects' as CollectionType, title: 'Effects & Elevation', count: counts['effects'] || 0, desc: 'Elevation drop shadows, glassmorphism blur recipes, and gradients.', icon: <Zap className="w-5 h-5 text-emerald-400" /> },
    { type: 'animations' as CollectionType, title: 'Motion & Keyframes', count: counts['animations'] || 0, desc: 'CSS @keyframes, cubic-bezier easing curves, and WAAPI animation records.', icon: <Play className="w-5 h-5 text-rose-400" /> },
    { type: 'layouts' as CollectionType, title: 'Layout Skeletons', count: counts['layouts'] || 0, desc: 'Semantic section landmarks, wireframe structures, and grid rules.', icon: <Layout className="w-5 h-5 text-cyan-400" /> },
  ];

  return (
    <div className="space-y-8">
      {/* Hero Banner */}
      <div className="relative overflow-hidden rounded-2xl bg-gradient-to-r from-specimen-surface via-[#171b29] to-specimen-surface border border-specimen-border p-8 shadow-xl">
        <div className="relative z-10 max-w-2xl">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-specimen-accent/10 border border-specimen-accent/20 text-specimen-accent text-xs font-semibold mb-4">
            <span>Specimen Studio • Living Design System</span>
          </div>
          <h1 className="text-3xl sm:text-4xl font-extrabold text-white tracking-tight">
            Real Design System Evidence
          </h1>
          <p className="mt-3 text-sm sm:text-base text-specimen-muted leading-relaxed">
            Specimen inspects living websites to capture, normalize, and verify exact colors, typography, components, icons, and motion. Stored as plain git-friendly files on your local machine.
          </p>
          <div className="mt-6 flex flex-wrap gap-3">
            <button
              onClick={onOpenCapture}
              className="px-5 py-2.5 rounded-lg bg-specimen-accent hover:bg-specimen-accent-hover text-white text-sm font-semibold shadow-lg shadow-specimen-accent/30 transition flex items-center gap-2 active:scale-95"
            >
              <span>Capture Target Site</span>
              <ArrowRight className="w-4 h-4" />
            </button>
            <button
              onClick={() => onSelectTab('export')}
              className="px-5 py-2.5 rounded-lg bg-[#0b0d12] hover:bg-specimen-hover text-specimen-text border border-specimen-border text-sm font-semibold transition flex items-center gap-2"
            >
              <FileCode className="w-4 h-4 text-specimen-muted" />
              <span>Export Tokens</span>
            </button>
          </div>
        </div>

        {/* Ambient background glow */}
        <div className="absolute right-0 top-0 -mr-20 -mt-20 w-80 h-80 rounded-full bg-specimen-accent/15 blur-3xl pointer-events-none" />
      </div>

      {/* Real Metrics Header */}
      <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
        <div className="p-4 rounded-xl border border-specimen-border bg-specimen-surface">
          <span className="text-xs text-specimen-muted block">Total Extracted Items</span>
          <span className="text-2xl font-extrabold text-white mt-1 block">{totalTokens}</span>
        </div>
        <div className="p-4 rounded-xl border border-specimen-border bg-specimen-surface">
          <span className="text-xs text-specimen-muted block">Recorded Sources</span>
          <span className="text-2xl font-extrabold text-white mt-1 block">{status.sources_count || 0}</span>
        </div>
        <div className="p-4 rounded-xl border border-specimen-border bg-specimen-surface">
          <span className="text-xs text-specimen-muted block">Loopback Security</span>
          <span className="text-sm font-bold text-specimen-success mt-1 block">127.0.0.1 (Strict)</span>
        </div>
        <div className="p-4 rounded-xl border border-specimen-border bg-specimen-surface">
          <span className="text-xs text-specimen-muted block">Engine Status</span>
          <span className="text-sm font-bold text-white mt-1 block capitalize">{status.status} (v{status.version})</span>
        </div>
      </div>

      {/* Collection Grid Cards */}
      <div>
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold text-white">Collections Directory</h2>
          <span className="text-xs text-specimen-muted">Live library state</span>
        </div>
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {collectionCards.map((s) => (
            <div
              key={s.type}
              onClick={() => onSelectTab(s.type)}
              className="group cursor-pointer rounded-xl bg-specimen-surface border border-specimen-border hover:border-specimen-accent/50 p-5 transition-all hover:-translate-y-0.5 shadow-sm hover:shadow-lg hover:shadow-specimen-accent/5 flex flex-col justify-between"
            >
              <div>
                <div className="flex items-center justify-between mb-3">
                  <div className="p-2 rounded-lg bg-[#0b0d12] border border-specimen-border">
                    {s.icon}
                  </div>
                  <span className="text-xs font-semibold px-2.5 py-0.5 rounded-full bg-[#0b0d12] text-specimen-accent border border-specimen-border">
                    {s.count} items
                  </span>
                </div>
                <h3 className="text-base font-bold text-white group-hover:text-specimen-accent transition-colors">
                  {s.title}
                </h3>
                <p className="mt-1 text-xs text-specimen-muted leading-relaxed">{s.desc}</p>
              </div>
              <div className="mt-4 pt-3 border-t border-specimen-border/60 flex items-center justify-between text-xs text-specimen-muted group-hover:text-white">
                <span>Browse collection</span>
                <ArrowRight className="w-3.5 h-3.5 group-hover:translate-x-1 transition-transform" />
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Security & System Guarantees */}
      <div className="rounded-xl border border-specimen-border bg-specimen-surface p-6">
        <h3 className="text-sm font-bold text-white uppercase tracking-wider mb-4 flex items-center gap-2">
          <Shield className="w-4 h-4 text-specimen-success" />
          <span>Core Invariants (Hardened Architecture)</span>
        </h3>
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 text-xs text-specimen-muted">
          <div className="p-3.5 rounded-lg bg-[#0b0d12] border border-specimen-border">
            <span className="font-semibold text-white block mb-1">Measured Values Immutable</span>
            Physical values come strictly from computed styles and loaded files. No hallucinated values.
          </div>
          <div className="p-3.5 rounded-lg bg-[#0b0d12] border border-specimen-border">
            <span className="font-semibold text-white block mb-1">Loopback & Rebinding Defended</span>
            Web UI binds strictly to 127.0.0.1 with Host verification and strict Content-Security-Policy.
          </div>
          <div className="p-3.5 rounded-lg bg-[#0b0d12] border border-specimen-border">
            <span className="font-semibold text-white block mb-1">Zero Secrets Stored</span>
            Passwords, authorization headers, and session tokens are automatically sanitized during capture.
          </div>
        </div>
      </div>
    </div>
  );
};
