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
        "Read the current contents of the WSL file. Returns the file contents with line numbers prefixed (e.g., '   1│content'). Use read_wsl first before editing to see current state.",
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
        r#"Apply search/replace edits to the WSL file. Each edit specifies an old_string to find and a new_string to replace it with.

Rules:
- Each old_string must match exactly (including whitespace/indentation)
- Each old_string must appear exactly once in the file (include more context if ambiguous)
- To insert new content, use old_string to match an existing line and include it plus your new lines in new_string
- To delete content, use an empty new_string
- Multiple edits are applied sequentially

The tool validates the result against WSL syntax rules before writing."#,
        json!({
            "type": "object",
            "properties": {
                "edits": {
                    "type": "array",
                    "description": "List of search/replace operations to apply sequentially",
                    "items": {
                        "type": "object",
                        "properties": {
                            "old_string": {
                                "type": "string",
                                "description": "Exact string to find (must be unique in file). Include full lines with proper indentation."
                            },
                            "new_string": {
                                "type": "string",
                                "description": "String to replace it with. Use empty string to delete."
                            }
                        },
                        "required": ["old_string", "new_string"]
                    }
                }
            },
            "required": ["edits"]
        }),
    )
}

/// Handle the read_wsl tool call
fn handle_read_wsl(file_path: &PathBuf) -> String {
    if !file_path.exists() {
        return "File does not exist yet. Use edit_wsl with edits to create it.".to_string();
    }

    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            // Return with line numbers in codey format
            content
                .lines()
                .enumerate()
                .map(|(i, line)| format!("{:4}│{}", i + 1, line))
                .collect::<Vec<_>>()
                .join("\n")
        }
        Err(e) => format!("Error reading file: {}", e),
    }
}

/// Handle the edit_wsl tool call
fn handle_edit_wsl(file_path: &PathBuf, params: &serde_json::Value) -> String {
    // Parse edits array
    let edits = match params.get("edits").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return "Error: 'edits' array is required".to_string(),
    };

    if edits.is_empty() {
        return "Error: 'edits' array cannot be empty".to_string();
    }

    // Read current file content (or start empty for new files)
    let mut content = if file_path.exists() {
        match std::fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(e) => return format!("Error reading file: {}", e),
        }
    } else {
        String::new()
    };

    // Validate and apply each edit
    for (i, edit) in edits.iter().enumerate() {
        let old_string = match edit.get("old_string").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => return format!("Edit {}: missing 'old_string'", i + 1),
        };
        let new_string = match edit.get("new_string").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => return format!("Edit {}: missing 'new_string'", i + 1),
        };

        // For new files, old_string should be empty to append
        if content.is_empty() {
            if !old_string.is_empty() {
                return format!(
                    "Edit {}: file is empty, old_string must be empty to create new content",
                    i + 1
                );
            }
            content = new_string.to_string();
            continue;
        }

        // Check that old_string exists and is unique
        let count = content.matches(old_string).count();
        match count {
            0 => {
                return format!(
                    "Edit {}: old_string not found in file. \
                     Make sure the string matches exactly, including whitespace and indentation.",
                    i + 1
                );
            }
            1 => {} // good
            n => {
                return format!(
                    "Edit {}: old_string found {} times (must be unique). \
                     Include more surrounding context to make the match unique.",
                    i + 1,
                    n
                );
            }
        }

        // Apply the replacement
        content = content.replacen(old_string, new_string, 1);
    }

    // Ensure file ends with newline
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }

    // Validate the new content before writing
    let validation = wsl_validator::validate(&content);

    if !validation.is_valid() {
        let errors: Vec<String> = validation.errors.iter().map(|e| e.to_string()).collect();
        return format!(
            "Validation failed - file not modified:\n{}",
            errors.join("\n")
        );
    }

    // Write the file
    if let Err(e) = std::fs::write(file_path, &content) {
        return format!("Error writing file: {}", e);
    }

    // Return success with edit count and any warnings
    let edit_count = edits.len();
    let base_msg = format!(
        "Successfully applied {} edit{}.",
        edit_count,
        if edit_count == 1 { "" } else { "s" }
    );

    if validation.has_warnings() {
        let warnings: Vec<String> = validation.warnings.iter().map(|w| w.to_string()).collect();
        format!("{} Warnings:\n{}", base_msg, warnings.join("\n"))
    } else {
        format!("{} File validated.", base_msg)
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
