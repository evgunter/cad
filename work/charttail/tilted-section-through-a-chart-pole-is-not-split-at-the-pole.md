---
id: tilted-section-through-a-chart-pole-is-not-split-at-the-pole
kind: issue
title: The join mints a tilted section arc that runs through a chart pole without a vertex there, where a pole on a face is a vertex of it
status: closed
opened: 2026-10-02
priority: P2
cost: M
refs: [validity-refuses-an-interior-chart-singularity, tilted-section-near-a-chart-pole-refuses-arc-near-pole]
rides_with: a-section-circle-through-a-sphere-faces-pole-mints-no-pcurve
pr: 7180
closed: 2026-11-20
---


## Measured

Found in the review of PR 3817 and re-measured on its fix pass: two
unit balls, both `y`-poled, centres `c` and
`c + 1.4·(√(1 − 0.7²), 0.7, 0)`, whose radical circle passes exactly
through A's north pole. The join mints the section arc through the
pole with no vertex there, and the pcurve mint refuses
`Pcurves { Certify { ArcNearPole } }` (the near-pole sibling is
`pcert/tilted-section-near-a-chart-pole-refuses-arc-near-pole`).

By Ev's ruling recorded at
`restfront/validity-refuses-an-interior-chart-singularity` — a chart
singularity inside a face is a vertex of it — and by the same token
inside an edge: the chord should carry a vertex at the pole, as a
revolved ball's meridians end there.

## What a fix owes

Split a section chord at any pole of the divided face's chart its arc
passes through (a closed-form root: the circle meets the pole point),
so the arc's pieces each have a chart image; the row above, under
every op, against the lens closed form.

Re-measured on `pcert/projected-image` (2026-10-08): with the Hermite
image retired, the same union refuses `Pcurves { Certify {
SectorRefused { channel: Azimuth, .. } } }` at ε 1e-6, 1e-9 and 1e-12.
The projected image refines toward the pole, and the piece holding it
has no azimuth sector. Pinned by `crates/sweep/tests/tilted_sphere_pair.rs`,
`a_section_passing_near_a_pole_builds_and_one_through_it_refuses_typed`.
The near-pole sibling is closed: δ ≥ 1e-5 builds at every ε.

Moved from ROOTS to GERM on 2026-11-02, by `git mv`, when ROOTS merged into GERM: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so GERM, GERMCERT, ROOTS and APEX came in under one budget.

## Claimed by CHARTTAIL (2026-11-20)

Moved from GERM by `git mv`, id and body unchanged. It rides with `a-section-circle-through-a-sphere-faces-pole-mints-no-pcurve` (P1), which is the same defect on a plane × hemisphere witness. Here the witness is the two-ball radical circle, and the fix is the same: split the section at the pole, so the pole is a vertex and the two pieces meet at its reset.

## The split exists; the lane was the gap (2026-11-20)

Re-measured on `charttail/pole-section` at main `d44f46d328`, with the
finished body dumped before the boolean's mint (δ = 0, ε 1e-9). The arc
that refuses runs FROM A's north pole, a vertex of four edges, to its
other site. So the join does split the section at the pole, as the
chart-singularity clause asks, and "the join mints the section arc
through the pole with no vertex there" no longer describes main.

What refused was the general lane. The arc's first piece holds its end
on the pole, its radial hull reaches the axis, and no sector holds it
(`SectorRefused { piece: 0, Azimuth }`, read as `Pose`). The fix is in
that lane. The edge's producer states which of its ends lie on the
chart's singular set, decided at its vertices by the incidence reading
the loop walk uses (`topo::pcurves::singular_ends`). A conic image on a
sphere with such an end reads its azimuth there off the reduced radial
(`geom_brep::PoleEnd`). The union builds at the lens closed form at ε
1e-6, 1e-9 and 1e-12, and meshes at δ 1e-2, 2e-2 and 4e-2.

## Closed (2026-11-20)

By PR 7180: an arc that ends on a sphere chart's pole images through the projected lane's pole-ended piece (its azimuth read off the reduced radial part `g`, its end decided from the vertex's incidence, the envelope charged a pole term). The plane-through-pole cap, its subtract at (1,2,0) and the two-ball radical circle build at their closed forms at ε 1e-6, 1e-9 and 1e-12. The section was already split at the pole; the gap was the lane's.
