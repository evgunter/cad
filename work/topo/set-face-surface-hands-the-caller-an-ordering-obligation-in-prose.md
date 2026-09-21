---
id: set-face-surface-hands-the-caller-an-ordering-obligation-in-prose
kind: issue
title: set_face_surface's certification note is a prose-held caller obligation: attach surfaces before upgrading edge descriptions
status: open
opened: 2026-09-14
refs: [S93, 713, set-face-surface-leaves-a-complete-face-certified-against-the-chart-it-left]
priority: P3
cost: E
---

## What

`Body::set_face_surface` (`crates/topo/src/attach.rs`) closes its
certification note with:

> (In particular, replacing a surface can invalidate an adjacent edge's
> certification; tier 3 reports it — attach surfaces before upgrading
> edge descriptions.)

That trailing clause is an ORDERING obligation held in prose, enforced
by nothing. It is the last live instance of the class `S93` was raised
to retire: `#713`'s prose-held-invariant sweep minted three of these,
and `Body::mev`'s fan note named this one as "the same posture" when it
still carried its own. PR 2562 replaced `mev`'s with a mechanism and
`kev`'s with a statement of the defect
(`kevs-fan-merge-needs-a-re-describing-kill-door`), and in doing so
dropped the pointer at this sibling — which is exactly how an instance
of a class stops being visible. This row is the pointer.

## The defect

A surface swap can falsify an adjacent edge's stored certificate: the
edge's description names the surface (an `Intersection`'s operand, a
`Chart`'s chart), and the certificate was derived against the surface
that was there. Nothing re-checks it at the swap — the door says so and
then tells the caller which order to work in instead. Tier 1 does not
constrain it, no operator repairs it, tier 3 reports it at rest, and
`split_edge` and `set_edge_curve` refuse typed on such an edge, which
is the same failure surface `kev`'s fan merge leaves.

## Not the pcurve row

`set-face-surface-leaves-a-complete-face-certified-against-the-chart-it-left`
is also about this door, and is a different half: that row is the
PCURVE MAP (rows left stated in a chart the face is not on, invisible
to `validate_pcurves` when the new surface mints nothing). This row is
the EDGE CERTIFICATE and the sentence that hands its upkeep to the
caller. Whoever takes them should take both — one door, one answer —
but they refuse different things and neither subsumes the other.

## Shapes

- **Re-certify the adjacent edges in the plan phase and refuse typed**,
  the shape `mev`'s fan site took in PR 2562 (`certify_rebased_run`
  through `EdgeCurve::recertify`): the affected set is the faces' loops'
  edges whose descriptions name the swapped surface. A setter CAN
  refuse — this one already refuses `StaleKey`/`StaleGeometry` — so the
  objection that stopped the pcurve half does not stop this half.
- **State the defect and delete the instruction**, the shape `kev`'s
  paragraph took: honest, cheap, and leaves the hole.
- **Drop the certificates the swap falsifies**, so the edges are
  undescribed rather than wrongly described — symmetrical with the
  pcurve row's cheap shape, and it makes tier 3's verdict the honest
  one.

The first is the one that retires the class rather than documenting it;
its cost is a walk over the face's edges at every surface attach.
