import React from 'react';
import { Terminal, PlusCircle, Globe } from 'lucide-react';

interface EmptyStateProps {
  title: string;
  description: string;
  onOpenCapture: () => void;
  suggestedCommand?: string;
}

export const EmptyState: React.FC<EmptyStateProps> = ({
  title,
  description,
  onOpenCapture,
  suggestedCommand = 'specimen capture https://example.com',
}) => {
  const [copied, setCopied] = React.useState(false);

  const copyCmd = () => {
    navigator.clipboard.writeText(suggestedCommand);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  return (
    <div className="rounded-2xl border border-specimen-border bg-specimen-surface p-12 text-center max-w-2xl mx-auto my-12 shadow-sm">
      <div className="w-14 h-14 rounded-2xl bg-specimen-accent/10 border border-specimen-accent/20 flex items-center justify-center mx-auto mb-5 text-specimen-accent">
        <Globe className="w-7 h-7" />
      </div>
      <h3 className="text-xl font-bold text-white mb-2">{title}</h3>
      <p className="text-sm text-specimen-muted leading-relaxed max-w-lg mx-auto mb-6">
        {description}
      </p>

      {/* CLI Copy Box */}
      <div className="bg-[#0b0d12] border border-specimen-border rounded-xl p-3 max-w-md mx-auto mb-6 flex items-center justify-between font-mono text-xs text-white">
        <div className="flex items-center gap-2 truncate pr-2">
          <Terminal className="w-4 h-4 text-specimen-accent flex-shrink-0" />
          <span className="truncate text-specimen-text">{suggestedCommand}</span>
        </div>
        <button
          onClick={copyCmd}
          className="px-2.5 py-1 rounded bg-specimen-surface border border-specimen-border hover:bg-specimen-hover text-specimen-muted hover:text-white transition flex-shrink-0 font-sans text-xs"
        >
          {copied ? 'Copied' : 'Copy'}
        </button>
      </div>

      <div className="flex justify-center gap-3">
        <button
          onClick={onOpenCapture}
          className="flex items-center gap-2 px-5 py-2.5 rounded-lg bg-specimen-accent hover:bg-specimen-accent-hover text-white text-sm font-semibold shadow-md shadow-specimen-accent/25 transition active:scale-95"
        >
          <PlusCircle className="w-4 h-4" />
          <span>Capture a Website</span>
        </button>
      </div>
    </div>
  );
};
