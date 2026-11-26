import React, { useState, useEffect } from 'react';
import { fetchExportTokens } from '../api';
import { Download, Copy, Check, FileCode } from 'lucide-react';

export const ExportView: React.FC = () => {
  const [selectedFormat, setSelectedFormat] = useState('css');
  const [tokensContent, setTokensContent] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [copied, setCopied] = useState(false);

  const formats = [
    { id: 'css', label: 'CSS Variables', desc: ':root and dark scheme media queries', ext: '.css' },
    { id: 'tailwind', label: 'Tailwind CSS v4', desc: '@theme declaration block', ext: '.css' },
    { id: 'tailwind-v3', label: 'Tailwind CSS v3', desc: 'module.exports preset', ext: '.js' },
    { id: 'scss', label: 'SCSS Variables', desc: 'Sass maps and variable definitions', ext: '.scss' },
    { id: 'dtcg', label: 'W3C DTCG Tokens', desc: 'Design Tokens Community Group JSON', ext: '.tokens.json' },
    { id: 'tokens-studio', label: 'Tokens Studio (Figma)', desc: 'Tokens Studio plugin sets format', ext: '.json' },
  ];

  useEffect(() => {
    let isMounted = true;
    setIsLoading(true);
    fetchExportTokens(selectedFormat).then((content) => {
      if (isMounted) {
        setTokensContent(content);
        setIsLoading(false);
      }
    });
    return () => {
      isMounted = false;
    };
  }, [selectedFormat]);

  const copyCode = () => {
    navigator.clipboard.writeText(tokensContent);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  const downloadFile = () => {
    const fmt = formats.find((f) => f.id === selectedFormat);
    const blob = new Blob([tokensContent], { type: 'text/plain;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `tokens${fmt?.ext || '.txt'}`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="pb-4 border-b border-specimen-border">
        <h2 className="text-xl font-bold text-white flex items-center gap-2">
          <Download className="w-5 h-5 text-specimen-accent" />
          <span>Export Design Tokens</span>
        </h2>
        <p className="text-xs text-specimen-muted mt-1">
          Export verified colors, font families, modular scales, spacing, and radii into production framework tokens.
        </p>
      </div>

      {/* Format Selector Pills */}
      <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3">
        {formats.map((fmt) => (
          <button
            key={fmt.id}
            onClick={() => setSelectedFormat(fmt.id)}
            className={`p-3 rounded-xl border text-left transition flex flex-col justify-between ${
              selectedFormat === fmt.id
                ? 'bg-specimen-accent text-white border-specimen-accent shadow-md shadow-specimen-accent/20'
                : 'bg-specimen-surface text-specimen-muted border-specimen-border hover:border-specimen-accent/40 hover:text-white'
            }`}
          >
            <span className="text-xs font-bold block">{fmt.label}</span>
            <span className={`text-[10px] mt-1 block truncate ${selectedFormat === fmt.id ? 'text-white/80' : 'text-specimen-muted'}`}>
              {fmt.desc}
            </span>
          </button>
        ))}
      </div>

      {/* Code Editor Preview Stage */}
      <div className="rounded-xl border border-specimen-border bg-specimen-surface overflow-hidden shadow-sm">
        {/* Editor Toolbar */}
        <div className="px-4 py-3 bg-[#0b0d12] border-b border-specimen-border flex items-center justify-between">
          <div className="flex items-center gap-2">
            <FileCode className="w-4 h-4 text-specimen-accent" />
            <span className="text-xs font-mono text-specimen-muted font-medium">
              tokens{formats.find((f) => f.id === selectedFormat)?.ext}
            </span>
          </div>
          <div className="flex items-center gap-2">
            <button
              onClick={copyCode}
              disabled={isLoading || !tokensContent}
              className="flex items-center gap-1.5 px-3 py-1 rounded-lg bg-specimen-surface border border-specimen-border text-xs text-white hover:bg-specimen-hover transition active:scale-95 disabled:opacity-50"
            >
              {copied ? <Check className="w-3.5 h-3.5 text-green-400" /> : <Copy className="w-3.5 h-3.5" />}
              <span>{copied ? 'Copied' : 'Copy'}</span>
            </button>
            <button
              onClick={downloadFile}
              disabled={isLoading || !tokensContent}
              className="flex items-center gap-1.5 px-3 py-1 rounded-lg bg-specimen-accent hover:bg-specimen-accent-hover text-xs font-semibold text-white transition active:scale-95 disabled:opacity-50"
            >
              <Download className="w-3.5 h-3.5" />
              <span>Download</span>
            </button>
          </div>
        </div>

        {/* Code Content */}
        <pre className="p-6 font-mono text-xs text-blue-300 bg-[#050608] overflow-x-auto min-h-[320px] max-h-[500px]">
          {isLoading ? '// Generating export tokens...' : tokensContent || '// No tokens found to export'}
        </pre>
      </div>
    </div>
  );
};
