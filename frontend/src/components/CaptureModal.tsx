import React, { useState } from 'react';
import { X, Globe, Terminal, Play, ShieldAlert, Copy, Check } from 'lucide-react';

interface CaptureModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const CaptureModal: React.FC<CaptureModalProps> = ({ isOpen, onClose }) => {
  const [targetUrl, setTargetUrl] = useState('https://');
  const [mode, setMode] = useState<'page' | 'crawl'>('page');
  const [isHeaded, setIsHeaded] = useState(false);
  const [isDryRun, setIsDryRun] = useState(false);
  const [allowPrivate, setAllowPrivate] = useState(false);
  const [copied, setCopied] = useState(false);

  if (!isOpen) return null;

  const buildCliCommand = () => {
    let cmd = `specimen ${mode === 'crawl' ? 'crawl' : 'capture'} "${targetUrl || 'https://example.com'}"`;
    if (isDryRun) cmd += ' --dry-run';
    if (isHeaded) cmd += ' --headed';
    if (allowPrivate) cmd += ' --allow-private';
    return cmd;
  };

  const copyCli = () => {
    navigator.clipboard.writeText(buildCliCommand());
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/75 backdrop-blur-sm animate-fade-in">
      <div className="w-full max-w-lg rounded-2xl border border-specimen-border bg-specimen-surface p-6 shadow-2xl space-y-5">
        {/* Header */}
        <div className="flex items-center justify-between pb-3 border-b border-specimen-border">
          <div className="flex items-center gap-2">
            <Globe className="w-5 h-5 text-specimen-accent" />
            <h3 className="text-base font-bold text-white">Capture Target Website</h3>
          </div>
          <button onClick={onClose} className="p-1 rounded text-specimen-muted hover:text-white transition">
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Inputs */}
        <div className="space-y-4 text-xs">
          <div>
            <label className="block font-semibold text-white mb-1.5">Target Website URL</label>
            <input
              type="url"
              value={targetUrl}
              onChange={(e) => setTargetUrl(e.target.value)}
              placeholder="https://example.com"
              className="w-full bg-[#0b0d12] border border-specimen-border rounded-lg px-3.5 py-2 text-white font-mono placeholder-specimen-muted focus:border-specimen-accent outline-none"
            />
          </div>

          <div>
            <label className="block font-semibold text-white mb-1.5">Capture Scope</label>
            <div className="grid grid-cols-2 gap-2">
              <button
                type="button"
                onClick={() => setMode('page')}
                className={`py-2 px-3 rounded-lg border text-left font-medium transition ${
                  mode === 'page'
                    ? 'bg-specimen-accent text-white border-specimen-accent'
                    : 'bg-[#0b0d12] text-specimen-muted border-specimen-border hover:text-white'
                }`}
              >
                <span className="font-bold block">Single Page</span>
                <span className="text-[10px] text-white/70">Responsive viewports & schemes</span>
              </button>
              <button
                type="button"
                onClick={() => setMode('crawl')}
                className={`py-2 px-3 rounded-lg border text-left font-medium transition ${
                  mode === 'crawl'
                    ? 'bg-specimen-accent text-white border-specimen-accent'
                    : 'bg-[#0b0d12] text-specimen-muted border-specimen-border hover:text-white'
                }`}
              >
                <span className="font-bold block">Site Crawl</span>
                <span className="text-[10px] text-white/70">Bounded breadth-first walk</span>
              </button>
            </div>
          </div>

          {/* Flags */}
          <div className="space-y-2 pt-2 border-t border-specimen-border/60">
            <label className="flex items-center gap-2 cursor-pointer text-specimen-muted hover:text-white">
              <input
                type="checkbox"
                checked={isDryRun}
                onChange={(e) => setIsDryRun(e.target.checked)}
                className="rounded accent-specimen-accent"
              />
              <span>--dry-run (Simulate execution without launching browser or writing to disk)</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer text-specimen-muted hover:text-white">
              <input
                type="checkbox"
                checked={isHeaded}
                onChange={(e) => setIsHeaded(e.target.checked)}
                className="rounded accent-specimen-accent"
              />
              <span>--headed (Visible browser window for manual OAuth or two-factor sign-in)</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer text-specimen-muted hover:text-white">
              <input
                type="checkbox"
                checked={allowPrivate}
                onChange={(e) => setAllowPrivate(e.target.checked)}
                className="rounded accent-specimen-accent"
              />
              <span className="flex items-center gap-1 text-amber-400">
                <ShieldAlert className="w-3.5 h-3.5" />
                --allow-private (Allow localhost and private network addresses)
              </span>
            </label>
          </div>

          {/* Generated Command Box */}
          <div>
            <label className="block font-semibold text-white mb-1.5 flex items-center justify-between">
              <span>Execute via Terminal</span>
              <button onClick={copyCli} className="text-specimen-accent hover:underline flex items-center gap-1">
                {copied ? <Check className="w-3 h-3 text-green-400" /> : <Copy className="w-3 h-3" />}
                <span>{copied ? 'Copied' : 'Copy'}</span>
              </button>
            </label>
            <div className="p-3 rounded-lg bg-[#050608] border border-specimen-border font-mono text-xs text-green-400 select-all overflow-x-auto">
              {buildCliCommand()}
            </div>
          </div>
        </div>

        {/* Action Button */}
        <div className="flex justify-end gap-3 pt-3 border-t border-specimen-border">
          <button
            onClick={onClose}
            className="px-4 py-2 rounded-lg bg-[#0b0d12] hover:bg-specimen-hover text-specimen-muted hover:text-white text-xs font-semibold transition"
          >
            Close
          </button>
          <button
            onClick={copyCli}
            className="px-5 py-2 rounded-lg bg-specimen-accent hover:bg-specimen-accent-hover text-white text-xs font-semibold transition flex items-center gap-1.5 shadow-md shadow-specimen-accent/25"
          >
            <Terminal className="w-3.5 h-3.5" />
            <span>Copy Terminal Command</span>
          </button>
        </div>
      </div>
    </div>
  );
};
