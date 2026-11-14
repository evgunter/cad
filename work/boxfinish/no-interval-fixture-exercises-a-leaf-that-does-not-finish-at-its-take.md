---
id: no-interval-fixture-exercises-a-leaf-that-does-not-finish-at-its-take
kind: issue
title: No executed row exercises an Interval sweep leaf that does not finish at its taker
status: open
opened: 2026-11-14
---

Found by `boxfinish/chart-ends`. Corrected in that PR's fix pass, after
the review restored half the coverage.

**Restored.** A union's `NodeErrorKind::UnfinishedLeaf { input, findings }`
naming its unfinished member at a box scalar is executed again. The row is
`crates/editor-core/tests/a_turned_discs_union_names_its_unfinished_member.rs`,
the review's port of the M3 probe's turned disc: 30° ± 1° about `y`,
united with a clear copy. It refuses with
`SurfaceDatumUndecided { Cylinder, … }` at ε default, 1e-6 and 1e-12.
Which member it names is
`work/onehome/an-unfinished-leaf-names-the-copy-that-was-not-turned.md`.

**Still missing: no executed row reaches the connectedness check's
`CheckEvidence::NotAtRest` finding** (`editor_core::checks`), nor the walk
going on past it.

- The only two rows that reached it used the `Interval` square loft
  (`eval10_section_reads_the_nominal::square_loft_doc`). Once that loft's
  rims stored their chart ends, it finishes, and
  `onehome_switch_parity_interval::a_boxed_lofts_placements_finish_at_the_walks_take`
  now pins that.
- The review ran `run_checks` on the turned-disc document, whose union
  does refuse at its take. The walk reported no `NotAtRest` finding.
  Cause not found: not measured whether the walk takes each placement's
  body the way the union does, or whether the turned copy's own leaf
  finishes alone.
- The value is still rendered from hand-built values
  (`refusal_concision_chains.rs`), which cannot see whether the walk
  reaches it.

**What a fixture needs.** A document whose placement's body is
tier-2 valid and refused by tier 3 when the connectedness walk takes it.
