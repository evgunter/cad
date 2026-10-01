---
id: unclaimed-half-edge-read-as-a-minus-half-in-chart
kind: issue
title: a half-edge its own edge does not claim is read as that edge's minus half: 1 site(s) on this ground
status: open
opened: 2026-09-30
priority: P3
cost: E
---

Filed by TOPO's fix pass on PR 3532 (the review's MINOR-3), which
fixed the same default in `crates/topo/src/merge_faces.rs` and
`crates/topo/src/loop_winding.rs`. One class, filed once per owning
slate; the table at the end lists every row.

## What

An edge claims its two half-edges in `he_plus` and `he_minus`. A
half-edge that neither slot holds (a torn edge ↔ half-edge bijection)
is neither half, but a test of one slot alone reads it as the other:

- a **direction** read, `edge.he_plus == he`, calls it the minus half,
  so it runs against the carrier;
- a **mate** ladder, `if e.he_plus == he { e.he_minus } else { e.he_plus }`,
  hands back the edge's plus half, which is not `he`'s mate (the edge
  does not hold `he`), and the caller asks about the wrong face.

Neither refuses. On a tier-1 body the state cannot occur, so the site
answers wrongly only where the body is torn: a pass that reads the
arena mid-surgery, or a gate that missed the tear. That is the D2
addendum's silent-discard shape, not a wrong answer on valid input.

## This slate's sites

- `crates/topo/src/chart_region.rs`, the trim polygon walk, ~:2447. The direction: `let forward = edge.he_plus == he`.

## Repair shape

Inside `topo`, read the pairing through `Edge::claim(he)`
(`crates/topo/src/entity.rs`), which returns `None` for an unclaimed
half and otherwise the half's side (`plus`) and its `mate`, and refuse
on `None` with the crate's typed corruption error, naming the
half-edge and the edge (`EulerOpError::UnclaimedHalfEdge { he, edge }`
where the site speaks the Euler vocabulary). `Body::mate` reads through
it; `merge_faces`' `outermost_survivor` and `edge_mate` and
`loop_winding`'s walk do since PR 3532. `Edge::claim` is `pub(crate)`,
so a site outside `topo` has `Body::mate` (public, `None` for an
unclaimed half) and no public direction reading: making `claim`
public is part of that fix.

## The class, every slate (PR 3532's sweep)

The sweep grepped `he_plus == `, `he_minus == `, `== e.he_plus` and
`== edge.he_plus` over `crates/` outside `tests/`. What it cannot
match: a pairing read through a renamed binding (`let (a, b) =
(e.he_plus, e.he_minus)` compared later), or through `Body::mate`
followed by an unconditional default; those were not searched.
Excluded as not this class: `step-export/src/writer.rs`'s flag ladder
and `euler_kill.rs`'s two mate ladders (both test both slots and
refuse), `euler_ring.rs`'s `claims_both` (tests both), `validate.rs`'s
self-pair checks, and `merge_faces.rs`' `#[cfg(test)]` tear hooks.

| owner | row | site | shape |
|---|---|---|---|
| reach | `unclaimed-half-edge-read-as-a-minus-half-in-reach` | `crates/topo/src/splitting/finish.rs`, `describe_section_boundary`, ~:537 | mate |
| reach | `unclaimed-half-edge-read-as-a-minus-half-in-reach` | `crates/topo/src/splitting/finish.rs`, the section-face removal walk, ~:881 | claim filter |
| reach | `unclaimed-half-edge-read-as-a-minus-half-in-reach` | `crates/topo/src/splitting/join.rs`, the conic run term, ~:367 | direction |
| reach | `unclaimed-half-edge-read-as-a-minus-half-in-reach` | `crates/topo/src/chord_join.rs`, the run pcurve stitch, ~:2009 (shared with TANG) | direction |
| reach | `unclaimed-half-edge-read-as-a-minus-half-in-reach` | `crates/topo/src/splitting/neighborhood.rs`, the base-endpoint jet, ~:185 | direction |
| reach | `unclaimed-half-edge-read-as-a-minus-half-in-reach` | `crates/topo/src/boolean/sectors.rs`, the sector tangent, ~:196 (shared with GERM) | direction |
| chart | `unclaimed-half-edge-read-as-a-minus-half-in-chart` | `crates/topo/src/chart_region.rs`, the trim polygon walk, ~:2447 | direction |
| zip | `unclaimed-half-edge-read-as-a-minus-half-in-zip` | `crates/topo/src/boolean/join.rs`, `ring_run_ccw`'s run term, ~:1471 | direction |
| zip | `unclaimed-half-edge-read-as-a-minus-half-in-zip` | `crates/topo/src/boolean/rest.rs`, the REST lane's first-run far half, ~:1807 (shared with TANG) | mate |
| contact | `unclaimed-half-edge-read-as-a-minus-half-in-contact` | `crates/topo/src/boolean/solid_contain.rs`, the run parameter interval, ~:1960 | direction |
| boxes | `unclaimed-half-edge-read-as-a-minus-half-in-boxes` | `crates/topo/src/boolean/boxes.rs`, the face pcurve steps, ~:1047 | direction |
| tess | `unclaimed-half-edge-read-as-a-minus-half-in-tess` | `crates/mesh/src/trimmed.rs`, the trimmed-face loop walk, ~:987 | direction |
| tess | `unclaimed-half-edge-read-as-a-minus-half-in-tess` | `crates/mesh/src/walk.rs`, the loop walk, ~:204 | direction |
| band | `unclaimed-half-edge-read-as-a-minus-half-in-band` | `crates/sweep/src/blend/surgery.rs`, the rim mate, ~:1741 (shared with CARVE) | mate |
| band | `unclaimed-half-edge-read-as-a-minus-half-in-band` | `crates/sweep/src/blend/surgery.rs`, the link mate, ~:1832 | mate |
| band | `unclaimed-half-edge-read-as-a-minus-half-in-band` | `crates/sweep/src/blend/surgery.rs`, the rim arc record, ~:2372 | direction |
| band | `unclaimed-half-edge-read-as-a-minus-half-in-band` | `crates/sweep/src/blend/surgery.rs`, the rim edge mate, ~:2845 | mate |
| band | `unclaimed-half-edge-read-as-a-minus-half-in-band` | `crates/sweep/src/blend/surgery.rs`, the rim arc mate, ~:3651 | mate |
| no owner | `unclaimed-half-edge-read-as-a-minus-half-unowned` | `crates/topo/src/props.rs`, the curved-face edge record, ~:1825 | direction |
| no owner | `unclaimed-half-edge-read-as-a-minus-half-unowned` | `crates/topo/src/pcurves.rs`, `is_plus`, ~:1199 | direction |
