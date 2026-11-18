---
id: a-named-pick-only-the-cut-reads-refuses-as-a-part-edit
kind: issue
title: split refuses a cut that takes a named pick with its only readers as PartEdit UnresolvedInput: the part's carry does not carry a pick
status: open
opened: 2026-11-18
priority: P3
cost: M
---


## What

Found in the second delta review of PR 7087 (its scene A2). Take `copy_of_the_union` in `crates/editor-core/tests/p2_split.rs`: a copy `placed` of `c ⊔ a ⊔ b`. Let `P` be a named pick of `a`'s piece of that copy, and `x` a placement of `P`. Cut `{c, a, b, the ⊔, placed, x}`.

Every reader of `P` is cut, so since PR 7087 split lets `P` go with the cut instead of refusing it as `NamedReadUncarried`. The part's carry, though, cannot carry a pick. `carry` → `remap_with_selects` reads a pick through `rd_output`, which holds only the carried operations' outputs, so the cut placement's read misses. Split refuses `SplitError::PartEdit { UnresolvedInput { input: the placement } }`. The refusal is typed and true about where the edit failed, but it names no pick, and the cut is one the variable rule admits.

A named source in the same position is carried: `p2_split::a_named_source_only_the_cut_reads_goes_with_the_cut`.

## Owed

The carry writes a pick its cut reader reads as a pick of the carried copy (`Operand::pick` over the carried placement's output, the path through `read_map`), as it writes a selection afresh. A row: the scene above splits, round-trips, and the part holds `P`.
