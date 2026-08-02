use serde::{Deserialize, Serialize};

/// The Abstract Semantic Graph (ASG).
/// This is the intermediate representation that the parser outputs and the compiler reads.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asg {
    pub nodes: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub kind: NodeKind,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub conflicts_with: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeKind {
    Package(PackageInfo),
    Service(ServiceInfo),
    EnvironmentVariable(String, String),
    /// Catch-all for unresolved literals or arbitrary config
    RawNix(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub enable: bool,
}
