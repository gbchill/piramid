/**
 * API Client - Helper functions for calling the Piramid REST API
 *
 * This module handles all HTTP communication with the server.
 * Keeps API logic separate from UI components.
 */

const API_BASE = typeof window !== 'undefined' 
  ? `${window.location.protocol}//${window.location.hostname}:6333/api`
  : 'http://localhost:6333/api';

// Enhanced error type with more details
export class APIError extends Error {
  constructor(
    message: string,
    public status: number,
    public statusText: string,
    public endpoint: string,
    public details?: unknown
  ) {
    super(message);
    this.name = 'APIError';
  }

  toString(): string {
    return `[${this.status} ${this.statusText}] ${this.message}`;
  }
}

// Generic fetch wrapper with enhanced error handling
export async function fetchAPI<T>(endpoint: string, options?: RequestInit): Promise<T> {
  try {
    const res = await fetch(`${API_BASE}${endpoint}`, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...options?.headers,
      },
    });
    
    if (!res.ok) {
      // Try to parse error response
      let errorData;
      let errorMessage = res.statusText;
      
      try {
        const text = await res.text();
        if (text) {
          errorData = JSON.parse(text);
          errorMessage = errorData.message || errorData.error || text;
        }
      } catch {
        // If parsing fails, use status text
        errorMessage = res.statusText || 'Request failed';
      }
      
      throw new APIError(
        errorMessage,
        res.status,
        res.statusText,
        endpoint,
        errorData
      );
    }
    
    if (res.status === 204) {
      return {} as T;
    }
    
    return res.json();
  } catch (error) {
    // If it's already an APIError, re-throw
    if (error instanceof APIError) {
      throw error;
    }
    
    // Network error or other issues
    throw new APIError(
      error instanceof Error ? error.message : 'Network error',
      0,
      'Network Error',
      endpoint
    );
  }
}

// =============================================================================
// HEALTH
// =============================================================================

export async function checkHealth(): Promise<boolean> {
  try {
    await fetchAPI('/health');
    return true;
  } catch {
    return false;
  }
}

// =============================================================================
// COLLECTIONS
// =============================================================================

export interface Collection {
  name: string;
  count: number;  // Rust server returns 'count' not 'vector_count'
}

export async function listCollections(): Promise<Collection[]> {
  const data = await fetchAPI<{ collections: Collection[] }>('/collections');
  return data.collections;
}

export async function createCollection(name: string): Promise<Collection> {
  return fetchAPI<Collection>('/collections', {
    method: 'POST',
    body: JSON.stringify({ name }),
  });
}

export async function deleteCollection(name: string): Promise<void> {
  await fetchAPI(`/collections/${name}`, { method: 'DELETE' });
}

// =============================================================================
// VECTORS
// =============================================================================

export interface InsertVectorRequest {
  vector: number[];
  text?: string;
  metadata?: Record<string, unknown>;
}

export async function insertVector(
  collection: string, 
  data: InsertVectorRequest
): Promise<{ id: string }> {
  return fetchAPI(`/collections/${collection}/vectors`, {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

export async function deleteVector(collection: string, id: string): Promise<void> {
  await fetchAPI(`/collections/${collection}/vectors/${id}`, { method: 'DELETE' });
}

// =============================================================================
// SEARCH
// =============================================================================

export interface SearchRequest {
  vector: number[];
  k?: number;  // Rust uses 'k' not 'limit'
  metric?: 'cosine' | 'euclidean' | 'dot';
}

export interface SearchResult {
  id: string;
  score: number;
  text: string;
  metadata: Record<string, unknown>;
}

export interface SearchResponse {
  results: SearchResult[];
  took_ms?: number;
}

export async function searchVectors(
  collection: string, 
  query: SearchRequest
): Promise<SearchResponse> {
  return fetchAPI(`/collections/${collection}/search`, {
    method: 'POST',
    body: JSON.stringify(query),
  });
}

// =============================================================================
// EMBEDDINGS
// =============================================================================

export interface EmbedRequest {
  text: string;
  metadata?: Record<string, unknown>;
}

export interface EmbedResponse {
  id: string;
  embedding: number[];
  tokens?: number;
}

export async function embedText(
  collection: string,
  data: EmbedRequest
): Promise<EmbedResponse> {
  return fetchAPI(`/collections/${collection}/embed`, {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

export interface EmbedBatchRequest {
  texts: string[];
  metadata?: Record<string, unknown>[];
}

export interface EmbedBatchResponse {
  ids: string[];
  total_tokens?: number;
}

export async function embedBatch(
  collection: string,
  data: EmbedBatchRequest
): Promise<EmbedBatchResponse> {
  return fetchAPI(`/collections/${collection}/embed/batch`, {
    method: 'POST',
    body: JSON.stringify(data),
  });
}

export interface TextSearchRequest {
  query: string;
  k?: number;
  metric?: 'cosine' | 'euclidean' | 'dot';
}

export async function searchByText(
  collection: string,
  query: TextSearchRequest
): Promise<SearchResponse> {
  return fetchAPI(`/collections/${collection}/search/text`, {
    method: 'POST',
    body: JSON.stringify(query),
  });
}
