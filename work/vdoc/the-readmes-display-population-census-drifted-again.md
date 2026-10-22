---
id: the-readmes-display-population-census-drifted-again
kind: issue
title: The viewer README's Display census says 41 impls with six over structs; the population is 46 with eight
status: open
opened: 2026-10-22
priority: P4
cost: E
---


Found while updating the `..` census in `crates/viewer/README.md`
(Module boundaries, *"A `match` is exhaustive over VARIANTS, not over a
variant's FIELDS"*) for `zoom-to-fit-frames-no-committed-profile`.

The paragraph says *"The six `Display`s above are the struct half of a
population of 41 `Display` impls under `src/` — `rg 'impl.*fmt::Display
for ' crates/viewer/src | wc -l` prints that 41 … the other 35 are over
enums"*. At this row's merge base the command prints **46**, and the
subjects split **38 enums and 8 structs**: `BlendTarget`,
`Disagreement`, `EdgeNamesRefused`, `Message`, `PanelRefusal`,
`StoreError`, `Unusable`, `Withdrawal` — so the struct list above the
paragraph is short by two as well.

The same paragraph's *"one more site that is not an instance"*,
`frame.rs`'s `matches!(w.cause, AdmissionFault::FusedGeometry { .. })`,
is now an exhaustive `match` over `AdmissionFault` inside
`Display for Withdrawal` (seven `{ .. }` arms, each a variant test on
another type rather than a pattern over the subject), so the sentence
quotes code that is no longer there.

The `..` sweep over all 46 itself was re-run and is current
(three hits, listed in the README after that PR).

This is the second drift of the same count
(`census-table-in-the-viewer-readme-is-not-its-own-population`, closed):
a fix might state the command and not its output.
