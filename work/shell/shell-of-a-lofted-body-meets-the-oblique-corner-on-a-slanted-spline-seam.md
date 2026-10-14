---
id: shell-of-a-lofted-body-meets-the-oblique-corner-on-a-slanted-spline-seam
kind: issue
title: the per-chart offset door moves a cap rigidly, so on a loft whose seams slant the moved corner leaves the seam and shell refuses ReanchorOffCarrier before any wall fit
status: closed
opened: 2026-10-06
priority: P1
cost: H
refs: [shell-refuses-every-lofted-body-at-a-wall-seam-carrier]
branch: shell/oblique-corner-derives
pr: 4351
closed: 2026-10-08
---


Filed by the `shell/lofted-wall-seam` lane, which gave
`replace_face::plan_reanchors` a NURBS re-anchor and measured where
the lofted bodies go next. This is the wall the twisted loft and the
vase now meet; it was hidden behind the carrier-lane refusal.

## Measured

`topo::shell(&body, 0.05, Tol::witness())` at the default ε, release
build:

| body | refusal (on a cap) | gap |
|---|---|---|
| twisted loft, 0.3 rad (`common::approx::twisted_loft`) | `ReanchorOffCarrier` on a wall–wall seam | 0.019466 m |
| circular vase (circle sections r = 1, 1.3, 1 at z = 0, 1, 2, degree 2) | `ReanchorOffCarrier` on a wall–wall seam | 0.025370 m |
| straight square prism (`common::approx::prism`) | passes the re-anchor; see `work/iso/nurbs-iso-derive-line-rim-arm-refuses-an-interior-row.md` | — |

The twisted loft's gap is exactly `0.05 · sin(slant)`, where the slant
is the seam's chord against the cap normal (the seam drifts 0.4227 m
over its 1 m height): `crates/sweep/tests/encl_curved_loft_shell.rs`,
`shelling_the_curved_loft_refuses_at_the_oblique_cap_corner_before_any_fit`,
asserts that to 1e-12. The vase's seams are curved, so its gap is that
to first order (0.025725 from the seam's tangent at the cap):
`shelling_the_vase_refuses_at_its_oblique_cap_corner` in the same file
builds the vase and asserts the gap within 5% of it.

The M7-8 cube (`crates/topo/tests/fixture/m7_8.rs`, pcurves minted)
meets a different wall at each scalar: `shell(0.1)` at `f64` refuses
`FittedBoundaryUnsupported` on the wall's edge, and at `Interval` the
re-chart refuses "the plane × NURBS Intersection lane refused —
outside the plane × NURBS lane: the analytic operand's structural
parameters are not exact at this scalar".

## Why

`replace_face_offset` moves ONE chart and keeps every neighbour
(`crates/topo/src/replace_face.rs` module docs, "What moves and what
does not"). A plane cap's boundary edges are transported rigidly by
`d·normal` (`transport_curve`), so the cap's corner lands `d` along
the cap normal. On a loft whose wall seams are parallel to that normal
(the prism) the corner stays on the seam and re-anchors; where the
seam slants it does not, and the gate `offset_reanchor_on_carrier`
refuses. The refusal is right: the cap's new boundary on an untouched
slanted wall is the moved plane ∩ the wall, not the old rim
translated, so the rigid transport would build the wrong body.

This is the #1081 oblique-junction class
(`ReanchorOffCarrier`'s docs), met on a plane × spline-wall corner.
`offset_planes_together` solves such corners only when every face is
a plane, and `offset_charts_together` only for coaxial charts.

## What a fix has to decide

A cap offset beside an untouched spline wall needs its rim re-derived
as `plane' ∩ wall` (the plane × NURBS lane already certifies that
class, `geom_brep::plane_nurbs_limbs`) and its corners at `plane' ∩
seam`, i.e. the corner slides ALONG the seam rather than along the cap
normal. Whether that is a new arm of the per-chart door (re-intersect
instead of transport, for a plane against a spline neighbour) or a
spline-wall member of the simultaneous doors is the design question.
Behind it the walls still meet the fit's budget at the default ε and
O4's `FittedBoundaryUnsupported`, measured in
`shell-refuses-every-lofted-body-at-a-wall-seam-carrier`.

## Decided

2026-10-08, by the orchestrator on a designer pair that converged over
two rounds (byte 172 and the rounds on
`analysis/design-fork/shell-lofted-oblique-corner`). No ratified text
moves, so no `[ev]` PR and no fork-log row. Both designers rejected the
item's two framings (a per-chart arm for plane × spline wall; a
spline-wall member of the simultaneous doors).

**The defect is transport, not which door.** The per-chart door
translates a moved face's boundary carriers rigidly, which is exact only
where the neighbour is unchanged by the move (a plane cap on vertical
walls). The loft's cap rims make it worse: they are described as images
in the cap's own chart with a declared `PlacedSegment`
(`sweep::swept::describe_face_rim_at_rest`; `crates/sweep/src/loft.rs`
"loft does not upgrade these rims"), so `plan_edge` never reads the wall
and the corner gate is the only place the slant is caught.

**The per-chart door derives:**

1. **Edges.** An edge between the moved surface and a distinct held
   surface is their section through the C5 table (closed form for the
   analytic pairs; plane × NURBS by `geom_brep::plane_nurbs_ssi`,
   certified at attach by `plane_nurbs_limbs`; the old carrier seeds
   branch and sense; where the section is exactly a row of a spline
   wall, the exact row). Rigid transport survives only between two sides
   of one chart (seams, wraps, iso images) and as a *decided* shortcut
   where the held neighbour contains the move direction (the straight
   prism), so that case keeps working at every scalar.
2. **Corners.** A moved vertex is a derived edge ∩ the third surface,
   checked against the vertex's other edges (`VertexDisagreement`) — on a
   NURBS seam, the moved plane's root along it near the old parameter
   (`geom_brep::boundary_section`, graze refused by name). Never a
   transported point tested after the fact: the one-door merge
   (`offset-doors-are-one-door-with-a-held-distance`) must generalise
   this solver, not add a third.
3. **The declared record.** An offset edge between two distinct surfaces
   is derived and its declaration dropped (the cavity's rim is not a
   curve the user drew). A declared conventional edge between two
   distinct surfaces the move tilts refuses typed; no native producer
   reaches it once the loft changes. Check `step-import`'s
   `AdoptionCandidate::MappedCurve` order (`adopt.rs`) for an imported
   one and pin it if found.
4. **The loft** describes its cap rims as `Intersection{cap, wall}` at
   rest (the extrude's Phase-6 idiom; the plane × NURBS lane's
   `min_sin_theta` is the transversality verdict), with the wall's row
   as carrier. D2's NURBS-adjacent exemption waives the demand, not the
   preference.
5. **Refusals.** `ReanchorOffCarrier` retires as a class; a graze or a
   missing root refuses with the section's own verdict.
   `NeighborPairUnroutable` / `NeighborPoseUnroutable`,
   `ReanchorPastCarrierEnd`, `ReanchorCollapse`, `NurbsLaneUnsupported`
   (the march is `f64`-only, so an `Interval` slanted edge refuses by
   name) are unchanged.

**Carried:** `work/iso/nurbs-iso-derive-line-rim-arm-refuses-an-interior-row`
lands first (ISO has nothing dispatched; the moved rim on a parallel-
section loft is an interior row of the wall, and the mint must place it
there either way). The stale docs: `replace_face.rs`'s module docs
("ALL PLANES has another route") and `ReanchorOffCarrier`'s ("every
curved corner still does"), the loft module doc's waiver, and the
geom-brep README's "Volume, area and tessellation still refuse typed on
a face carrying a `General` pcurve" (check against `props/quad.rs`).

**Expected outcome:** the twisted loft and the vase move their cap and
then refuse at the first wall, plane × fitted surface
(`a-fitted-wall-has-no-section-with-a-moved-cap`, the next question for
Ev). Re-baseline both `ReanchorOffCarrier` rows in
`encl_curved_loft_shell.rs`; `an_outward_cap_offset_runs_past_the_seam_patchs_end`
holds. Not measured by the pair: whether `plane_nurbs_ssi` certifies at
the default ε on the bilinear saddle and the vase wall — measure first.

## Closed

PR #4351. The per-chart door derives an edge between the moved surface
and a held one as their section, and a moved corner as a root along an
edge meeting it; transport survives on one chart and under the decided
`holds_the_move` shortcut. `ReanchorOffCarrier` is retired. Step 4 (the
loft's at-rest `Intersection` rims) was dropped on measurement — 44
building rows refused by the plane × NURBS certificate,
`work/ssiedge/plane-nurbs-certificate-refuses-at-rest-rows-of-lofts-and-sweeps.md`
— so item 3 applies to declared chart edges: derived, record dropped.
The twisted loft's cap now moves and its shell refuses at the first
wall's fit (`BudgetExhausted` 4.14e-9 vs 1e-9); the vase's at its rim's
limb 2 (4.787e-4 m,
`work/ssiedge/plane-nurbs-limb-two-refuses-a-non-row-section.md`).
