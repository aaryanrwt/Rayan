use anyhow::Result;
use rayan_core::asg::{Node, NodeKind, PackageInfo};
use tracing::info;

pub struct RayanHubClient {
    _client: reqwest::Client,
    base_url: String,
}

impl Default for RayanHubClient {
    fn default() -> Self {
        Self::new("https://hub.rayan.dev/api/v1")
    }
}

impl RayanHubClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            _client: reqwest::Client::new(),
            base_url: base_url.to_string(),
        }
    }

    /// Pulls a semantic ASG Node from the Rayan Hub.
    /// For Phase 1, we mock the network request if it's the `theprimeagen/neovim` node.
    pub async fn pull_node(&self, id: &str) -> Result<Node> {
        info!("Fetching node {} from Rayan Hub ({})", id, self.base_url);

        // Mocked response for MVP validation
        if id == "theprimeagen/neovim" {
            // Simulate network latency
            tokio::time::sleep(tokio::time::Duration::from_millis(600)).await;

            return Ok(Node {
                id: "theprimeagen_neovim_config".to_string(),
                kind: NodeKind::Package(PackageInfo {
                    name: "neovim".to_string(),
                    version: Some("nightly".to_string()),
                }),
                dependencies: vec!["ripgrep_pkg".to_string(), "fd_pkg".to_string()],
                conflicts_with: vec!["emacs_pkg".to_string()],
            });
        }

        // In the real implementation, this would be:
        // let url = format!("{}/nodes/{}", self.base_url, id);
        // let node = self.client.get(&url).send().await?.json::<Node>().await?;
        // Ok(node)

        anyhow::bail!("Node {} not found in Global Knowledge Commons (Mocked)", id)
    }
}
