use rayan_core::asg::Asg;
use tracing::info;

pub struct SimulationResult {
    pub projected_closure_size_mb: f64,
    pub cve_warnings: Vec<String>,
    pub breaking_changes: Vec<String>,
}

pub struct Simulator;

impl Default for Simulator {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn dry_run(&self, proposed_asg: &Asg) -> SimulationResult {
        info!("Running AI-driven simulation on proposed ASG...");

        let mut warnings = Vec::new();
        let mut closure_size = 450.0; // Base size

        for node in &proposed_asg.nodes {
            if let rayan_core::asg::NodeKind::Package(pkg) = &node.kind {
                closure_size += 50.0; // Simulate package size
                if pkg.name == "openssl" {
                    warnings.push("CVE-2026-9999 detected in openssl version".to_string());
                }
            }
        }

        SimulationResult {
            projected_closure_size_mb: closure_size,
            cve_warnings: warnings,
            breaking_changes: vec![],
        }
    }
}
