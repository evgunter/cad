---
id: a-faces-loops-are-walked-inline-beside-face-loops-linked
kind: issue
title: A face's loops are walked inline at about fifty sites beside Body::face_loops_linked
status: open
opened: 2026-11-02
priority: P1
cost: M
---

Found by the sweep of `slit-reads-a-faces-edges-through-its-own-walk`,
which moved `solid_contain::face_edges` beside the walk it re-spelled,
as `Body::face_edges` (`crates/topo/src/live.rs`).

## What

`crates/topo/src/live.rs` holds the walk of a face's boundary:
`Body::face_loops_linked` ("the one order every walk of a face's
boundary takes, and the one spelling of the fields a miss names"),
`Body::face_boundary_linked` / `face_boundary_by_loop` over it, and now
`Body::face_edges`. Each hop is a link and a miss panics (D2 row 4).

`core::iter::once(<face>.outer).chain(<face>.rings…)` spells the same
walk inline at about fifty production sites in `crates/topo/src` (the
sweep pattern `once\(.*\.outer\)|\.outer\)\s*\.chain`), each then
resolving loops, cycles and half-edges with its own reading of a miss:
`?` into `None`, a typed refusal, `unwrap_or_default`, or a skip.
Among them: `cell_region.rs`, `census.rs` (`face_loops`), `chart_region.rs`,
`clearance.rs` (two), `euler.rs` (two), `iso.rs`, `merge_faces.rs`,
`offset_together.rs`, `pcurves.rs` (two), `props.rs` (three, one a
`face_loops`), `separation.rs`, `seqgen.rs` (three), `shell.rs`,
`side_reader.rs`, `stands.rs` (`face_loops`), `surface_boundary.rs`,
`splitting/finish.rs` (four), and in `boolean/`: `contain.rs`
(`face_loops`), `discard.rs`, `mod.rs` (three), `ops.rs` (three),
`reduce.rs`, `region.rs`, `solid_contain.rs` (two), `spline_region.rs`.

## Direction

Not every one is a copy: `validate.rs` (five sites) and
`coherence.rs` read bodies whose links may not resolve and report a
miss rather than panic, which is their job; an Euler operator's own
mid-operation walk may be too. The rest read through
`face_loops_linked` / `face_boundary_linked` / `face_edges`, each
deciding at its call what a stale face key is; where a site's shape is
not expressible there, extend the home rather than write beside it.
