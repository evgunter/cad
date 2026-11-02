---
id: main-red-certified-endpoint-census-moved-in-six-files
kind: issue
title: Main red: the certified endpoint census moved in nine files (PRs 5855, 5868, 5886, 5728, 5897, 5909, 5927); 27 new reads sit outside an is_certified function
status: closed
opened: 2026-10-28
priority: P0
cost: E
closed: 2026-10-29
branch: splice/census-reds
pr: 6040
---


Found by nightly triage 2026-10-28, run
[40637814390](https://github.com/evgunter/cad/actions/runs/40637814390)
(main `a1cfd1cfdb`), job `full suite (every eps row, slow set included)`.
Red at every ε row.

## What

`geom-core` `certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`
(`crates/geom-core/tests/certified_endpoint_census.rs:835`) reports the
census moved, in six files (live count against `ROSTER`, as
`(endpoint-read lines, of them in a function that asks is_certified())`):

| file | roster | live | moved by |
|---|---|---|---|
| `crates/geom-brep/src/curve_face/spline_face.rs` | (5, 5) | (10, 7) | PR 5855 (`splice/spline-span-arm`) |
| `crates/geom-brep/src/krawczyk.rs` | (2, 1) | (5, 4) | PR 5855 |
| `crates/geom-brep/src/offset_meters.rs` | (7, 7) | (9, 7) | PR 5868 (`splice/spline-seam-dihedral`) |
| `crates/geom-brep/src/ssi/enclose.rs` | (12, 12) | (19, 19) | PR 5886 (`sect/torus-certificate`) |
| `crates/geom-brep/src/pcurve_cache.rs` | (2, 2) | (3, 2) | PR 5728 (`offset/limb2-for-every-carrier`) |
| `crates/geom-core/src/spline/blossom.rs` | absent | (1, 0) | PR 5728 |

Attributed by swapping each file to its version at each merge in
`fb476030a9..a1cfd1cfdb` that touched it and re-running the census.

## What closing it takes

What the census's own message asks: each new read outside an
`is_certified()` function (spline_face +3, offset_meters +2,
pcurve_cache +1, blossom +1) either asks `is_certified()` by name or is
argued safe at its site with the file's remainder note extended; then
the roster takes the new counts. The new reads in `krawczyk.rs` (+3)
and `enclose.rs` (+7) all sit in functions that ask, so only their
roster counts move. The SPLICE share is the largest, so the row sits here; the
`enclose.rs` count is SECT's and the `pcurve_cache.rs` and `blossom.rs`
reads are OFFSET's.

Reproduce: `cargo nextest run -p geom-core --ignore-default-filter -E
'test(every_production_endpoint_read_is_counted_and_dispositioned)'`.

## 2026-10-28 — pinned on main (GREEN, `green/full-suite-pins`)

Nightly run 40637814390 (main `a1cfd1cfdb`), `full suite`; re-run on main `91c83c38`. Three more files moved after the nightly, attributed the same way
(each file swapped to each merge that touched it):

| file | roster | live | moved by |
|---|---|---|---|
| `crates/geom-brep/src/intersect/section_bounds.rs` | absent | (13, 13) | PR 5897 (`offset/tube-exemption-by-pair`) |
| `crates/topo/src/clearance.rs` | absent | (17, 0), then (20, 3) | PR 5909 (`clear/shell-3-engine-into-topo`), then PR 5927 (`clear/nurbswin`) |
| `crates/geom/src/surfaces/nurbs.rs` | absent | (4, 1) | PR 5927 |

The roster now holds main's counts. Where every read sits in a function
that asks (`krawczyk.rs`, `ssi/enclose.rs`, `intersect/section_bounds.rs`)
the count is re-blessed. Where reads sit outside one (`spline_face.rs` +3,
`offset_meters.rs` +2, `pcurve_cache.rs` +1, `spline/blossom.rs` +1,
`surfaces/nurbs.rs` +3, `topo/src/clearance.rs` +17) the entry is pinned
with a note naming this row. Closing it dispositions each of those reads
and replaces each note. The `clearance.rs` and `surfaces/nurbs.rs` reads
are CLEAR's.

Today's count: nine files moved, and 27 of the new reads sit outside a
function that asks (`spline_face.rs` 3, `offset_meters.rs` 2,
`pcurve_cache.rs` 1, `spline/blossom.rs` 1, `surfaces/nurbs.rs` 3,
`topo/src/clearance.rs` 17). **Closing this row owes the notes too**:
each of those six roster entries in `certified_endpoint_census.rs` now
carries a "pinned red at main's count since PR …" note naming this row.
Whoever argues a read safe (or makes it ask) replaces that note with the
argument; the census checks the counts, not the notes, so a stale note
does not go red by itself.

**2026-10-29, nightly triage** (run 40823227622, main `5648e577eb`): four files moved past the pinned roster since this row's pin, at every ε. `ssi/four.rs` (3, 2) → (4, 2), from PR 5938 (`splice/tube-side-tripwire`). `geom/src/curves/nurbs.rs` is new at (3, 3), from PR 5955 (`clear/selfx`). `geom/src/surfaces/nurbs.rs` (4, 1) → (8, 2) and `topo/src/clearance.rs` (20, 3) → (28, 8), from PRs 5955 and 5976 (`clear/loops`). Seven of the new reads sit outside a function that asks: `four.rs` +1 (SPLICE), `surfaces/nurbs.rs` +3 and `clearance.rs` +3 (CLEAR). All three of `curves/nurbs.rs`'s reads sit in functions that ask.

## Closed

Closed 2026-10-29 by `splice/census-reds`, measured on main `0011f58a`.
The census was red there at `ssi/enclose.rs` (20, 19), `ssi/four.rs`
(5, 2), `surfaces/nurbs.rs` (8, 2), `clearance.rs` (23, 6), and at two
files new to it, `curves/nurbs.rs` (3, 3) and `topo/src/injective.rs`
(52, 1). Every read outside an asking function was read and
dispositioned. Fixed rather than rostered:
- `spline_face.rs`: `Path::at`/`tangent` (NURBS arm) now ask through
  `certified_window`;
- `offset_meters.rs`: `cell_distance` reads a cell with a partial that
  does not certify at distance 0 (`a_refused_partial_puts_its_cell_at_the_point`);
- `pcurve_cache.rs`: `limb2_offset_sup`'s `upper` goes through
  `certified_bracket()`;
- `spline/blossom.rs`: `whole_bezier_span` asks before it takes a refused
  end for an exact knot (`a_whole_span_restriction_refuses_as_the_pyramid_does`);
- `ssi/enclose.rs`: `first_order_continuous_in` reads crease knots through
  `certified_bracket()` (`a_refused_crease_reads_as_not_continuous`);
- `ssi/four.rs`: `along_side` reads controls through `certified_bracket()`,
  since its answer moves a refusal from `Undecided` to the sign-certain
  `AlongSide` (`a_refused_control_is_not_along_a_side`);
- `offset_meters::chart_arm`: a NaN cell distance now counts as 0, not as
  a skipped cell, and a non-finite `p` refuses as its doc said.

Argued sound at the site:
- `spline_face::end_state`, a seed;
- `four::row_crossing`, read after `tube_margin` has asked;
- `surfaces/nurbs.rs` and `curves/nurbs.rs`'s knot clip ends.

CLEAR's `clearance.rs` and `injective.rs` are pinned at main's counts.
Their reads that are not sound are filed on CLEAR:
`work/clear/clearance-lifts-trv-brackets-and-reads-their-ends`.
