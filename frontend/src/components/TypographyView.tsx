import React, { useState } from 'react';
import { Item, TypographyData } from '../types';
import { EmptyState } from './EmptyState';
import { Sliders, Copy, Check } from 'lucide-react';

interface TypographyViewProps {
  typographyItems: Item<TypographyData>[];
  onOpenCapture: () => void;
}

export const TypographyView: React.FC<TypographyViewProps> = ({ typographyItems, onOpenCapture }) => {
  const [selectedId, setSelectedId] = useState<string>(typographyItems[0]?.id || '');
  const [sampleText, setSampleText] = useState('Sphinx of black quartz, judge my vow.');
  const [fontSizeMultiplier, setFontSizeMultiplier] = useState(1);
  const [copiedRole, setCopiedRole] = useState<string | null>(null);

  if (typographyItems.length === 0) {
    return (
      <EmptyState
        title="No Typography in Library"
        description="Capture a website with Specimen to extract its loaded webfonts, font stacks, line heights, and inferred modular type scales."
        onOpenCapture={onOpenCapture}
        suggestedCommand="specimen capture https://example.com"
      />
    );
  }

  const activeItem = typographyItems.find((t) => t.id === selectedId) || typographyItems[0];
  const { families = [], scale = [] } = activeItem.data || {};

  const copySnippet = (role: string, size: number) => {
    navigator.clipboard.writeText(`font-size: ${size}px; font-family: ${families[0]?.name || 'sans-serif'};`);
    setCopiedRole(role);
    setTimeout(() => setCopiedRole(null), 1500);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="pb-4 border-b border-specimen-border flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div>
          <h2 className="text-xl font-bold text-white flex items-center gap-2">
            <span>Typography System</span>
            <span className="text-xs px-2.5 py-0.5 rounded-full bg-specimen-accent/20 text-specimen-accent font-medium">
              {families.length} Font Families
            </span>
          </h2>
          <p className="text-xs text-specimen-muted mt-1">{activeItem.name || activeItem.id}</p>
        </div>

        {/* Switcher */}
        {typographyItems.length > 1 && (
          <div className="flex items-center gap-2">
            {typographyItems.map((t) => (
              <button
                key={t.id}
                onClick={() => setSelectedId(t.id)}
                className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition border ${
                  activeItem.id === t.id
                    ? 'bg-specimen-accent text-white border-specimen-accent'
                    : 'bg-specimen-surface text-specimen-muted border-specimen-border hover:text-white'
                }`}
              >
                {t.name || t.slug}
              </button>
            ))}
          </div>
        )}
      </div>

      {/* Font Family Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {families.map((fam) => (
          <div key={fam.name} className="p-5 rounded-xl border border-specimen-border bg-specimen-surface space-y-3">
            <div className="flex items-center justify-between">
              <h3 className="text-base font-bold text-white">{fam.name}</h3>
              <span className="text-xs px-2 py-0.5 rounded bg-[#0b0d12] text-specimen-accent font-mono">
                {fam.is_custom ? 'Custom Webfont' : 'System Fallback'}
              </span>
            </div>
            <p className="text-xs font-mono text-specimen-muted">{fam.stack?.join(', ')}</p>
            <div className="flex gap-2 flex-wrap">
              {(fam.weights || [400]).map((w) => (
                <span key={w} className="px-2 py-0.5 rounded text-[11px] bg-[#0b0d12] border border-specimen-border text-white">
                  Weight {w}
                </span>
              ))}
            </div>
          </div>
        ))}
      </div>

      {/* Live Interactive Specimen Sandbox */}
      <div className="p-5 rounded-xl border border-specimen-border bg-specimen-surface space-y-4">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-3 border-b border-specimen-border">
          <div className="flex items-center gap-2">
            <Sliders className="w-4 h-4 text-specimen-accent" />
            <span className="text-sm font-bold text-white uppercase tracking-wider">Live Type Specimen Sandbox</span>
          </div>
          <div className="flex items-center gap-3">
            <span className="text-xs text-specimen-muted">Scale Adjust:</span>
            <input
              type="range"
              min="0.8"
              max="1.4"
              step="0.05"
              value={fontSizeMultiplier}
              onChange={(e) => setFontSizeMultiplier(parseFloat(e.target.value))}
              className="w-28 accent-specimen-accent cursor-pointer"
            />
            <span className="text-xs font-mono text-white">{fontSizeMultiplier.toFixed(2)}x</span>
          </div>
        </div>

        <div>
          <input
            type="text"
            value={sampleText}
            onChange={(e) => setSampleText(e.target.value)}
            className="w-full bg-[#0b0d12] border border-specimen-border rounded-lg px-4 py-2 text-sm text-white placeholder-specimen-muted focus:border-specimen-accent outline-none"
            placeholder="Type custom preview text..."
          />
        </div>

        {/* Modular Type Scale Ladder */}
        <div className="divide-y divide-specimen-border/40 space-y-4 pt-2">
          {scale.map((entry) => {
            const computedSize = Math.round(entry.size_px * fontSizeMultiplier);
            return (
              <div key={entry.role} className="pt-4 flex flex-col md:flex-row md:items-center justify-between gap-4 group">
                <div className="w-48 flex-shrink-0">
                  <div className="flex items-center gap-2">
                    <span className="text-xs font-bold text-white capitalize">{entry.role}</span>
                    <button
                      onClick={() => copySnippet(entry.role, computedSize)}
                      className="opacity-0 group-hover:opacity-100 p-1 text-specimen-muted hover:text-white transition"
                      title="Copy CSS"
                    >
                      {copiedRole === entry.role ? <Check className="w-3 h-3 text-green-400" /> : <Copy className="w-3 h-3" />}
                    </button>
                  </div>
                  <span className="text-[11px] font-mono text-specimen-muted">
                    {computedSize}px / {entry.line_height}lh / {entry.weight}w
                  </span>
                </div>

                <div
                  className="flex-1 text-white truncate"
                  style={{
                    fontSize: `${computedSize}px`,
                    lineHeight: entry.line_height,
                    fontWeight: entry.weight,
                    letterSpacing: entry.letter_spacing || 'normal',
                    fontFamily: families[0]?.name || 'sans-serif',
                  }}
                >
                  {sampleText}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};
