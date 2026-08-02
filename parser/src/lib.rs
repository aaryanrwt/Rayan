use anyhow::{anyhow, Result};
use rayan_core::asg::{Asg, Node};

/// Parses a literate markdown document into the Abstract Semantic Graph (ASG).
/// This implementation extracts JSON blocks embedded in ` ```rayan ` markdown fences.
pub fn parse_literate_document(input: &str) -> Result<Asg> {
    let mut nodes = Vec::new();
    let mut current_input = input;

    while let Some(start) = current_input.find("```rayan\n") {
        current_input = &current_input[start + "```rayan\n".len()..];
        if let Some(end) = current_input.find("\n```") {
            let block = &current_input[..end];
            current_input = &current_input[end + "\n```".len()..];

            // Attempt to parse the block directly as a JSON array or single object
            if block.trim().starts_with('[') {
                let parsed_nodes: Vec<Node> = serde_json::from_str(block)
                    .map_err(|e| anyhow!("Failed to parse Rayan JSON array block: {}", e))?;
                nodes.extend(parsed_nodes);
            } else {
                let node: Node = serde_json::from_str(block)
                    .map_err(|e| anyhow!("Failed to parse Rayan JSON object block: {}", e))?;
                nodes.push(node);
            }
        } else {
            break;
        }
    }

    Ok(Asg { nodes })
}
