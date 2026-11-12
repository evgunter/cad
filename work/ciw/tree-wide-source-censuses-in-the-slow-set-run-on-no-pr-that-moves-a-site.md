---
id: tree-wide-source-censuses-in-the-slow-set-run-on-no-pr-that-moves-a-site
kind: issue
title: A tree-wide source census in geom-core's slow set runs on no PR that moves a site, so three of them stood red on main for up to nine days
status: open
opened: 2026-11-09
priority: P2
cost: M
---



A source census reads the whole tree's text: `crates/geom-core/tests/`'s
`bounds_census` (`every_sole_bracket_bound_door_is_in_the_roster`),
`sym_rule_f_rows` (`the_copysign_mint_sites_the_tree_holds_are_these`),
`certified_endpoint_census` (`every_production_endpoint_read_is_counted_and_dispositioned`)
and `flagged_census` each walk every `crates/*/src`. Each costs 2–3 s, so
each sits in the `ci` profile's slow set (`.config/nextest.toml`), and a
slow test runs on a PR only when the diff's added lines spell its name
(`scripts/slow_set.py`'s `is_new`). The PR that moves a site is almost
never in geom-core and never spells the test's name, so it never runs the
census; the nightly does, and it is red elsewhere. A `gated_to!` list
naming every crate (`certified_endpoint_census`'s, held to the walk by
`the_suite_is_gated_to_every_crate_it_walks`) does nothing for a
slow-set row.

This is not `dependents-slow-sets-skip-a-kernel-pr`'s shape: geom-core is
not a cargo dependent of the crates a census reads, so no closure rule
reaches it. A census depends on them by text.

## Measured (bisected first-parent over main, 2026-11-09, `onecopy/main-reds-censuses`)

| census | moved by | what moved |
|---|---|---|
| copysign | PR 5956 (`2660be102b`, 2026-10-28) | 4 sites in `topo/src/sector_face.rs`, hidden by the census's own cut (`work/tcost/census-files-each-cut-a-files-shipped-part-by-hand.md`) |
| copysign | PR 6139 (`f2f34e16b0`, 2026-10-31) | `geom-brep/src/pair_section/cylinders.rs` +1 |
| copysign | PR 6345 (`7d6855d09c`, 2026-11-02) | `topo/src/boolean/fan.rs` +1, `join.rs` +1 |
| copysign | PR 6486 (`5a5ce31ef7`, 2026-11-03) | `geom/src/surfaces.rs` +1 |
| copysign | PR 6510 (`ce806b09fd`, 2026-11-04) | `sweep/src/blend/arms.rs` +2 |
| endpoint | PR 6486 (`5a5ce31ef7`) | `ssi/residual.rs` 6 → 7 |
| endpoint | PR 6478 (`898df16c9c`, 2026-11-03) | six `geom-brep/src/props/` files; the PR edited the roster (one row renamed) and still landed it red |
| endpoint | PR 6479 (`8383beaba2`, 2026-11-04) | `critical.rs` 13 → 12 |
| bounds | PR 6579 (`3b7794bb5d`, 2026-11-08) | `topo/src/boolean/boxes.rs`'s `face_box_of`, no roster line |

All four (with `flagged_census`, PR 6603) were red together on
`1b31bac52e`.

## Options

- **A. A slow test whose `gated_to!` names a path the diff touches runs on that PR.** Where a marker exists it already says which paths move the test; the slow set ignores it. But only `certified_endpoint_census.rs` carries a `gated_to!` today. `bounds_census.rs`, `sym_rule_f_rows.rs` and `flagged_census.rs` carry none, so as the tree stands A catches one census of the four. The other three each need a marker added: a hand-kept path list, which a check like `the_suite_is_gated_to_every_crate_it_walks` can hold to the walk. With all four marked, about 10 s on any kernel PR.
- **B. Make the walks cheap enough for the fast set**: one parsed, code-only view of the tree shared by every census in the `geom-core::all` binary (nextest is process-per-test, so a shared view needs the four in one test, or a cached view on disk).
- **C. Status quo**, with a lane re-pinning after each nightly.

A is the smallest change to the gate, and with the three missing markers added it catches each move at its PR.

## A fifth instance: pncad-py's prose census (2026-11-09, `onecopy/prose-census-red`)

`crates/pncad-py/src/prose_census.rs`'s rows read every `crates/*/src` and take about 4–5 s each, so they're in the slow set. They stood red on main from 2026-11-03 to 2026-11-09:
- the vacuity floor, 138 sites under 200, from PR 6430;
- the undecided roster, from PRs 6448, 6549 and 6579.

None of those PRs runs a pncad-py slow row (`work/onecopy/main-red-prose-census-reads-no-write-renderer.md`). Under option A, `prose_census.rs` needs a `gated_to!` naming every crate its walk reads, like `certified_endpoint_census`'s. The prose census has gone main-red five times since 2026-10-27 (`work/{bind,clear,contacthold,splice}/main-red-prose-census-*`); every time a PR outside pncad-py moved a renderer.

## The class is wider than censuses (2026-11-09, `onecopy/loft-rim-red`)

A behaviour test in the slow set has the same blind spot: `sweep::all a_loft_computes_its_walls_in_t::a_lofted_arc_wall_is_a_function_of_the_sections_bulge` counts the iso-boundary decisions a loft records.
- **The mover.** PR 6484 (`bc927bc4b3`, 2026-11-03) changed what `topo::pcurves` certifies at the mint, so the test reads 18 Zeros where it pins 10. The PR touched six of sweep's test files and still did not seed this row. It added two (the test module `tests/a_wall_row_read_in_band_takes_the_lane_a_miss_takes.rs` and the fixture `tests/common/semicircle_wall.rs`) and modified four (`tests/a_wall_seam_takes_the_column_it_lies_on.rs`, `tests/all.rs`, `tests/common/mod.rs` and `tests/euler_site_row_frontiers.rs`): its added lines never spell the test's name, so its CI never ran the test.
- **How long it stood.** The nightly went red the next day. Two lanes filed it separately on 2026-11-04, and it stood red for six days (`work/green/main-red-a-lofted-arc-walls-rim-zeros-count-eighteen.md`).
- **What the options above catch.** Option A, as written, does not reach this case: no `gated_to!` marker could name every path that moves the decisions a loft makes, which is most of topo and geom-brep. The gap is the slow set's `is_new` rule for every cross-crate slow test, not only the source walks.

## Evidence: PR 6818 (TINT, 2026-11-11)

A fourth instance: `bounds_census::every_sole_bracket_bound_door_is_in_the_roster`.

- PR 6532 (PCERT, `bcbbe4fe5c`) added the sole-`T: Bounds` door
  `crates/geom-brep/src/edge_nurbs.rs` `knot_lines` and gave it no roster
  line. Main stayed red from that merge until PR 6818 wrote the line.
- That census had already been fixed twice in the preceding eight days
  for the same reason (`5cf5c2fbc5`, `11b103efcb`).

