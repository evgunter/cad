---
id: a-declared-union-refuses-where-its-undeclared-twin-glues
kind: issue
title: A union whose rest is declared refuses DeclareResolve where its undeclared twin glues
status: open
opened: 2026-10-10
priority: P2
cost: M
---


INTENT stage 4 PR E glues an undeclared Zero rest. The declared twin
of such a rest is still fed to the fold step as a declaration, and
that step can refuse where the undeclared one builds:

- `crates/editor-core/tests/docm8_flat_merged.rs`
  `a_contact_against_a_fold_minted_fragment_glues_undeclared`. `d`
  rests on `a`'s end cap after `s` has fragmented that cap. Undeclared,
  the union fuses. With the rest declared, it refuses
  `Consumed(a.Cap(End), Split)`, because the declaration names a face
  that only survives in pieces.
- `crates/editor-core/tests/emit_union_rim_piece_ranks.rs` and
  `emit_union_flush_names.rs` (their `KNOWN_MIXED` / `KNOWN_REFUSING`
  rows for `cross`, `row` and `rowids`). The declared orders refuse
  `DeclareResolve` or `Naming` where the undeclared ones fuse.

D10 makes a declaration a statement the value then decides. So a
declared pair that the value glues should not refuse at the name
layer. One fix is to resolve a declaration naming a fragmented face
to the fragments the value decides carry the contact, rather than
refusing. Another is to drop such a declaration in favour of the
glue. Either is a design call for the stage that retires the
declared seat (`declared-pairs-retire`).
