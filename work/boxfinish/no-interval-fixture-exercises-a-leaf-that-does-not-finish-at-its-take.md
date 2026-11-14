---
id: no-interval-fixture-exercises-a-leaf-that-does-not-finish-at-its-take
kind: issue
title: No executed row exercises an Interval sweep leaf that does not finish at its taker
status: open
opened: 2026-11-14
---


Found by `boxfinish/chart-ends`. Two rows in
`crates/editor-core/tests/onehome_switch_parity_interval.rs` were the
only executed rows of two refusal paths at a box scalar, and both used
the `Interval` square loft of `eval10_section_reads_the_nominal::square_loft_doc`
because check 7 refused its walls on inexact rim ends. Once the rims
store their chart ends (`geom_brep::Pcurve::iso_row`, `span_ends`) the
loft finishes, so both rows now pin that it finishes
(`a_boxed_lofts_placements_finish_at_the_walks_take`,
`a_union_of_a_block_and_a_boxed_loft_evaluates`), and nothing executes:

- the connectedness check's `CheckEvidence::NotAtRest` finding for a
  placement whose body does not finish at the take, and the walk going
  on past it (`editor_core::checks`);
- a union's `NodeErrorKind::UnfinishedLeaf { input, findings }` naming
  the member that does not finish (`editor_core::eval`).

Both are still rendered from hand-built values
(`refusal_concision_at_rest.rs`, `refusal_concision_chains.rs`), which
cannot see whether the walk reaches them.

**What a fixture needs.** An `Interval` sweep leaf, built from a
document, that is tier-2 valid and refused by tier 3. Measured on the
M1 probe (`boxfinish/probe-switch-off`, `probe/M1-REPORT.md` §1): the
tour's hole extrudes refuse check 1 (`SurfaceDatumUndecided` on a
cylinder's `u_ref`) over a macroscopic box, but
`editor_core::LoopProgram::circle` takes literal values only, so a
document cannot box the circle itself. Not measured: which document
parameter boxes that `u_ref`, and whether the slab's `OrientationUndecided`
(§1) reaches a taker with the switch on.
