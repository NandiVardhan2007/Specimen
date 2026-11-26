import React, { useState } from 'react';
import { Item, ComponentData, StateStyles } from '../types';
import { EmptyState } from './EmptyState';
import { Copy, Check, Code, Eye } from 'lucide-react';

interface ComponentsViewProps {
  components: Item<ComponentData>[];
  onOpenCapture: () => void;
}

export const ComponentsView: React.FC<ComponentsViewProps> = ({ components, onOpenCapture }) => {
  const [selectedId, setSelectedId] = useState<string>(components[0]?.id || '');
  const [activeState, setActiveState] = useState<'default' | 'hover' | 'focus' | 'active' | 'disabled'>('default');
  const [activeTab, setActiveTab] = useState<'preview' | 'html' | 'styles'>('preview');
  const [copied, setCopied] = useState(false);

  if (components.length === 0) {
    return (
      <EmptyState
        title="No Extracted Components in Library"
        description="Specimen extracts component roles (buttons, inputs, cards, badges), state variations, and sanitized HTML skeletons from rendered pages."
        onOpenCapture={onOpenCapture}
        suggestedCommand="specimen capture https://example.com"
      />
    );
  }

  const activeItem = components.find((c) => c.id === selectedId) || components[0];
  const { role, skeleton_html, variants = [] } = activeItem.data || {};
  const primaryVariant = variants[0];
  const stateStyles: StateStyles = (primaryVariant?.states && (primaryVariant.states as any)[activeState]) || primaryVariant?.states?.default || {};

  const copyCode = (text: string) => {
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="pb-4 border-b border-specimen-border flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div>
          <h2 className="text-xl font-bold text-white flex items-center gap-2">
            <span>Component States & Roles</span>
            <span className="text-xs px-2.5 py-0.5 rounded-full bg-specimen-accent/20 text-specimen-accent font-medium uppercase">
              Role: {role || 'Component'}
            </span>
          </h2>
          <p className="text-xs text-specimen-muted mt-1">{activeItem.name || activeItem.id}</p>
        </div>

        {/* Switcher */}
        {components.length > 1 && (
          <div className="flex items-center gap-2 flex-wrap">
            {components.map((c) => (
              <button
                key={c.id}
                onClick={() => setSelectedId(c.id)}
                className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition border ${
                  activeItem.id === c.id
                    ? 'bg-specimen-accent text-white border-specimen-accent'
                    : 'bg-specimen-surface text-specimen-muted border-specimen-border hover:text-white'
                }`}
              >
                {c.name || c.slug}
              </button>
            ))}
          </div>
        )}
      </div>

      {/* Main Component Workbench Card */}
      <div className="rounded-xl border border-specimen-border bg-specimen-surface overflow-hidden">
        {/* Controls bar */}
        <div className="p-4 border-b border-specimen-border bg-[#0b0d12]/50 flex flex-wrap items-center justify-between gap-4">
          {/* State Switcher */}
          <div className="flex items-center gap-1.5 bg-[#0b0d12] p-1 rounded-lg border border-specimen-border text-xs">
            {(['default', 'hover', 'focus', 'active', 'disabled'] as const).map((st) => (
              <button
                key={st}
                onClick={() => setActiveState(st)}
                className={`px-3 py-1 rounded font-medium capitalize transition ${
                  activeState === st
                    ? 'bg-specimen-accent text-white shadow-sm'
                    : 'text-specimen-muted hover:text-white'
                }`}
              >
                :{st}
              </button>
            ))}
          </div>

          {/* View Tab Switcher */}
          <div className="flex items-center gap-2">
            <button
              onClick={() => setActiveTab('preview')}
              className={`flex items-center gap-1.5 px-3 py-1 rounded text-xs font-medium transition ${
                activeTab === 'preview' ? 'bg-specimen-hover text-white' : 'text-specimen-muted hover:text-white'
              }`}
            >
              <Eye className="w-3.5 h-3.5" />
              <span>Preview</span>
            </button>
            <button
              onClick={() => setActiveTab('html')}
              className={`flex items-center gap-1.5 px-3 py-1 rounded text-xs font-medium transition ${
                activeTab === 'html' ? 'bg-specimen-hover text-white' : 'text-specimen-muted hover:text-white'
              }`}
            >
              <Code className="w-3.5 h-3.5" />
              <span>Skeleton HTML</span>
            </button>
          </div>
        </div>

        {/* Content Canvas */}
        <div className="p-12 min-h-[260px] flex items-center justify-center bg-gradient-to-b from-[#0b0d12]/30 to-[#0b0d12]/70">
          {activeTab === 'preview' ? (
            <div className="flex flex-col items-center gap-4">
              <div
                className="transition-all duration-150 inline-flex items-center justify-center"
                style={{
                  backgroundColor: stateStyles.background || '#7c5cff',
                  color: stateStyles.color || '#ffffff',
                  border: stateStyles.border || 'none',
                  boxShadow: stateStyles.shadow || 'none',
                  opacity: stateStyles.opacity !== undefined ? stateStyles.opacity : 1,
                  transform: stateStyles.transform || 'none',
                  padding: '10px 24px',
                  borderRadius: '8px',
                  fontSize: '14px',
                  fontWeight: 600,
                  cursor: activeState === 'disabled' ? 'not-allowed' : 'pointer',
                }}
              >
                {activeItem.name || 'Interactive Component'}
              </div>
              <span className="text-[11px] font-mono text-specimen-muted">
                Displaying pseudo-state <code className="text-specimen-accent">:{activeState}</code>
              </span>
            </div>
          ) : (
            <div className="w-full max-w-xl">
              <div className="flex justify-end mb-2">
                <button
                  onClick={() => copyCode(skeleton_html || '')}
                  className="flex items-center gap-1 px-2.5 py-1 rounded bg-[#0b0d12] border border-specimen-border text-xs text-specimen-muted hover:text-white transition"
                >
                  {copied ? <Check className="w-3 h-3 text-green-400" /> : <Copy className="w-3 h-3" />}
                  <span>Copy HTML</span>
                </button>
              </div>
              <pre className="p-4 rounded-lg bg-[#050608] border border-specimen-border font-mono text-xs text-blue-300 overflow-x-auto">
                {skeleton_html || '<!-- No skeleton HTML available -->'}
              </pre>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
