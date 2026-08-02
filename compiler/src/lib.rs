use anyhow::Result;
use rayan_core::asg::{Asg, NodeKind};

/// Compiles a validated ASG into deterministically reproducible Nix expressions.
/// Defaults to generating a home-manager compatible output.
pub fn compile_to_nix(asg: &Asg) -> Result<String> {
    let mut out = String::new();
    out.push_str("{ config, pkgs, ... }:\n{\n");

    let mut packages = Vec::new();
    let mut services = Vec::new();
    let mut env_vars = Vec::new();

    for node in &asg.nodes {
        match &node.kind {
            NodeKind::Package(pkg) => {
                packages.push(pkg.name.clone());
            }
            NodeKind::Service(svc) => {
                services.push(svc.clone());
            }
            NodeKind::EnvironmentVariable(k, v) => {
                env_vars.push((k.clone(), v.clone()));
            }
            NodeKind::RawNix(raw) => {
                out.push_str(&format!("  {}\n", raw));
            }
        }
    }

    if !packages.is_empty() {
        out.push_str("  home.packages = with pkgs; [\n");
        for pkg in packages {
            out.push_str(&format!("    {}\n", pkg));
        }
        out.push_str("  ];\n\n");
    }

    if !services.is_empty() {
        for svc in services {
            out.push_str(&format!(
                "  services.{} = {{\n    enable = {};\n  }};\n",
                svc.name, svc.enable
            ));
        }
        out.push('\n');
    }

    if !env_vars.is_empty() {
        out.push_str("  home.sessionVariables = {\n");
        for (k, v) in env_vars {
            out.push_str(&format!("    {} = \"{}\";\n", k, v));
        }
        out.push_str("  };\n");
    }

    out.push_str("}\n");
    Ok(out)
}

/// Compiles a validated ASG into nix-darwin compatible expressions for macOS.
pub fn compile_to_darwin(asg: &Asg) -> Result<String> {
    let mut out = String::new();
    out.push_str("{ config, pkgs, ... }:\n{\n");

    let mut packages = Vec::new();
    for node in &asg.nodes {
        if let NodeKind::Package(pkg) = &node.kind {
            packages.push(pkg.name.clone());
        }
    }

    if !packages.is_empty() {
        out.push_str("  environment.systemPackages = with pkgs; [\n");
        for pkg in packages {
            out.push_str(&format!("    {}\n", pkg));
        }
        out.push_str("  ];\n\n");
    }

    out.push_str("  services.nix-daemon.enable = true;\n");
    out.push_str("  system.stateVersion = 4;\n");
    out.push_str("}\n");
    Ok(out)
}

/// Compiles a validated ASG into an OCI-compliant Containerfile.
pub fn compile_to_oci(asg: &Asg) -> Result<String> {
    let mut out = String::new();
    out.push_str("FROM ubuntu:24.04\n\n");
    out.push_str("RUN apt-get update && apt-get install -y \\\n");

    let mut packages = Vec::new();
    let mut env_vars = Vec::new();

    for node in &asg.nodes {
        match &node.kind {
            NodeKind::Package(pkg) => {
                packages.push(pkg.name.clone());
            }
            NodeKind::EnvironmentVariable(k, v) => {
                env_vars.push((k.clone(), v.clone()));
            }
            _ => {}
        }
    }

    if !packages.is_empty() {
        for (i, pkg) in packages.iter().enumerate() {
            if i == packages.len() - 1 {
                out.push_str(&format!("    {} && rm -rf /var/lib/apt/lists/*\n\n", pkg));
            } else {
                out.push_str(&format!("    {} \\\n", pkg));
            }
        }
    } else {
        out.push_str("    ca-certificates && rm -rf /var/lib/apt/lists/*\n\n");
    }

    for (k, v) in env_vars {
        out.push_str(&format!("ENV {}={}\n", k, v));
    }

    Ok(out)
}
