---
id: nightly-reds-the-source-censuses
kind: unit
title: Nightly reds: six source censuses and two gate self-tests read a tree that moved under them
status: closed
opened: 2026-11-14
priority: P1
cost: M
branch: green/censuses
pr: 6921
closed: 2026-11-14
---

From nightly 43371638371 (main `e36675c1`, 2026-11-13), each leg's junit read whole. GREEN's rule holds (`work/green/program.md`): a red is greened here, not fixed. A move a merge explains is re-blessed, naming the merge; anything else is pinned at what main produces, or `#[ignore]`d naming the row it waits on, and the fix row goes on the program that owns the code.

**Difficulty letter:** M. **Review tier:** single review, STYLE — A census can be read and believed, and the risk is a wrong disposition of a new site. Both logged before any draw (2026-11-14).

## What is red

All three ε rows:
- `editor-core::all node_standing::every_node_result_reader_goes_through_usable_or_is_listed` (READERS: `eval/mod.rs` 35 against 33, `eval/parts.rs` 2 against 1)
- `geom-core::all certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`
- `topo::all recourse_roster::every_decided_name_has_its_door_and_every_site_is_read` (`ring_path.rs` `spline_crossings`; `ring_path/spline_wall.rs` `paths`, `reach`)
- `test-utils::all hand_written_impl_census::every_hand_written_walk_is_held_to_its_declaration` (geom-brep `spline_implicit/sweep/critical.rs`, `impl PartialEq for NodeFrame`) and `::the_walk_still_sees_every_file_holding_one`
- `topo face_normal::tests::no_source_file_folds_the_bit_by_hand`

Job `gate self-tests`: `breakpoints-by-hand.sh --selftest` (the copy fixture names `to_owned`, which its array lacks) and `reporting-margin-door.sh --selftest` (`crates/topo/src/validate.rs`: 0 mints, the entry pins 1).

Rows that may already own a cause (unchecked): `work/sectarms/main-red-recourse-roster-misses-two-ring-path-in…`, `work/spiricarm/main-red-pcurve-cache-gained-two-endpoint-reads-…`, `work/tess/main-red-trimmed-rs-folds-the-face-sense-by-hand`, `work/tesswalk/main-red-the-wrap-strip-folds-its-period-by-the-…`, `work/census/main-red-inert-deny-on-the-qualifier-enums`, `work/doors/node-standing-census-goes-red-on-main-between-ni…`, and this slate's `main-red-slow-rows-uncovered-at-b2337e9e` and `a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster`.

## Closed (2026-11-14, PR 6921)

All six censuses and both gate self-tests are green on main `77cf43f7b2` with this PR. Re-blessed against the merge that moved each: `node_standing`, the endpoint census, the recourse roster, one fold-guard hit. Pinned with rows on the owners: `NodeFrame`'s and `Stood`'s field-by-name equality (SECTARMS, DOORS), `one_plane`'s endpoint reads (EMBED), `pcurve_cache/speed.rs` (TESSLANE). Fixed test-side: the two gate self-tests, and the SIGPIPE-under-pipefail race at three sites through one `gate_contains` in `scripts/gates/lib.sh`. The class that made these red, a source change that moves a census's site without running the census on its PR, stays open on `a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster` and CIW's `tree-wide-source-censuses-in-the-slow-set-run-on-no-pr-that-moves-a-site`; main moved under this PR four times in a day.
