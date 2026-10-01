---
id: mef-and-mfkrh-onto-a-new-chart-strand-the-edges-they-move
kind: issue
title: mef and mfkrh onto a chart of their own, and the loop-moving kills, leave the edges they move described against the chart those edges left
status: review
opened: 2026-09-30
priority: P3
cost: M
refs: [set-face-surface-hands-the-caller-an-ordering-obligation-in-prose, loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart, description-staleness-ladder-three-spellings, kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers]
pr: 3673
branch: topo/euler-doors-vouch-their-charts
---

## What

Found by the receipt sweep of
`set-face-surface-hands-the-caller-an-ordering-obligation-in-prose`
(PR 3580). An edge description names its surfaces by key (an
`Intersection`'s operands, a `Chart`'s chart). `Body::set_face_surface`
now refuses a swap that would leave an edge described against a surface
neither of its faces wears (`EulerOpError::RechartStrandsDescriptions`),
and `Body::set_face_surfaces_describing` takes their re-descriptions
under a band (`Body::carried_redescriptions` states each stored
description on the moved chart, for a caller that chooses it). The Euler operators that put an EXISTING
half-edge on a face wearing another key make the same move and ask
nothing.

- **`Body::mef` (`mef_chords`, `crates/topo/src/euler.rs`) with
  `FaceSurface::New` or a `Shared` key of another chart.** The run
  between `he1` and `he2` moves onto the new face; each run edge's
  description still names the parent's key. Probe: the unit brick
  (`test_support_fixtures::brick`, described as intersections),
  `mef` across its top cap between opposite corners with
  `FaceSurface::New` holding the cap's own plane. It returns `Ok`, and
  `validate_geometric` reports `DescriptionNotAdjacent` on both run
  edges (plus the new chord's `ScaffoldAtRest`, which is the spec's).
- **`Body::mfkrh` / `mfkrh_minting` (`mfkrh_with`,
  `crates/topo/src/euler_kill.rs`) with `New` or `Shared`.** The ring's
  loop becomes a face on the stated chart; the same shape over a whole
  loop. Not probed.
- **The loop-moving doors — `kef`, `kfmrh`, `ring_move`** — move a
  loop or run onto a face whose key may differ from the one its edges
  name. `kef` across the brick's top rim edge strands the three edges
  it moves (`DescriptionNotAdjacent` ×3), though a kef between two
  non-coplanar faces is not a legal merge to begin with; a kef between
  two coplanar faces on distinct keys is the honest witness to build.
  Not probed further.

The pcurve half of the same move is
`loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart`
and its siblings; this row is the edge-description half.

## Shape

The re-chart doors' answer applies per operator: a keys-only door
refuses typed, before mutating, naming every edge it would strand; a
describing or minting twin takes a band and the re-descriptions,
reading a key the edge's face wore as the chart it moves to
(`attach.rs`'s `Sides::repoint`), and re-describes nothing by default. `crates/topo/src/shell.rs`'s rim re-point
(`remap_description` over a loop after a merge, then
`set_edge_curve`) is a caller doing this by hand today.

## Evidence: the off-boundary half (PR 3598)

`set_face_surface` now also refuses a swap onto another chart that a
certified edge of the face does not name
(`EulerOpError::RechartUnvouched`): only a certificate on the new chart
vouches that the face's boundary lies on it. The doors here mint or move
a face onto a chart the same way and ask neither question. Probed on
`attach::tests`' inlay (the brick's top cap with a square membrane
whose four edges are images in the cap's chart), with the membrane's
own plane raised four units (`far`):

- `kfmrh(cap, membrane)`, then `mfkrh(ring, New far)`: `Ok`, and tier 3
  reports `PlanarFaceResidual` and `PlanarBoundaryResidual` on the
  promoted face, with no `DescriptionNotAdjacent` (no edge is stranded,
  since the cap still wears the key they name).
- `mef(Chords)` across the membrane between opposite corners, `New far`
  (a scaffold chord): `Ok`, with the same two residuals beside the
  chord's `ScaffoldAtRest`.
- `kfmrh(cap, membrane)`, then `ring_move(ring, front)`: `Ok`, with the
  same two residuals (and `RingMeetsOuter`).

`mef_lone` mints its face the same way (`mint_face_surface` after
`resolve_face_surface`, `crates/topo/src/euler.rs`) and was not probed.
The keys-only answer for each is the one `set_face_surface` now gives:
refuse typed, before mutating, where a certified edge on the face names
no key it wears after the move; scaffold and null edges are not asked.

## Measured, and built (PR 3673)

Every door was instrumented first (topo's suite and the `ci` profiles
of sweep, mesh, step-import and editor-core): per caller, the calls
that would strand an edge and the calls that would leave a certified
edge unvouched on the face it lands on.

- **`mef` / `mef_chord` / `mef_lone`, `mfkrh` / `mfkrh_minting` /
  `mfkrh_plug`, `ring_move` / `ring_move_minting`:** no production
  caller refuses, except `shell.rs`'s rim promotion (`mfkrh` onto the
  host's key, then `rename_loop_surface`), which now mints the face on
  the guest's chart and moves it through `set_face_surfaces_describing`
  with the same re-descriptions (18 of 18 promotions build a body
  `Debug`-identical to the old route's). These doors now refuse typed,
  before mutating, through `Body::vouch_move` (`attach.rs`), the one
  home `set_face_surface` uses too: `RechartStrandsDescriptions`, then
  `RechartUnvouched`, each naming its door (`RechartDoor`) and ending
  in the door's own lever. A certified chord `mef` mints is asked too.
- **`kef` / `kef_minting`, `kfmrh` / `kfmrh_minting`:** nine and two
  production call sites rely on the move, so their refusal is not
  built; the measurement and the design question are filed as
  `kef-and-kfmrh-across-keys-want-a-describing-door-or-reordered-callers`.
