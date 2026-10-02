---
id: a-boolean-result-carries-a-seam-meridian-certificate-a-fresh-run-does-not-reproduce
kind: issue
title: a boolean result's seam meridian carries a certificate a fresh re-certification of its own geometry does not reproduce (the snowman lens, 1.48e-16 carried against 2.22e-16 fresh)
status: open
opened: 2026-10-02
priority: P3
cost: E
refs: [the-union-fallback-graft-re-certifies-a-disjoint-operand-through-the-lane-free-door]
---


Found by the measurement in
`the-union-fallback-graft-re-certifies-a-disjoint-operand-through-the-lane-free-door`.

## Measured

`intersect(ball(1.0, 0.0), ball(0.8, 1.4))` (`sweep`'s snowman lens,
`Tol::witness()`, `f64`). Re-certify every certified edge of the
result against its own endpoints and surfaces
(`EdgeCurve::recertify_via` with `NurbsLane::certified()`, at
`Band::linear(Tol::witness())`) and compare the certificate it
carries:
- one edge differs: the B ball's seam meridian (carrier
  `Circle { center: (0, 1.4, 0), axis: −z, radius: 0.8000000000000002 }`,
  params `(2.366399280279432, π)`, ends `(0.5598833697790122,
  0.8285714285714285, 0)` and `(0, 0.6, 0)`) carries
  `max_residual: 1.4806895100970427e-16`; a fresh run mints
  `2.220446049250313e-16`;
- in the other operand order, `intersect(ball(0.8, 1.4), ball(1.0, 0.0))`,
  two edges differ, again the B ball's seam meridians (radius 1.0),
  `1.5700924586837752e-16` carried against `2.482534153247273e-16`
  fresh, ending at `(±0.5598833697790122, 0.8285714285714283, …)`.

Both balls alone re-certify bit for bit. Every other edge of the
lenses does too.

## Cause: not established

Consistent with the seam zip keeping A's copy of a junction vertex
that B's meridian was certified against: the junction's y differs in
the last bits between the two orders (`…285` against `…283`), and the
stale edge is always B's. Not traced to the line.

## Why it matters

A certificate is documented as the attachment-time run over the
edge's geometry, and both carrying doors (`boolean::voids` and, since
the same PR, the containment fallback's assembly) say a carried
certificate is the one a fresh run would mint. That holds for every
source whose certificates match its geometry. The lens's do not, at
the ulp, so a graft carries its stale residual forward. The
difference sits far inside the band, and the at-rest gate re-derives
carriers without reading a stored certificate. Pinned as it stands by
`sweep`'s `snowman::a_disjoint_union_carries_the_kept_operands_certificates`.

## What is owed

Find the step that moves a certified edge's endpoint without
re-certifying it, and re-certify there (or show the move cannot
happen and the difference has another cause).
