/**
 * Type definitions for LLM Memory Graph Client
 * These types mirror the protobuf definitions for TypeScript usage
 */

// Session types
export interface Session {
  id: string;
  createdAt: Date;
  updatedAt: Date;
  metadata: Record<string, string>;
  isActive: boolean;
}

export interface CreateSessionOptions {
  metadata?: Record<string, string>;
}

// Node types
export enum NodeType {
  UNSPECIFIED = 0,
  SESSION = 1,
  PROMPT = 2,
  RESPONSE = 3,
  TOOL_INVOCATION = 4,
  AGENT = 5,
  TEMPLATE = 6,
}

export interface Node {
  id: string;
  type: NodeType;
  createdAt: Date;
  data?: PromptNode | ResponseNode | ToolInvocationNode | AgentNode | TemplateNode;
}

export interface PromptNode {
  id: string;
  sessionId: string;
  content: string;
  timestamp: Date;
  metadata?: PromptMetadata;
}

export interface ResponseNode {
  id: string;
  promptId: string;
  content: string;
  timestamp: Date;
  tokenUsage: TokenUsage;
  metadata?: ResponseMetadata;
}

export interface ToolInvocationNode {
  id: string;
  responseId: string;
  toolName: string;
  parameters: string; // JSON string
  status: string;
  result?: string; // JSON string
  error?: string;
  durationMs: number;
  retryCount: number;
  timestamp: Date;
  metadata: Record<string, string>;
}

export interface AgentNode {
  id: string;
  name: string;
  role: string;
  capabilities: string[];
  status: string;
  createdAt: Date;
  metadata: Record<string, string>;
}

export interface TemplateNode {
  id: string;
  name: string;
  templateText: string;
  variables: VariableSpec[];
  version: string;
  usageCount: number;
  createdAt: Date;
  metadata: Record<string, string>;
}

export interface VariableSpec {
  name: string;
  typeHint: string;
  required: boolean;
  defaultValue?: string;
  validationPattern?: string;
  description: string;
}

// Edge types
export enum EdgeType {
  UNSPECIFIED = 0,
  BELONGS_TO = 1,
  RESPONDS_TO = 2,
  FOLLOWS = 3,
  INVOKES = 4,
  HANDLED_BY = 5,
  INSTANTIATES = 6,
  INHERITS = 7,
  TRANSFERS_TO = 8,
  REFERENCES = 9,
}

export enum EdgeDirection {
  UNSPECIFIED = 0,
  OUTGOING = 1,
  INCOMING = 2,
  BOTH = 3,
}

export interface Edge {
  id: string;
  fromNodeId: string;
  toNodeId: string;
  type: EdgeType;
  createdAt: Date;
  properties: Record<string, string>;
}

// Metadata types
export interface TokenUsage {
  promptTokens: number;
  completionTokens: number;
  totalTokens: number;
}

export interface PromptMetadata {
  model: string;
  temperature: number;
  maxTokens?: number;
  toolsAvailable: string[];
  custom: Record<string, string>;
}

export interface ResponseMetadata {
  model: string;
  finishReason: string;
  latencyMs: number;
  custom: Record<string, string>;
}

// Query types
export interface QueryOptions {
  sessionId?: string;
  nodeType?: NodeType;
  after?: Date;
  before?: Date;
  limit?: number;
  offset?: number;
  filters?: Record<string, string>;
}

export interface QueryResult {
  nodes: Node[];
  totalCount: number;
}

// Request types
export interface AddPromptRequest {
  sessionId: string;
  content: string;
  metadata?: PromptMetadata;
}

export interface AddResponseRequest {
  promptId: string;
  content: string;
  tokenUsage: TokenUsage;
  metadata?: ResponseMetadata;
}

export interface AddToolInvocationRequest {
  toolInvocation: ToolInvocationNode;
}

export interface CreateTemplateRequest {
  template: TemplateNode;
}

export interface InstantiateTemplateRequest {
  templateId: string;
  variableValues: Record<string, string>;
  sessionId: string;
}

// Event types
export enum EventType {
  UNSPECIFIED = 0,
  NODE_CREATED = 1,
  NODE_UPDATED = 2,
  NODE_DELETED = 3,
  EDGE_CREATED = 4,
  EDGE_DELETED = 5,
  SESSION_CREATED = 6,
  SESSION_CLOSED = 7,
}

export interface Event {
  id: string;
  type: EventType;
  timestamp: Date;
  payload: string; // JSON string
}

export interface SessionEvent {
  event: Event;
  sessionId: string;
}

// Health and Metrics
export enum ServingStatus {
  UNKNOWN = 0,
  SERVING = 1,
  NOT_SERVING = 2,
}

export interface HealthResponse {
  status: ServingStatus;
  version: string;
  uptimeSeconds: number;
}

export interface MetricsResponse {
  totalNodes: number;
  totalEdges: number;
  totalSessions: number;
  activeSessions: number;
  avgWriteLatencyMs: number;
  avgReadLatencyMs: number;
  requestsPerSecond: number;
}

// Client configuration
export interface ClientConfig {
  address: string;
  port?: number;
  useTls?: boolean;
  tlsOptions?: {
    rootCerts?: Buffer;
    privateKey?: Buffer;
    certChain?: Buffer;
  };
  credentials?: {
    username?: string;
    password?: string;
  };
  timeout?: number; // milliseconds
  retryPolicy?: {
    maxRetries?: number;
    initialBackoff?: number;
    maxBackoff?: number;
    backoffMultiplier?: number;
  };
}

// Stream callback types
export type StreamCallback<T> = (data: T) => void;
export type StreamErrorCallback = (error: Error) => void;
export type StreamEndCallback = () => void;

export interface StreamOptions {
  onData: StreamCallback<Node>;
  onError?: StreamErrorCallback;
  onEnd?: StreamEndCallback;
}

export interface EventStreamOptions {
  sessionId?: string;
  eventTypes?: EventType[];
  onData: StreamCallback<Event>;
  onError?: StreamErrorCallback;
  onEnd?: StreamEndCallback;
}

export interface SessionEventStreamOptions {
  sessionId: string;
  onData: StreamCallback<SessionEvent>;
  onError?: StreamErrorCallback;
  onEnd?: StreamEndCallback;
}
