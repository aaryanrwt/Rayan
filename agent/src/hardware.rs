use anyhow::{anyhow, Result};
use rayan_core::asg::{Asg, NodeKind};

pub struct HardwareAnalyzer {
    pub current_gpu: String,
    pub current_cpu: String,
}

impl Default for HardwareAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl HardwareAnalyzer {
    pub fn new() -> Self {
        Self {
            current_gpu: "amd".to_string(), // Mocked hardware detection
            current_cpu: "x86_64".to_string(),
        }
    }

    /// Prevents deployment of known-bad package combinations based on hardware telemetry
    pub fn validate_against_hardware(&self, asg: &Asg) -> Result<()> {
        for node in &asg.nodes {
            if let NodeKind::Package(pkg) = &node.kind {
                if pkg.name.contains("nvidia") && self.current_gpu == "amd" {
                    return Err(anyhow!("Hardware conflict: Cannot install NVIDIA drivers on an AMD GPU detected on this host."));
                }
            }
        }
        Ok(())
    }
}
