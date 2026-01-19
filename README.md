# Worldview

A compact notation format for encoding and maintaining conceptual worldviews over time, designed for LLM context persistence.

## Overview

The **Worldview format** (file extension: `.wvf`) is a declarative notation for storing beliefs, stances, and understanding about concepts. Unlike retrieval-augmented generation (RAG) which selectively includes information, Worldview documents are designed to remain *entirely in context* across all LLM interactions.

This solves a fundamental problem: LLMs have foundational beliefs and stances that should inform all reasoning, not just topically-matched queries. The Worldview format is dense enough that an entire belief system can remain in context permanently.

## Quick Start

### Example Document

```wvf
Trust
  .formation
    - slow
    - requires consistency | over time
    - contextual @personal-experience
  .erosion
    - fast !
    - single violation => collapse?
    - asymmetric vs formation &Trust.formation
```

### Key Concepts

- **Concepts**: Subjects of belief (e.g., Trust, Power, Human-nature)
- **Facets**: Aspects or dimensions (e.g., .formation, .erosion)
- **Claims**: Assertions about a facet
- **Conditions**: When/if the claim applies (prefix: `|`)
- **Sources**: Basis for belief (prefix: `@`)
- **References**: Links to other concepts (prefix: `&`)

### Brief Form Operators

| Symbol | Meaning |
|--------|---------|
| `=>` | causes, leads to |
| `<=` | caused by |
| `<>` | mutual, bidirectional |
| `><` | tension, conflicts with |
| `~` | similar to |
| `!` | emphatic, strong |
| `?` | uncertain |
| `^` | increasing |
| `v` | decreasing |

## Tools

This repository includes a complete toolchain for working with Worldview documents:

### Validator (`worldview-validate`)

Validates `.wvf` files for syntactic correctness.

```bash
# Validate a file
worldview-validate example.wvf

# Validate from stdin
cat example.wvf | worldview-validate --stdin
```

### Agent CLI (`worldview`)

An AI-powered tool that converts plain-text statements into proper Worldview notation.

```bash
# Add a fact to a Worldview file
worldview "Trust is built slowly through consistent actions" --file worldview.wvf

# Use a specific model
worldview "Power corrupts when unchecked" --model claude-opus-4-5-20251101
```

The agent understands the full Worldview specification and will:
1. Read existing content to understand structure
2. Determine appropriate concept/facet placement
3. Format statements using proper notation
4. Validate before writing

### Evaluation Framework

A Python framework for testing how well LLMs can leverage Worldview-encoded beliefs.

```bash
# Install dependencies
pip install -r evals/requirements.txt

# Run evaluations
python -m evals run --models claude-sonnet gpt-5.2

# Run specific difficulty
python -m evals run --difficulty extreme

# List available models and test cases
python -m evals list-models
python -m evals list-cases
```

## Project Structure

```
worldview/
├── agent/                 # Rust agent CLI
│   └── src/main.rs        # Agent implementation with embedded spec
├── validator/             # Rust validator
│   ├── src/lib.rs         # Validation library
│   └── src/main.rs        # CLI tool
├── evals/                 # Python evaluation framework
│   ├── cli.py             # Evaluation CLI
│   ├── runner.py          # Test orchestration
│   ├── evaluator.py       # Response scoring
│   ├── test_cases.py      # Test case definitions
│   ├── worldview_prompt.py # System prompts
│   └── llm_clients.py     # LLM provider clients
├── SPEC.md                # Full format specification
├── system.md              # Condensed system prompt
└── example.wvf            # Example document
```

## Building

### Prerequisites

- Rust 1.70+
- Python 3.11+

### Build Tools

```bash
# Build validator
cd validator && cargo build --release

# Build agent (requires setup for vendored dependencies)
cd agent && ./setup.sh && cargo build --release
```

## Design Principles

The Worldview format follows five core principles:

1. **State over narrative**: Capture what is believed, not how it came to be believed
2. **Predictability allows omission**: If structure makes something inferable, don't write it
3. **Conflict tolerance**: Real worldviews contain tensions—hold them without forcing resolution
4. **Freeform vocabulary**: Structure is defined; content remains unconstrained
5. **LLM-native, human-inspectable**: Optimized for machine parsing while remaining readable

## Why "Worldview"?

The name emphasizes the core use case: encoding *how concepts are understood* rather than just storing facts. This is different from:

- **Knowledge bases**: Store facts, not interpretations
- **RAG systems**: Retrieve relevant content per query
- **Memory systems**: Log events chronologically

Worldview documents capture the *lens* through which all subsequent reasoning should be filtered.

## File Extension

- **Extension**: `.wvf` (Worldview Format)
- **MIME type**: `text/x-worldview` (proposed)

## License

MIT

## Specification

See [SPEC.md](SPEC.md) for the complete format specification.
