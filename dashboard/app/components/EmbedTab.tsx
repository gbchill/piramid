/**
 * Embed Tab - Convert text to vectors using embedding providers
 */
"use client";

import { useState } from 'react';
import { embedText, embedBatch, EmbedResponse, EmbedBatchResponse, APIError } from '../lib/api';
import { ErrorDisplay } from './ErrorDisplay';

interface EmbedTabProps {
  collection: string;
}

export function EmbedTab({ collection }: EmbedTabProps) {
  const [mode, setMode] = useState<'single' | 'batch'>('single');
  
  // Single embed
  const [text, setText] = useState('');
  const [metadata, setMetadata] = useState('{}');
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<EmbedResponse | null>(null);
  const [error, setError] = useState<Error | APIError | null>(null);
  
  // Batch embed
  const [batchText, setBatchText] = useState('');
  const [batchLoading, setBatchLoading] = useState(false);
  const [batchResult, setBatchResult] = useState<EmbedBatchResponse | null>(null);
  const [batchError, setBatchError] = useState<Error | APIError | null>(null);

  async function handleEmbed() {
    if (!text.trim()) return;
    
    try {
      setLoading(true);
      setResult(null);
      setError(null);
      
      const metadataObj = metadata.trim() ? JSON.parse(metadata) : {};
      const res = await embedText(collection, {
        text: text.trim(),
        metadata: metadataObj,
      });
      
      setResult(res);
      setText('');
    } catch (e) {
      setError(e instanceof Error ? e : new Error('Embedding failed'));
    } finally {
      setLoading(false);
    }
  }

  async function handleBatchEmbed() {
    const texts = batchText.split('\n').filter(t => t.trim());
    if (texts.length === 0) return;
    
    try {
      setBatchLoading(true);
      setBatchResult(null);
      setBatchError(null);
      
      const res = await embedBatch(collection, { texts });
      
      setBatchResult(res);
      setBatchText('');
    } catch (e) {
      setBatchError(e instanceof Error ? e : new Error('Batch embedding failed'));
    } finally {
      setBatchLoading(false);
    }
  }

  return (
    <div className="space-y-6">
      {/* Mode Toggle */}
      <div className="flex gap-2">
        <button
          onClick={() => setMode('single')}
          className={`px-4 py-2 rounded-lg ${
            mode === 'single' 
              ? 'bg-[var(--accent)] text-white' 
              : 'bg-[var(--bg-secondary)] border border-[var(--border-color)]'
          }`}
        >
          Single Text
        </button>
        <button
          onClick={() => setMode('batch')}
          className={`px-4 py-2 rounded-lg ${
            mode === 'batch' 
              ? 'bg-[var(--accent)] text-white' 
              : 'bg-[var(--bg-secondary)] border border-[var(--border-color)]'
          }`}
        >
          Batch
        </button>
      </div>

      {mode === 'single' ? (
        <>
          {/* Single Embed Form */}
          <div className="bg-[var(--bg-secondary)] rounded-xl p-6 border border-[var(--border-color)]">
            <h3 className="font-semibold mb-4">Embed Text</h3>
            <p className="text-sm text-[var(--text-secondary)] mb-4">
              Convert text to a vector using the configured embedding provider (OpenAI, Ollama, etc.)
            </p>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm text-[var(--text-secondary)] mb-1">
                  Text
                </label>
                <textarea
                  value={text}
                  onChange={(e) => setText(e.target.value)}
                  placeholder="Enter the text you want to embed..."
                  rows={4}
                  className="w-full px-4 py-2 bg-[var(--bg-tertiary)] border border-[var(--border-color)] rounded-lg focus:outline-none focus:border-[var(--accent)] resize-none"
                />
              </div>
              
              <div>
                <label className="block text-sm text-[var(--text-secondary)] mb-1">
                  Metadata (JSON)
                </label>
                <input
                  type="text"
                  value={metadata}
                  onChange={(e) => setMetadata(e.target.value)}
                  placeholder='{"category": "example"}'
                  className="w-full px-4 py-2 bg-[var(--bg-tertiary)] border border-[var(--border-color)] rounded-lg focus:outline-none focus:border-[var(--accent)]"
                />
              </div>
              
              <button
                onClick={handleEmbed}
                disabled={loading || !text.trim()}
                className="px-6 py-2 bg-[var(--accent)] hover:bg-[var(--accent-hover)] rounded-lg disabled:opacity-50"
              >
                {loading ? 'Embedding...' : 'Embed & Store'}
              </button>
            </div>
          </div>

          {/* Error Display */}
          {error && <ErrorDisplay error={error} onDismiss={() => setError(null)} />}

          {/* Single Result */}
          {result && (
            <div className="bg-[var(--bg-secondary)] rounded-xl p-6 border border-[var(--border-color)]">
              <h3 className="font-semibold mb-4">Embedded Successfully</h3>
              <div className="space-y-2 text-sm">
                <div className="flex items-center justify-between">
                  <span className="text-[var(--text-secondary)]">Vector ID:</span>
                  <span className="font-mono text-[var(--accent)]">{result.id}</span>
                </div>
                <div className="flex items-center justify-between">
                  <span className="text-[var(--text-secondary)]">Dimensions:</span>
                  <span>{result.embedding.length}</span>
                </div>
                {result.tokens && (
                  <div className="flex items-center justify-between">
                    <span className="text-[var(--text-secondary)]">Tokens Used:</span>
                    <span>{result.tokens}</span>
                  </div>
                )}
                <div className="mt-4">
                  <span className="text-[var(--text-secondary)] block mb-1">Embedding Preview:</span>
                  <div className="font-mono text-xs bg-[var(--bg-tertiary)] p-3 rounded border border-[var(--border-color)] overflow-x-auto">
                    [{result.embedding.slice(0, 5).map(n => n.toFixed(4)).join(', ')}, ...]
                  </div>
                </div>
              </div>
            </div>
          )}
        </>
      ) : (
        <>
          {/* Batch Embed Form */}
          <div className="bg-[var(--bg-secondary)] rounded-xl p-6 border border-[var(--border-color)]">
            <h3 className="font-semibold mb-4">Batch Embed</h3>
            <p className="text-sm text-[var(--text-secondary)] mb-4">
              Embed multiple texts at once. Enter one text per line.
            </p>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm text-[var(--text-secondary)] mb-1">
                  Texts (one per line)
                </label>
                <textarea
                  value={batchText}
                  onChange={(e) => setBatchText(e.target.value)}
                  placeholder="First document&#10;Second document&#10;Third document"
                  rows={8}
                  className="w-full px-4 py-2 bg-[var(--bg-tertiary)] border border-[var(--border-color)] rounded-lg focus:outline-none focus:border-[var(--accent)] resize-none font-mono text-sm"
                />
              </div>
              
              <button
                onClick={handleBatchEmbed}
                disabled={batchLoading || !batchText.trim()}
                className="px-6 py-2 bg-[var(--accent)] hover:bg-[var(--accent-hover)] rounded-lg disabled:opacity-50"
              >
                {batchLoading ? 'Embedding...' : `Embed ${batchText.split('\n').filter(t => t.trim()).length} Texts`}
              </button>
            </div>
          </div>

          {/* Error Display */}
          {batchError && <ErrorDisplay error={batchError} onDismiss={() => setBatchError(null)} />}

          {/* Batch Result */}
          {batchResult && (
            <div className="bg-[var(--bg-secondary)} rounded-xl p-6 border border-[var(--border-color)]">
              <h3 className="font-semibold mb-4">Batch Embedded Successfully</h3>
              <div className="space-y-2 text-sm">
                <div className="flex items-center justify-between">
                  <span className="text-[var(--text-secondary)]">Vectors Created:</span>
                  <span className="font-semibold text-[var(--success)]">{batchResult.ids.length}</span>
                </div>
                {batchResult.total_tokens && (
                  <div className="flex items-center justify-between">
                    <span className="text-[var(--text-secondary)]">Total Tokens:</span>
                    <span>{batchResult.total_tokens}</span>
                  </div>
                )}
                <div className="mt-4">
                  <span className="text-[var(--text-secondary)] block mb-1">Vector IDs:</span>
                  <div className="font-mono text-xs bg-[var(--bg-tertiary)] p-3 rounded border border-[var(--border-color)] max-h-40 overflow-y-auto space-y-1">
                    {batchResult.ids.map((id, i) => (
                      <div key={id}>{i + 1}. {id}</div>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          )}
        </>
      )}
    </div>
  );
}
