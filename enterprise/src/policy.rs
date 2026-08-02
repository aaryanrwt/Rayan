use anyhow::{anyhow, Result};
use rayan_core::asg::{Asg, NodeKind};

pub struct PolicyEngine {
    pub require_pinned_versions: bool,
    pub allow_experimental_modules: bool,
}

impl Default for PolicyEngine {
    fn default() -> Self {
        Self {
            require_pinned_versions: true,
            allow_experimental_modules: false,
        }
    }
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Evaluates an ASG against enterprise policies. Returns Ok if compliant, Err with violation details.
    pub fn evaluate(&self, asg: &Asg) -> Result<()> {
        for node in &asg.nodes {
            if let NodeKind::Package(pkg) = &node.kind {
                if self.require_pinned_versions && pkg.version.is_none() {
                    return Err(anyhow!(
                        "Policy Violation: Package '{}' is unpinned. Enterprise policy requires strict version pinning.",
                        pkg.name
                    ));
                }
            }
        }
        Ok(())
    }
}
