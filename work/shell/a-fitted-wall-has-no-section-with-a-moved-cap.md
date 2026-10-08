---
id: a-fitted-wall-has-no-section-with-a-moved-cap
kind: issue
title: C5 routes no Approx x Plane section, so every lofted shell refuses at its first moved wall: may a fitted chart's section be its fitted spline's, with the fit certificate as an error bound?
status: open
opened: 2026-10-08
priority: P2
cost: H
refs: [shell-of-a-lofted-body-meets-the-oblique-corner-on-a-slanted-spline-seam]
---

Filed by the unit 9 designer pair. Once a moved cap's corner is solved
on the slanted seam (unit 9), `shell` moves each spline wall of a loft,
whose offset is a fitted (`Approx`) chart, and the cap–wall edge is a
plane × fitted-surface section. C5 routes no `Approx × anything` pair
(`crates/geom-brep/README.md`, C5: the composition "is not a ratified
rule"; O4's last sentence), so the edge refuses
`NeighborPairUnroutable` / `FittedBoundaryUnsupported`: every lofted
shell (twisted loft, vase; `crates/sweep/tests/encl_curved_loft_shell.rs`)
stops there, one face later than today. The fit's budget at the default
ε is untouched.

The question, a design fork for a designer pair and then Ev: may a
fitted chart's section be the section of its fitted spline, with the
fit's certificate counted as one term of the error bound? This decides
whether a loft shells at all. ENCL's
`a-rigid-map-can-still-refuse-…` stays parked on it.

## Decided

2026-10-08, by the orchestrator on a designer pair that agreed on the
final state (byte 133, `analysis/design-fork/shell-fitted-wall-section`).
Not put to Ev: the refusal this retires is agent text (C5's
"`SurfaceKind::Approx` is its own kind, not `Nurbs` … not a ratified
rule", commit 297fb2ba4f, PR 1012, no review; O4's close, 4eda8abec4),
and the change restores what OFFSET-DESIGN said when Ev ratified it at
#907–909 (5eafd903e1; 1ae5ad8efe): each dispatch site "must SAY what it
does with an approximating surface (most delegate to the fitted NURBS;
some — dihedral classification, census — must consult the
description)". Both designers rejected the item's framing.

**A fitted face's section is its fit's section; nothing is composed.**
As an intersection operand an `Approx` surface is its fit — as it
already is for evaluation, boxes, tessellation, its pcurves
(`pcurve_cache::fitted_lane`) and its own iso-row edges (`resolve_iso`).
The fit's distance from its description is the face's claim, made by O3
and re-derived at rest by O5; no edge stores, restates or adds it.
Composing it into the edge's bound is rejected (the budget does not
close — a fit stops just under ε — and limb 3's tube cannot lift through
a C⁰ bound); certifying against `S + d·n` is rejected (one face with two
geometries).

1. **Routing.** `SurfaceKind::Approx` stays its own kind, and C5 says what
   it does with one: `(Plane, Approx)` / `(Approx, Plane)` route to the
   plane × NURBS arm over `approx.fit()`; every other `Approx` pair routes
   as the fit's kind does (all unimplemented today, with that arm's note).
2. **The edge** stores `Intersection { s1: plane, s2: the Approx key }`,
   carrier from `plane_nurbs_ssi(plane, approx.fit(), ..)`, pcurves on the
   plane and the fit's chart; its certificate is `plane_nurbs_limbs`
   against the plane and the fit, transversality on the fit's jet.
3. **Tier 3** re-derives the fit against `Offset { base, d }` once per face
   (O5) and the edge's limbs against the fit (`certify.rs`'s plane/NURBS
   resolver admits `Surface::Approx(a)` as `a.fit()`).
4. **Stated once** in the README, not enforced: the carrier lies within
   `2ε / sin θ` of the described section along the wall, the same
   conditioning every certified `Intersection` edge already has.
5. **Still refused:** `Approx` × anything but a plane (`NeighborPairUnroutable`
   with the fit's kind honest in the payload); a smooth seam shared by two
   fits (`FittedBoundaryUnsupported`); a section outside the fit's window
   (the section's own verdict); `Interval` (`NurbsLaneUnsupported` /
   `ApproxLaneUnsupported`).
6. **README text:** C5's sentence replaced as above, plane × `Approx` added
   to the implemented list; O2's kind-table sentence says geometry
   questions read the fit and intended-surface questions (O5, dihedral
   classification, census) read the description; O4's closing "a
   NURBS-walled body still cannot be shelled" restated as the refusals in 5.
   Code docs on `Surface::Approx`, `SurfaceKind::Approx` and the C5 arm
   follow.

**Expected outcome: no loft shells yet.** The rational limb-2 defect does
not bear on this (the offset fit mints unit weights). But every wall moves:
the twisted loft's wall–wall creases become fit × fit
(`a-wall-seam-between-two-fits-has-no-section`, P2) and the vase's smooth
seams have no transverse section
(`two-fits-sharing-a-smooth-seam-disagree-by-their-certificates`, P3).
Hence P2: this lands after unit 9 (both touch `replace_face.rs`'s
fitted-boundary arm), and a re-fit re-derives the face's edges — the edge
half of ENCL's parked `a-rigid-map-can-still-refuse-…`.
