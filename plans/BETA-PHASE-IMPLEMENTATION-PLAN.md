# LLM-Memory-Graph: Beta Phase Implementation Plan

**Version**: 2.0 Beta
**Status**: Planning
**Target Timeline**: 6 weeks
**Prerequisites**: MVP v0.1.0 (Complete ✅)
**Document Date**: 2025-11-06

---

## Executive Summary

This plan outlines the Beta Phase implementation for LLM-Memory-Graph, transforming the MVP into a production-ready system with advanced features, async capabilities, and ecosystem integration. The Beta Phase focuses on scalability, performance optimization, and seamless integration with the LLM DevOps ecosystem.

### Beta Phase Objectives

1. **Extended Node Types**: Add ToolInvocation, AgentNode, PromptTemplate
2. **Advanced Relationships**: Implement INSTANTIATES, INHERITS, TRANSFERS_TO edges
3. **Async Architecture**: Migrate to Tokio-based async API
4. **Observatory Integration**: Real-time event streaming and metrics
5. **Performance Optimization**: Achieve <10ms writes, <1ms reads
6. **Production Readiness**: Handle 1M+ nodes, 10k concurrent operations

### Key Deliverables

- ✅ 3 new node types with full CRUD operations
- ✅ 5 additional edge types with traversal support
- ✅ Complete async API (Tokio-based)
- ✅ LLM-Observatory integration (Kafka + gRPC)
- ✅ Performance benchmarks and optimization
- ✅ Migration tooling from MVP to Beta
- ✅ Comprehensive documentation and examples

---

## Table of Contents

1. [Technical Architecture](#1-technical-architecture)
2. [Extended Data Model](#2-extended-data-model)
3. [Async API Implementation](#3-async-api-implementation)
4. [LLM-Observatory Integration](#4-llm-observatory-integration)
5. [Performance Optimization](#5-performance-optimization)
6. [Implementation Roadmap](#6-implementation-roadmap)
7. [Testing Strategy](#7-testing-strategy)
8. [Migration Guide](#8-migration-guide)
9. [Risk Assessment](#9-risk-assessment)
10. [Success Metrics](#10-success-metrics)

---

## 1. Technical Architecture

### 1.1 System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    Beta Architecture                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────┐      ┌──────────────┐      ┌──────────────┐  │
│  │   Async API  │─────▶│  MemoryGraph │─────▶│ LLM-         │  │
│  │   (Tokio)    │      │   (Engine)   │      │ Observatory  │  │
│  └──────────────┘      └──────┬───────┘      └──────────────┘  │
│                               │                                 │
│                               ▼                                 │
│                    ┌──────────────────┐                         │
│                    │  Storage Backend │                         │
│                    │  (Async Sled)    │                         │
│                    └──────────────────┘                         │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 Module Structure (Beta)

```
llm-memory-graph/
├── src/
│   ├── lib.rs                      # Async API exports
│   ├── error.rs                    # Enhanced error types
│   ├── types/
│   │   ├── ids.rs                  # [MVP] Existing IDs
│   │   ├── nodes.rs                # [BETA] +ToolInvocation, AgentNode, PromptTemplate
│   │   ├── edges.rs                # [BETA] +INSTANTIATES, INHERITS, etc.
│   │   └── config.rs               # [BETA] Enhanced with async options
│   ├── storage/
│   │   ├── mod.rs                  # [BETA] Async StorageBackend trait
│   │   ├── sled_backend.rs         # [BETA] Async Sled implementation
│   │   └── serialization.rs        # [MVP] Unchanged
│   ├── engine/
│   │   ├── mod.rs                  # [BETA] Async MemoryGraph
│   │   ├── template_engine.rs      # [NEW] Template instantiation
│   │   └── agent_coordinator.rs    # [NEW] Multi-agent coordination
│   ├── query/
│   │   ├── mod.rs                  # [BETA] Async QueryBuilder
│   │   ├── temporal.rs             # [NEW] Time-series queries
│   │   └── lineage.rs              # [NEW] Prompt lineage tracking
│   ├── observatory/                # [NEW] Observatory integration
│   │   ├── mod.rs                  # Public API
│   │   ├── events.rs               # Event definitions
│   │   ├── kafka_producer.rs       # Kafka event streaming
│   │   └── metrics.rs              # Prometheus metrics
│   └── indexing/                   # [NEW] Advanced indexing
│       ├── mod.rs                  # Index manager
│       ├── temporal_index.rs       # Time-based index
│       └── agent_index.rs          # Agent-centric index
├── tests/
│   ├── integration_test.rs         # [MVP] Enhanced
│   ├── beta_features_test.rs       # [NEW] Beta-specific tests
│   ├── async_test.rs               # [NEW] Async operation tests
│   └── observatory_test.rs         # [NEW] Integration tests
├── examples/
│   ├── simple_chatbot.rs           # [MVP] Migrated to async
│   ├── multi_agent_system.rs       # [NEW] Agent coordination
│   ├── template_system.rs          # [NEW] Template management
│   └── observatory_demo.rs         # [NEW] Event streaming demo
└── benches/
    ├── async_operations.rs         # [NEW] Async benchmarks
    └── large_scale.rs              # [NEW] 1M+ node tests
```

### 1.3 Technology Stack (Beta)

#### Core Dependencies (New)
```toml
[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }
tokio-stream = "0.1"
futures = "0.3"

# Observatory integration
rdkafka = { version = "0.36", features = ["tokio"] }
tonic = "0.10"
prost = "0.12"

# Metrics
prometheus = "0.13"
metrics = "0.21"

# Additional graph features
petgraph = { version = "0.6", features = ["serde-1"] }
```

#### Existing Dependencies (Enhanced)
```toml
# All MVP dependencies remain
sled = "0.34"
serde = { version = "1.0", features = ["derive"] }
rmp-serde = "1.1"
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
# ... etc
```

---

## 2. Extended Data Model

### 2.1 New Node Types

#### 2.1.1 ToolInvocation Node

Tracks tool/function calls made by LLMs.

```rust
/// A tool invocation node representing a function call by an LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInvocation {
    /// Unique node identifier
    pub id: NodeId,
    /// Response that triggered this tool call
    pub response_id: NodeId,
    /// Name of the tool/function
    pub tool_name: String,
    /// JSON parameters passed to the tool
    pub parameters: serde_json::Value,
    /// Tool execution result (if completed)
    pub result: Option<serde_json::Value>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Execution duration in milliseconds
    pub duration_ms: u64,
    /// When the tool was invoked
    pub timestamp: DateTime<Utc>,
    /// Success/failure status
    pub success: bool,
    /// Retry count (for failed invocations)
    pub retry_count: u32,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ToolInvocation {
    /// Create a new pending tool invocation
    pub fn new(response_id: NodeId, tool_name: String, parameters: serde_json::Value) -> Self;

    /// Mark tool invocation as successful
    pub fn mark_success(&mut self, result: serde_json::Value, duration_ms: u64);

    /// Mark tool invocation as failed
    pub fn mark_failed(&mut self, error: String, duration_ms: u64);

    /// Record retry attempt
    pub fn record_retry(&mut self);
}
```

**Use Cases**:
- Track tool usage patterns across conversations
- Analyze tool success/failure rates
- Debug tool integration issues
- Measure tool execution performance
- Build tool dependency graphs

#### 2.1.2 AgentNode

Represents autonomous agents in multi-agent systems.

```rust
/// An agent node representing an autonomous AI agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentNode {
    /// Unique agent identifier
    pub id: AgentId,
    /// Internal node ID for graph storage
    pub node_id: NodeId,
    /// Human-readable agent name
    pub name: String,
    /// Agent role/specialization (e.g., "researcher", "coder", "reviewer")
    pub role: String,
    /// List of agent capabilities
    pub capabilities: Vec<String>,
    /// Model used by this agent
    pub model: String,
    /// When the agent was created
    pub created_at: DateTime<Utc>,
    /// Last activity timestamp
    pub last_active: DateTime<Utc>,
    /// Agent status (active, idle, busy, paused)
    pub status: AgentStatus,
    /// Configuration parameters
    pub config: AgentConfig,
    /// Performance metrics
    pub metrics: AgentMetrics,
    /// Tags for categorization
    pub tags: Vec<String>,
}

/// Agent status enum
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Active,
    Idle,
    Busy,
    Paused,
    Terminated,
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub temperature: f32,
    pub max_tokens: usize,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub tools_enabled: Vec<String>,
}

/// Agent performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetrics {
    pub total_prompts: u64,
    pub successful_tasks: u64,
    pub failed_tasks: u64,
    pub average_latency_ms: f64,
    pub total_tokens_used: u64,
}

impl AgentNode {
    /// Create a new agent
    pub fn new(name: String, role: String, capabilities: Vec<String>) -> Self;

    /// Update agent status
    pub fn set_status(&mut self, status: AgentStatus);

    /// Record agent activity
    pub fn record_activity(&mut self);

    /// Update performance metrics
    pub fn update_metrics(&mut self, success: bool, latency_ms: u64, tokens: u64);
}
```

**Use Cases**:
- Coordinate multiple specialized agents
- Track agent workload and performance
- Implement agent handoff protocols
- Monitor agent health and status
- Build agent collaboration networks

#### 2.1.3 PromptTemplate Node

Versioned prompt templates with variable substitution.

```rust
/// A prompt template node for reusable prompts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    /// Unique template identifier
    pub id: TemplateId,
    /// Internal node ID for graph storage
    pub node_id: NodeId,
    /// Semantic version (e.g., "1.2.3")
    pub version: Version,
    /// Human-readable template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template content with {{variables}}
    pub template: String,
    /// Variable specifications
    pub variables: Vec<VariableSpec>,
    /// Parent template ID (for inheritance)
    pub parent_id: Option<TemplateId>,
    /// When the template was created
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
    /// Template author
    pub author: String,
    /// Usage count
    pub usage_count: u64,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Semantic version
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

/// Variable specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableSpec {
    /// Variable name (e.g., "user_query")
    pub name: String,
    /// Type hint (e.g., "string", "number", "array")
    pub type_hint: String,
    /// Whether the variable is required
    pub required: bool,
    /// Default value if not provided
    pub default: Option<String>,
    /// Validation regex pattern
    pub validation_pattern: Option<String>,
    /// Human-readable description
    pub description: String,
}

impl PromptTemplate {
    /// Create a new template
    pub fn new(name: String, template: String, variables: Vec<VariableSpec>) -> Self;

    /// Create template from parent (inheritance)
    pub fn from_parent(parent_id: TemplateId, name: String, template: String) -> Self;

    /// Instantiate template with variable values
    pub fn instantiate(&self, values: HashMap<String, String>) -> Result<String>;

    /// Validate variable values
    pub fn validate(&self, values: &HashMap<String, String>) -> Result<()>;

    /// Increment usage counter
    pub fn record_usage(&mut self);

    /// Bump version
    pub fn bump_version(&mut self, level: VersionLevel);
}

/// Version bump level
#[derive(Debug, Clone)]
pub enum VersionLevel {
    Major,
    Minor,
    Patch,
}
```

**Use Cases**:
- Reusable prompt libraries
- Template versioning and evolution
- Variable validation and type checking
- Template inheritance and composition
- A/B testing different prompt versions

### 2.2 New Edge Types

#### 2.2.1 INSTANTIATES Edge

Links a prompt to the template it was instantiated from.

```rust
/// INSTANTIATES edge (Prompt → PromptTemplate)
/// Properties:
/// - template_version: Version used
/// - variable_bindings: JSON of variable values
/// - instantiation_time: When template was instantiated
pub struct InstantiatesEdge {
    pub template_version: String,
    pub variable_bindings: HashMap<String, String>,
    pub instantiation_time: DateTime<Utc>,
}
```

**Queries Enabled**:
- Find all prompts using a specific template
- Track template usage over time
- Analyze which variables are most commonly used
- Compare performance of different template versions

#### 2.2.2 INHERITS Edge

Links a template to its parent template.

```rust
/// INHERITS edge (PromptTemplate → PromptTemplate)
/// Properties:
/// - override_sections: Which parts were modified
/// - version_diff: Semantic diff between versions
/// - inheritance_depth: How many levels deep
pub struct InheritsEdge {
    pub override_sections: Vec<String>,
    pub version_diff: String,
    pub inheritance_depth: u32,
}
```

**Queries Enabled**:
- Traverse template inheritance chains
- Find all descendants of a template
- Analyze template evolution over time
- Detect circular inheritance

#### 2.2.3 INVOKES Edge

Links a response to the tools it invoked.

```rust
/// INVOKES edge (Response → ToolInvocation)
/// Properties:
/// - invocation_order: Sequence number
/// - success: Whether tool call succeeded
/// - required: Whether tool was mandatory
pub struct InvokesEdge {
    pub invocation_order: u32,
    pub success: bool,
    pub required: bool,
}
```

**Queries Enabled**:
- Find all tools used in a conversation
- Analyze tool calling patterns
- Track tool success rates
- Build tool dependency graphs

#### 2.2.4 TRANSFERS_TO Edge

Links a response to the agent it handed off to.

```rust
/// TRANSFERS_TO edge (Response → AgentNode)
/// Properties:
/// - handoff_reason: Why the transfer occurred
/// - context_summary: Summary of conversation so far
/// - priority: Urgency level
pub struct TransfersToEdge {
    pub handoff_reason: String,
    pub context_summary: String,
    pub priority: Priority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}
```

**Queries Enabled**:
- Track agent handoff patterns
- Analyze collaboration networks
- Measure handoff latency
- Optimize agent routing

#### 2.2.5 REFERENCES Edge

Links a prompt to external context sources.

```rust
/// REFERENCES edge (Prompt → ExternalContext)
/// Properties:
/// - context_type: Type of reference (document, web, database)
/// - relevance_score: How relevant the context is
/// - chunk_id: Specific chunk referenced
pub struct ReferencesEdge {
    pub context_type: ContextType,
    pub relevance_score: f32,
    pub chunk_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextType {
    Document,
    WebPage,
    Database,
    VectorSearch,
    Memory,
}
```

**Queries Enabled**:
- Find all context used for a prompt
- Track context usage patterns
- Measure context effectiveness
- Build knowledge graphs

### 2.3 Enhanced Node Enum

```rust
/// Extended node wrapper with all Beta node types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    // MVP nodes
    Prompt(PromptNode),
    Response(ResponseNode),
    Session(ConversationSession),

    // Beta nodes
    ToolInvocation(ToolInvocation),
    Agent(AgentNode),
    Template(PromptTemplate),
}

impl Node {
    pub fn id(&self) -> NodeId {
        match self {
            Node::Prompt(p) => p.id,
            Node::Response(r) => r.id,
            Node::Session(s) => s.node_id,
            Node::ToolInvocation(t) => t.id,
            Node::Agent(a) => a.node_id,
            Node::Template(t) => t.node_id,
        }
    }

    pub fn node_type(&self) -> NodeType {
        match self {
            Node::Prompt(_) => NodeType::Prompt,
            Node::Response(_) => NodeType::Response,
            Node::Session(_) => NodeType::Session,
            Node::ToolInvocation(_) => NodeType::ToolInvocation,
            Node::Agent(_) => NodeType::Agent,
            Node::Template(_) => NodeType::Template,
        }
    }
}
```

### 2.4 Enhanced EdgeType Enum

```rust
/// Extended edge types with all Beta relationships
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdgeType {
    // MVP edges
    Follows,        // Prompt → Prompt
    RespondsTo,     // Response → Prompt
    HandledBy,      // Prompt → Agent
    PartOf,         // Prompt → Session

    // Beta edges
    Instantiates,   // Prompt → Template
    Inherits,       // Template → Template
    Invokes,        // Response → ToolInvocation
    TransfersTo,    // Response → Agent
    References,     // Prompt → ExternalContext
}
```

---

## 3. Async API Implementation

### 3.1 Async Architecture Strategy

#### 3.1.1 Migration Approach

**Parallel APIs** (Recommended for Beta):
- Keep synchronous API for backward compatibility
- Add async API alongside (feature flag)
- Gradual migration path for users

```rust
// Synchronous API (deprecated but supported)
#[cfg(feature = "sync")]
impl MemoryGraph {
    pub fn open(config: Config) -> Result<Self> { ... }
    pub fn create_session(&self) -> Result<ConversationSession> { ... }
}

// Async API (default in Beta)
#[cfg(not(feature = "sync"))]
impl MemoryGraph {
    pub async fn open(config: Config) -> Result<Self> { ... }
    pub async fn create_session(&self) -> Result<ConversationSession> { ... }
}
```

#### 3.1.2 Tokio Integration

```rust
use tokio::sync::{RwLock, Mutex};
use tokio::task;

pub struct MemoryGraph {
    storage: Arc<dyn AsyncStorageBackend>,
    session_cache: Arc<RwLock<HashMap<SessionId, ConversationSession>>>,
    runtime: tokio::runtime::Handle,
}

impl MemoryGraph {
    /// Open or create a memory graph (async)
    pub async fn open(config: Config) -> Result<Self> {
        let storage = Arc::new(SledBackend::open_async(config.path).await?);

        Ok(Self {
            storage,
            session_cache: Arc::new(RwLock::new(HashMap::new())),
            runtime: tokio::runtime::Handle::current(),
        })
    }

    /// Create a new conversation session (async)
    pub async fn create_session(&self) -> Result<ConversationSession> {
        let session = ConversationSession::new();

        // Store in backend
        self.storage.store_node(&Node::Session(session.clone())).await?;

        // Update cache
        let mut cache = self.session_cache.write().await;
        cache.insert(session.id, session.clone());

        Ok(session)
    }

    /// Add a prompt node (async)
    pub async fn add_prompt(
        &self,
        session_id: SessionId,
        content: String,
        metadata: Option<PromptMetadata>,
    ) -> Result<NodeId> {
        let prompt = PromptNode {
            id: NodeId::new(),
            session_id,
            content,
            metadata: metadata.unwrap_or_default(),
            timestamp: Utc::now(),
            template_id: None,
            variables: HashMap::new(),
        };

        let node = Node::Prompt(prompt.clone());
        self.storage.store_node(&node).await?;

        Ok(prompt.id)
    }
}
```

### 3.2 Async Storage Backend

```rust
#[async_trait::async_trait]
pub trait AsyncStorageBackend: Send + Sync {
    /// Store a node asynchronously
    async fn store_node(&self, node: &Node) -> Result<()>;

    /// Retrieve a node asynchronously
    async fn get_node(&self, id: &NodeId) -> Result<Option<Node>>;

    /// Delete a node asynchronously
    async fn delete_node(&self, id: &NodeId) -> Result<()>;

    /// Store an edge asynchronously
    async fn store_edge(&self, edge: &Edge) -> Result<()>;

    /// Batch operations for performance
    async fn store_nodes_batch(&self, nodes: &[Node]) -> Result<Vec<NodeId>>;

    /// Stream results for large queries
    fn query_stream(&self, query: Query) -> impl Stream<Item = Result<Node>>;
}
```

### 3.3 Async Query Interface

```rust
impl MemoryGraph {
    /// Create an async query builder
    pub fn query(&self) -> AsyncQueryBuilder {
        AsyncQueryBuilder::new(self.storage.clone())
    }
}

pub struct AsyncQueryBuilder {
    storage: Arc<dyn AsyncStorageBackend>,
    session_filter: Option<SessionId>,
    node_type_filter: Option<NodeType>,
    time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    limit: Option<usize>,
    offset: usize,
}

impl AsyncQueryBuilder {
    /// Execute query asynchronously and return all results
    pub async fn execute(&self) -> Result<Vec<Node>> {
        let stream = self.execute_stream();
        pin_mut!(stream);

        let mut results = Vec::new();
        while let Some(node) = stream.next().await {
            results.push(node?);
        }

        Ok(results)
    }

    /// Execute query and return a stream of results
    pub fn execute_stream(&self) -> impl Stream<Item = Result<Node>> {
        // Return async stream for memory-efficient iteration
        self.storage.query_stream(self.build_query())
    }

    /// Count matching nodes without loading them
    pub async fn count(&self) -> Result<usize> {
        // Efficient count without loading all nodes
        self.storage.count_nodes(self.build_query()).await
    }
}
```

### 3.4 Concurrent Operations

```rust
impl MemoryGraph {
    /// Process multiple prompts concurrently
    pub async fn add_prompts_batch(
        &self,
        prompts: Vec<(SessionId, String)>,
    ) -> Result<Vec<NodeId>> {
        let futures: Vec<_> = prompts
            .into_iter()
            .map(|(session_id, content)| {
                self.add_prompt(session_id, content, None)
            })
            .collect();

        futures::future::try_join_all(futures).await
    }

    /// Concurrent graph traversal
    pub async fn parallel_traversal(
        &self,
        start_nodes: Vec<NodeId>,
        depth: usize,
    ) -> Result<Vec<Vec<Node>>> {
        let futures: Vec<_> = start_nodes
            .into_iter()
            .map(|node_id| {
                self.traversal()
                    .bfs_from(node_id)
                    .max_depth(depth)
                    .execute()
            })
            .collect();

        futures::future::try_join_all(futures).await
    }
}
```

### 3.5 Async Error Handling

```rust
/// Enhanced error types for async operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // ... existing errors ...

    /// Async operation timeout
    #[error("Operation timed out after {0}ms")]
    Timeout(u64),

    /// Concurrent modification conflict
    #[error("Concurrent modification detected: {0}")]
    ConcurrentModification(String),

    /// Connection pool exhausted
    #[error("Connection pool exhausted")]
    PoolExhausted,

    /// Async runtime error
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}
```

---

## 4. LLM-Observatory Integration

### 4.1 Observatory Architecture

```
┌───────────────────────────────────────────────────────┐
│               LLM-Memory-Graph                        │
├───────────────────────────────────────────────────────┤
│                                                        │
│  ┌──────────────┐                                     │
│  │  Event Bus   │──────┐                              │
│  │  (Internal)  │      │                              │
│  └──────────────┘      │                              │
│                        ▼                              │
│              ┌─────────────────┐                      │
│              │ Event Processor │                      │
│              └────────┬────────┘                      │
│                       │                               │
│                       ├──────────────┐                │
│                       ▼              ▼                │
│              ┌──────────────┐  ┌──────────────┐      │
│              │    Kafka     │  │  Metrics     │      │
│              │   Producer   │  │  Exporter    │      │
│              └──────┬───────┘  └──────┬───────┘      │
└──────────────────────┼──────────────────┼─────────────┘
                       │                  │
                       ▼                  ▼
              ┌─────────────────┐  ┌──────────────┐
              │ LLM-Observatory │  │  Prometheus  │
              │  (Kafka Topic)  │  │              │
              └─────────────────┘  └──────────────┘
```

### 4.2 Event Types

```rust
/// Events emitted by the memory graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryGraphEvent {
    /// Node created event
    NodeCreated {
        node_id: NodeId,
        node_type: NodeType,
        session_id: Option<SessionId>,
        timestamp: DateTime<Utc>,
        metadata: HashMap<String, String>,
    },

    /// Edge created event
    EdgeCreated {
        edge_id: EdgeId,
        edge_type: EdgeType,
        from: NodeId,
        to: NodeId,
        timestamp: DateTime<Utc>,
    },

    /// Prompt submitted event
    PromptSubmitted {
        prompt_id: NodeId,
        session_id: SessionId,
        content_length: usize,
        model: String,
        timestamp: DateTime<Utc>,
    },

    /// Response generated event
    ResponseGenerated {
        response_id: NodeId,
        prompt_id: NodeId,
        content_length: usize,
        tokens_used: TokenUsage,
        latency_ms: u64,
        timestamp: DateTime<Utc>,
    },

    /// Tool invoked event
    ToolInvoked {
        tool_id: NodeId,
        tool_name: String,
        success: bool,
        duration_ms: u64,
        timestamp: DateTime<Utc>,
    },

    /// Agent handoff event
    AgentHandoff {
        from_agent: AgentId,
        to_agent: AgentId,
        session_id: SessionId,
        reason: String,
        timestamp: DateTime<Utc>,
    },

    /// Template instantiated event
    TemplateInstantiated {
        template_id: TemplateId,
        prompt_id: NodeId,
        version: String,
        variables: HashMap<String, String>,
        timestamp: DateTime<Utc>,
    },

    /// Query executed event
    QueryExecuted {
        query_type: String,
        results_count: usize,
        duration_ms: u64,
        timestamp: DateTime<Utc>,
    },
}
```

### 4.3 Kafka Integration

```rust
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

/// Kafka producer for Observatory events
pub struct ObservatoryProducer {
    producer: FutureProducer,
    topic: String,
}

impl ObservatoryProducer {
    /// Create a new Observatory producer
    pub async fn new(
        brokers: &str,
        topic: String,
    ) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .set("compression.type", "snappy")
            .set("batch.size", "16384")
            .create()?;

        Ok(Self { producer, topic })
    }

    /// Send event to Observatory
    pub async fn send_event(&self, event: MemoryGraphEvent) -> Result<()> {
        let payload = serde_json::to_vec(&event)?;
        let key = event.get_key();

        let record = FutureRecord::to(&self.topic)
            .key(&key)
            .payload(&payload);

        self.producer
            .send(record, Duration::from_secs(5))
            .await
            .map_err(|(err, _)| Error::Observatory(err.to_string()))?;

        Ok(())
    }

    /// Send batch of events
    pub async fn send_batch(&self, events: Vec<MemoryGraphEvent>) -> Result<()> {
        let futures: Vec<_> = events
            .into_iter()
            .map(|event| self.send_event(event))
            .collect();

        futures::future::try_join_all(futures).await?;
        Ok(())
    }
}
```

### 4.4 Metrics Export

```rust
use prometheus::{Counter, Histogram, Gauge, Registry};

/// Prometheus metrics for MemoryGraph
pub struct MemoryGraphMetrics {
    // Counters
    pub nodes_created: Counter,
    pub edges_created: Counter,
    pub prompts_submitted: Counter,
    pub responses_generated: Counter,
    pub tools_invoked: Counter,

    // Histograms
    pub write_latency: Histogram,
    pub read_latency: Histogram,
    pub query_duration: Histogram,
    pub tool_duration: Histogram,

    // Gauges
    pub active_sessions: Gauge,
    pub total_nodes: Gauge,
    pub total_edges: Gauge,
    pub cache_size: Gauge,
}

impl MemoryGraphMetrics {
    /// Create and register metrics
    pub fn new(registry: &Registry) -> Result<Self> {
        let nodes_created = Counter::new(
            "memory_graph_nodes_created_total",
            "Total number of nodes created"
        )?;
        registry.register(Box::new(nodes_created.clone()))?;

        let write_latency = Histogram::with_opts(
            prometheus::HistogramOpts::new(
                "memory_graph_write_latency_seconds",
                "Write operation latency"
            )
            .buckets(vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0])
        )?;
        registry.register(Box::new(write_latency.clone()))?;

        // ... register other metrics ...

        Ok(Self {
            nodes_created,
            edges_created: /* ... */,
            write_latency,
            // ... etc
        })
    }
}
```

### 4.5 Integration with MemoryGraph

```rust
impl MemoryGraph {
    /// Create graph with Observatory integration
    pub async fn open_with_observatory(
        config: Config,
        observatory_config: ObservatoryConfig,
    ) -> Result<Self> {
        let storage = Arc::new(SledBackend::open_async(config.path).await?);

        // Initialize Observatory producer
        let producer = ObservatoryProducer::new(
            &observatory_config.kafka_brokers,
            observatory_config.topic,
        ).await?;

        // Initialize metrics
        let registry = Registry::new();
        let metrics = MemoryGraphMetrics::new(&registry)?;

        Ok(Self {
            storage,
            session_cache: Arc::new(RwLock::new(HashMap::new())),
            runtime: tokio::runtime::Handle::current(),
            observatory: Some(Arc::new(producer)),
            metrics: Some(Arc::new(metrics)),
        })
    }

    /// Add prompt with Observatory event
    pub async fn add_prompt(
        &self,
        session_id: SessionId,
        content: String,
        metadata: Option<PromptMetadata>,
    ) -> Result<NodeId> {
        let start = Instant::now();

        // Create and store prompt
        let prompt = PromptNode { /* ... */ };
        self.storage.store_node(&Node::Prompt(prompt.clone())).await?;

        // Record metrics
        if let Some(metrics) = &self.metrics {
            metrics.prompts_submitted.inc();
            metrics.write_latency.observe(start.elapsed().as_secs_f64());
        }

        // Send Observatory event
        if let Some(observatory) = &self.observatory {
            let event = MemoryGraphEvent::PromptSubmitted {
                prompt_id: prompt.id,
                session_id,
                content_length: content.len(),
                model: prompt.metadata.model.clone(),
                timestamp: Utc::now(),
            };

            // Send async without blocking
            let obs_clone = observatory.clone();
            tokio::spawn(async move {
                if let Err(e) = obs_clone.send_event(event).await {
                    tracing::warn!("Failed to send Observatory event: {}", e);
                }
            });
        }

        Ok(prompt.id)
    }
}
```

### 4.6 Observatory Configuration

```rust
/// Configuration for Observatory integration
#[derive(Debug, Clone)]
pub struct ObservatoryConfig {
    /// Kafka broker addresses
    pub kafka_brokers: String,
    /// Kafka topic for events
    pub topic: String,
    /// Enable metrics export
    pub enable_metrics: bool,
    /// Metrics export port
    pub metrics_port: u16,
    /// Event batching size
    pub batch_size: usize,
    /// Flush interval in milliseconds
    pub flush_interval_ms: u64,
}

impl Default for ObservatoryConfig {
    fn default() -> Self {
        Self {
            kafka_brokers: "localhost:9092".to_string(),
            topic: "llm-memory-graph-events".to_string(),
            enable_metrics: true,
            metrics_port: 9090,
            batch_size: 100,
            flush_interval_ms: 1000,
        }
    }
}
```

---

## 5. Performance Optimization

### 5.1 Target Performance Metrics

| Metric | MVP | Beta Target | Improvement |
|--------|-----|-------------|-------------|
| Write Latency (p95) | <100ms | <10ms | 10x faster |
| Read Latency (p95) | <10ms | <1ms | 10x faster |
| Graph Traversal (p95) | <50ms | <20ms | 2.5x faster |
| Concurrent Ops | 1k/sec | 10k/sec | 10x throughput |
| Node Capacity | 10k | 1M+ | 100x scale |
| Query Throughput | 1k/sec | 100k/sec | 100x faster |

### 5.2 Optimization Strategies

#### 5.2.1 Async I/O

Replace blocking I/O with async operations:
- Use `tokio::fs` for async file operations
- Implement async Sled backend
- Use async channels for inter-component communication

#### 5.2.2 Connection Pooling

```rust
use bb8::Pool;
use bb8_sled::SledConnectionManager;

pub struct PooledStorage {
    pool: Pool<SledConnectionManager>,
}

impl PooledStorage {
    pub async fn new(config: Config) -> Result<Self> {
        let manager = SledConnectionManager::new(config.path);
        let pool = Pool::builder()
            .max_size(50)
            .build(manager)
            .await?;

        Ok(Self { pool })
    }

    pub async fn get_connection(&self) -> Result<PooledConnection<'_>> {
        self.pool.get().await.map_err(|e| Error::PoolExhausted)
    }
}
```

#### 5.2.3 Write Batching

```rust
impl MemoryGraph {
    /// Batch writer for high-throughput ingestion
    pub async fn batch_write(&self, batch: WriteBatch) -> Result<()> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Collect all operations
        for op in batch.operations {
            match op {
                WriteOp::Node(node) => nodes.push(node),
                WriteOp::Edge(edge) => edges.push(edge),
            }
        }

        // Write in parallel
        let (node_result, edge_result) = tokio::join!(
            self.storage.store_nodes_batch(&nodes),
            self.storage.store_edges_batch(&edges),
        );

        node_result?;
        edge_result?;

        Ok(())
    }
}
```

#### 5.2.4 Query Optimization

```rust
// Index-backed queries
impl AsyncStorageBackend for SledBackend {
    async fn query_with_index(
        &self,
        index_name: &str,
        key: &[u8],
    ) -> Result<Vec<Node>> {
        // Use index for O(log n) lookup instead of full scan
        let index = self.get_index(index_name)?;
        let node_ids = index.scan_prefix(key);

        // Parallel node retrieval
        let futures: Vec<_> = node_ids
            .map(|id| self.get_node(&id))
            .collect();

        let nodes: Vec<_> = futures::future::try_join_all(futures)
            .await?
            .into_iter()
            .flatten()
            .collect();

        Ok(nodes)
    }
}
```

#### 5.2.5 Caching Strategy

```rust
use moka::future::Cache;

pub struct MemoryGraph {
    // ... existing fields ...

    // Multi-level caching
    node_cache: Cache<NodeId, Node>,
    edge_cache: Cache<EdgeId, Edge>,
    query_cache: Cache<String, Vec<NodeId>>,
}

impl MemoryGraph {
    pub async fn get_node_cached(&self, id: &NodeId) -> Result<Option<Node>> {
        // Check cache first
        if let Some(node) = self.node_cache.get(id).await {
            return Ok(Some(node));
        }

        // Load from storage
        if let Some(node) = self.storage.get_node(id).await? {
            // Populate cache
            self.node_cache.insert(*id, node.clone()).await;
            return Ok(Some(node));
        }

        Ok(None)
    }
}
```

### 5.3 Benchmarking Suite

```rust
// benches/beta_performance.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use llm_memory_graph::*;

fn bench_async_writes(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("async_write_1k_prompts", |b| {
        b.to_async(&runtime).iter(|| async {
            let graph = MemoryGraph::open(Config::default()).await.unwrap();
            let session = graph.create_session().await.unwrap();

            for i in 0..1000 {
                graph.add_prompt(
                    session.id,
                    format!("Prompt {}", i),
                    None
                ).await.unwrap();
            }
        });
    });
}

fn bench_concurrent_reads(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    for concurrency in [10, 50, 100, 500].iter() {
        c.bench_with_input(
            BenchmarkId::new("concurrent_reads", concurrency),
            concurrency,
            |b, &conc| {
                b.to_async(&runtime).iter(|| async move {
                    let graph = setup_test_graph().await;

                    let futures: Vec<_> = (0..conc)
                        .map(|_| graph.query().execute())
                        .collect();

                    futures::future::join_all(futures).await
                });
            },
        );
    }
}

criterion_group!(benches, bench_async_writes, bench_concurrent_reads);
criterion_main!(benches);
```

---

## 6. Implementation Roadmap

### 6.1 Phase 1: Extended Data Model (Weeks 1-2)

#### Week 1: New Node Types
- **Days 1-2**: Implement ToolInvocation node
  - [ ] Define struct and methods
  - [ ] Add serialization support
  - [ ] Write unit tests
  - [ ] Update storage backend

- **Days 3-4**: Implement AgentNode
  - [ ] Define struct and methods
  - [ ] Add status management
  - [ ] Add metrics tracking
  - [ ] Write unit tests

- **Days 5**: Implement PromptTemplate
  - [ ] Define struct and methods
  - [ ] Add version management
  - [ ] Add variable validation
  - [ ] Write unit tests

#### Week 2: New Edge Types
- **Days 1-2**: Implement INSTANTIATES, INHERITS edges
  - [ ] Define edge properties
  - [ ] Add traversal support
  - [ ] Write unit tests

- **Days 3-4**: Implement INVOKES, TRANSFERS_TO edges
  - [ ] Define edge properties
  - [ ] Add query support
  - [ ] Write unit tests

- **Day 5**: Integration testing
  - [ ] Test complete data model
  - [ ] Test edge traversals
  - [ ] Performance benchmarks

### 6.2 Phase 2: Async API (Weeks 3-4)

#### Week 3: Core Async Migration
- **Days 1-2**: Async storage backend
  - [ ] Refactor StorageBackend trait to async
  - [ ] Implement async Sled backend
  - [ ] Add connection pooling
  - [ ] Write async tests

- **Days 3-4**: Async MemoryGraph
  - [ ] Migrate MemoryGraph to async
  - [ ] Implement async CRUD operations
  - [ ] Add concurrent operation support
  - [ ] Write async tests

- **Day 5**: Async query interface
  - [ ] Migrate QueryBuilder to async
  - [ ] Add stream-based queries
  - [ ] Implement parallel traversal
  - [ ] Write async tests

#### Week 4: Async Features
- **Days 1-2**: Batch operations
  - [ ] Implement batch writes
  - [ ] Implement batch reads
  - [ ] Add write coalescing
  - [ ] Write tests

- **Days 3-4**: Performance optimization
  - [ ] Implement caching layer
  - [ ] Add index optimization
  - [ ] Optimize serialization
  - [ ] Run benchmarks

- **Day 5**: Migration guide
  - [ ] Write sync-to-async migration guide
  - [ ] Create migration examples
  - [ ] Document breaking changes

### 6.3 Phase 3: Observatory Integration (Week 5)

#### Week 5: Event System
- **Days 1-2**: Event definitions
  - [ ] Define all event types
  - [ ] Implement event serialization
  - [ ] Add event validation
  - [ ] Write tests

- **Days 2-3**: Kafka integration
  - [ ] Implement Kafka producer
  - [ ] Add event batching
  - [ ] Implement retry logic
  - [ ] Write integration tests

- **Days 4-5**: Metrics system
  - [ ] Define Prometheus metrics
  - [ ] Implement metrics collection
  - [ ] Add metrics export endpoint
  - [ ] Create Grafana dashboards

### 6.4 Phase 4: Testing & Optimization (Week 6)

#### Week 6: Finalization
- **Days 1-2**: Integration testing
  - [ ] Test all Beta features
  - [ ] Test Observatory integration
  - [ ] Test async operations
  - [ ] Load testing (1M+ nodes)

- **Days 3-4**: Performance optimization
  - [ ] Profile critical paths
  - [ ] Optimize hot spots
  - [ ] Verify all performance targets
  - [ ] Run comprehensive benchmarks

- **Day 5**: Documentation
  - [ ] Update API documentation
  - [ ] Write migration guide
  - [ ] Create example applications
  - [ ] Update README

### 6.5 Dependencies and Critical Path

```
Week 1: Extended Data Model
   ↓
Week 2: Edge Types (depends on Week 1)
   ↓
Week 3: Async Core (independent, can start early)
   ↓
Week 4: Async Features (depends on Week 3)
   ↓
Week 5: Observatory (depends on Weeks 1-4)
   ↓
Week 6: Integration & Optimization (depends on all previous)
```

### 6.6 Resource Allocation

| Phase | Engineers | Specialization | Time |
|-------|-----------|----------------|------|
| Phase 1 | 2 | Rust, Data Modeling | 2 weeks |
| Phase 2 | 2 | Rust, Async, Tokio | 2 weeks |
| Phase 3 | 2 | Rust, Kafka, Metrics | 1 week |
| Phase 4 | 3 | Testing, Performance | 1 week |

**Total**: 2-3 engineers, 6 weeks

---

## 7. Testing Strategy

### 7.1 Test Pyramid

```
          ┌────────────┐
          │   E2E (5)  │  - Observatory integration
          └────────────┘  - Multi-agent workflows
         ┌──────────────┐
         │  Integration │  - Async operations
         │   Tests (20) │  - Event streaming
         └──────────────┘  - Complete workflows
       ┌──────────────────┐
       │   Unit Tests     │  - Node/edge operations
       │     (100+)       │  - Query builders
       └──────────────────┘  - Type validation

```

### 7.2 Unit Tests (Target: 100+ tests)

#### Node Type Tests
```rust
#[cfg(test)]
mod tool_invocation_tests {
    #[test]
    fn test_tool_creation() { ... }

    #[test]
    fn test_mark_success() { ... }

    #[test]
    fn test_mark_failed() { ... }

    #[test]
    fn test_retry_count() { ... }
}

#[cfg(test)]
mod agent_node_tests {
    #[test]
    fn test_agent_creation() { ... }

    #[test]
    fn test_status_transitions() { ... }

    #[test]
    fn test_metrics_update() { ... }
}

#[cfg(test)]
mod template_tests {
    #[test]
    fn test_template_creation() { ... }

    #[test]
    fn test_instantiation() { ... }

    #[test]
    fn test_variable_validation() { ... }

    #[test]
    fn test_version_bumping() { ... }
}
```

#### Async Operation Tests
```rust
#[tokio::test]
async fn test_concurrent_writes() {
    let graph = MemoryGraph::open(Config::default()).await.unwrap();
    let session = graph.create_session().await.unwrap();

    let futures: Vec<_> = (0..100)
        .map(|i| {
            graph.add_prompt(
                session.id,
                format!("Prompt {}", i),
                None
            )
        })
        .collect();

    let results = futures::future::try_join_all(futures).await;
    assert!(results.is_ok());

    let nodes = graph.query()
        .session(session.id)
        .execute()
        .await
        .unwrap();

    assert_eq!(nodes.len(), 100);
}

#[tokio::test]
async fn test_stream_query() {
    let graph = setup_large_graph().await;

    let stream = graph.query()
        .session(session_id)
        .execute_stream();

    pin_mut!(stream);

    let mut count = 0;
    while let Some(node) = stream.next().await {
        assert!(node.is_ok());
        count += 1;
    }

    assert!(count > 1000);
}
```

### 7.3 Integration Tests (Target: 20 tests)

```rust
// tests/beta_integration_test.rs

#[tokio::test]
async fn test_tool_invocation_workflow() {
    let graph = MemoryGraph::open(Config::default()).await.unwrap();
    let session = graph.create_session().await.unwrap();

    // 1. Add prompt
    let prompt_id = graph.add_prompt(session.id, "Use calculator".to_string(), None).await.unwrap();

    // 2. Add response
    let response_id = graph.add_response(prompt_id, "Calling calculator...".to_string(), TokenUsage::new(10, 20), None).await.unwrap();

    // 3. Add tool invocation
    let tool = ToolInvocation::new(response_id, "calculator".to_string(), json!({"operation": "add", "a": 2, "b": 3}));
    let tool_id = graph.add_tool_invocation(tool).await.unwrap();

    // 4. Verify edge creation
    let edges = graph.get_outgoing_edges(&response_id).await.unwrap();
    assert_eq!(edges.len(), 2); // RESPONDS_TO + INVOKES

    // 5. Query tool invocations
    let tools = graph.query()
        .session(session.id)
        .node_type(NodeType::ToolInvocation)
        .execute()
        .await
        .unwrap();

    assert_eq!(tools.len(), 1);
}

#[tokio::test]
async fn test_agent_handoff() {
    let graph = MemoryGraph::open(Config::default()).await.unwrap();

    // Create agents
    let agent1 = AgentNode::new("Agent1".to_string(), "researcher".to_string(), vec![]);
    let agent2 = AgentNode::new("Agent2".to_string(), "coder".to_string(), vec![]);

    let agent1_id = graph.add_agent(agent1).await.unwrap();
    let agent2_id = graph.add_agent(agent2).await.unwrap();

    // Create session and conversation
    let session = graph.create_session().await.unwrap();
    let prompt_id = graph.add_prompt(session.id, "Research then code".to_string(), None).await.unwrap();
    let response_id = graph.add_response(prompt_id, "Transferring to coder".to_string(), TokenUsage::new(10, 20), None).await.unwrap();

    // Create handoff edge
    let edge = Edge::new(response_id, agent2_id.into(), EdgeType::TransfersTo);
    graph.add_edge(edge).await.unwrap();

    // Verify handoff
    let handoffs = graph.query()
        .edge_type(EdgeType::TransfersTo)
        .execute()
        .await
        .unwrap();

    assert_eq!(handoffs.len(), 1);
}

#[tokio::test]
async fn test_template_instantiation() {
    let graph = MemoryGraph::open(Config::default()).await.unwrap();

    // Create template
    let mut template = PromptTemplate::new(
        "greeting".to_string(),
        "Hello, {{name}}! Welcome to {{place}}.".to_string(),
        vec![
            VariableSpec {
                name: "name".to_string(),
                type_hint: "string".to_string(),
                required: true,
                default: None,
                validation_pattern: None,
                description: "User name".to_string(),
            },
            VariableSpec {
                name: "place".to_string(),
                type_hint: "string".to_string(),
                required: true,
                default: Some("the system".to_string()),
                validation_pattern: None,
                description: "Place name".to_string(),
            },
        ],
    );

    let template_id = graph.add_template(template.clone()).await.unwrap();

    // Instantiate template
    let mut values = HashMap::new();
    values.insert("name".to_string(), "Alice".to_string());
    values.insert("place".to_string(), "LLM-Memory-Graph".to_string());

    let content = template.instantiate(values.clone()).unwrap();
    assert_eq!(content, "Hello, Alice! Welcome to LLM-Memory-Graph.");

    // Create prompt from template
    let session = graph.create_session().await.unwrap();
    let prompt_id = graph.add_prompt_from_template(
        session.id,
        template_id,
        values,
    ).await.unwrap();

    // Verify INSTANTIATES edge
    let edges = graph.get_outgoing_edges(&prompt_id).await.unwrap();
    let instantiates_edge = edges.iter()
        .find(|e| e.edge_type == EdgeType::Instantiates);
    assert!(instantiates_edge.is_some());
}
```

### 7.4 Performance Tests

```rust
// tests/performance_test.rs

#[tokio::test]
async fn test_1m_nodes_ingestion() {
    let config = Config::new("./test_data/large_scale.db")
        .with_cache_size(500);

    let graph = MemoryGraph::open(config).await.unwrap();
    let session = graph.create_session().await.unwrap();

    let start = Instant::now();

    // Ingest 1M prompts
    for batch in 0..10_000 {
        let futures: Vec<_> = (0..100)
            .map(|i| {
                graph.add_prompt(
                    session.id,
                    format!("Prompt {}", batch * 100 + i),
                    None
                )
            })
            .collect();

        futures::future::try_join_all(futures).await.unwrap();
    }

    let duration = start.elapsed();
    let throughput = 1_000_000.0 / duration.as_secs_f64();

    println!("Ingested 1M nodes in {:?}", duration);
    println!("Throughput: {:.2} ops/sec", throughput);

    assert!(throughput > 10_000.0, "Throughput below target");

    // Verify storage
    let stats = graph.stats().await.unwrap();
    assert_eq!(stats.node_count, 1_000_001); // +1 for session
}

#[tokio::test]
async fn test_concurrent_query_performance() {
    let graph = setup_test_graph_1k_nodes().await;

    let start = Instant::now();

    // 1000 concurrent queries
    let futures: Vec<_> = (0..1000)
        .map(|_| {
            graph.query()
                .limit(100)
                .execute()
        })
        .collect();

    let results = futures::future::try_join_all(futures).await.unwrap();

    let duration = start.elapsed();
    let throughput = 1000.0 / duration.as_secs_f64();

    println!("1000 concurrent queries in {:?}", duration);
    println!("Throughput: {:.2} queries/sec", throughput);

    assert!(throughput > 100.0, "Query throughput below target");
    assert_eq!(results.len(), 1000);
}
```

### 7.5 Observatory Integration Tests

```rust
// tests/observatory_test.rs

#[tokio::test]
async fn test_event_emission() {
    let config = Config::default();
    let obs_config = ObservatoryConfig::default();

    let graph = MemoryGraph::open_with_observatory(config, obs_config).await.unwrap();
    let session = graph.create_session().await.unwrap();

    // Add prompt - should emit event
    let prompt_id = graph.add_prompt(session.id, "Test".to_string(), None).await.unwrap();

    // Wait for async event send
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Verify event was sent (would need Kafka test consumer)
    // This is a placeholder - actual implementation would verify via Kafka
}

#[tokio::test]
async fn test_metrics_collection() {
    let config = Config::default();
    let obs_config = ObservatoryConfig::default();

    let graph = MemoryGraph::open_with_observatory(config, obs_config).await.unwrap();
    let session = graph.create_session().await.unwrap();

    // Perform operations
    for i in 0..100 {
        graph.add_prompt(session.id, format!("Prompt {}", i), None).await.unwrap();
    }

    // Check metrics
    let metrics = graph.get_metrics().await.unwrap();
    assert_eq!(metrics.prompts_submitted, 100);
    assert!(metrics.write_latency_p95 < 0.01); // <10ms
}
```

---

## 8. Migration Guide

### 8.1 MVP to Beta Migration Path

#### 8.1.1 Backward Compatibility Strategy

**Option 1: Feature Flags** (Recommended)
```toml
[dependencies]
llm-memory-graph = { version = "0.2", features = ["sync"] }  # Keep sync API
llm-memory-graph = { version = "0.2" }  # Default: async API
```

**Option 2: Parallel Modules**
```rust
// Sync API (deprecated)
use llm_memory_graph::sync::MemoryGraph;

// Async API (default)
use llm_memory_graph::MemoryGraph;
```

#### 8.1.2 Code Migration Examples

**MVP (Sync) Code:**
```rust
use llm_memory_graph::{MemoryGraph, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graph = MemoryGraph::open(Config::default())?;
    let session = graph.create_session()?;
    let prompt_id = graph.add_prompt(session.id, "Hello".to_string(), None)?;
    Ok(())
}
```

**Beta (Async) Code:**
```rust
use llm_memory_graph::{MemoryGraph, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graph = MemoryGraph::open(Config::default()).await?;
    let session = graph.create_session().await?;
    let prompt_id = graph.add_prompt(session.id, "Hello".to_string(), None).await?;
    Ok(())
}
```

**Changes Required:**
1. Add `#[tokio::main]` to main function
2. Add `.await?` to all graph operations
3. Add `async` to function signatures
4. Update Cargo.toml to include Tokio

#### 8.1.3 Data Migration

**Database Schema Compatibility:**
- Beta uses same storage format as MVP
- No data migration required for existing databases
- New node types have separate prefixes
- Indexes are backward compatible

**Migration Script** (if needed):
```rust
use llm_memory_graph::{MemoryGraph, Config};

#[tokio::main]
async fn migrate_mvp_to_beta() -> Result<()> {
    println!("Starting MVP to Beta migration...");

    // Open existing MVP database
    let config = Config::new("./data/graph.db");
    let graph = MemoryGraph::open(config).await?;

    // Verify compatibility
    let stats = graph.stats().await?;
    println!("Found {} nodes, {} edges", stats.node_count, stats.edge_count);

    // Beta features are now available
    // No data transformation needed

    println!("Migration complete! Database is Beta-compatible.");
    Ok(())
}
```

### 8.2 Breaking Changes

#### 8.2.1 API Changes

| MVP API | Beta API | Notes |
|---------|----------|-------|
| `MemoryGraph::open(config)` | `MemoryGraph::open(config).await` | Now async |
| `graph.create_session()` | `graph.create_session().await` | Now async |
| `graph.query().execute(&graph)` | `graph.query().execute().await` | No graph reference needed |

#### 8.2.2 Deprecation Timeline

- **v0.2.0 (Beta)**: Async API introduced, sync API deprecated
- **v0.3.0**: Sync API moved behind feature flag
- **v1.0.0**: Sync API removed (async only)

### 8.3 Migration Checklist

- [ ] Review Beta breaking changes
- [ ] Update Cargo.toml dependencies
- [ ] Add Tokio runtime
- [ ] Add `.await` to all graph operations
- [ ] Convert function signatures to `async`
- [ ] Test with existing database
- [ ] Update error handling for async context
- [ ] Review performance characteristics
- [ ] Update monitoring/logging
- [ ] Deploy and monitor

---

## 9. Risk Assessment

### 9.1 Technical Risks

| Risk | Severity | Likelihood | Mitigation |
|------|----------|------------|------------|
| Async complexity introduces bugs | High | Medium | Comprehensive testing, code review |
| Performance regression | High | Low | Continuous benchmarking, profiling |
| Kafka integration issues | Medium | Medium | Thorough integration testing, fallback |
| Breaking API changes | High | High | Feature flags, migration guide |
| Sled async limitations | Medium | Low | Evaluate alternatives (RocksDB) |
| Memory leaks in async code | Medium | Low | Careful resource management, testing |

### 9.2 Operational Risks

| Risk | Severity | Likelihood | Mitigation |
|------|----------|------------|------------|
| Migration failures | High | Medium | Backup strategy, rollback plan |
| Observatory downtime | Medium | Low | Async event queue, retry logic |
| Increased resource usage | Medium | Medium | Resource limits, monitoring |
| Compatibility issues | Medium | Low | Version matrix testing |

### 9.3 Project Risks

| Risk | Severity | Likelihood | Mitigation |
|------|----------|------------|------------|
| Timeline slippage | Medium | Medium | Buffer time, parallel work streams |
| Resource unavailability | High | Low | Cross-training, documentation |
| Scope creep | Medium | Medium | Strict feature freeze, prioritization |
| Testing gaps | High | Low | Test automation, coverage targets |

### 9.4 Mitigation Strategies

#### 9.4.1 Technical Mitigations

1. **Async Complexity**
   - Use async-trait for clean trait definitions
   - Implement comprehensive async tests
   - Use tokio-console for debugging
   - Follow Tokio best practices

2. **Performance Regression**
   - Continuous benchmarking in CI
   - Performance regression tests
   - Profiling before each release
   - Load testing with realistic workloads

3. **Integration Reliability**
   - Circuit breaker pattern for Kafka
   - Async retry with exponential backoff
   - Local event queue for buffering
   - Graceful degradation

#### 9.4.2 Operational Mitigations

1. **Migration Safety**
   - Automated database backups
   - Rollback procedures documented
   - Canary deployments
   - Feature flags for gradual rollout

2. **Monitoring**
   - Enhanced metrics in Beta
   - Alerting on performance degradation
   - Distributed tracing
   - Log aggregation

---

## 10. Success Metrics

### 10.1 Performance Metrics

| Metric | MVP Baseline | Beta Target | Measurement Method |
|--------|--------------|-------------|-------------------|
| Write Latency (p95) | 50-80ms | <10ms | Prometheus histogram |
| Read Latency (p95) | 1-5ms | <1ms | Prometheus histogram |
| Query Throughput | 1k ops/sec | 100k ops/sec | Load testing |
| Concurrent Operations | 100 | 10,000 | Stress testing |
| Node Capacity | 10k tested | 1M+ tested | Scale testing |
| Graph Traversal (p95) | 10-30ms | <20ms | Benchmark suite |

### 10.2 Functional Metrics

| Feature | Target | Measurement |
|---------|--------|-------------|
| New Node Types | 3 types | Code review |
| New Edge Types | 5 types | Code review |
| Async API Coverage | 100% | API audit |
| Observatory Events | 8 event types | Integration tests |
| Test Coverage | >90% | Coverage report |
| Documentation | 100% public APIs | Doc tests |

### 10.3 Quality Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Test Pass Rate | 100% | CI pipeline |
| Code Coverage | >90% | cargo-tarpaulin |
| Clippy Warnings | 0 | CI linting |
| Compiler Warnings | 0 | CI build |
| Doc Test Pass Rate | 100% | cargo test --doc |
| Benchmark Stability | <5% variance | Repeated runs |

### 10.4 Adoption Metrics (Post-Release)

| Metric | 1 Month | 3 Months | 6 Months |
|--------|---------|----------|----------|
| Active Users | 50 | 200 | 500 |
| GitHub Stars | 100 | 500 | 1000 |
| Production Deployments | 10 | 50 | 100 |
| Issues Reported | <20 | <50 | <100 |
| Pull Requests | 5 | 20 | 50 |

### 10.5 Success Criteria

Beta release is considered successful if:

**Must Have:**
- ✅ All 3 new node types implemented and tested
- ✅ All 5 new edge types implemented and tested
- ✅ Complete async API with >90% coverage
- ✅ Observatory integration functional
- ✅ All performance targets met (p95 latencies)
- ✅ 100% test pass rate
- ✅ Zero critical bugs in production

**Should Have:**
- ✅ >90% code coverage
- ✅ Complete migration guide
- ✅ 3+ example applications
- ✅ Grafana dashboards for monitoring
- ✅ Load testing with 1M+ nodes successful

**Nice to Have:**
- ✅ Community contributions
- ✅ Blog post about Beta features
- ✅ Video tutorial
- ✅ Rust community showcase

---

## 11. Conclusion

### 11.1 Summary

This Beta Phase implementation plan transforms LLM-Memory-Graph from an MVP into a production-ready, enterprise-grade system with:

1. **Extended Capabilities**: 3 new node types, 5 new edge types
2. **Modern Architecture**: Full async API with Tokio
3. **Ecosystem Integration**: Real-time Observatory event streaming
4. **Performance**: 10x improvements across all metrics
5. **Scalability**: Handle 1M+ nodes, 10k concurrent operations

### 11.2 Timeline Overview

```
Week 1: Extended Data Model
Week 2: Edge Types
Week 3: Async Core
Week 4: Async Features
Week 5: Observatory Integration
Week 6: Testing & Optimization

Total: 6 weeks to Beta release
```

### 11.3 Resource Requirements

- **Engineers**: 2-3 Rust developers
- **Infrastructure**: Kafka instance, Prometheus, test environments
- **Budget**: Estimated $50-75k (6 weeks × 2.5 engineers × $150/hr)

### 11.4 Next Steps After Beta

Upon successful Beta completion:

1. **Gather Feedback**: 2-4 weeks of user feedback
2. **Bug Fixes**: Address any Beta issues
3. **v1.0 Planning**: Plan gRPC service, plugin system
4. **Documentation**: Comprehensive guides and tutorials
5. **Community Building**: Blog posts, talks, showcase

### 11.5 Approval and Sign-off

This plan requires approval from:
- [ ] Technical Lead
- [ ] Product Manager
- [ ] Engineering Manager
- [ ] DevOps Team

**Approved By**: _________________
**Date**: _________________
**Signature**: _________________

---

**Document Version**: 1.0
**Last Updated**: 2025-11-06
**Next Review**: Start of Beta implementation
**Contact**: LLM DevOps Team

---

*This is a living document and will be updated as implementation progresses.*
