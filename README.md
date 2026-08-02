# Rayan

<div align="center">
  <p>The Neuro-Symbolic Declarative Environment Engine</p>
  
  [![CI](https://github.com/aaryanrawat/rayan/actions/workflows/ci.yml/badge.svg)](https://github.com/aaryanrawat/rayan/actions/workflows/ci.yml)
  [![Security Audit](https://github.com/aaryanrawat/rayan/actions/workflows/security.yml/badge.svg)](https://github.com/aaryanrawat/rayan/actions/workflows/security.yml)
  [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
</div>

---

**Original Research, Product Vision and Architecture by Aaryan Rawat**

## Overview

Rayan is a next-generation, cloud-native declarative environment engine. Built entirely in Rust, it merges the absolute determinism of functional package managers (like Nix) with the adaptability of Anthropic's Claude-3 Large Language Models. 

At its core, Rayan parses your configuration into an **Abstract Semantic Graph (ASG)**. This ASG can be mutated via natural language, validated against hardware telemetry, protected by enterprise Policy-as-Code constraints, and deterministically compiled into native `nix-darwin` macOS configurations, NixOS environments, or OCI-compliant `Containerfile` builds.

## Key Features

- **Neuro-Symbolic Agent:** Chat with your infrastructure. The agent reads your intent, utilizes RAG (Learning Mode) to parse NixOS documentation, and safely mutates your system.
- **Zero-Copy Parser:** Blazing fast (sub-microsecond) literate markdown parsing that converts `.md` or `.rayan` documents directly into an ASG.
- **Universal Compilation:** Generates bulletproof Nix closures, `nix-darwin` configurations, and OCI Dockerfiles from the same source of truth.
- **Enterprise Governance (Free & Open Source):** Natively enforce Policy-as-Code (e.g., rejecting unpinned packages) and generate CycloneDX SBOMs for supply-chain security without enterprise paywalls.
- **Rollback Engine (TUI):** A `ratatui`-powered git-log dashboard mapping generations to local SQLite tracking.
- **Global Commons API:** A Cloud backend (`rayan-cloud`) and Next.js frontend (`rayan-web`) to search, publish, and visually build configurations in a public marketplace.

## Architecture

Rayan is constructed from 13 highly cohesive, independently compiled Rust workspace crates:

| Layer | Component | Description |
|---|---|---|
| **Core** | `rayan-core` | The ASG Definition and SMT State Engine validator |
| **Parsing** | `rayan-parser` | Literate Markdown parser |
| **Compiler** | `rayan-compiler` | Multi-target code generator (Nix, Darwin, OCI) |
| **AI** | `rayan-agent` | Anthropic Claude-3 RAG Planner with Hardware & Voice telemetry stubs |
| **Enterprise**| `rayan-enterprise` | CycloneDX SBOM generator & Policy-as-Code |
| **Analysis** | `rayan-simulator` | CVE Dry Runner & Size Estimator |
| **Daemons** | `rayan-daemon`, `lsp` | Filesystem Drift Detector and Language Server |
| **Clients** | `rayan-tui`, `cli`, `desktop` | Terminal, CLI, and Tauri Desktop GUI interfaces |
| **Cloud** | `rayan-cloud`, `commons` | Axum/Postgres backend and Cloud Sync Engine |

## Quick Start

### Installation

Clone the repository and build the workspace. (A pre-compiled binary release will be available soon).

```bash
git clone https://github.com/aaryanrawat/rayan.git
cd rayan
cargo build --release
```

### Usage

**Ask the AI to mutate your environment:**
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
cargo run --release -p rayan -- reason "Install neovim and docker"
```

**View the Rollback TUI:**
```bash
cargo run --release -p rayan -- tui
```

**Start the Language Server:**
```bash
cargo run --release -p rayan -- lsp
```

## Security & Privacy Philosophy

We operate on a **Zero Trust** architecture. 
- The AI Agent is **fail-closed**. The LLM acts only as a *Reasoner*. Its output is strictly validated by the local deterministic `StateEngine` before any change is permitted to compile. 
- The `rayan-enterprise` policies prevent the deployment of untrusted or unpinned packages.
- Absolutely no PII, hardware MAC addresses, or absolute paths are transmitted to the LLM during planning.

## Client Ecosystem

Rayan integrates directly into your existing workflow:
- **VS Code Extension:** Available in `extensions/vscode/`.
- **Emacs Major Mode:** Available in `extensions/emacs/rayan-mode.el`.
- **Tauri Desktop:** Available in `desktop/`.

## Contributing

We welcome community contributions. Please read our [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). 

All PRs must pass `cargo fmt --all` and `cargo clippy --workspace -- -D warnings`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
