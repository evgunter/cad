---
id: loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk
kind: issue
title: props, tier 3's check 6, the mesh walk and the boolean's sectors each walk a face's loop and derive a corner's turn on their own; nothing shares the walk
status: dispatched
opened: 2026-10-29
priority: P1
cost: H
design: true
branch: restcurve/surface-cycles
---


Found by FOREIGN's designer pair on `import-normalizes-the-rim-only-cap` and its probe (`probe/foreign-wraptip`, 2026-10-29). A face's loop is walked, and the turn at each corner read, separately in `geom_brep::props::curved` (`sphere_loop`, `cone_loop_winding`), tier 3's check 6 (which reads through the same props reader, so the two cannot disagree when that reader is wrong), `mesh::walk::loop_polygon`, and `topo::boolean::sectors` (`SlitTips`, `build_sectors`). Teaching the kernel one new corner (a same-edge reversal at a wrap tip) took four separate edits (the probe's E2, E4, E6, E7), and one of them, written by hand, gave a volume of 22.81 against a true 23.86 that passed tier 3 because validation and measurement share the reader.

This is the class two-implementations-of-one-logic (P1): one loop walk that reads each corner's turn once (the same-edge reversal by Ev's 2026-10-14 ruling, fork row 135, as `topo::chord_join`'s pole branch already does), which every reader cites. Owner undecided: the readers span PROPS (geom-brep props), TOPO (validate, boolean sectors) and TESS (mesh). Independent of `[ev]` PR 6014's pole decision; under the wrap tip it would be a precondition.

Signed: (FOREIGN orchestrator)

## First in the wrap-tip sequence (Ev, `[ev]` PR 6014, 2026-10-29)

Ev approved the wrap tip "if the complexity it adds is tolerable", asking that keeping it down be investigated. This row is that investigation's answer: one loop walk that reads each corner's turn once, the tip's included (a walk out and back along one edge contributes nothing to a loop integral: row 135's structural reading, which the designers expect to equal the probe's corrected slit-base turn; re-run the probe's tilted-sphere union and slab-minus-pip rows against it before any reader cites it), cited by props' sphere and cone loops (and so check 6), the mesh walk and the boolean sectors. The CARVE, TOPO, SPLITDOOR, TESSWALK and FOREIGN wrap-tip rows read the tip through it. If the walk cannot be made one home, say so on Ev's channel before the readers each learn the tip. Raised to the first step; still P1. (FOREIGN orchestrator)

## One more walker (RESTCURVE, PR 6054, 2026-10-30)

`geom_brep::chart_singularity::walk_loop` walks a face's loop through
its chart to read which side of it a pole or apex lies on (tier 3's
chart-singularity arm and step-import's normalization), for the loops
props' closed form does not read. It reads a corner only as the azimuth
step across it. Beside it, `chart_singularity::without_spurs` takes a
loop's out-and-back pairs out before either reading (a spur bounds no
area): the structural reading of a same-edge reversal this row's walk
would own. Both are this row's to absorb. (RESTCURVE implementer)

## Claimed by RESTCURVE (2026-10-30)

Moved from `work/issues/`. A face's loop read on its chart is RESTCURVE's charter: the face region off the plane. Two of RESTCURVE's rows wait on this one:
- `tier-3-admits-a-wrap-tip-only-on-its-faces-chart-singularity`, which is parked on it;
- `validate-tier3-curved-boundary-containment`, which asks the same question of a curved face's region.

PR 6054 added `geom_brep::chart_singularity::walk_loop` / `without_spurs` as one more walker. The two rows are weighed together by one designer pair before either is built. (RESTCURVE orchestrator)

## Spec, unit 1: the surface boundary (RESTCURVE orchestrator, 2026-10-30)

The designer pair on this row and on `validate-tier3-curved-boundary-containment` agreed on this unit in every round (fork record: the `[ev]` PR that follows, A/B only). This row's premise was "one walk that reads each corner's turn once", and both designers corrected it. **There is no turn at a wrap tip, or at a spur.** What every loop reader needs to share is structure: which edges the face has on BOTH sides. Row 135's principle applies ("a whole turn only from structure … an edge the face lies on both sides of").

Branch `restcurve/surface-cycles`. Review: dual tier. It changes what every curved integrator integrates over, which is broad. Difficulty M, so the arm is drawn at dispatch: byte 83, mod 3 = 2, SEQUENTIAL.

### The predicate and the cycles

1. **One structural predicate in `topo`:** the face lies on both sides of this edge, meaning both of its half-edges are in loops of this face. Seams, wrap edges and spurs are the one class. It reads half-edge identity only, with no tolerance and no geometry.
2. **`surface_cycles(face)`:** the face's loops with every such edge crossed to its mate and dropped. What remains is the cycles of edges that have another face on their far side: the face's boundary on its SURFACE. Define the empty case: a face whose every edge is such an edge, like the one-face ball, has an empty surface boundary. State in the docs what that means for each consumer.
3. **Cite the predicate from:**
   - `boolean::region::boundary`, whose seam filter becomes this predicate;
   - the props flattening (`topo::props::loop_edges`, or wherever the integrators' loops are built), so every integrator (sphere Gauss–Bonnet, cone azimuth sum, cylinder and torus Green forms, spiric and NURBS lanes) receives surface cycles;
   - `geom_brep::chart_singularity`, whose `without_spurs` and `retraces` retire. They re-derive this fact numerically from carrier midpoints. The home must receive structure from its caller; it cannot see keys, so pass the cycles in.
   - `SlitTips` / the boolean sectors only if a both-sides edge reaches them today; otherwise name it for the wrap-tip unit.
4. **`walk_loop` stays for now.** It retires in a later unit, once the side reading's instrument is ruled.

### What must hold, and must be measured rather than assumed

- **The loop integrals are unchanged on every body without a spur**, where seams and wrap edges cancel. Claim to check: the cylinder and torus Green forms read over surface cycles give flux enclosures that contain today's values on the corpus. This is the one place where dropping a seam changes an integrand's path. If a form is written in a lifted azimuth so that a seam term does not cancel, **stop and report**; do not adapt the form.
- **A row with an Euler-built spur** (a same-edge reversal in a face's loop) whose volume and area are read correctly. Pin it against an independent closed form. The FOREIGN probe (`probe/foreign-wraptip`, rows `tilted-sphere union` and `slab-minus-pip`) measured 22.81 against a true 23.86 for a hand-written cusp rule. Port its geometry if it can be built without the wrap-tip tier rules (no valence-1 exemption is added here). Otherwise say why not.
- Re-baseline anything that moves, and say why (implementer discipline §3).

### Drive-bys

- `crates/topo/src/pcurves.rs`'s module header (near "has no general lane yet and refuses lever-only 'not yet'") contradicts `not_owed`, which excuses only a dual scalar's fitted refusal. Correct the header. Add an `f64` row asserting that an odd cone circle either mints or refuses as a check-8 finding, and is never excused.
- **The wrap-tip incidence is read twice:** `singular_at` at mint, and `chart_singularity::locate`'s on-boundary decision at tier 3. File it on this slate as its own row. Do not fix it here.

## Re-spec, unit 1 (2026-10-30, after the implementer's stop)

The implementer's probe (`probe/restcurve-surface-cycles`, `crates/sweep/tests/zz_surface_cycles_probe.rs`) falsified the seam-cancel claim:
- The cylinder reads identical bits.
- The torus does not. `torus_chart` reads `−∮ F(v) du` in the lifted tube angle, so a dropped seam carries the whole area term. A `Partial` revolve then refuses, and a `Full` one reads flux 0 silently.

Both designers were shown the probe and chose **option (b)**: one predicate, two views.

- **The predicate** (unchanged): the face lies on both sides of the edge. It reads half-edge identity, in `topo`.
- **Surface boundary** (`surface_cycles`): drops every both-sides edge. Region questions and point placement read it: `boolean::region::boundary`'s seam filter, and later tier 3's region arm. An empty surface boundary is the whole surface.
- **Chart image** (spur-free loops): drops only spurs, meaning a consecutive same-edge reversal (`next(he)` is `he`'s mate). Seams stay. Removal repeats until no consecutive reversal remains, because a wrap edge split mid-way shows its inner reversal first.
  - Every integrator reads this view: sphere Gauss–Bonnet, cone azimuth sum, cylinder and torus Green forms, the spiric, NURBS and quadrature lanes. Every chart reader reads it too.
  - `chart_singularity`'s `without_spurs` and `retraces` retire into it. The caller passes the spur-free loops (for sides) alongside the full loops (for at-a-vertex or inside-an-edge).
- **The empty rule:** a chart image that is empty on a closed surface (the one-face ball, a meridian walked up and straight back) is the whole surface, signed by the face's sense. It is ONE rule that every integrator cites. It is never `flux 0`.
- **Rows:**
  - the probe's three bodies, each reading Pappus-correct over the chart image: the cylinder strut, the `Partial(1.25)` torus, and the `Full` torus;
  - the one-face ball over its empty chart image;
  - an Euler-built spur pinned against a closed form.
- **Out of scope:**
  - The mesher (`mesh::walk::loop_polygon`) takes the chart image with its reset at the tip. That lands in a later unit with the wrap tip, and is not needed now because no tip exists yet.
  - The drive-bys stand as specced.
