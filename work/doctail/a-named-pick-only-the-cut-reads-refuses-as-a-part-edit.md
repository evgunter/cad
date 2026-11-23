---
id: a-named-pick-only-the-cut-reads-refuses-as-a-part-edit
kind: issue
title: split refuses a cut that takes a named pick with its only readers as PartEdit UnresolvedInput: the part's carry does not carry a pick
status: closed
opened: 2026-11-18
priority: P3
cost: M
closed: 2026-11-18
---


## What

Found in the second delta review of PR 7087 (its scene A2). Take `copy_of_the_union` in `crates/editor-core/tests/p2_split.rs`: a copy `placed` of `c ⊔ a ⊔ b`. Let `P` be a named pick of `a`'s piece of that copy, and `x` a placement of `P`. Cut `{c, a, b, the ⊔, placed, x}`.

Every reader of `P` is cut, so since PR 7087 split lets `P` go with the cut instead of refusing it as `NamedReadUncarried`. The part's carry, though, cannot carry a pick. `carry` → `remap_with_selects` reads a pick through `rd_output`, which holds only the carried operations' outputs, so the cut placement's read misses. Split refuses `SplitError::PartEdit { UnresolvedInput { input: the placement } }`. The refusal is typed and true about where the edit failed, but it names no pick, and the cut is one the variable rule admits.

A named source in the same position was carried only where it read no pick (`p2_split::a_named_source_only_the_cut_reads_goes_with_the_cut`). A source of a pick hit the same miss through `sourced`.

## Owed

The carry writes a pick its cut reader reads as a pick of the carried copy (`Operand::pick` over the carried placement's output, the path through `read_map`), as it writes a selection afresh. A row: the scene above splits, round-trips, and the part holds `P`.

## Closed

Closed in PR 7087's last fix pass, which made two changes.

- **The carry writes a pick afresh.** `remap_with_selects` reads a pick, and a source of one, down to the copy it reaches (`Doc::pick_path`). `resourced` then writes the pick again over the carried copy: a pick of a pick over the inner one written the same way, its path on the carried pieces. `name_selections` names a carried source's or pick's inner read as the source document named it.
- **"Every reader is cut" sees the anonymous definition between a pick and its reader.** `read_only_by_cut` follows anonymous definitions, such as the source a placement's door mints over `P`, to the seats that own them. The variable rule therefore lets `P` go where the review found it refused.

Row: `p2_split::a_named_pick_only_the_cut_reads_goes_with_it`. It covers a placement of `P`, a placement of `s = P`, and a placement of an anonymous pick of `P1`. Each splits; the part holds the names, the remainder holds none, and nothing dangles. Read by the product slot instead, `P1` refuses `NamedReadUncarried` (`Pick`).
