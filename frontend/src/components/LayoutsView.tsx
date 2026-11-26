import React, { useState } from 'react';
import { Item, LayoutData } from '../types';
import { EmptyState } from './EmptyState';
import { Layout, Smartphone, Tablet, Monitor } from 'lucide-react';

interface LayoutsViewProps {
  layouts: Item<LayoutData>[];
  onOpenCapture: () => void;
}

export const LayoutsView: React.FC<LayoutsViewProps> = ({ layouts, onOpenCapture }) => {
  const [viewportWidth, setViewportWidth] = useState<'mobile' | 'tablet' | 'desktop'>('desktop');

  if (layouts.length === 0) {
    return (
      <EmptyState
        title="No Layout Skeletons in Library"
        description="Specimen detects layout landmarks (header, nav, hero, grid, footer) and records content-free skeletons with relative positions and grid properties."
        onOpenCapture={onOpenCapture}
        suggestedCommand="specimen capture https://example.com"
      />
    );
  }

  const widthPx = viewportWidth === 'mobile' ? '390px' : viewportWidth === 'tablet' ? '768px' : '100%';

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="pb-4 border-b border-specimen-border flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div>
          <h2 className="text-xl font-bold text-white flex items-center gap-2">
            <span>Semantic Layout Skeletons</span>
            <span className="text-xs px-2.5 py-0.5 rounded-full bg-specimen-accent/20 text-specimen-accent font-medium">
              {layouts.length} Sections
            </span>
          </h2>
          <p className="text-xs text-specimen-muted mt-1">
            Content-free wireframe skeletons capturing flex and grid layout rules.
          </p>
        </div>

        {/* Viewport switch */}
        <div className="flex items-center bg-[#0b0d12] p-1 rounded-lg border border-specimen-border text-xs">
          <button
            onClick={() => setViewportWidth('mobile')}
            className={`flex items-center gap-1.5 px-3 py-1 rounded font-medium transition ${
              viewportWidth === 'mobile' ? 'bg-specimen-accent text-white' : 'text-specimen-muted hover:text-white'
            }`}
          >
            <Smartphone className="w-3.5 h-3.5" />
            <span>Mobile (390px)</span>
          </button>
          <button
            onClick={() => setViewportWidth('tablet')}
            className={`flex items-center gap-1.5 px-3 py-1 rounded font-medium transition ${
              viewportWidth === 'tablet' ? 'bg-specimen-accent text-white' : 'text-specimen-muted hover:text-white'
            }`}
          >
            <Tablet className="w-3.5 h-3.5" />
            <span>Tablet (768px)</span>
          </button>
          <button
            onClick={() => setViewportWidth('desktop')}
            className={`flex items-center gap-1.5 px-3 py-1 rounded font-medium transition ${
              viewportWidth === 'desktop' ? 'bg-specimen-accent text-white' : 'text-specimen-muted hover:text-white'
            }`}
          >
            <Monitor className="w-3.5 h-3.5" />
            <span>Desktop</span>
          </button>
        </div>
      </div>

      {/* Wireframe Stage Container */}
      <div className="flex justify-center bg-[#050608] border border-specimen-border rounded-xl p-6 overflow-x-auto min-h-[380px]">
        <div
          className="transition-all duration-300 w-full space-y-4"
          style={{ maxWidth: widthPx }}
        >
          {layouts.map((lay) => {
            const data = lay.data || {};
            return (
              <div
                key={lay.id}
                className="p-5 rounded-xl border border-dashed border-specimen-border bg-specimen-surface/60 space-y-3"
              >
                <div className="flex items-center justify-between text-xs text-specimen-muted">
                  <span className="font-bold text-white uppercase tracking-wider">{data.section_name || lay.slug}</span>
                  <span className="font-mono">{data.display || 'flex'} ({data.columns || 'auto'})</span>
                </div>

                {/* Simulated Wireframe boxes */}
                <div
                  className="grid gap-3"
                  style={{
                    gridTemplateColumns:
                      viewportWidth === 'mobile' ? '1fr' : data.columns || 'repeat(3, 1fr)',
                  }}
                >
                  <div className="h-16 rounded-lg bg-specimen-hover/50 border border-specimen-border/60 flex items-center justify-center text-[11px] text-specimen-muted font-mono">
                    Element 1
                  </div>
                  <div className="h-16 rounded-lg bg-specimen-hover/50 border border-specimen-border/60 flex items-center justify-center text-[11px] text-specimen-muted font-mono">
                    Element 2
                  </div>
                  <div className="h-16 rounded-lg bg-specimen-hover/50 border border-specimen-border/60 flex items-center justify-center text-[11px] text-specimen-muted font-mono">
                    Element 3
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};
