use crate::asg::Asg;
use anyhow::{bail, Result};
use std::collections::HashSet;

/// A pure state machine engine that handles the transition from current machine state
/// to the desired ASG state.
pub struct StateEngine {
    current_state: Option<Asg>,
}

impl Default for StateEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateEngine {
    pub fn new() -> Self {
        Self {
            current_state: None,
        }
    }

    /// Loads the actual machine state by introspecting the system
    pub fn load_actual_state(&mut self) -> Result<()> {
        // TODO: Interface with Nix store/OCI to build actual ASG
        Ok(())
    }

    /// Validates the transition to the new target ASG
    pub fn validate_transition(&self, target: &Asg) -> Result<()> {
        let mut declared_nodes = HashSet::new();
        for node in &target.nodes {
            declared_nodes.insert(node.id.clone());
        }

        let mut violations = Vec::new();

        for node in &target.nodes {
            // Check dependencies
            for dep in &node.dependencies {
                if !declared_nodes.contains(dep) {
                    violations.push(format!(
                        "Node '{}' depends on missing node '{}'",
                        node.id, dep
                    ));
                }
            }

            // Check conflicts
            for conflict in &node.conflicts_with {
                if declared_nodes.contains(conflict) {
                    violations.push(format!(
                        "Node '{}' conflicts with declared node '{}'",
                        node.id, conflict
                    ));
                }
            }
        }

        if !violations.is_empty() {
            let error_msg = violations.join("\n");
            bail!(
                "Neuro-Symbolic Validation Failed with {} violations:\n{}",
                violations.len(),
                error_msg
            );
        }

        Ok(())
    }

    /// Returns the currently loaded machine state, if any.
    pub fn current_state(&self) -> Option<&Asg> {
        self.current_state.as_ref()
    }
}
