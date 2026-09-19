---
id: n3r1-prune-and-s16-box-soundness-hold-one-corpus-twice
kind: issue
title: n3r1_prune and s16_box_soundness carry the same seven-fixture corpus, byte for byte
status: open
opened: 2026-09-19
---

## Finding

- **Where**: `crates/sweep/tests/n3r1_prune.rs` and
  `crates/sweep/tests/s16_box_soundness.rs`.
- **Importance**: medium — two whole fixture sets, not one helper
- **Confidence**: sure; read side by side at the merge base
- **Raised by**: the `dup/private-box-builders` lane, 2026-09-19

Seven fixtures are byte-identical across the two files:
`cylinder_at`, `cylinder`, `small_box`, `nested_box`, `plate`,
`rim_plate` and `top_rim_plate`. `n3r1_prune.rs`'s own header says why
— it was *"adopted from a reviewer probe (the CERT-N3 dual review)"* —
so the copy is the adoption, recorded and then left.

The box builders of that set (`small_box`, `plate`) are now one line
of `sweep::test_support::brick` in both files (PR for
`private-extruded-box-builders-outside-the-brick-door`), which closes
the *box* duplication and leaves this one: the three-arc cylinder and
the four plate placements are still written out twice.

The homes exist. `sweep::test_support` already holds the extrusion
family, and the cylinder here is one loop of three bulged vertices
through `prism`. What has to be decided first is whether the two
suites want one corpus — `s16_box_soundness` measures the census's
containment arm, `n3r1_prune` the pruning delta, and they share the
corpus because one was adopted from the other's review.

**Not folded by the lane that found it**: the box half was its unit,
and this half is a different question (does one corpus serve both
subjects?) that wants its own measurement.
