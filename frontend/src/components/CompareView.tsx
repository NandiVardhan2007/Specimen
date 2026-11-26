import React, { useState } from 'react';
import { Item } from '../types';
import { compareItems } from '../api';
import { GitCompare, ArrowRight, Check, Copy } from 'lucide-react';

interface CompareViewProps {
  onOpenCapture: () => void;
}

export const CompareView: React.FC<CompareViewProps> = () => {
  const [idA, setIdA] = useState('');
  const [idB, setIdB] = useState('');
  const [itemA, setItemA] = useState<Item | null>(null);
  const [itemB, setItemB] = useState<Item | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [copiedCol, setCopiedCol] = useState<'a' | 'b' | null>(null);

  const handleCompare = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!idA && !idB) return;
    setIsLoading(true);
    const res = await compareItems(idA, idB);
    setItemA(res.item_a);
    setItemB(res.item_b);
    setIsLoading(false);
  };

  const copyPayload = (col: 'a' | 'b', data: any) => {
    navigator.clipboard.writeText(JSON.stringify(data, null, 2));
    setCopiedCol(col);
    setTimeout(() => setCopiedCol(null), 1500);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="pb-4 border-b border-specimen-border">
        <h2 className="text-xl font-bold text-white flex items-center gap-2">
          <GitCompare className="w-5 h-5 text-specimen-accent" />
          <span>Cross-Source Compare & Merge Review</span>
        </h2>
        <p className="text-xs text-specimen-muted mt-1">
          Inspect two design system items side-by-side to review differences, assess similarity links, or verify deduplication candidates.
        </p>
      </div>

      {/* Query Form */}
      <form onSubmit={handleCompare} className="p-4 rounded-xl border border-specimen-border bg-specimen-surface flex flex-col sm:flex-row gap-3">
        <input
          type="text"
          placeholder="Item A ID (e.g. palette-dark-core-a8f219c0)..."
          value={idA}
          onChange={(e) => setIdA(e.target.value)}
          className="flex-1 bg-[#0b0d12] border border-specimen-border rounded-lg px-4 py-2 text-sm text-white placeholder-specimen-muted outline-none focus:border-specimen-accent font-mono text-xs"
        />
        <input
          type="text"
          placeholder="Item B ID (e.g. palette-github-dark-90a1f2)..."
          value={idB}
          onChange={(e) => setIdB(e.target.value)}
          className="flex-1 bg-[#0b0d12] border border-specimen-border rounded-lg px-4 py-2 text-sm text-white placeholder-specimen-muted outline-none focus:border-specimen-accent font-mono text-xs"
        />
        <button
          type="submit"
          disabled={isLoading}
          className="px-5 py-2 rounded-lg bg-specimen-accent hover:bg-specimen-accent-hover text-white text-sm font-semibold transition active:scale-95 disabled:opacity-50"
        >
          {isLoading ? 'Comparing...' : 'Compare Side-by-Side'}
        </button>
      </form>

      {/* Side-by-side comparison cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Column A */}
        <div className="rounded-xl border border-specimen-border bg-specimen-surface p-5 space-y-3">
          <div className="flex items-center justify-between pb-3 border-b border-specimen-border">
            <h3 className="text-sm font-bold text-white">Item A Payload</h3>
            {itemA && (
              <button
                onClick={() => copyPayload('a', itemA)}
                className="flex items-center gap-1 text-xs text-specimen-muted hover:text-white"
              >
                {copiedCol === 'a' ? <Check className="w-3.5 h-3.5 text-green-400" /> : <Copy className="w-3.5 h-3.5" />}
                <span>Copy JSON</span>
              </button>
            )}
          </div>
          {itemA ? (
            <div className="space-y-3">
              <div className="flex items-center justify-between text-xs">
                <span className="font-bold text-white">{itemA.name || itemA.slug}</span>
                <span className="px-2 py-0.5 rounded bg-[#0b0d12] text-specimen-accent font-mono text-[11px]">
                  {itemA.item_type}
                </span>
              </div>
              <pre className="p-3 rounded-lg bg-[#050608] border border-specimen-border font-mono text-[11px] text-blue-300 overflow-x-auto max-h-96">
                {JSON.stringify(itemA, null, 2)}
              </pre>
            </div>
          ) : (
            <div className="py-12 text-center text-xs text-specimen-muted font-mono">
              {idA ? `Item '${idA}' not found in library.` : 'Specify Item A ID to inspect.'}
            </div>
          )}
        </div>

        {/* Column B */}
        <div className="rounded-xl border border-specimen-border bg-specimen-surface p-5 space-y-3">
          <div className="flex items-center justify-between pb-3 border-b border-specimen-border">
            <h3 className="text-sm font-bold text-white">Item B Payload</h3>
            {itemB && (
              <button
                onClick={() => copyPayload('b', itemB)}
                className="flex items-center gap-1 text-xs text-specimen-muted hover:text-white"
              >
                {copiedCol === 'b' ? <Check className="w-3.5 h-3.5 text-green-400" /> : <Copy className="w-3.5 h-3.5" />}
                <span>Copy JSON</span>
              </button>
            )}
          </div>
          {itemB ? (
            <div className="space-y-3">
              <div className="flex items-center justify-between text-xs">
                <span className="font-bold text-white">{itemB.name || itemB.slug}</span>
                <span className="px-2 py-0.5 rounded bg-[#0b0d12] text-specimen-accent font-mono text-[11px]">
                  {itemB.item_type}
                </span>
              </div>
              <pre className="p-3 rounded-lg bg-[#050608] border border-specimen-border font-mono text-[11px] text-blue-300 overflow-x-auto max-h-96">
                {JSON.stringify(itemB, null, 2)}
              </pre>
            </div>
          ) : (
            <div className="py-12 text-center text-xs text-specimen-muted font-mono">
              {idB ? `Item '${idB}' not found in library.` : 'Specify Item B ID to inspect.'}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
