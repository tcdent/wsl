//! WSL Agent CLI - A tool for adding facts to WSL files using an AI agent
//!
//! This CLI accepts plain-text facts and uses an AI agent to properly format
//! and incorporate them into a WSL (Worldview State Language) file.

use anyhow::Result;
use clap::Parser;
use codey::{Agent, AgentRuntimeConfig, AgentStep, RequestMode, SimpleTool, ToolRegistry};
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;

/// System prompt that gives the agent full knowledge of WSL syntax
const SYSTEM_PROMPT: &str = r#"You are a WSL (Worldview State Language) agent. Your task is to take plain-text facts or statements and incorporate them into a WSL file using the proper notation.

## WSL Structure

A WSL document is hierarchical:
```
Concept           (unindented, column 0)
  .facet          (2-space indent, dot prefix)
    - claim       (4-space indent, dash prefix)
```

Every concept must have at least one facet. Every facet must have at least one claim.

## Notation Reference

### Inline Elements (used within claims)
| Symbol | Meaning | Position |
|--------|---------|----------|
| `|` | condition (when/if) | After claim text |
| `@` | source | After claim/conditions |
| `&` | reference to other concept.facet | After claim |

### Brief Form Operators (relationships)
| Symbol | Meaning | Example |
|--------|---------|---------|
| `=>` | causes, leads to | `power => corruption` |
| `<=` | caused by, results from | `trust <= consistency` |
| `<>` | mutual, bidirectional | `accountability <> trust` |
| `><` | tension, conflicts with | `efficiency >< thoroughness` |
| `~` | similar to, resembles | `authority ~ influence` |
| `=` | equivalent to, means | `liberty = freedom` |
| `vs` | in contrast to | `fast vs slow` |
| `//` | regardless of | `persists // context` |

### Modifiers (suffix markers)
| Symbol | Meaning | Example |
|--------|---------|---------|
| `^` | increasing, trending up | `concentration^` |
| `v` | decreasing, trending down | `trust v` |
| `!` | strong, emphatic, high confidence | `fast !` |
| `?` | uncertain, contested, tentative | `free-will?` |
| `*` | notable, important, flagged | `paradigm-shift*` |

### Evolution Markers
Use `[<= prior belief]` to show that a belief supersedes an earlier one:
```
- adaptive, context-dependent [<= inherently good]
```

## Claim Syntax Order

Claims follow this order:
```
- [claim text] | [condition] | [condition] @[source] @[source] &[reference]
```

## Your Task

When given a plain-text fact or statement:
1. First, read the current WSL file to understand its structure and existing concepts
2. Determine if this fact belongs to an existing concept/facet or requires a new one
3. Format the fact as proper WSL notation
4. Use the edit_wsl tool to add or modify the appropriate line(s)
5. After editing, briefly confirm what you added

## Important Guidelines

- **Preserve density**: No articles (a, the), no copulas (is, are), no filler words
- **Use brief forms**: Express relationships with symbols, not prose
- **Be precise**: Place facts in the most appropriate concept and facet
- **Create structure as needed**: Add new concepts or facets if the fact doesn't fit existing ones
- **Maintain hierarchy**: Always ensure concepts have facets, facets have claims
- **Use references**: Link related concepts with `&Concept.facet` rather than duplicating

## Examples

Plain text: "I believe that power tends to corrupt those who hold it without oversight"
WSL: `- corrupts | unchecked !`
Under: `Power` > `.nature`

Plain text: "Trust takes a long time to build but can be destroyed instantly"
WSL:
```
Trust
  .formation
    - slow
  .erosion
    - fast !
    - asymmetric vs formation &Trust.formation
```

Plain text: "According to behavioral economics, humans are loss averse"
WSL: `- loss-averse @behavioral-economics`
Under: `Human-nature` > `.cognition`
"#;

/// CLI for adding facts to WSL files using an AI agent
#[derive(Parser, Debug)]
#[command(name = "wsl")]
#[command(about = "Add facts to WSL files using an AI agent")]
#[command(version)]
struct Cli {
    /// The fact or statement to add to the WSL file
    #[arg(required = true)]
    fact: String,

    /// Path to the WSL file to modify
    #[arg(short, long, default_value = "worldview.wsl")]
    file: PathBuf,

    /// Model to use (claude-sonnet-4-20250514 or claude-opus-4-5-20251101)
    #[arg(short, long, default_value = "claude-sonnet-4-20250514")]
    model: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

/// Create the read_wsl tool definition
fn create_read_tool() -> SimpleTool {
    SimpleTool::new(
        "read_wsl",
        "Read the current contents of the WSL file. Returns the file contents with line numbers.",
        json!({
            "type": "object",
            "properties": {},
            "required": []
        }),
    )
}

/// Create the edit_wsl tool definition
fn create_edit_tool() -> SimpleTool {
    SimpleTool::new(
        "edit_wsl",
        r#"Edit the WSL file. You can either:
1. Replace a specific line by providing line_number and new_content
2. Insert a new line after a specific line by providing line_number and new_content with insert=true
3. Append to the end of the file by providing only new_content (no line_number)
4. Delete a line by providing line_number and delete=true

The tool will validate the resulting file against WSL syntax rules."#,
        json!({
            "type": "object",
            "properties": {
                "line_number": {
                    "type": "integer",
                    "description": "The line number to edit (1-indexed). If omitted, appends to end."
                },
                "new_content": {
                    "type": "string",
                    "description": "The new content for this line. For concepts, use no indentation. For facets, use 2-space indent with '.' prefix. For claims, use 4-space indent with '-' prefix."
                },
                "insert": {
                    "type": "boolean",
                    "description": "If true, insert new_content after line_number instead of replacing. Default false."
                },
                "delete": {
                    "type": "boolean",
                    "description": "If true, delete the line at line_number. Default false."
                }
            },
            "required": []
        }),
    )
}

/// Handle the read_wsl tool call
fn handle_read_wsl(file_path: &PathBuf) -> String {
    if !file_path.exists() {
        return "File does not exist yet. You can create it by using edit_wsl with new_content.".to_string();
    }

    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            // Return with line numbers
            content
                .lines()
                .enumerate()
                .map(|(i, line)| format!("{:4}: {}", i + 1, line))
                .collect::<Vec<_>>()
                .join("\n")
        }
        Err(e) => format!("Error reading file: {}", e),
    }
}

/// Handle the edit_wsl tool call
fn handle_edit_wsl(file_path: &PathBuf, params: &serde_json::Value) -> String {
    let line_number = params.get("line_number").and_then(|v| v.as_i64()).map(|n| n as usize);
    let new_content = params.get("new_content").and_then(|v| v.as_str());
    let insert = params.get("insert").and_then(|v| v.as_bool()).unwrap_or(false);
    let delete = params.get("delete").and_then(|v| v.as_bool()).unwrap_or(false);

    // Read current file content (or start empty)
    let current_content = if file_path.exists() {
        match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => return format!("Error reading file: {}", e),
        }
    } else {
        String::new()
    };

    let mut lines: Vec<String> = current_content.lines().map(|s| s.to_string()).collect();

    // Perform the edit operation
    let result = match (line_number, new_content, delete) {
        (Some(ln), _, true) => {
            // Delete line
            if ln == 0 || ln > lines.len() {
                return format!("Error: Line number {} out of range (1-{})", ln, lines.len());
            }
            lines.remove(ln - 1);
            Ok(())
        }
        (Some(ln), Some(content), false) if insert => {
            // Insert after line
            if ln > lines.len() {
                return format!("Error: Line number {} out of range for insert (1-{})", ln, lines.len());
            }
            lines.insert(ln, content.to_string());
            Ok(())
        }
        (Some(ln), Some(content), false) => {
            // Replace line
            if ln == 0 || ln > lines.len() {
                return format!("Error: Line number {} out of range (1-{})", ln, lines.len());
            }
            lines[ln - 1] = content.to_string();
            Ok(())
        }
        (None, Some(content), false) => {
            // Append to end
            // Add a blank line before if the file doesn't end with one and isn't empty
            if !lines.is_empty() && !lines.last().map(|l| l.trim().is_empty()).unwrap_or(true) {
                lines.push(String::new());
            }
            for line in content.lines() {
                lines.push(line.to_string());
            }
            Ok(())
        }
        _ => {
            Err("Invalid parameters: must provide either new_content or delete=true")
        }
    };

    if let Err(e) = result {
        return format!("Error: {}", e);
    }

    // Reconstruct the file content
    let new_file_content = lines.join("\n");

    // Ensure file ends with newline
    let new_file_content = if new_file_content.ends_with('\n') || new_file_content.is_empty() {
        new_file_content
    } else {
        format!("{}\n", new_file_content)
    };

    // Validate the new content
    let validation = wsl_validator::validate(&new_file_content);

    if !validation.is_valid() {
        let errors: Vec<String> = validation.errors.iter().map(|e| e.to_string()).collect();
        return format!(
            "Validation failed - file not modified:\n{}",
            errors.join("\n")
        );
    }

    // Write the file
    if let Err(e) = std::fs::write(file_path, &new_file_content) {
        return format!("Error writing file: {}", e);
    }

    // Return success with any warnings
    if validation.has_warnings() {
        let warnings: Vec<String> = validation.warnings.iter().map(|w| w.to_string()).collect();
        format!("Edit successful with warnings:\n{}", warnings.join("\n"))
    } else {
        "Edit successful. File validated.".to_string()
    }
}

/// Handle a tool call from the agent
fn handle_tool_call(file_path: &PathBuf, tool_name: &str, params: &serde_json::Value) -> String {
    match tool_name {
        "read_wsl" => handle_read_wsl(file_path),
        "edit_wsl" => handle_edit_wsl(file_path, params),
        _ => format!("Unknown tool: {}", tool_name),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Check for API key
    if std::env::var("ANTHROPIC_API_KEY").is_err() {
        eprintln!("Error: ANTHROPIC_API_KEY environment variable not set");
        std::process::exit(1);
    }

    // Resolve the file path
    let file_path = if cli.file.is_absolute() {
        cli.file.clone()
    } else {
        std::env::current_dir()?.join(&cli.file)
    };

    if cli.verbose {
        eprintln!("WSL file: {:?}", file_path);
        eprintln!("Model: {}", cli.model);
        eprintln!("Fact: {}", cli.fact);
    }

    // Create tool registry with our custom tools
    let mut registry = ToolRegistry::empty();
    registry.register(Arc::new(create_read_tool()));
    registry.register(Arc::new(create_edit_tool()));

    // Configure the agent
    let config = AgentRuntimeConfig {
        model: cli.model.clone(),
        max_tokens: 4096,
        thinking_budget: 1024,  // Minimum required
        max_retries: 3,
        compaction_thinking_budget: 2000,
    };

    // Create the agent
    let mut agent = Agent::new(
        config,
        SYSTEM_PROMPT,
        None, // Use ANTHROPIC_API_KEY env var
        registry,
    );

    // Format the user message
    let user_message = format!(
        "Please add this fact to the WSL file at {:?}:\n\n{}",
        file_path, cli.fact
    );

    // Send the request
    agent.send_request(&user_message, RequestMode::Normal);

    // Process the agent loop
    while let Some(step) = agent.next().await {
        match step {
            AgentStep::TextDelta(text) => {
                print!("{}", text);
            }
            AgentStep::ThinkingDelta(thinking) => {
                if cli.verbose {
                    eprint!("[thinking] {}", thinking);
                }
            }
            AgentStep::CompactionDelta(_) => {
                // Not used in our simple case
            }
            AgentStep::ToolRequest(tool_calls) => {
                for call in tool_calls {
                    if cli.verbose {
                        eprintln!("\n[tool] {} with {:?}", call.name, call.params);
                    }

                    let result = handle_tool_call(&file_path, &call.name, &call.params);

                    if cli.verbose {
                        eprintln!("[result] {}", result);
                    }

                    agent.submit_tool_result(&call.call_id, result);
                }
            }
            AgentStep::Retrying { attempt, error } => {
                if cli.verbose {
                    eprintln!("[retry] Attempt {} after error: {}", attempt, error);
                }
            }
            AgentStep::Finished { usage } => {
                if cli.verbose {
                    eprintln!("\n[done] {}", usage.format_log());
                }
                break;
            }
            AgentStep::Error(e) => {
                eprintln!("\nError: {}", e);
                std::process::exit(1);
            }
        }
    }

    println!("\n\nWSL file updated: {:?}", file_path);
    Ok(())
}
