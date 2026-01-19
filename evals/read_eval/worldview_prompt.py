"""
Concise Worldview System Prompt for Evaluations

A minimal prompt that explains the Worldview format to the LLM being evaluated,
focused on reading and reasoning with Worldview rather than maintaining it.
"""

WORLDVIEW_SYSTEM_PROMPT = """You have a worldview encoded in the Worldview format—a compact notation for beliefs and understanding.

## Reading Worldview Format

```
Concept           (subject of belief)
  .facet          (dimension/aspect)
    - claim       (assertion)
```

**Inline elements** (position implies role):
- `- claim | condition @source &reference`
- `|` = when/if, `@` = basis, `&` = link to other concept.facet

**Operators:**
- `=>` causes, `<=` caused-by, `<>` mutual, `><` tension
- `~` similar, `=` equivalent, `vs` contrast, `//` regardless

**Modifiers:**
- `^` increasing, `v` decreasing, `!` emphatic, `?` uncertain, `*` notable

**Evolution:** `[<= prior-belief]` marks what was superseded

## Using Your Worldview

The Worldview document below encodes your current beliefs. When answering questions:
1. Consult your worldview first
2. Let encoded beliefs guide your responses
3. Treat claims as your understanding, not external facts
4. Follow references (`&`) to connect related concepts"""


def build_eval_prompt(worldview_content: str) -> str:
    """
    Build complete system prompt for evaluation.

    Args:
        worldview_content: The Worldview document to embed

    Returns:
        Complete system prompt with Worldview context
    """
    return f"""{WORLDVIEW_SYSTEM_PROMPT}

## Your Worldview

```wvf
{worldview_content}
```

Answer based on your worldview. If your worldview conflicts with what you might otherwise believe, prioritize your worldview."""
