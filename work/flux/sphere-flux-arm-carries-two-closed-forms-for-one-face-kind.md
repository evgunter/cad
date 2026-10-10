---
id: sphere-flux-arm-carries-two-closed-forms-for-one-face-kind
kind: issue
title: The sphere flux arm measures a tilted-circle face by Gauss-Bonnet and every other face by the iso-rectangle form, where the first subsumes the second
status: open
opened: 2026-10-02
priority: P1
cost: H
design: true
needs_ev: true
---


## What

Found by the `reach/tilted-sphere-pair` lane, which added the second.
`geom_brep::props::curved::sphere` dispatches on
`sphere_loop_has_tilted_circle`: a face with a boundary circle tilted
against the chart takes `sphere_circle_loop` (area by Gauss–Bonnet over
its circle arcs — constant geodesic curvature per arc, turning angles
at the vertices — flux `R·Area` under the sense bit plus `c·A⃗`); every
other sphere face keeps the iso-rectangle arm (`sphere_boundary`, the
rim and meridian parse, `props_rim_level`, the wedge and two-band
branches).

Gauss–Bonnet is exact for every one-loop sphere face bounded by circle
arcs, so it measures every face the iso arm measures, and the L-shaped
faces the iso arm refuses (`props_rim_interior_side`, issue 1598) too.
Two closed forms for one face kind is the P1 shape: the S58 guards the
iso arm carries exist because its formula is only right on a
rectangle, and a face measured by the other formula needs none of them.

## The question

Retire the sphere branch of the iso arm in favour of the loop form (the
rimless band's and the wedge's `Δu` derivations, and the rim-side
premises, then have no consumer on the sphere; their bits move by
rounding), or keep the iso arm where it applies and state why. What the
two encodings of the radial side mean on the loop form — it reads the
sense bit, as the rimless band does, so `boundary_material_sign` has
nothing to cross-check it against — is part of the same answer.

## Weighed (2026-10-10): the sphere arm, decided apart from one ruling

(FLUX orchestrator) A designer pair (fork log row 108) weighed this row
together with `sphere-face-with-a-hole-has-no-closed-form`,
`a-sphere-face-whose-boundary-encodes-no-side-is-measured-under-its-bit-alone`,
`sphere-wedge-arm-does-not-fold-split-meridians-by-lineage` and the
sphere half of `rim-side-and-rim-dir-group-signs-are-facts-about-cycle-order`.
The pair converged after four rounds. The first three crossed; the
fourth settled on a fact the orchestrator verified on main. In
`crates/sweep/tests/tilted_sphere_pair.rs`,
`a_flipped_tilted_face_is_refused_by_name` shows that a flipped
one-loop lens face is caught only by the sense cross-check, because
tier 3's check 4 reads material pairing only on `Smooth` edges.

**The design.**

- **Flux.** Every sphere face bounded by circle arcs is measured by one
  closed form, Gauss–Bonnet over all its loops, holes included:
  - `Area/R² = 2π(1 − r) − Σ_loops(Σ∫κ_g ds + Σε)`, read against
    `N = σ·(p − c)/R`, where σ is the sense bit;
  - `flux = σ·R·Area + c·A⃗`;
  - the premises are the formula's own, each a named decide: every edge
    is a circle on the sphere, every loop closes, no junction is a cusp,
    the outer area lies in `(0, 4πR²)`, and each hole's area is positive
    and less than the outer area.
- **What retires from the flux arm.** The iso-rectangle branch goes:
  `SphereFluxSide`, `sphere_rim_only_pole_level`,
  `require_rim_interior_sides`, `require_band_opposite`,
  `sphere_wedge_azimuth`, the sphere's use of `du_of_rims`, and the
  pole fold. The iso parse survives only in `require_iso_rectangle`,
  which is mesh's door.
- **Rings and lineage pieces.** `topo::props::face_flux` hands a sphere
  face its rings, and `RingOnCurvedFace` is kept for the spline kinds
  only. A meridian arriving in lineage pieces turns by zero at each
  joint, so it needs no fold.
- **The side.** The sense bit is the face's side. One reader,
  `sphere_loop_side`, cross-checks it:
  - a face with holes is read by the sign of its own Gauss–Bonnet area,
    with no chart involved;
  - a one-loop face is read as the side of the loop that holds no chart
    pole, by each pole's winding number under stereographic projection;
  - the answer is `Encoded` when at least one pole is off the loop, and
    `Unencoded` only when both poles lie on it.
  - It replaces `sphere_circle_loop_side` and `side_on_meridian`; the
    latter gives up on the whole meridian when any crossing sits on the
    axis.
- **Who refuses a contradicted bit.** The flux arm and tier 3's check 6
  both call the reader; the sphere's flux reads the bit, so the arm must
  fail loudly. The refusal names both causes: the bit is inverted, or
  the face holds a pole that import must normalise.
- **The cone gate.** It reads `props_cone_area_side`, the quantity the
  cone's flux integrates, and `linear_rim_side` is deleted.

**What is Ev's.** The one-loop reading rests on Ev's provisional,
chat-only ruling on PR 2850: *a chart singularity inside a face is a
vertex of it*. The `[ev]` PR states that rule in
`crates/geom-brep/README.md` C12 (7) and asks whether it may bear a
refusal. If the answer is no, the fallback is the bit alone on one-loop
faces. That leaves a known silent gap: a flipped one-loop face whose
edges are all transverse to its neighbours (box ∖ a shallow ball, the
dimple's cap missing the seam) would measure a wrong positive volume
while tier 3 stays green.

**What the other rows become.**

- `sphere-face-with-a-hole-has-no-closed-form`: fixed by the χ = 1 − r
  term.
- `a-sphere-face-whose-boundary-encodes-no-side…`: shrinks to faces with
  both poles on the loop. Those are seam siblings that meet tangentially,
  where check 4's lamina reading catches a flip.
- `sphere-wedge-arm-does-not-fold-split-meridians-by-lineage`: moot.
- `rim-side-and-rim-dir-group-signs-are-facts-about-cycle-order`: closes
  with the cone-gate change.
- `work/fluxtail/props-sphere-side-in-arc-is-levered-by-arc-length-not-the-roots-slope`:
  moot under the winding reader.
- `work/fluxtail/sphere-rim-level-cosine-cancels-near-a-pole`: lives on
  only in mesh's door.

The build owes a row executing the box ∖ shallow-ball flip, as the
witness for why the reader stays.
