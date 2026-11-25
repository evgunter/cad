---
id: a-pipe-does-not-union-with-an-elbow-revolved-off-its-own-cap
kind: issue
title: A pipe does not union with an elbow revolved off its own end cap: the union refuses a recorded contact as undecided at a margin of -7.07
status: closed
opened: 2026-11-21
priority: P0
cost: H
branch: heldrecords/elbow
pr: 7432
closed: 2026-11-24
---


Found by PROFILE's elbow probe (2026-11-21), run on hosted CI as the
throwaway PR 7280 (run 44889924284, `test (app)`,
`editor-core::zz_probe_elbow`), for Ev's question whether a hollow
cylinder's end loops can be revolved into an elbow
(`work/faceloop/a-faces-loop-or-closed-trace-cannot-become-a-profile-loop.md`).
Filed here because the refusal is about a recorded contact
(`crates/topo/src/boolean/refusal_routes.rs`, `Coincide::Contact`,
"whether a recorded contact holds along its witness"); re-home it if
another program's charter fits it better.

## Measured

A pipe: `circle((0,0), 1.0)` and `circle((0,0), 0.6)` extruded 2. The
same two circles drawn on its end cap (`editor_core::draw_on_face`,
`OnFace::nearest()`), revolved a quarter turn about a line in the cap's
plane (`axis_through = (3, 0)`, `axis_heading = −π/2`, or `(−3, 0)`
and `+π/2`). The elbow builds. `Node::Union` of the pipe and the elbow
refuses, in all eight cases where the revolve builds (either cap,
either sign of the angle, either side), at ε = 1e-9, 1e-6 and 1e-12:

> the Boolean op refused: whether a recorded contact holds along its
> witness is undecided: margin -7.0685834705770345e0 lies past the
> ambiguity band (1e-9, 1e-8) — a decided sign this decision cannot
> use. Recourse: move the parts so they clearly meet or clearly stand
> apart there

## The defect

The two bodies share the whole annular cap, and their walls meet
tangent along both rims: this is the plain case of building onto a
body's own face and joining the result. The margin is not near the
band: −7.0686 is −9π/4 to the printed digits, a length or an angle
measure decided far on the wrong side, so the reading is asking the
wrong question of this contact (or reading it at the wrong place), not
losing a close call. The recourse cannot be followed: the parts meet
by construction.

Banded P0: a normal verb broken on normal geometry. It also gates
`work/faceloop/a-faces-loop-or-closed-trace-cannot-become-a-profile-loop.md`:
once a face's loop can be a profile loop, the elbow the person makes
by two clicks and a revolve is this union.

Not traced: which contact record, which witness, and why the margin is
that value. Not measured: an extrude off the same cap unioned with the
pipe (the same shared cap, with the walls coincident and not only
tangent); a solid rod and its elbow (no inner rim).

The probe's source is on the branch `profile/probe-elbow` until PR 7280
is closed; its construction is the four lines above.

## 2026-11-22 — Traced: the glue door's tangency table, on the rim, at a neutral point

Reproduced on main (`3,0`/`−π/2` and the other three building poses
at End; the probe of PR 7280 run locally) with the refusal text
unchanged. Instrumented, not committed.

**Which pair, which locus, which reading.** The outer wall of the pipe
(a `Cylinder`, r = 1) against the outer wall of the elbow (a `Torus`,
R = 3, r = 1), glued at `boolean::glue` → `glued_tangency` →
`verify_tangency` (`crates/topo/src/boolean/mod.rs`). No closed-form
line exists for the pair, so the locus is the rim circle the two
faces share (`rim_wedge::shared_rim`), t ∈ [0, 2π]. The table is
`contact_verify::tangency_reading` (`Claim::Tangency`) →
`tangent_pass`, row (4), the second-order margin: at sample i = 2,
t = π/2, p = (0, 1, 2), `κ_rel` = 0 exactly, the span bound
`kappa_drift` = 14.137 (= 4.5π), arm = 1, so
`Margin::sagitta(|κ_rel| − drift, arm)` = (0 − 4.5π)·1²/2 = −9π/4 =
−7.0686. Decided `Negative`, not `deferred` (the at-sample reading
`Margin::sagitta(|κ_rel|, arm)` is an exact Zero, not Positive), so it
falls to `k_stats::gate_rejection(SECOND_ORDER, …,
SizedPass::Positive)` — "a decided sign this decision cannot use" —
which `verify_tangency` raises as `Coincide::Contact`.

**Why κ_rel is zero there.** Across the rim the cylinder is straight
(normal curvature 0 along its axis) and the torus's normal curvature
along the sweep is cos θ/(3 − cos θ) at the rim point (cos θ, sin θ,
2): it changes sign at θ = ±π/2, where the torus's normal (±y) is
perpendicular to the revolve plane. Every elbow off a round tube has
these two neutral points; it is the G1 tube chain's neutral
meridians (`crates/sweep/tests/mate7a_torus_rest.rs`,
`the_g1_tube_chain_refuses_at_the_glue_door`, pinned refusing by
PR 6079). So the margin is not a close call read wrong; the table
asks for a jet-determinate tangency (`κ_rel` definitely positive),
and a smooth seam between a straight tube and a bend never has one.

**The next wall, behind it.** With row (4) skipped for
`Claim::Tangency` (experiment only), the same unions refuse
`TangencyUnsupported` from `Uncertified::SagUnreached`: along the rim
circle the span bounds are `residual_sag` = `gap_sag` = 3.99 at 9
samples, quartering per halving, ≈ 1e-3 after `REFINE_HALVINGS` = 6,
against a band of 1e-9. The sampled touch cannot certify a curved rim
whose surfaces vary along it, though both faces end on that circle by
construction.

**The neighbours.** A solid rod (`circle(1)` alone) and its elbow:
the same refusal, same margin. An extrude of the cap's ring off the
same cap, unioned with the pipe: builds (the walls are one carrier,
glued `Rest`/continuation, not `Tangent`).

**Why this is a design question, not a fix.** Ratified text pulls two
ways. The topo README's C4 `Tangent` row (the glue door's table)
requires "a definite second-order κ_rel … An in-band κ_rel, or an
exact zero at an isolated point, escalates" — written by PR 6079
(2026-10-30, not an `[ev]` PR), which replaced the earlier clause
that *bridged* exact zeros at isolated points ("a G1 tube chain's
neutral meridians"). The MATE-7 ruling (`docs/MATE-7-TANGENCY-DESIGN.md`,
Ev 2026-09-01) and tier 3 put jet-determinacy on the cusp arm only:
a π rim is the smooth-seam zip with no second-order condition. Options
seen: (A) a shared rim glues on its structure (identity, first-order
parallelism, the routing's wedge π), the sampled touch and second
order kept for loci derived from values; (B) keep the table, grow
exact span bounds for a circle carrier and bridge an isolated
`κ_rel` zero on a π-routed rim only; (C) keep refusing, with a
recourse that is true.

## 2026-11-22 — Ruled a junction; what stands behind the glue door

Ruled (HELDRECORDS orchestrator, two designers converging on option
(A)): two faces that both end on one curve are a junction, owing the
curve's identity and its wedge, nothing sampled along it; the C4
`Tangent` table keeps the tangencies along a locus through the faces'
interiors. The glue door's half lands first (`heldrecords/elbow`).

Measured behind it, with the probe hacks in `probes/heldrecords-elbow/`:

- **Routing first** makes the rim routing's `Transverse` arm
  reachable: a wall against the other body's coplanar cap shares the
  rim at a right angle (the extrude neighbour), which the table used to
  rule out first. It now means unglued, the crossing lanes'.
- **The crossing layer** (`reduce::curved_face_arm`, the one-end-on-
  the-carrier arm), both directions: the pipe's seam ruling ends on the
  rim tangent to the torus (`wall_crossing` → `SpanVerdict::Unsettled`
  → `CurvedPierceUnsupported`), and the elbow's sweep arcs ending on the
  rim graze the pipe's cylinder in band (`ArcCylinderRoots`, margins
  ±1.3e-9 … ±3.3e-9). The ruling x = 1 meets the torus CARRIER again at
  z = 2 − √12, off the face, so only a reading at the edge's end is
  true, never a global one-sided cover. Next: `heldrecords/elbow-ends`.
- Passed blind, an inward pose stops at a third arm of
  `curved_face_arm` (Circle × Cylinder, untraced), and an outward pose
  reaches the finish and refuses four edges still carrying the
  construction stand-in description: the seam's description where a
  station is not jet-determinate, untraced.
- **Inward poses**: +π/2 off either cap bends back into the
  pipe; filed as `work/tang/an-elbow-bent-back-into-its-pipe-meets-it-along-a-rim-and-a-crossing.md`.

## 2026-11-22 — Design: an edge that ends on a glued junction (`heldrecords/elbow-ends`)

**The wall.** `reduce::curved_face_arm` meets an edge `e` of operand X
against a curved face `Q` of Y, where `e` lies in a face `P` of X and
one end `a` of `e` lies on a rim that `P` and `Q` both end on, glued as
a π junction by the glue door (`TangentCert { line: false }`). The
residual `f(t) = F_Q(c(t))` has a root at `a` of multiplicity at least
two, so the root doors refuse at `a`: `wall_crossing` reads
`Unsettled` for Line × Torus, and `ArcCylinderRoots` escalates in band
(|margin| 1.3e-9 … 3.3e-9) for Circle × Cylinder. Two arms reach it:
`(Zero, ±)`, one end on `Q`'s carrier, and `(Zero, Zero)` undeclared,
the arm that ran untraced at ~3008. That second arm is an arc already
split at a real pierce, so both its ends are roots and one of them is
the tangent rim end; it is the inward poses' (filed on TANG) and the
same question at that end.

**The reading, in two halves of one.**

1. *Structural (the junction).* `a` is a root: the vertex lies on `Q`'s
   carrier, which the endpoint row decides Zero. Its order is at least
   two: `f′(a) = ∇F_Q·c′(a)`, `c′(a)` lies in `T_P`, and `T_P = T_Q`
   at a π junction's rim. Near `a`, `e` runs inside `P`, which leaves
   the rim on the side opposite `Q`'s face (departures `Opposite`), so
   the end's root is the end's own incidence: the vertex on `Q`'s
   boundary, which `vertex_on_curved_face` already places. No value
   decides any of this. The vertex has to lie on the glued rim
   (point-on-circle, D10: Zero is on it, in band refuses), not just on
   `Q`'s carrier.
2. *Certified (the rest of the span).* `f` is expanded about `a`,
   `f(a + s) = Σ c_j s^j`, polynomial in the line's `t`, or in the
   conic's angle with the end at phase zero. `c_0` and `c_1` are
   dropped on the structural half. The quotient's roots on the span
   are certified by the door's own lower-degree arm and placed against
   `Q`'s trim exactly as any crossing is. This is deflation, after the
   precedent `conic_quadric_roots` already sets for a cone's apex
   (`conic_quadric_cone_deflated_*`): its `G` after the apex's double
   root, and its spread bound.

**Where the multiplicity comes from: decided, never guessed.** `m` is
the least `j ≥ 2` whose coefficient `c_j` is decided nonzero over a
window at the end, as the apex's spread reads `|G(0)|` with a lower
bound across its window. `c_2 ∝ κ_rel(c′)`, the carriers' relative
normal curvature in the edge's direction at `a`.
- Decided nonzero gives `m = 2`.
- Decided Zero (D10's coincidence, as the glue reads one) moves to
  `c_3`, and so on. At a neutral point (the bend's κ_rel crossing zero)
  that is where the higher-order root comes from.
- Any `c_j` in band refuses typed. A new `Unsettled` arm, "the end's
  order is undecided", escalates with that coefficient's diag.
- Every `c_j` up to the door's degree decided Zero is the edge lying on
  the carrier: the existing `LiesOn` arm.

The dropped `c_0`, `c_1` are bounded by the geometry and not by the
rounded coefficients, as in the apex's spread: `c_0` by the vertex's
distance from the carrier, `c_1` by the tangency's angle at the
vertex, levered. The roots they move lie within
`(|c_1| + √(c_1² + 2λ|c_0|))/λ` of `a`, with `λ` the lower bound on
`|c_m|`, and are `a`'s own incidence. A spread past the band refuses
typed, never as a crossing.

**Kind pairs covered** (each door with its arm after the end root):
- `line_quadric_roots`: Line × {Cylinder, Sphere, Cone}. A quadratic
  with a double root at `a` is `c_2 s²` and nothing else.
- Line × Torus: the quartic leaves a quadratic.
- `conic_quadric_roots`: {Circle, Ellipse} × {Sphere, Cylinder, Cone}.
  A degree-2 trigonometric form with a double root at phase 0 is
  `2 sin²(φ/2) · G(φ)`, with `G` of degree 1. That is the apex's
  factorisation, at the end in place of the apex.
- `circle_torus_roots`, `ellipse_torus_roots`: degree 4, leaving degree
  3 after the double root.
- The plane lane (`plane_crossing`, conic × plane: a torus's meridian
  ending on a plane–torus junction) is reached only where the cover
  does not already answer. `tangency_certifies_side` admits
  plane–torus, so that pair keeps its covered arm and is not touched.

The junction's pairs that the cover excludes are the ones this serves:
cylinder–torus (the elbow), torus–torus (the G1 tube chain), and cone–
sphere and cone–torus wherever a π rim exists. Every covered arm stays
upstream and unchanged.

**Fail loud.**
- An end on a glued junction rim whose (curve kind, surface kind) has
  no end-root lane (Spiric, NURBS or periodic carriers, spline faces)
  refuses a new typed Limit, `BooleanError::JunctionEndUnread
  { operand, edge, face, curve, surface }`, naming the two kinds. It is
  never `CurvedPierceUnsupported`'s "move the geometry", which cannot
  be followed when the parts meet by construction.
- An end not on a glued rim keeps today's arms verbatim.
- An end on a rim the glue left unglued (an inward pose's `Same`
  departures, a cusp) is not a junction and gets nothing from this
  reading. It refuses as today, and its true answer is TANG's row.

**No false global cover is possible.**
- The reading asserts no side of `Q`'s carrier for `e`. It factors one
  root at one end, which the junction supplies, and leaves every other
  root to the door's certification and the trim.
- `edge_covers` and `tangency_certifies_side` are untouched:
  Cylinder × Torus stays excluded, and the README's reason (a straight
  tube leaving a bend G1 crosses its continuation) stays true.
- The pin for this is a pipe long enough that its seam ruling `x = 1`
  meets the torus CARRIER again inside the edge's span, at
  `z = 2 − √12` for a pipe longer than √12. That root has to be
  certified and placed off the torus face (`SpanVerdict::Elsewhere`),
  and the union builds at its closed form. A cover would have skipped
  the root; this reading finds it.

**Pins planned.**
- The four outward poses at the closed form, with tier 3 and 3′ (once
  PR 3's seam description lands, if the stand-in still stands).
- The rod.
- The long pipe above.
- The G1 tube chain, flipped to building, with tier 3, 3′ and the
  Pappus volume.
- A neutral-point end, a seam ruling placed at θ = π/2 where
  `c_2 = 0`, giving `m = 3`.
- An end whose `c_2` is put in band, refusing typed on the end's order.
- An uncovered kind pair refusing `JunctionEndUnread`.
- The inward poses still refusing typed, as filed.

## 2026-11-22 — Traced: the four edges left on a stand-in (PR 3's subject)

This was measured with the crossing layer's half of
`probes/heldrecords-elbow/probe-hacks.patch` applied on top of the
junction glue door. It shows the outward poses' finish refusal,
`ValidationError::ScaffoldAtRest` ×4.

The four edges are the two rims, each in two halves split at x = ±r:
the outer rim (r = 1) and the inner (r = 0.6). Each bounds the
pipe's Cylinder and the elbow's Torus. Each still carries
`EdgeDescription::Scaffold(MappedCurve { source: RevolvedPoint { … },
… })`, a point on the rim revolved by π about the pipe's axis. That
is a construction's stand-in, which nothing re-described once the two
coplanar caps merged away and the edge came to bound the two walls.

So the result's seam edge is never given a two-surface description at
all. The question the ruling names comes behind this: the plain
circle where a station is not jet-determinate, `TangentIntersection`
where it is (`must_carry_over_edge`). Not yet traced: which stage owns
the re-description of a rim edge whose faces change under the zip,
and why the edge reaches the result with the stand-in rather than the
operand's at-rest description.

## 2026-11-22 — Built: the end-root door, and where it moved from the design

`geom_brep::curve_face::end_root` and `reduce::junction_window`, on
`heldrecords/elbow-ends`. These moved from the design above.

- **The spread is not band-sized, and cannot be.** A double root's
  position is ill-conditioned: with the end decided only within ε of
  the carrier, the roots the dropped terms can move lie within about
  `√(2ε/κ)` of the end. Measured at 8.9e-5 m for the elbow's sweep arc
  against the pipe at ε = 1e-9. "A spread past the band refuses" would
  therefore refuse every double root. What holds instead:
  - inside the spread the edge runs within the charge, the zero band
    at the end's scale, of the other face's carrier;
  - it does so on its own face's side of the junction's rim, where the
    other face is not (departures `Opposite`);
  - so the gate is that the spread ends strictly inside the window
    (`curve_face_end_root_spread`), and the window's far end stands off
    the surface (`curve_face_end_root_window_clear`).
- **The order walk starts at `j = 1`**, so the first-order coefficient
  is decided like the rest rather than dropped on the structure. At a
  junction it decides Zero.
- **Charges are the most each verdict admits**, the zero band's width
  at the coefficient's power over the window's local extent. This is
  the orchestrator's condition (1) on the D10 step, and it holds for
  `c₀` and `c₁` too.
- **The predicate names take the table's prefix** (`curve_face_end_root_*`),
  since the door is not one of the table's `Door`s.
- **Measured order at the elbow's neutral point is 4, not 3.** The
  pipe's ruling through `(0, 1, 2)` meets the bend at
  `(√(9 + s²) − 3)² ≈ s⁴/36`.

Measured after it:
- The four outward poses (−π/2 off either cap; the earlier sections'
  "Start cap −π/2 inward" had the Start cap backwards), the rod's
  outward pose and the G1 tube chain's union all pass the crossing
  layer. Each stops at the finish on the seam's stand-in descriptions
  (PR 3).
- The tube chain under ∖ and ∩ stops at the fallback's extent, filed
  as `a-junction-under-subtract-or-intersect-refuses-at-the-fallbacks-extent.md`.
- Inward poses are unchanged: they are no junction.

## 2026-11-22 — PR 3's stage, located

The stand-in survives in the describe pass, `ops::describe_edges`. Its
smooth-seam arm restates a seam's description where it is stale or
where the must-carry rule reads it intrinsic:
- `JetDeterminate` restates it as `TangentIntersection`;
- `UnderDetermined` restates it at rest (`rest_locus`) only if stale.

A `Scaffold` description counts as stale only between two faces of a
recorded merge skip (`recorded_skip`, "any other scaffold stays one, and
tier 3 refuses it"). The junction's rim reads `UnderDetermined`,
because the bend's neutral points are not jet-determinate. It is not a
recorded skip, so the scaffold rests, and tier 3 refuses it
(`ScaffoldAtRest`).

The stand-in itself is `MappedCurve { RevolvedPoint … }` about the cap
circle's own axis. Neither operand carries it in. Both bodies' rim
edges are at rest as `Intersection { pair: {cap plane, wall}, … }`,
measured by printing every edge's description of the pipe and of the
elbow before the union. So the stand-in is minted inside the boolean:
the zip's scaffolding `mekr`/`mef` carriers, the coplanar caps' merge,
or the graft. Not yet traced: which of these. That decides the fix:
either the stage that fuses the rim keeps an at-rest description, or
`describe_edges` describes a scaffold that rests between a glued
junction's two faces where it lies.

## 2026-11-23 — PR 3's fix pass: the walk certified, the stand-in fixed where it is minted

- **The bend is determinate only where it is certified.** The reviews
  found the sign rule both too strong and too weak: a neutral point in
  a half-rim's end gap, or two zeros π apart outside the seven
  stations, read `JetDeterminate`, and the must-carry rule demanded a
  `TangentIntersection` its certificate refuses. `second_order_walk`
  now ends `Determinate` only where the least `|κ_rel|` over its
  stations and the span's ends, less `kappa_drift`, decides positive
  under the certificate's own `tangent_tube_margin`
  (`dihedral::tube_separation`). Otherwise it is `Separated` and the
  seam is described at rest. The material fold keeps reading the
  stations, so an opposed contact whose end flips stays `Mixed`.
- **The stand-in is gone at its minter.** `chord_join::along_edge_spec`
  re-minted a copied circle edge as `arc_of_circle`'s scaffold (and a
  copied line as `line_between`'s); the copy now restates the edge's
  own at-rest spec, so the zip's seam reaches the describe pass as
  `Intersection { cap plane, wall }`, and `describe_edges_seamed` is
  gone: a scaffold rests only on a recorded merge skip.
- **Pinned:** R1's converse arc, R2's end-gap rows and dome in
  `dihedral::bend_rows`; R2's end-gap poses (δ = 1.15, 1.18, ±1.21,
  hollow and rod) unioning at the closed form.

## 2026-11-23 — Fix pass on the end-root door (PR 7463's two reviews)

- **`whole` past a quarter turn.** A circle's window starts from a
  quarter turn at most, so it is the whole span only where the run is the
  span's own and unhalved; a longer arc's rest goes to `wall_crossing`.
- **The side, decided here** (`reduce::junction_end_window`,
  `junction_end_side`). The end is read only on the stretch of the rim
  the junction was glued over (`rim_wedge::Touch`, carried on the
  junction's certificate). The departure is signed along the edge's own
  face's inward conormal. The margin is literal for a circle, and carries
  no charge: a root inside the spread lies on the other face's carrier,
  so only its side is asked, and that is the edge's conormal offset; the
  charge prices carrier distance, not side. Roots nearer the end than
  where the margin clears the band are band-coincident with the other
  face's boundary, not necessarily the vertex. An end where the glued
  stretch stops is read as on it: the other face does not hold the rim
  past the touch (the departures read it so, or the pair is not glued),
  so near the corner it lies on its own side of the rim's tangent.
- **For every junction the door reads, the rim is a line of curvature of
  both carriers**, so the relative curvature across it falls as `sin²β`
  with the departure angle, and at the cone–sphere rim's latitude `½`
  the end-root window refuses before the side check can. Near the
  sphere's equator it does not: at `h = 0.02` and `β` from about 3e-3 to
  8e-3 (and at `h = 0.05`) the window holds and the side refuses, on real
  rim data (the second review's scan). That refusal is conservative: the
  margin charges the full bends where the geodesic curvatures set the
  offset, filed as
  `the-junction-side-margin-charges-full-bends-and-refuses-a-held-side.md`.
- **End to end, near a neutral point**: order 4 on it, `JunctionEndOrder`
  in band, and a belt of `JunctionEndUnread { Window }` from about 2e-7
  to 2e-4 rad, filed as
  `the-junction-end-window-refuses-in-a-belt-around-a-neutral-point.md`.
- **Not built**: a G1 tube chain whose segment arc past 90° comes back
  through the other torus. Two coplanar rings tangent at the junction
  meet again only after one tube curls through the other's volume, and a
  second root off the face reads the same either way at union level.

## Closed (2026-11-24, HELDRECORDS orchestrator)

**A pipe unions with an elbow revolved off its own end cap.** The outward poses, the rod, turned axes up to ±1.22 rad and the hooked elbow build at their closed-form volumes and pass t3 and t3′ at three ε. It took a stack of three PRs, each dual-reviewed by two Opus reviewers and merged on local evidence (Ev, 2026-11-23):
- **PR 7432** (`9863cf938e`): the glue door reads a rim both faces end on as a `Junction`, glued only on Seam with Opposite departures and both rim edges `Crease{Convex}`; everything else takes main's path.
- **PR 7463** (`09be7534f2`): the crossing layer reads an edge ending on a glued junction by its end root (deflated, its order decided, its side decided on the glued stretch), refusing typed at a reflex corner.
- **PR 7471** (`fa133fb26b`): the seam's description is `TangentIntersection` only where |κ_rel| is certified along the edge; the stand-in is fixed at its minter; the rim routing reads an aligned in-band bend as the π seam.

**Residue, each on its own row:**
- inward poses (`tang/an-elbow-bent-back-into-its-pipe-meets-it-along-a-rim-and-a-crossing`, P1);
- the refusal belt near a neutral point (`the-junction-end-window-refuses-in-a-belt-around-a-neutral-point`, P1);
- the side margin's full bends (P2);
- the junction sector arms, which now refuse typed;
- n-ary rim junctions (P2);
- SELECT's drift P1, wedge-end P2 and in-band P2;
- ONECOPY's composite P1.
