---
id: a-split-name-picks-a-pattern-copy-by-its-evaluated-index
kind: issue
title: Split's name re-anchor matches a Part(Instance(k)) pick by evaluating k at the document's values, a positional pick that retires with Member keys
status: parked
priority: P2
opened: 2026-10-09
parent: patterns-are-index-variables
blocked_on: [patterns-are-index-variables]
---


Found by INTENT stage 2 C's reviews (PR #4359: first review MINOR-2,
second review style Q6).

`split`'s `in_world` (`crates/editor-core/src/refactor.rs`) re-anchors a
remainder name under the one cut placement whose copy carries it. For a
pattern copy it matches the name's outermost `Instance { i }` against a
`Part(Instance(k))` pick, with `k` evaluated at the document's values
(`eval_var_count`). That decides document structure from a value, a
positional pick that goes stale when a count changes, against Ev's
2026-10-03 principles. The member walk (`mate::member`) judges a pick
the same way.

It retires with `[ev]` #4341 (patterns become index variables, members
named by `Member` keys): the re-anchor then matches the pick's key, not
an evaluated index. Pinned today by
`p2_face::a_face_side_on_a_pattern_copy_crosses_split_and_inline_unmoved`
and `fix_pattern_mate_crossing::an_underqualified_pattern_head_…`.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to REFS's charter (names scoped by their read). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).
