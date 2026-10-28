---
id: main-red-certified-endpoint-census-moved-in-six-files
kind: issue
title: Main red: the certified endpoint census moved in nine files (PRs 5855, 5868, 5886, 5728, 5897, 5909, 5927); 27 new reads sit outside an is_certified function
status: open
opened: 2026-10-28
priority: P0
cost: E
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
