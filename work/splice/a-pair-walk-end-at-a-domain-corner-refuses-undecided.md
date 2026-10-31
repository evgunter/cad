---
id: a-pair-walk-end-at-a-domain-corner-refuses-undecided
kind: issue
title: ssi/four: a pair-walk carrier ending at a domain corner refuses Undecided, because face_roots resolves no corner root
status: open
opened: 2026-10-14
priority: P3
cost: H
---


(Found at the FULL review of PR 5094, `nurbs/pair-walk-domain-side-end`.)

## What

`ssi/four.rs` `one_arc_pair` asks each box for exactly two boundary roots (`face_roots`) before it reads either end (`reaches_end`). A locus that enters the domain through a corner of a chart has its root on two of the box's faces at once, on their shared edge:
- a face walk can never isolate that root by Krawczyk on either face;
- the walk runs to `EXIT_DEPTH` and returns `None`;
- `one_arc_pair` therefore refuses `Shortfall::Undecided` before `reaches_end`'s root reading (PR 5094) is reached.

The refusal is conservative, not unsound.

**Probe**, measured on the PR 5094 branch:
- the walls are the `four::tests` bilinears `(u, v) ↦ (0, u, v)` and `(u, v) ↦ (u − ½, 2v − ½, 2v − ½)`;
- they cross on the diagonal `u₁ = v₁`, from the first chart's corner `(0, 0)` to its corner `(1, 1)`;
- the ends are `[0, 0, ½, ¼]` and `[1, 1, ½, ¾]`, at axis 1.

Over the one box that is the whole domain, `face_roots` gives `None` and `one_arc_pair` gives `Err(Undecided)` at ε = 1e-9.

## What is open

A corner root needs a reading of its own. On the edge where the two faces meet, two coordinates are held, which leaves three equations in two unknowns: overdetermined, so no square Krawczyk test applies there either. The reading could be the edge's own: a root certified on the edge, by a least-squares or one-equation-dropped test whose third residual is then checked. Or the walk could count a root on a shared edge once, rather than failing on both faces. Whether this is worth building waits on a consumer; no current row reaches a corner end.

## Named, not answered (2026-10-28)

The walk's `Undecided` for this shape now refuses by name
(`the-two-chart-tube-refuses-a-carrier-along-a-chart-side-undecided`):
`OneArcRefusal::AlongSide { chart }` for a carrier along a chart's
domain side, `EndOnSides { sides }` for an end on two or more domain
sides at once. That changes the payload, not the reach: no row the
shape refused now certifies.

## Re-banded P0 → P3 (SPLICE, 2026-10-29)

Lily wall 8 no longer waits on this row. PR 5959 gives a rim that is a
domain-side row of two spline walls a structural selector, certified per
wall with no limb 3. Lily's own rim then refuses at a tangency (two
points), and the scene is being re-spelled as one loft. No consumer
reaches this shape today. It is still a real gap for a witness-selected
carrier, so it stays open at P3 until one does.

