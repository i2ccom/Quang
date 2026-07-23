//! Core HyperGraph types for the Workplace collaboration graph.
//!
//! All entities (WorkSpace, Team, Project, Task, etc.) are typed nodes
//! connected by typed edges. Views are graph projections that select
//! subsets of nodes/edges and render them.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// NodeId
// ---------------------------------------------------------------------------

/// A unique identifier for any node in the collaboration graph.
/// Encodes the entity kind in the prefix for human readability and graph routing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl NodeId {
    pub fn new(kind: &str) -> Self {
        Self(format!(
            "{}_{}",
            kind,
            Uuid::new_v4().to_string().replace('-', "")
        ))
    }

    pub fn kind(&self) -> &str {
        self.0.split('_').next().unwrap_or("unknown")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// EdgeId
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EdgeId(pub String);

impl EdgeId {
    pub fn new() -> Self {
        Self(format!("e_{}", Uuid::new_v4().to_string().replace('-', "")))
    }
}

impl Default for EdgeId {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// EdgeKind
// ---------------------------------------------------------------------------

/// Typed relationships between nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeKind {
    /// Node belongs to a parent (e.g., Task -> Project)
    BelongsTo,
    /// Node is assigned to an actor (e.g., Task -> User)
    AssignedTo,
    /// Node depends on another (e.g., Task -> Task)
    DependsOn,
    /// Node is reviewed by an actor (e.g., Review -> User)
    ReviewedBy,
    /// Node was created by an actor
    CreatedBy,
    /// Node was generated from another (e.g., Summary -> Meeting)
    GeneratedFrom,
    /// Node is a member of a group (e.g., User -> Team)
    MemberOf,
    /// Node is a child in a hierarchy (e.g., Project -> WorkSpace)
    ChildOf,
    /// Node references another (generic)
    References,
    /// Custom edge type for extensibility
    Custom(String),
}

impl std::fmt::Display for EdgeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EdgeKind::BelongsTo => write!(f, "belongs_to"),
            EdgeKind::AssignedTo => write!(f, "assigned_to"),
            EdgeKind::DependsOn => write!(f, "depends_on"),
            EdgeKind::ReviewedBy => write!(f, "reviewed_by"),
            EdgeKind::CreatedBy => write!(f, "created_by"),
            EdgeKind::GeneratedFrom => write!(f, "generated_from"),
            EdgeKind::MemberOf => write!(f, "member_of"),
            EdgeKind::ChildOf => write!(f, "child_of"),
            EdgeKind::References => write!(f, "references"),
            EdgeKind::Custom(s) => write!(f, "custom_{}", s),
        }
    }
}

// ---------------------------------------------------------------------------
// Edge
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: EdgeId,
    pub kind: EdgeKind,
    pub source: NodeId,
    pub target: NodeId,
    pub metadata: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

impl Edge {
    pub fn new(kind: EdgeKind, source: NodeId, target: NodeId) -> Self {
        Self {
            id: EdgeId::new(),
            kind,
            source,
            target,
            metadata: HashMap::new(),
            created_at: Utc::now(),
        }
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

// ---------------------------------------------------------------------------
// NodeKind
// ---------------------------------------------------------------------------

/// Discriminator for node types in the graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeKind {
    WorkSpace,
    Team,
    Project,
    Channel,
    ChatMessage,
    Task,
    Goal,
    Review,
    Summary,
    Meeting,
    MeetingRoom,
    Participant,
    MediaStream,
    Recording,
    Custom(String),
}

impl std::fmt::Display for NodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::WorkSpace => write!(f, "workspace"),
            NodeKind::Team => write!(f, "team"),
            NodeKind::Project => write!(f, "project"),
            NodeKind::Channel => write!(f, "channel"),
            NodeKind::ChatMessage => write!(f, "chat_message"),
            NodeKind::Task => write!(f, "task"),
            NodeKind::Goal => write!(f, "goal"),
            NodeKind::Review => write!(f, "review"),
            NodeKind::Summary => write!(f, "summary"),
            NodeKind::Meeting => write!(f, "meeting"),
            NodeKind::MeetingRoom => write!(f, "meeting_room"),
            NodeKind::Participant => write!(f, "participant"),
            NodeKind::MediaStream => write!(f, "media_stream"),
            NodeKind::Recording => write!(f, "recording"),
            NodeKind::Custom(s) => write!(f, "custom_{}", s),
        }
    }
}

// ---------------------------------------------------------------------------
// HyperGraph
// ---------------------------------------------------------------------------

/// The central collaboration graph. All state lives here.
/// Nodes are entities, edges are relationships. Views project subsets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperGraph {
    pub nodes: HashMap<NodeId, serde_json::Value>,
    pub node_kinds: HashMap<NodeId, NodeKind>,
    pub edges: Vec<Edge>,
    /// Index: source node -> outgoing edges
    pub out_edges: HashMap<NodeId, Vec<usize>>,
    /// Index: target node -> incoming edges
    pub in_edges: HashMap<NodeId, Vec<usize>>,
}

impl Default for HyperGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl HyperGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            node_kinds: HashMap::new(),
            edges: Vec::new(),
            out_edges: HashMap::new(),
            in_edges: HashMap::new(),
        }
    }

    /// Add a typed node to the graph. `data` must be serializable.
    pub fn add_node<T: Serialize>(&mut self, id: NodeId, kind: NodeKind, data: &T) {
        let value = serde_json::to_value(data).expect("Node data must be serializable");
        self.node_kinds.insert(id.clone(), kind);
        self.nodes.insert(id, value);
    }

    /// Add an edge between two nodes.
    pub fn add_edge(&mut self, edge: Edge) {
        let idx = self.edges.len();
        self.edges.push(edge);
        let edge = &self.edges[idx];

        self.out_edges
            .entry(edge.source.clone())
            .or_default()
            .push(idx);

        self.in_edges
            .entry(edge.target.clone())
            .or_default()
            .push(idx);
    }

    /// Get outgoing edges from a node.
    pub fn outgoing(&self, id: &NodeId) -> Vec<&Edge> {
        self.out_edges
            .get(id)
            .map(|indices| indices.iter().map(|&i| &self.edges[i]).collect())
            .unwrap_or_default()
    }

    /// Get incoming edges to a node.
    pub fn incoming(&self, id: &NodeId) -> Vec<&Edge> {
        self.in_edges
            .get(id)
            .map(|indices| indices.iter().map(|&i| &self.edges[i]).collect())
            .unwrap_or_default()
    }

    /// Find nodes of a specific kind.
    pub fn nodes_of_kind(&self, kind: &NodeKind) -> Vec<(NodeId, &serde_json::Value)> {
        self.node_kinds
            .iter()
            .filter(|(_, k)| *k == kind)
            .filter_map(|(id, _)| self.nodes.get(id).map(|val| (id.clone(), val)))
            .collect()
    }

    /// Get the children of a node via ChildOf edges.
    pub fn children(&self, id: &NodeId) -> Vec<(Edge, &serde_json::Value)> {
        self.outgoing(id)
            .into_iter()
            .filter(|e| e.kind == EdgeKind::ChildOf)
            .filter_map(|e| self.nodes.get(&e.target).map(|val| (e.clone(), val)))
            .collect()
    }

    /// Get the parent of a node via ChildOf edge.
    pub fn parent(&self, id: &NodeId) -> Option<(Edge, &serde_json::Value)> {
        self.incoming(id)
            .into_iter()
            .find(|e| e.kind == EdgeKind::ChildOf)
            .and_then(|e| self.nodes.get(&e.source).map(|val| (e.clone(), val)))
    }

    /// Remove a node and all its edges.
    pub fn remove_node(&mut self, id: &NodeId) {
        self.nodes.remove(id);
        self.node_kinds.remove(id);

        // Remove edges referencing this node
        let to_remove: Vec<usize> = self
            .edges
            .iter()
            .enumerate()
            .filter(|(_, e)| e.source == *id || e.target == *id)
            .map(|(i, _)| i)
            .collect();

        for &i in to_remove.iter().rev() {
            let edge = &self.edges[i];
            if let Some(edges) = self.out_edges.get_mut(&edge.source) {
                edges.retain(|&e| e != i);
            }
            if let Some(edges) = self.in_edges.get_mut(&edge.target) {
                edges.retain(|&e| e != i);
            }
            self.edges.remove(i);
        }
    }

    /// Number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

// ---------------------------------------------------------------------------
// ActorId — who performed an action
// ---------------------------------------------------------------------------

/// Identifier for an actor: either a human user or an AI agent.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorId {
    Human(String),
    Agent(String),
}

impl ActorId {
    pub fn human(id: &str) -> Self {
        Self::Human(id.to_string())
    }

    pub fn agent(id: &str) -> Self {
        Self::Agent(id.to_string())
    }

    pub fn as_str(&self) -> &str {
        match self {
            ActorId::Human(s) | ActorId::Agent(s) => s.as_str(),
        }
    }

    pub fn is_human(&self) -> bool {
        matches!(self, ActorId::Human(_))
    }

    pub fn is_agent(&self) -> bool {
        matches!(self, ActorId::Agent(_))
    }
}

impl std::fmt::Display for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActorId::Human(id) => write!(f, "human:{}", id),
            ActorId::Agent(id) => write!(f, "agent:{}", id),
        }
    }
}

// ---------------------------------------------------------------------------
// Timestamp
// ---------------------------------------------------------------------------

pub type Timestamp = DateTime<Utc>;

pub fn now() -> Timestamp {
    Utc::now()
}
