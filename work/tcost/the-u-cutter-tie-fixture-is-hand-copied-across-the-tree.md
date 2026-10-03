---
id: the-u-cutter-tie-fixture-is-hand-copied-across-the-tree
kind: issue
title: the symmetric U-cutter tie document is hand-built in thirteen test files with no shared builder
status: open
opened: 2026-09-15
priority: P4
cost: E
---


## Finding

Found by the style review of PR 2681 (WIRE, `wire/tie-before-kind`),
which was about to add the fifteenth copy.

The **symmetric U cutter** — a 4x4x4 block with a U-shaped prism
subtracted through it, whose two congruent arms nothing covariant
discriminates, so the subtract's table records an `Entry::Tied` row —
is the tree's one document that mints a REAL N2 tie. Every row that
needs one hand-builds the same four nodes. `rg -c '\(5\.0, 1\.5\)'
crates/` (the U's inner corner, which only this profile has) names the
files:

```
crates/editor-core/tests/docm2_part.rs                 crates/editor-core/tests/m4_pr4_resolve.rs
crates/editor-core/tests/fixture/pr4.rs                crates/editor-core/tests/m4_pr5_declare.rs
crates/editor-core/tests/lib_g14_split_walls.rs        crates/editor-core/tests/m4_pr7_appearance.rs
crates/editor-core/tests/m4_pr3_names_bool.rs          crates/editor-core/tests/m6_5_selection_refusals.rs
crates/editor-core/tests/m4_pr4_appearance_hook.rs     crates/editor-core/tests/msolve5_read_below_a_root.rs
crates/editor-core/tests/wire_product_gather_tie.rs    crates/sweep/tests/bool1_r1_probes.rs
```

`crates/editor-core/tests/fixture/pr4.rs` already HOLDS a copy and
exposes no builder for it, which is why each new row copies the
coordinates instead of calling something.

This is S-TCOST's lever "merging tests that share initialization",
stated on one fixture: thirteen copies of four nodes, each of which
must be re-read to know whether it is the same document, and each of
which drifts on its own.

## What PR 2681 already did

It added **`fixture::u_cutter_tie(doc) -> (ProfileDoc, RecipeNodeId)`**
to `crates/editor-core/tests/fixture/mod.rs` — the builder that was
missing — and used it for the two rows it added, so it added zero new
copies. The work left is re-pointing the existing copies at it (and
deciding whether `fixture/pr4.rs`'s copy becomes a caller or stays,
since that tree has its own reasons for standing alone).

A copy that is NOT this document must stay a copy: the check a taker
owes each site is that the four nodes really are these four, not that
the corner coordinate matched.
