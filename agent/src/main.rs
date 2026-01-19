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

Below is the complete WSL specification. Study it carefully before making any edits.

---

# Worldview State Language (WSL)

**Specification v0.1 — Draft**

---

## Abstract

Worldview State Language (WSL) is a compact, declarative notation for encoding and maintaining conceptual worldviews over time. It provides a structured format for storing beliefs, stances, and understanding about concepts—designed to be included in full within every interaction context rather than retrieved selectively.

WSL is not a general-purpose communication language. It is a specialized format for preserving *how concepts are understood*, optimized for semantic density without sacrificing clarity. The notation is intended to be intuitive for large language models to parse, reason about, and autonomously maintain, while remaining human-inspectable. Beliefs are stored as structured claims with conditions and sources, enabling an LLM to hold persistent context about a user, domain, or system across extended interactions.

---

## Motivation

Large language models operate within fixed context windows. Existing approaches to persistent memory—such as retrieval-augmented generation (RAG)—selectively include information based on relevance to the current query. This works for factual lookup but fails for *worldview*: the foundational beliefs and stances that should inform all reasoning, not just topically-matched queries.

WSL solves this by defining a notation dense enough that an entire belief system (potentially tens of thousands of tokens) can remain in context permanently. Rather than retrieving relevant memories, the LLM carries its complete understanding forward into every interaction.

The format prioritizes:
- **Semantic density** — Strip prose, keep meaning
- **Structural consistency** — Predictable hierarchy for reliable parsing
- **Evolutionary tracking** — Beliefs change; the notation accommodates revision
- **Autonomous maintenance** — The LLM updates the document without user intervention

---

## Design Principles

**State over narrative**
WSL captures what is believed, not the story of how it came to be believed. History is preserved compactly when relevant, but the primary representation is current state.

**Predictability allows omission**
Borrowed from stenographic shorthand: if structure or context makes something inferable, don't write it. No articles, no copulas, no filler.

**Conflict tolerance**
Real worldviews contain tensions and contradictions. WSL holds conflicting claims without forcing resolution.

**Freeform vocabulary**
No predefined concept names, facet labels, or claim terms. The notation defines structure and relationships; content remains unconstrained.

**LLM-native, human-inspectable**
Optimized for machine parsing and reasoning. Human readability is a secondary benefit, not a design constraint.

---

## Inspirations

### Stenographic Shorthand
Systems like Gregg, Pitman, and Teeline informed WSL's approach to density:
- **Omission of predictable elements** — Common words and inferable structure are dropped
- **Brief forms** — High-frequency relationships get compact symbols
- **Positional grammar** — Location within a line implies role
- **Affix modification** — Small markers inflect meaning

### Belief Representation
WSL draws on concepts from epistemology and knowledge representation:
- Beliefs as claims with conditions (contextualism)
- Sources as grounding for confidence (evidentialism)
- Tolerance of contradiction (paraconsistent approaches)

### Configuration Languages
The hierarchical structure echoes YAML and similar formats, using indentation for nesting while avoiding syntactic overhead like quotes and brackets.

---

## Structure

A WSL document is a hierarchical collection of beliefs organized as:

```
Document
  └── Concept (one or more)
        └── Facet (one or more per concept)
              └── Claim (one or more per facet)
                    ├── Condition (zero or more)
                    └── Source (zero or more)
```

### Definitions

| Element | Description |
|---------|-------------|
| **Concept** | A subject of belief—a noun in the worldview (e.g., Power, Trust, Human nature) |
| **Facet** | An aspect or dimension of a concept (e.g., formation, erosion, institutional) |
| **Claim** | An assertion about a facet—what is believed to be true |
| **Condition** | Circumstances under which the claim applies |
| **Source** | Basis for the belief (observation, experience, citation, intuition) |

### Constraints

- Every concept must have at least one facet
- Every facet must have at least one claim
- Conditions and sources are optional per claim
- Facet names are freeform (no controlled vocabulary)
- Concepts may reference other concepts, creating a web of related beliefs

---

## Notation

### Hierarchy

| Element | Notation | Indentation |
|---------|----------|-------------|
| Concept | Bare text | None (column 0) |
| Facet | `.` prefix | 2 spaces |
| Claim | `-` prefix | 4 spaces |

### Inline Elements

| Element | Symbol | Position |
|---------|--------|----------|
| Condition | `|` | After claim |
| Source | `@` | After claim/conditions |
| Reference | `&` | After claim, links to other concept.facet |

### Positional Grammar

Claims follow a consistent order:

```
- [claim] | [condition] | [condition] @[source] @[source] &[reference]
```

Position implies role—no labels needed:
1. Claim text (required)
2. Conditions (zero or more, `|` prefixed)
3. Sources (zero or more, `@` prefixed)
4. References (zero or more, `&` prefixed)

---

## Brief Forms

Common relationships use compact symbols:

| Symbol | Meaning |
|--------|---------|
| `=>` | causes, leads to |
| `<=` | caused by, results from |
| `<>` | mutual, bidirectional |
| `><` | tension, conflicts with |
| `~` | similar to, resembles |
| `=` | equivalent to, means |
| `vs` | in contrast to |
| `//` | regardless of |

### Examples

```
- power => corruption | unchecked
- trust <= consistency | over time
- efficiency >< thoroughness
- formal-authority ~ informal-influence
```

---

## Modifiers

Suffix markers inflect claim meaning:

| Modifier | Meaning |
|----------|---------|
| `^` | increasing, trending up |
| `v` | decreasing, trending down |
| `!` | strong, emphatic, high confidence |
| `?` | uncertain, contested, tentative |
| `*` | notable, important, flagged |

### Examples

```
- institutional-trust v | recent decades
- free-will? @philosophy
- single violation => collapse !
- paradigm-shift* | in progress
```

---

## Evolution

Beliefs change. WSL represents evolution through:

### Supersession Markers

Prior beliefs can be noted inline with `[<= prior belief]`:

```
- adaptive, context-dependent [<= inherently good]
```

This reads: "Currently believed to be adaptive and context-dependent; this supersedes a prior belief that it was inherently good."

### Implicit Evolution

When claims in the same facet track change over time, newer claims are listed first. The array order itself implies evolution without explicit markers.

---

## References

Claims can reference other concepts using `&Concept.facet`:

```
Trust
  .erosion
    - asymmetric to formation &Trust.formation
    - single violation => collapse &Human-nature.memory

Human-nature
  .memory
    - negative events more salient
    - loss-averse @behavioral-economics
```

References create a graph of related beliefs, enabling the LLM to traverse connections without duplicating content.

---

## Non-Goals

WSL explicitly does not attempt to:

- **Prove logical consistency** — Contradictions are permitted
- **Enforce ontology** — No required categories or hierarchies beyond structure
- **Replace natural language** — WSL is for belief state, not communication
- **Assert objective truth** — Claims represent understanding, not facts
- **Store predictions, evaluations, or identity** — These are derived from beliefs, not stored directly

---

## Summary

WSL is a notation for meaning, not conversation. It exists to preserve how concepts are understood—compactly enough to remain always in context, structured enough to reason about reliably, and flexible enough to evolve as understanding changes.

The format encodes:
- **What** is believed (claims)
- **When** it applies (conditions)
- **Why** it's believed (sources)
- **How** beliefs connect (references)
- **That** beliefs change (evolution markers)

It deliberately omits:
- Prose and filler
- Explicit confidence scores (derived from conditions and sources)
- Detailed history (supersession markers suffice)
- Evaluative or predictive statements (derived at runtime)

---

# Your Task

When given a plain-text fact or statement:
1. First, read the current WSL file to understand its structure and existing concepts
2. Determine if this fact belongs to an existing concept/facet or requires a new one
3. Format the fact as proper WSL notation following the specification above
4. Use the edit_wsl tool to add or modify the appropriate line(s)
5. After editing, briefly confirm what you added

Remember the design principles: state over narrative, predictability allows omission, conflict tolerance, freeform vocabulary, and LLM-native density.
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
