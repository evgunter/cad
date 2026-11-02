---
id: cylinder-sphere-germ-pair-has-no-join-lane
kind: issue
title: A transverse cylinder wall x sphere germ pair has its section and its matching and no chord lane at the join (CurvedBooleanUnsupported): its section is a space quartic, and every lane rides a plane
status: closed
opened: 2026-10-04
priority: P0
cost: H
refs: [4025, skew-cylinder-germ-pair-has-no-section-frame]
branch: sect/cylinder-sphere-chord
closed: 2026-10-28
---

Found by the cylinder × sphere frame lane (branch
`join/cylinder-sphere-frame`, 2026-10-04): the frame is the door before
this one.

## Measured

`crates/sweep/tests/cylinder_sphere_frame.rs` pins nine poses, each
under ∪, ∩ and both differences in both operand orders (54 runs): a ball
against a radius-0.5 drum `z ∈ [−1, 1]` crossing its wall in one loop
(centre on the wall, inside it, outside it, a turned chart, a ball wider
than the wall) or two (a
ball reaching past the far side, one with a turned chart), and the
tilted drum cut's lower part against the two balls straddling its rim
(radius 0.2 at `(0.5, 0, 0.35)`, radius 0.1 at `(0.45, 0, 0.3)`). Every
run passes the section frame and the matcher, which consumes every germ
and joins only neighbouring sites along each loop
(`every_segment_joins_neighbouring_sites_along_its_loop`), and then
refuses

```
CurvedBooleanUnsupported { operand: A, face: <the wall or the sphere>,
                           kind: Cylinder | Sphere }
```

at `boolean::join::bool_connect`'s lane dispatch, the `(a_s, b_s)`
catch-all. Not every pose reaches this door: a ball whose turned chart
crosses a cap passes the frame and the matcher and stops earlier, at
`Join(SectionNotPolar)` (144 of 1 152 runs in the PR 4025 review's drum
× ball grid; `tilted-sphere-pair-section-refuses-at-the-polar-gate` (REACH, closed by PR 3817)).
The rows hold each run to its volume (an independent oracle,
the slices' closed-form areas integrated) the day it builds.

## What a fix has to supply

A chord lane for a non-planar section. Every lane the dispatch has
(`GermLane::{Planar, PlaneWall, WallPlane, Radical}`) lays both sides'
chords in one plane, and a wall face's ring lane closes its island along
that plane (`RingClosure::Wall`). The transverse cylinder × sphere
section is a space quartic: the chord between two germ sites is an arc
of it, minted as a fitted curve (`geom_brep::ssi::cylinder_sphere_ssi`
marches and fits this pair; `Pcurve::Fitted` certifies at rest), with
its pcurve on each face's chart, and the ring lane needs an island
closing that is not a plane section. How a fitted chord between two
germ sites is minted and certified, and how the island closes, is a
design question; the skew cylinder pair reaches the same gap once it has
a frame (`skew-cylinder-germ-pair-has-no-section-frame`).

## The shape to give

The join builds on one object: **the section of a germ face pair**. It is the certified intersection of the two faces' surfaces, at the rung the C5 table gives that kind pair: a line, a conic, or a fitted carrier with its C2 certificate. It is computed once per face pair, and both operands and the matcher read it.

- **Matching.** A germ site's place on its section loop is its foot parameter on the component. Its partner is the next site along the component, in the direction the germ leaves. No per-pair section frame, and no proof that a loop turns monotonically about an axis, is owed. Two sites on different components never pair.
- **Chord.** The sub-arc of the component between two partner sites, described `Intersection { own, copy of partner }` (D2), with each face's pcurve derived from the carrier (C4's projected image where no closed form exists). Both solids cut their copies from the same carrier bits.
- **Ring-lane islands** close by the chord's own curve on every face kind. `RingClosure::Wall`'s section plane retires. So does `RingClosure::AlongEdge`'s "nothing to close along", because that chord copies an edge whose curve is known.
- **Lanes.** The planar lanes (plane × anything, the radical plane of two spheres or of two parallel cylinders) are the closed-form instances of the same object. `GermLane` carries a section datum per side, not an arm per kind pair. Kind dispatch lives only in C5.
- **Refusals.** A pair whose section C5 cannot certify refuses there, typed, and the join has no opinion. Today that means torus and cone pairs (C2 limb 2's conversion to metres). A transverse pair does not depend on D10.
- **Coincident and tangent sections are decided at the section door, never in the join.** That covers a coaxial pair, an axis offset or a tangency decided Zero, and an in-band section. Until INTENT stage 4's PR E (`booleans-glue-on-zero`), the door refuses them as it does today. From E on, the door glues and records them, and the join builds what it certifies. E's spec puts the coaxial cylinder × sphere record in `cs_pair_frame` (`docs/INTENT-STAGE4-SPEC.md` §6); with frames retired, that decision and its `OnCarrier` record live at the section door, its one home.
- **Order.** The build lands after INTENT stage 4's PR A (`the-join-builds-what-the-rest-zip-builds`, PR 4364), which rewrites the join's ring re-homing on a wall chart (`chord_join::chart_ring_side`) and deletes the REST zip. That PR re-homes rings, deciding which face holds a ring; this row closes islands. They meet in `chord_join`, not in a decision.

Downstream, the volume (`props`, which reads only `Harmonic` boundaries on analytic charts) and the mesher (which refuses `Pcurve::Fitted` on a trimmed face) each need a fitted-boundary arm on analytic charts before these poses' rows go `SOUND`. The spline-carrier mint on an analytic chart is PCERT's (`mint-has-no-route-from-the-closed-form-door-to-a-spline-carrier`, fork row 89).

## Split at design (2026-10-27)

The two designers (SECT step 1) converged on cutting step 1 in two:
`the-section-of-a-surface-pair-is-one-object` (1a) builds the section
value, its door and matching by foot parameter; this row is 1b, the chord
lane and island closure on it, and waits on 1a. Its design is that row's
"The design". Two facts the paragraph above predates: PCERT's
spline-carrier mint closed (PR 4304), so a spline carrier on an analytic
chart mints `Pcurve::Projected`, and the mesher reads `Projected` on a
trimmed face (`mesh/src/trimmed.rs`); props' `Projected` arm on an
analytic chart is what remains for these rows to go `SOUND` (1b
measures it).

## Closed

2026-10-28, by `sect/cylinder-sphere-chord`. `bool_connect` takes a
segment inside a face of each operand on a pair the section table
carries as `GermLane::Carried`, from the table and not from the kinds.
Its chords are arcs cut from the component's carried loop between the
two sites, the way the germ leaves (`chord_join::JoinLane::Carried`),
described as the intersection of the face's surface and the copy of the
partner's, certified at the edge door (`analytic_rung3` over the arc),
with projected pcurves. A closed loop is seated at its first site in
match order before any chord is cut from it (`PairSection::seat`: split
there, the pieces joined the other way round, certified again), so no
chord straddles the carrier's seam. Islands close by the chord on every
curved face (`RingClosure::Chord`): `ring_path::Quadric` reads a
cylinder, and `LoopArc::Spline` reads a spline arc through the
curve-pair table. The 54 pinned runs now pass the join and stop past
it: 28 at the result gate's volume on a face the chords bound (the
cylinder's projected image, the sphere's lane, the sphere's ring gate:
`spiricarm/a-projected-boundary-on-an-analytic-chart-has-no-volume-lane`)
and 26 at the spline edge join (`splice/joining-a-spline-carrier-is-unbuilt`).
A coaxial pair's circles are a coincidence the door decided, and keep
their refusal at the join until the door records them.
