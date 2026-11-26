import React from 'react';
import { SourceRecord } from '../api';
import { EmptyState } from './EmptyState';
import { Globe, Calendar, FileText, CheckCircle2 } from 'lucide-react';

interface SourcesViewProps {
  sources: SourceRecord[];
  onOpenCapture: () => void;
}

export const SourcesView: React.FC<SourcesViewProps> = ({ sources, onOpenCapture }) => {
  if (sources.length === 0) {
    return (
      <EmptyState
        title="No Captured Sources in Library"
        description="Captured websites are recorded under sources/<domain>/<timestamp>/ with DOM snapshots, stylesheets, and network logs."
        onOpenCapture={onOpenCapture}
        suggestedCommand="specimen capture https://example.com"
      />
    );
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="pb-4 border-b border-specimen-border">
        <h2 className="text-xl font-bold text-white flex items-center gap-2">
          <span>Captured Sources & Evidence</span>
          <span className="text-xs px-2.5 py-0.5 rounded-full bg-specimen-accent/20 text-specimen-accent font-medium">
            {sources.length} Sources
          </span>
        </h2>
        <p className="text-xs text-specimen-muted mt-1">
          Historical record of captured domains, manifests, and raw evidence bundles.
        </p>
      </div>

      {/* Sources List */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {sources.map((s) => (
          <div
            key={s.path}
            className="p-5 rounded-xl border border-specimen-border bg-specimen-surface space-y-3 shadow-sm hover:border-specimen-accent/40 transition"
          >
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Globe className="w-4 h-4 text-specimen-accent" />
                <h3 className="text-base font-bold text-white">{s.host}</h3>
              </div>
              {s.has_manifest && (
                <span className="inline-flex items-center gap-1 text-[11px] text-green-400 font-semibold px-2 py-0.5 rounded bg-green-950/40 border border-green-800/40">
                  <CheckCircle2 className="w-3 h-3" /> Manifest OK
                </span>
              )}
            </div>

            <div className="text-xs font-mono text-specimen-muted flex items-center gap-2">
              <Calendar className="w-3.5 h-3.5 text-specimen-muted" />
              <span>Timestamp: {s.timestamp}</span>
            </div>

            <div className="pt-2 border-t border-specimen-border/60 text-xs text-specimen-muted flex justify-between">
              <span>Path: sources/{s.path}</span>
              <span className="text-specimen-accent font-medium">Recorded bundle</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
