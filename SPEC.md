# Worldview Format

**Specification v0.1 — Draft**

---

## Abstract

The Worldview format is a compact, declarative notation for encoding and maintaining conceptual worldviews over time. It provides a structured format for storing beliefs, stances, and understanding about concepts—designed to be included in full within every interaction context rather than retrieved selectively.

The Worldview format is not a general-purpose communication language. It is a specialized format for preserving *how concepts are understood*, optimized for semantic density without sacrificing clarity. The notation is intended to be intuitive for large language models to parse, reason about, and autonomously maintain, while remaining human-inspectable. Beliefs are stored as structured claims with conditions and sources, enabling an LLM to hold persistent context about a user, domain, or system across extended interactions.

---

## Motivation

Large language models operate within fixed context windows. Existing approaches to persistent memory—such as retrieval-augmented generation (RAG)—selectively include information based on relevance to the current query. This works for factual lookup but fails for *worldview*: the foundational beliefs and stances that should inform all reasoning, not just topically-matched queries.

The Worldview format solves this by defining a notation dense enough that an entire belief system (potentially tens of thousands of tokens) can remain in context permanently. Rather than retrieving relevant memories, the LLM carries its complete understanding forward into every interaction.

The format prioritizes:
- **Semantic density** — Strip prose, keep meaning
- **Structural consistency** — Predictable hierarchy for reliable parsing
- **Evolutionary tracking** — Beliefs change; the notation accommodates revision
- **Autonomous maintenance** — The LLM updates the document without user intervention

---

## Design Principles

**State over narrative**
The Worldview format captures what is believed, not the story of how it came to be believed. History is preserved compactly when relevant, but the primary representation is current state.

**Predictability allows omission**
Borrowed from stenographic shorthand: if structure or context makes something inferable, don't write it. No articles, no copulas, no filler.

**Conflict tolerance**
Real worldviews contain tensions and contradictions. The Worldview format holds conflicting claims without forcing resolution.

**Freeform vocabulary**
No predefined concept names, facet labels, or claim terms. The notation defines structure and relationships; content remains unconstrained.

**LLM-native, human-inspectable**
Optimized for machine parsing and reasoning. Human readability is a secondary benefit, not a design constraint.

---

## Inspirations

### Stenographic Shorthand
Systems like Gregg, Pitman, and Teeline informed the Worldview format's approach to density:
- **Omission of predictable elements** — Common words and inferable structure are dropped
- **Brief forms** — High-frequency relationships get compact symbols
- **Positional grammar** — Location within a line implies role
- **Affix modification** — Small markers inflect meaning

### Belief Representation
The Worldview format draws on concepts from epistemology and knowledge representation:
- Beliefs as claims with conditions (contextualism)
- Sources as grounding for confidence (evidentialism)
- Tolerance of contradiction (paraconsistent approaches)

### Configuration Languages
The hierarchical structure echoes YAML and similar formats, using indentation for nesting while avoiding syntactic overhead like quotes and brackets.

---

## Structure

A Worldview document is a hierarchical collection of beliefs organized as:

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
| Condition | `\|` | After claim |
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

Beliefs change. The Worldview format represents evolution through:

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

## Examples

### Minimal Document

```
Power
  .core
    - corrupts | unchecked
    - reveals character
```

### Expanded Document

```
Power
  .nature
    - corrupts | unchecked !
    - reveals character => self-knowledge
    - concentration^ => abuse^ @historical-pattern
  .institutional
    - self-preserving
    - accountability <> trust &Trust.institutional
    - diffusion => dilution-of-responsibility

Trust
  .formation
    - slow
    - requires consistency | over time
    - contextual @personal-experience
  .erosion
    - fast !
    - single violation => collapse?
    - asymmetric vs formation &Trust.formation
  .institutional
    - possible | high transparency
    - unlikely | low transparency
    - rational to withhold | unverifiable @game-theory

Human-nature
  .social
    - conformist | formal groups
    - authentic | solitary
    - status-aware @evolutionary-psychology
    - coalition-forming
  .cognition
    - pattern-seeking
    - confirmation-biased @cognitive-science
    - narrative-constructing
    - rationalizes post-hoc [<= rational actor]
  .self-perception
    - overconfident | familiar domains
    - miscalibrated @Dunning-Kruger
    - self-deception => comfort &Human-nature.cognition

Institutions
  .function
    - stabilize !
    - preserve knowledge
    - coordinate action @game-theory
  .dysfunction
    - ossify | over time
    - self-perpetuate // original purpose
    - capture-by-interests^ @public-choice-theory
```

---

## Non-Goals

The Worldview format explicitly does not attempt to:

- **Prove logical consistency** — Contradictions are permitted
- **Enforce ontology** — No required categories or hierarchies beyond structure
- **Replace natural language** — The Worldview format is for belief state, not communication
- **Assert objective truth** — Claims represent understanding, not facts
- **Store predictions, evaluations, or identity** — These are derived from beliefs, not stored directly

---

## Intended Use Cases

- **Long-term LLM context anchoring** — Persistent worldview across sessions
- **Belief drift analysis** — Track how understanding evolves
- **Conceptual memory compression** — Dense storage of learned stances
- **Domain modeling** — Capture expert understanding of a field
- **Value alignment documentation** — Record interpretive frameworks

---

## Summary

The Worldview format is a notation for meaning, not conversation. It exists to preserve how concepts are understood—compactly enough to remain always in context, structured enough to reason about reliably, and flexible enough to evolve as understanding changes.

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

The Worldview format is designed to be carried forward—a persistent lens through which all subsequent reasoning is filtered.
