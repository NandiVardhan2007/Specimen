import React, { useState } from 'react';
import { Item, EffectData } from '../types';
import { EmptyState } from './EmptyState';
import { Copy, Check, Zap } from 'lucide-react';

interface EffectsViewProps {
  effects: Item<EffectData>[];
  onOpenCapture: () => void;
}

export const EffectsView: React.FC<EffectsViewProps> = ({ effects, onOpenCapture }) => {
  const [copiedId, setCopiedId] = useState<string | null>(null);

  if (effects.length === 0) {
    return (
      <EmptyState
        title="No Effects or Elevation in Library"
        description="Specimen extracts drop shadows, backdrop filters, glassmorphism recipes, and CSS gradients with rendered previews."
        onOpenCapture={onOpenCapture}
        suggestedCommand="specimen capture https://example.com"
      />
    );
  }

  const copyCss = (id: string, css: string) => {
    navigator.clipboard.writeText(css);
    setCopiedId(id);
    setTimeout(() => setCopiedId(null), 1500);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="pb-4 border-b border-specimen-border">
        <h2 className="text-xl font-bold text-white flex items-center gap-2">
          <span>Effects, Elevation & Glassmorphism</span>
          <span className="text-xs px-2.5 py-0.5 rounded-full bg-specimen-accent/20 text-specimen-accent font-medium">
            {effects.length} Effects
          </span>
        </h2>
        <p className="text-xs text-specimen-muted mt-1">
          Rule-based recipes for elevation layers, shadows, and backdrop blurs.
        </p>
      </div>

      {/* Grid of Effects */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {effects.map((eff) => {
          const kind = eff.data?.kind || 'effect';
          const cssValue = eff.data?.css_value || '';

          return (
            <div
              key={eff.id}
              className="rounded-xl border border-specimen-border bg-specimen-surface overflow-hidden shadow-sm hover:border-specimen-accent/40 transition flex flex-col justify-between"
            >
              {/* Visual preview viewport */}
              <div className="h-44 w-full relative flex items-center justify-center p-6 bg-gradient-to-tr from-slate-950 via-slate-900 to-indigo-950 overflow-hidden">
                {/* Visual backdrop elements */}
                <div className="absolute -left-4 -top-4 w-24 h-24 rounded-full bg-specimen-accent/30 blur-xl" />
                <div className="absolute -right-4 -bottom-4 w-24 h-24 rounded-full bg-emerald-500/20 blur-xl" />

                {/* Effect Target Box */}
                <div
                  className="w-36 h-20 rounded-xl flex items-center justify-center text-xs font-semibold text-white relative z-10 transition-transform hover:scale-105"
                  style={
                    kind === 'shadow'
                      ? { backgroundColor: '#141721', boxShadow: cssValue }
                      : kind === 'glassmorphism'
                      ? {
                          background: 'rgba(20, 23, 33, 0.6)',
                          backdropFilter: 'blur(12px)',
                          border: '1px solid rgba(255, 255, 255, 0.1)',
                        }
                      : { backgroundColor: '#141721' }
                  }
                >
                  <span className="capitalize">{kind}</span>
                </div>
              </div>

              {/* Meta & Snippet */}
              <div className="p-4 space-y-3">
                <div className="flex items-center justify-between">
                  <span className="text-xs font-bold text-white">{eff.name || eff.slug}</span>
                  <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-[#0b0d12] text-specimen-accent border border-specimen-border">
                    {kind}
                  </span>
                </div>
                <div className="flex items-center justify-between gap-2 bg-[#0b0d12] p-2 rounded-lg border border-specimen-border">
                  <code className="text-[11px] font-mono text-specimen-muted truncate">{cssValue}</code>
                  <button
                    onClick={() => copyCss(eff.id, cssValue)}
                    className="p-1 rounded text-specimen-muted hover:text-white transition flex-shrink-0"
                    title="Copy CSS"
                  >
                    {copiedId === eff.id ? <Check className="w-3 h-3 text-green-400" /> : <Copy className="w-3 h-3" />}
                  </button>
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
