import React, { useState } from 'react';
import { Item, IconData } from '../types';
import { EmptyState } from './EmptyState';
import { Copy, Check, Download, Search, Sparkles } from 'lucide-react';

interface IconsViewProps {
  icons: Item<IconData>[];
  onOpenCapture: () => void;
}

export const IconsView: React.FC<IconsViewProps> = ({ icons, onOpenCapture }) => {
  const [filterText, setFilterText] = useState('');
  const [backdrop, setBackdrop] = useState<'dark' | 'light' | 'checkered'>('dark');
  const [copiedId, setCopiedId] = useState<string | null>(null);

  if (icons.length === 0) {
    return (
      <EmptyState
        title="No Vector Icons in Library"
        description="Specimen extracts SVG icons from inline SVGs, sprites, and background masks, running sanitization and computing SHA-256 geometry fingerprints."
        onOpenCapture={onOpenCapture}
        suggestedCommand="specimen capture https://example.com"
      />
    );
  }

  const filteredIcons = icons.filter((i) =>
    filterText
      ? i.name?.toLowerCase().includes(filterText.toLowerCase()) ||
        i.id.toLowerCase().includes(filterText.toLowerCase()) ||
        i.tags.some((t) => t.toLowerCase().includes(filterText.toLowerCase()))
      : true
  );

  const copySvg = (icon: Item<IconData>) => {
    const svgMarkup = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="${icon.data?.view_box || '0 0 24 24'}" fill="currentColor">\n  ${icon.data?.svg_content || ''}\n</svg>`;
    navigator.clipboard.writeText(svgMarkup);
    setCopiedId(icon.id);
    setTimeout(() => setCopiedId(null), 1500);
  };

  const downloadSvg = (icon: Item<IconData>) => {
    const svgMarkup = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="${icon.data?.view_box || '0 0 24 24'}" fill="currentColor">\n  ${icon.data?.svg_content || ''}\n</svg>`;
    const blob = new Blob([svgMarkup], { type: 'image/svg+xml' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${icon.slug || 'icon'}.svg`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="pb-4 border-b border-specimen-border flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div>
          <h2 className="text-xl font-bold text-white flex items-center gap-2">
            <span>Sanitized Vector Icons</span>
            <span className="text-xs px-2.5 py-0.5 rounded-full bg-specimen-accent/20 text-specimen-accent font-medium">
              {icons.length} Icons
            </span>
          </h2>
          <p className="text-xs text-specimen-muted mt-1">
            Scripts stripped, geometry normalized, and hashed with SHA-256.
          </p>
        </div>

        {/* Filter & Backdrop switcher */}
        <div className="flex items-center gap-3">
          <div className="relative">
            <Search className="w-3.5 h-3.5 text-specimen-muted absolute left-2.5 top-1/2 -translate-y-1/2" />
            <input
              type="text"
              placeholder="Filter icons..."
              value={filterText}
              onChange={(e) => setFilterText(e.target.value)}
              className="bg-[#0b0d12] border border-specimen-border rounded-lg pl-8 pr-3 py-1 text-xs text-white placeholder-specimen-muted outline-none focus:border-specimen-accent"
            />
          </div>

          <div className="flex items-center bg-[#0b0d12] p-1 rounded-lg border border-specimen-border text-[11px]">
            <button
              onClick={() => setBackdrop('dark')}
              className={`px-2 py-0.5 rounded ${backdrop === 'dark' ? 'bg-specimen-surface text-white' : 'text-specimen-muted'}`}
            >
              Dark
            </button>
            <button
              onClick={() => setBackdrop('light')}
              className={`px-2 py-0.5 rounded ${backdrop === 'light' ? 'bg-white text-black' : 'text-specimen-muted'}`}
            >
              Light
            </button>
          </div>
        </div>
      </div>

      {/* Grid of Icons */}
      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
        {filteredIcons.map((icon) => {
          const data = icon.data || {};
          return (
            <div
              key={icon.id}
              className="group rounded-xl border border-specimen-border bg-specimen-surface p-4 flex flex-col items-center justify-between text-center transition hover:border-specimen-accent/50 hover:shadow-md"
            >
              {/* SVG Canvas Box */}
              <div
                className={`w-full h-24 rounded-lg flex items-center justify-center transition-colors mb-3 ${
                  backdrop === 'light' ? 'bg-white text-slate-900' : 'bg-[#0b0d12] text-white'
                }`}
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox={data.view_box || '0 0 24 24'}
                  fill="none"
                  stroke="currentColor"
                  strokeWidth="2"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  className="w-8 h-8 text-specimen-accent transition-transform group-hover:scale-110"
                  dangerouslySetInnerHTML={{ __html: data.svg_content || '' }}
                />
              </div>

              {/* Icon Info */}
              <div className="w-full truncate mb-2">
                <span className="text-xs font-bold text-white block truncate" title={icon.name || icon.slug}>
                  {icon.name || icon.slug}
                </span>
                <span className="text-[10px] font-mono text-specimen-muted block truncate" title={data.geometry_hash}>
                  hash: {data.geometry_hash?.slice(0, 8) || 'unknown'}
                </span>
              </div>

              {/* Action Buttons */}
              <div className="flex items-center gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity">
                <button
                  onClick={() => copySvg(icon)}
                  className="p-1 rounded bg-[#0b0d12] border border-specimen-border text-specimen-muted hover:text-white transition"
                  title="Copy SVG"
                >
                  {copiedId === icon.id ? <Check className="w-3 h-3 text-green-400" /> : <Copy className="w-3 h-3" />}
                </button>
                <button
                  onClick={() => downloadSvg(icon)}
                  className="p-1 rounded bg-[#0b0d12] border border-specimen-border text-specimen-muted hover:text-white transition"
                  title="Download .svg"
                >
                  <Download className="w-3 h-3" />
                </button>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
