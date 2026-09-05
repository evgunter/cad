---
id: ring-clearance-refuses-a-nested-trim-circle
kind: issue
title: fillet: ring clearance refuses a ladder rim whose widened trim circle is NESTED inside its host's circular outer boundary; the doc's 'neither occurs today' has a witness
status: open
opened: 2026-09-04
---


Found by FILLET-H5's Phase 1 sweep, outside that unit's fence: the
LADDER path is unchanged by H5 and this defect is in it.

## The doc's claim

`ring_clearance_pass`'s outer-boundary arm
(`crates/sweep/src/blend/surgery.rs:2200`–`:2248` on the H5 head
`e44f1a7fe` and after; `:1937`–`:1951` at the merge base it was filed
against) scopes itself
honestly and then asserts reachability:

> the two false-refusal classes are (1) a trim circle NESTED inside a
> circular outer boundary, where the containment margin
> `aj − (‖cj − ci‖ + si)` is positive but the external form reads
> negative, and (2) a distant line edge whose EXTENSION passes near the
> trim circle. **Neither occurs on the bodies this kernel mints today**
> (planar outer boundaries are convex blank/trimline cycles).

## The witness for class (1)

`test_support::revolved_about_y` of `(0,0) (1,0) (1,1) (0.5,1)
[bulge tan(π/8)] (0,1.5)` — a cylinder of radius 1 and height 1 with a
flat top annulus in to radius 0.5 and a hemispherical dome of radius
0.5 rising to the pole — followed by `merge_coplanar_faces`. Census
after the repair: `V=7 E=10 F=6`.

The repaired flat top is one annular plane face: outer cycle the two
arcs of the cylinder's top rim at radius 1, one RING the two arcs of
the dome rim at radius 0.5. Filleting the dome rim (`rim_arcs_at(body,
0.5, 1.0)`, radius 0.1) resolves to a LADDER rim — the rim IS a ring of
its plane support — passes the ladder's gates, and then refuses:

```
RingClearance {
  face: FaceKey(3v1),
  margin: ClassifiedMargin {
    predicate: "fillet3_ring_clearance",
    reading: Value(-1.5916079783099617),
    band: Band { zero: 1e-9, escalate: 1e-8 },
    sign: Negative } }
```

The magnitude is the tell: `−(si + 1)`, i.e. the widened trim circle's
radius plus the outer boundary's radius, with both circles CONCENTRIC
on the axis (`‖center − ci‖ = 0`). That is the external-separation form
`‖cj − ci‖ − si − radius` applied to a circle that CONTAINS the trim
circle with room to spare: the containment margin is
`1 − (0 + si) ≈ +0.41`, positive. The carve is geometrically fine and
is refused.

Its dimple twin (the same profile with the arc dipping to `(0, 0.5)`,
bulge `−tan(π/8)`) refuses identically, so both material sides reach it.

## Relation to the closed item

Not `ring-clearance-reaches-front-door-off-lattice` (PR 1753), which is
about the SAMPLED screen overestimating a gap between two features that
do not face each other on the lattice. This one is the exact closed-form
check itself using the wrong form for a nested pair, on a body where the
screen has nothing to say. The two are independent.

## The ask

Either give the outer-boundary circle arm its containment form — take
the better of the external and containment margins when the boundary
circle contains the trim circle — or, if the refusal is deliberate,
retract the "neither occurs today" sentence and name this body. The
current state is a documented-unreachable arm with a two-line fixture
that reaches it.
