import React, { useState } from 'react';
import { Item, PaletteData, PaletteColor } from '../types';
import { EmptyState } from './EmptyState';
import { Copy, Check, Sliders, CheckCircle, XCircle } from 'lucide-react';

interface PalettesViewProps {
  palettes: Item<PaletteData>[];
  onOpenCapture: () => void;
}

export const PalettesView: React.FC<PalettesViewProps> = ({ palettes, onOpenCapture }) => {
  const [selectedPaletteId, setSelectedPaletteId] = useState<string>(palettes[0]?.id || '');
  const [copiedHex, setCopiedHex] = useState<string | null>(null);

  if (palettes.length === 0) {
    return (
      <EmptyState
        title="No Color Palettes in Library"
        description="Capture a website with Specimen to extract its color roles, OKLCH color space mappings, and WCAG contrast evaluations."
        onOpenCapture={onOpenCapture}
        suggestedCommand="specimen capture https://example.com"
      />
    );
  }

  const activePalette = palettes.find((p) => p.id === selectedPaletteId) || palettes[0];
  const colors: PaletteColor[] = activePalette.data?.colors || [];

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    setCopiedHex(text);
    setTimeout(() => setCopiedHex(null), 1500);
  };

  return (
    <div className="space-y-6">
      {/* Header & Selector */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-specimen-border">
        <div>
          <h2 className="text-xl font-bold text-white flex items-center gap-2">
            <span>Color Palettes</span>
            <span className="text-xs px-2.5 py-0.5 rounded-full bg-specimen-accent/20 text-specimen-accent font-medium">
              {activePalette.data?.scheme || 'standard'} mode
            </span>
          </h2>
          <p className="text-xs text-specimen-muted mt-1">
            {activePalette.description || `Canonical Item ID: ${activePalette.id}`}
          </p>
        </div>

        {/* Palette switcher */}
        <div className="flex items-center gap-2 flex-wrap">
          {palettes.map((p) => (
            <button
              key={p.id}
              onClick={() => setSelectedPaletteId(p.id)}
              className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition border ${
                activePalette.id === p.id
                  ? 'bg-specimen-accent text-white border-specimen-accent shadow-sm'
                  : 'bg-specimen-surface text-specimen-muted border-specimen-border hover:text-white'
              }`}
            >
              {p.name || p.slug}
            </button>
          ))}
        </div>
      </div>

      {/* Swatches Grid */}
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-4">
        {colors.map((c) => (
          <div
            key={c.hex}
            className="group rounded-xl border border-specimen-border bg-specimen-surface overflow-hidden shadow-sm hover:border-specimen-accent/40 transition"
          >
            {/* Color preview box */}
            <div
              className="h-28 w-full relative flex items-end justify-between p-3"
              style={{ backgroundColor: c.hex }}
            >
              <button
                onClick={() => copyToClipboard(c.hex)}
                className="opacity-0 group-hover:opacity-100 p-1.5 rounded bg-black/60 text-white backdrop-blur transition hover:scale-110 ml-auto"
                title="Copy HEX"
              >
                {copiedHex === c.hex ? <Check className="w-3.5 h-3.5 text-green-400" /> : <Copy className="w-3.5 h-3.5" />}
              </button>
            </div>

            {/* Swatch Metadata */}
            <div className="p-3.5 space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-xs font-bold text-white capitalize">{c.role}</span>
                <span className="font-mono text-xs text-specimen-muted font-medium">{c.hex}</span>
              </div>
              {c.oklch && (
                <div className="text-[11px] font-mono text-specimen-muted flex justify-between">
                  <span>OKLCH</span>
                  <span>
                    {c.oklch[0]?.toFixed(2)}, {c.oklch[1]?.toFixed(2)}, {c.oklch[2]?.toFixed(0)}°
                  </span>
                </div>
              )}
              {c.usage_weight !== undefined && (
                <div className="text-[11px] text-specimen-muted flex justify-between">
                  <span>Coverage</span>
                  <span>{(c.usage_weight * 100).toFixed(0)}%</span>
                </div>
              )}
            </div>
          </div>
        ))}
      </div>

      {/* WCAG Contrast Matrix */}
      {colors.some((c) => c.contrast_pairs && c.contrast_pairs.length > 0) && (
        <div className="rounded-xl border border-specimen-border bg-specimen-surface p-5">
          <h3 className="text-sm font-bold text-white uppercase tracking-wider mb-4 flex items-center gap-2">
            <Sliders className="w-4 h-4 text-specimen-accent" />
            <span>WCAG 2.1 Contrast Matrix & Accessibility</span>
          </h3>
          <div className="overflow-x-auto">
            <table className="w-full text-left text-xs">
              <thead>
                <tr className="border-b border-specimen-border text-specimen-muted">
                  <th className="pb-2 font-medium">Foreground</th>
                  <th className="pb-2 font-medium">Background</th>
                  <th className="pb-2 font-medium">Contrast Ratio</th>
                  <th className="pb-2 font-medium">AA Normal (≥ 4.5)</th>
                  <th className="pb-2 font-medium">AAA Large (≥ 7.0)</th>
                  <th className="pb-2 font-medium">Live Preview</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-specimen-border/40 font-mono">
                {colors.flatMap((c) =>
                  (c.contrast_pairs || []).map((pair, idx) => (
                    <tr key={`${c.hex}-${idx}`} className="hover:bg-specimen-hover/50 transition">
                      <td className="py-2.5">
                        <div className="flex items-center gap-2 font-sans font-medium text-white">
                          <span className="w-3 h-3 rounded-full border border-white/20" style={{ backgroundColor: pair.fg_hex }} />
                          <span>{pair.fg_hex}</span>
                        </div>
                      </td>
                      <td className="py-2.5">
                        <div className="flex items-center gap-2 font-sans font-medium text-white">
                          <span className="w-3 h-3 rounded-full border border-white/20" style={{ backgroundColor: pair.bg_hex }} />
                          <span>{pair.bg_hex}</span>
                        </div>
                      </td>
                      <td className="py-2.5 font-bold text-white">{pair.ratio.toFixed(1)}:1</td>
                      <td className="py-2.5">
                        {pair.wcag_aa ? (
                          <span className="inline-flex items-center gap-1 text-green-400 font-sans text-[11px] font-semibold">
                            <CheckCircle className="w-3 h-3" /> PASS
                          </span>
                        ) : (
                          <span className="inline-flex items-center gap-1 text-red-400 font-sans text-[11px] font-semibold">
                            <XCircle className="w-3 h-3" /> FAIL
                          </span>
                        )}
                      </td>
                      <td className="py-2.5">
                        {pair.wcag_aaa ? (
                          <span className="inline-flex items-center gap-1 text-green-400 font-sans text-[11px] font-semibold">
                            <CheckCircle className="w-3 h-3" /> PASS
                          </span>
                        ) : (
                          <span className="inline-flex items-center gap-1 text-specimen-muted font-sans text-[11px]">
                            —
                          </span>
                        )}
                      </td>
                      <td className="py-2.5 font-sans">
                        <div
                          className="px-3 py-1 rounded text-xs font-semibold inline-block border border-white/10"
                          style={{ backgroundColor: pair.bg_hex, color: pair.fg_hex }}
                        >
                          Sample Text Aa
                        </div>
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
};
