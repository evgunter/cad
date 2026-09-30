---
id: mef-and-mfkrh-onto-a-new-chart-strand-the-edges-they-move
kind: issue
title: mef and mfkrh onto a chart of their own, and the loop-moving kills, leave the edges they move described against the chart those edges left
status: open
opened: 2026-09-30
priority: P3
cost: M
refs: [set-face-surface-hands-the-caller-an-ordering-obligation-in-prose, loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart, description-staleness-ladder-three-spellings]
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
