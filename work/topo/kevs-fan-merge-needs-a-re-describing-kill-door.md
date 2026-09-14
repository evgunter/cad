---
id: kevs-fan-merge-needs-a-re-describing-kill-door
kind: issue
title: "kev's fan merge re-bases carriers and no precondition can refuse it: the kill needs a re-describing door"
status: open
opened: 2026-09-14
parent: S93
refs: [S93]
---

## What

`S93` asked both fan-rebasing operators to stop leaving a re-based edge
carrying a certificate against a point that is no longer its endpoint.
`Body::mev`'s fan site now does: it re-certifies the moved run in its
plan phase and refuses `EulerOpError::RebasedCarrier`. `Body::kev`'s
fan merge does not, and this row carries why, with the measurement.

## The defect, unchanged

`kev(he)` kills `end(he)` and hands that vertex's surviving fan to
`start(he)`. Every merged member keeps the `EdgeCurve` it was certified
with, which pins `carrier(t₀)` (or `carrier(t₁)`) to the DEAD vertex's
point. Where the two vertices' points differ, each merged edge
describes a locus that no longer ends where the edge does. Tier 1 does
not constrain it; tier 3 reports it at rest; `split_edge` and
`set_edge_curve` refuse on such an edge, and `seqgen`'s `split_site`
filter exists to route around exactly that (`crates/topo/src/seqgen.rs`,
the "Why the re-certification" paragraph — narrowed by S93 from two
causes to this one).

## Why the same gate could not land here

The gate that works for `mev` is a precondition: refuse before
mutating. Applied to `kev` it refuses the fan merge's live callers,
because they kill **mid-surgery** and re-describe the merged edges at
the end of the door, so the promise the precondition would need to see
does not exist yet at the call.

Measured on `topo/s93-rebased-carriers` with the gate wired into
`kev` (`self.certify_rebased_run(&fan, self.resolve_vertex_point(v)?, tol)`
in `Body::kev`'s plan phase, `kev` taking a `Tol`):

- `cargo test -p sweep`: **129 failures out of 1394**, every one of them
  the fillet/blend verb refusing. 118 carried the new error, at exactly
  two sites: `site: "annulus closure kev"` (92) and
  `site: "rim closure kev"` (26), both in
  `crates/sweep/src/blend/surgery.rs`. The remaining 11 are downstream
  rows that build a fillet first.
- Both sites sit inside `blend_surgery`'s surgery scope, which ends by
  running `attach_contact` over the `Described` list and then
  `topo::mint_pcurves`, and asserts `topo::validate_closed`. So the
  blend's OUTPUT is coherent and only its intermediate state is not —
  the door re-describes the merged edges after the surgery, and a
  precondition on the operator cannot see that it is going to.
- `cargo test -p topo` was green with the gate on `kev`, so nothing in
  `topo` itself depends on the merge; the cost is entirely at the
  composite doors.

Ordering does not rescue the caller either: any spec the merged edge
could carry BEFORE the kill must end at the vertex that is about to
die, and any spec it needs AFTER must end at the survivor, so no
sequence of today's doors expresses "merge this fan and re-describe
it". The same wall stands one level down: `split_edge`'s two-op inverse
(`kev` then `set_edge_curve`, `seqgen::roundtrip`) is the same shape.

## The shape of the fix

A kill that takes the merged fan's re-descriptions and certifies them
in the same step — `kev` with a `&[(EdgeKey, EdgeCurveSpec<T>)]`
alongside, or a named `kev_describing` — certifying each supplied spec
against the endpoints the merge WILL give the edge (through
`Body::certify_rebased_run`'s door, `geom_brep::EdgeCurve::recertify`),
refusing typed before any mutation, and writing the topology and the
descriptions together. Plain `kev` then keeps the merge only where
every merged member's carrier already passes the gate, and the two
blend sites pass the carriers they already compute for
`attach_contact`.

That is a new public door and a change to two other programs' callers
(`sweep`'s blend, and whatever else the sweep below finds), which is
why it is a row and not a residue of S93's unit.

## Receipt — every site that re-bases a half-edge run

Swept on `mev_null`/`kev`/`MevSite::Fan` (the pattern: a site struct
with `he1 != he2`, and every `.kev(` call), merge base
`42bfd53a0`. What the pattern cannot match: a re-basing written as a
direct `half_edge.start = v` write outside the two operators — the
sweep found none outside `euler.rs`'s `mev_fan_execute` and
`euler_kill.rs`'s `kev`, which are the two this row and S93 are about.

| site | re-certifies / carries / refuses |
|---|---|
| `mev` fan (`euler.rs`, `mev_fan`) | **re-certifies and refuses** — the S93 gate |
| `mev_null` fan (`null.rs`) | **carries** — the new point is `plan.p_old`, a bitwise copy, so no endpoint moves and no certificate is touched; structural, no comparison |
| `kev` fan merge (`euler_kill.rs`) | **carries, unchecked** — this row |
| `kemr` (`euler_ring.rs`) | nothing to re-base: it kills an edge and re-anchors loops, no half-edge changes its start vertex |
| `split_edge`'s children (`split.rs`) | **re-certifies** — both children certify against the new endpoints before any mutation (`certify_edge_spec`), and the carried pcurve rows with them |
| `merge_coplanar_faces`' absorption (`merge_faces.rs`) | **refuses or carries** — its only vertex-killing step is `kev(from_rim)` behind `strut_tip`, i.e. a valence-1 far vertex, so no fan merges and no carrier moves |
| `kfmrh` / `ring_move` (`euler_ring.rs`) | out of scope here: they re-parent LOOPS between faces, not half-edges between vertices (TOPO-B3 slot 0's subject) |
