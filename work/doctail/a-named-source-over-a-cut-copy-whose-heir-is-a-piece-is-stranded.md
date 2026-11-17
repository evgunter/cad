---
id: a-named-source-over-a-cut-copy-whose-heir-is-a-piece-is-stranded
kind: issue
title: split refuses a named source or pick the remainder keeps over a cut copy that crosses to a piece of the instance's copy, where it could carry it
status: open
opened: 2026-11-17
priority: P2
cost: M
design: true
---


## What

Found in the review of PR 7087.

`split` (`crates/editor-core/src/refactor.rs`, `split`, `crossing_sources`) can carry a named source the remainder keeps over a cut copy only where that copy crosses whole: the part's product is the one copy, or the cut `⊔` that is it. In that case the source is defined afresh over the instance's copy.

Where the copy crosses to a piece (the part has several world copies, so the heir is a pick of the instance's copy), carrying the source means defining it as a read of a pick written in place. `DocEdit::DefineVar`'s source lowering refuses that (`lower_source_decl`, `SourceFault::WrittenInPlace`: "only a seat writes" a pick). A named pick has no definition door at all.

The first fix pass of PR 7087 left such a variable reading a copy the remainder deletes. Its readers were re-pointed past it to a pick of the instance's copy, and the delta review showed even that was not always true: through a named source of a named pick, the reader was widened to the copy whole. Split now refuses typed instead, with `SplitError::NamedReadUncarried`, naming the first such variable in id order. The rows:

- `p2_split::a_named_source_of_a_named_pick_over_the_cut_refuses_typed`: `P` is a named pick of `a`'s piece of a cut copy of `c ⊔ a ⊔ b`, and `s` is a named source of `P`, read by the product slot or by a kept `⊔`.
- The reviewer's first scene: `s = a` declared over three pinned copies `c, a, b`, read by the product or by nothing, with the cut `{a, b}`.

With one cut world copy (`s = b`, the cut `{b}`) the source is carried: `p2_split::a_named_source_over_a_cut_copy_reads_the_instances_copy`.

## Owed

A choice, since it touches VR4's "a definition reads a variable by id or name, or a node or its port":

- **Carry the variable.** Let a source's definition, and a pick's, read a pick of a copy. Inline's and flatten's source re-definitions need the same thing, in `work/doctail/inline-and-flatten-redefine-a-placements-source-where-split-rewrites-it-with-the-seat.md`.
- **Keep the refusal** as the rule, and say so in A4.

Either way, a row: the scenes above, round-tripped or refused as ruled.
