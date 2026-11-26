import React, { useState } from 'react';
import { Item, AnimationData } from '../types';
import { EmptyState } from './EmptyState';
import { Play, Pause, RotateCcw, Clock, Activity } from 'lucide-react';

interface AnimationsViewProps {
  animations: Item<AnimationData>[];
  onOpenCapture: () => void;
}

export const AnimationsView: React.FC<AnimationsViewProps> = ({ animations, onOpenCapture }) => {
  const [isPlaying, setIsPlaying] = useState<Record<string, boolean>>({});

  if (animations.length === 0) {
    return (
      <EmptyState
        title="No Motion or Keyframes in Library"
        description="Specimen captures CSS @keyframes, CSS transitions, and Web Animations API (WAAPI) calls with timing curves and keyframe styles."
        onOpenCapture={onOpenCapture}
        suggestedCommand="specimen capture https://example.com"
      />
    );
  }

  const togglePlay = (id: string) => {
    setIsPlaying((prev) => ({ ...prev, [id]: !prev[id] }));
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="pb-4 border-b border-specimen-border">
        <h2 className="text-xl font-bold text-white flex items-center gap-2">
          <span>Motion, Keyframes & WAAPI</span>
          <span className="text-xs px-2.5 py-0.5 rounded-full bg-specimen-accent/20 text-specimen-accent font-medium">
            {animations.length} Animations
          </span>
        </h2>
        <p className="text-xs text-specimen-muted mt-1">
          Measured animation keyframes, easing curves, and duration parameters.
        </p>
      </div>

      {/* Grid of Motion Items */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {animations.map((anim) => {
          const data = anim.data || {};
          const timing = data.timing || { duration_ms: 300, easing: 'ease', iterations: 1 };
          const active = isPlaying[anim.id] !== false; // default to playing

          return (
            <div
              key={anim.id}
              className="rounded-xl border border-specimen-border bg-specimen-surface p-5 space-y-4 shadow-sm"
            >
              <div className="flex items-center justify-between">
                <div>
                  <h3 className="text-base font-bold text-white">{anim.name || anim.slug}</h3>
                  <span className="text-[11px] font-mono text-specimen-muted">Driver: {data.driver || 'css-keyframe'}</span>
                </div>
                <button
                  onClick={() => togglePlay(anim.id)}
                  className="p-2 rounded-lg bg-[#0b0d12] border border-specimen-border text-specimen-text hover:text-white transition"
                >
                  {active ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4 text-specimen-accent" />}
                </button>
              </div>

              {/* Looping preview stage */}
              <div className="h-40 rounded-xl bg-[#0b0d12] border border-specimen-border flex items-center justify-center overflow-hidden relative">
                <div
                  className={`w-16 h-16 rounded-xl bg-gradient-to-tr from-specimen-accent to-indigo-500 shadow-lg shadow-specimen-accent/30 ${
                    active ? 'animate-pulse' : ''
                  }`}
                  style={{
                    animationDuration: `${timing.duration_ms || 300}ms`,
                    animationTimingFunction: timing.easing || 'ease',
                  }}
                />
              </div>

              {/* Timing metrics bar */}
              <div className="grid grid-cols-3 gap-2 text-center text-xs">
                <div className="p-2 rounded bg-[#0b0d12] border border-specimen-border">
                  <span className="text-specimen-muted text-[10px] block">Duration</span>
                  <span className="font-mono font-bold text-white">{timing.duration_ms || 300} ms</span>
                </div>
                <div className="p-2 rounded bg-[#0b0d12] border border-specimen-border col-span-2 truncate">
                  <span className="text-specimen-muted text-[10px] block">Easing Curve</span>
                  <span className="font-mono text-white text-[11px] truncate block" title={timing.easing}>
                    {timing.easing || 'linear'}
                  </span>
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
