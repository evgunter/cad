---
id: python-operand-keywords-and-slot-words-name-one-slot-twice
kind: issue
title: Python's subtract tool=, place target= and assertion value= set_param under another word (cut, target_space, input), and two slot labels disagree with their words
status: open
opened: 2026-11-02
priority: P3
cost: M
---

Found by the sweep that closed `python-profile-keyword-says-plane-for-a-frame`:
every Python operand keyword checked against the word `DocEdit.set_param`
takes for the slot it fills (`crates/pncad-py/src/slot_word.rs`,
`slot_from_word`; forward, `crates/pncad-py/src/tags.rs`,
`operand_slot_tag`), the slot's reader label
(`crates/editor-core/src/operand.rs`, `OperandSlot::label`) and the Rust
field (`crates/editor-core/src/refactor.rs`, the node re-pointing match,
which pairs each field with its `OperandSlot`).

## What

One operand, two names:

| Python door | keyword / Rust field | slot | `set_param` word | label |
|---|---|---|---|---|
| `Node.subtract(from_, tool)` | `tool` | `OperandSlot::Cut` | `cut` | `tool` |
| `Node.place(source, target, constraints)`, `Node.instantiate_part(reference, target, …)` | `target` | `OperandSlot::Space` | `target_space` | `target space` |
| `Node.assertion(value, relation, bound)` | `value` | `OperandSlot::Input` | `input` | `input` |
| (a part projection's copies) | — | `OperandSlot::Of` | `of` | `source` |

The second row is the sharpest: a caller who writes `target=` at
`Node.place` and then `set_param(node, "target", …)` addresses
`OperandSlot::Target`, a split's body, not the placement's space.

The other operand keywords match their words (`profile`, `path`,
`target`/`tool` at `split`, `body`/`plane` at `mirror`, `members`,
`source` at `place`).

## Fix shape

One name per slot, as INTENT stage 2 unit B did for `frame`: pick the
name each slot is spelled by and move the others to it — the keyword,
the `slot_from_word` arm and `operand_slot_tag`, the label, the stub.
Nothing is released, so no alias.
