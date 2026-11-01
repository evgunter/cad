---
id: python-slot-words-stop-short-of-a-step-index
kind: issue
title: pncad-py: the slot words placement_step and profile carry no integer, so Python cannot write at a later placement step's slot or a profile program slot
status: open
opened: 2026-09-30
priority: P3
cost: M
---


Found by the EDIT placement unit's fix pass (PR 3497).

## What

A slot crosses the Python boundary as a word (`crates/pncad-py/src/tags.rs`,
`slot_id_tag`; read back by `crates/pncad-py/src/slot_word.rs`,
`slot_from_word`). The tag is a `&'static str`, so two slot families stop
one level short of their address:

- `placement_step` — `SlotId::PlacementStep { step, arg }`, a component of
  a transform's placement past its first step. The step index is a
  `usize`.
- `profile` — `SlotId::Profile { loop_, step, arg }`, one expression of a
  profile program.

`slot_from_text` (`crates/pncad-py/src/py/doc.rs`) refuses both words in
their own sentence. The kernel edits both at their slots
(`DocEdit::SetParam`, through the edit log too), so Python can read such a
refusal's `slot` word and cannot write back at the address it names. A
parameter-driven later step is authorable (an `Expr.param` in the step at
`Node.transform_by`), but its literal cannot be retyped at the slot the way
step 0's can (`translation_x`, `rotation_angle`).

## Repair shape

Carry the integers beside the word rather than inside it: a door that takes
`(word, step)` for `placement_step` and `(word, loop, step, role)` for
`profile`, with the read-back rows and the tag inventory moving with it.
One design for both words.
