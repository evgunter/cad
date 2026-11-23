---
id: main-red-predicate-name-coverage-reads-three-topo-test-literals-no-crate-decides
kind: issue
title: "main red (slow set): predicate_name_coverage reads three topo test literals no crate decides"
status: closed
opened: 2026-11-16
closed: 2026-11-17
priority: P0
cost: E
---


`test-utils::all predicate_name_coverage::every_deciding_crate_runs_a_roster_and_every_referenced_name_is_decided`
(`crates/test-utils/tests/predicate_name_coverage.rs:266`) is red on main
at `25469f806a`. It was measured on a checkout of main, 2026-11-16, after
ONEHOME's workspace suite on PR 7037 found it red at the merge base
`253d3bbd58` too. This row records the red; it does not fix it.

    cargo nextest run -p test-utils predicate_name_coverage

The test names three sites, each a predicate literal no crate decides:

- `crates/topo/src/boolean/solid_contain/curved_edges.rs:491`:
  `curve_carrier_straddle`, a test assertion matching `diag.predicate`.
  `geom`'s `on_carrier` passes this name to its `straddle` helper
  (`crates/geom/src/curves/on_carrier.rs:194`). I have not checked why
  the roster does not count it there.
- `crates/topo/src/boolean/solid_contain/curved_edges.rs:661`:
  `ray_order`, in a hand-built `MarginDiag`'s `predicate`.
- `crates/topo/src/side_reader.rs:1331`: `ray_order`, the same shape.

The test's message names the remedies: spell the name as the door does,
route it through the door's const, or, in test code, mark a deliberate
negative `// UNDECIDED-NAME: <why>` on the line above.

**Owner.** `work.py territory` puts `side_reader.rs` under BOXFINISH and
SIDETIES. `curved_edges.rs` is under many programs, and BOXFINISH wrote
its curved arm (`39196518c6`, `d4a20c1326`). The roster check reads
`tests/` references since `3bdf8fadab` (the 6715 review).

## Closed (2026-11-17, GREEN)

Green on main `8466ca3d` plus PR 7060. The three sites above were resolved on main before the 11-17 nightly (`curved_edges.rs` is gone; `side_reader.rs`'s `ray_order` carries an `// UNDECIDED-NAME:` mark). The nightly of 11-17 (main `cdee7461`) named one more, `crates/topo/src/chart_region.rs:4042`'s `"pcurve_incidence"` in `a_trim_off_its_face_stands_by_its_read` (ONECOPY's section-error-standing work); PR 7060 spells it as the plane chart's door, `pcurve_plane_chart_incident`. `predicate_name_coverage` passes locally on that tree. (GREEN)
