---
id: an-ovoid-cannot-be-authored
kind: issue
title: An ovoid cannot be authored: a profile has no ellipse arc to revolve, and a body cannot be scaled non-uniformly
status: parked
opened: 2026-10-25
priority: P1
cost: M
blocked_on: [the-lily-bud-is-an-ovoid-and-wall-4-is-deleted, the-egg-and-the-elliptic-prism-meet-the-bar]
---


Banded P1 on request in chat (2026-10-25), from lily wall 4.

## What

A lily bud is an ovoid, a sphere stretched along its own axis, and the
kernel has no way to say one:

- **No ellipse arc in a profile.** `crates/profile/src` has no ellipse
  segment (`git grep -i ellipse -- crates/profile/src` finds nothing),
  so the natural construction, revolving an elliptical meridian arc,
  cannot be sketched. The carrier exists: `Curve3::Ellipse` is a curve
  kind the boolean's operand gate admits (`crates/topo/src/boolean/reduce.rs`,
  `gate_operand_edges`).
- **No surface for the result.** `SurfaceKind`
  (`crates/geom/src/surfaces.rs:291`) has plane, cylinder, cone,
  sphere, torus and NURBS and no spheroid, so a revolved ellipse would
  be a rational NURBS wall, unless a spheroid kind is added.
- **No non-uniform map.** A body moves only by a `RigidMotion`.
  `RigidMotion::check_rigid` refuses a scale of 1.6 along one axis
  `NotRigid { check: "transform_rigid_col2_unit" }`. This is lily wall 4
  (`demos/tour/src/lily.rs:2087`), whose retirement instruction is
  "model buds as spheroids".

## The ask

The everyday route is the profile one: an ellipse arc, or a spline
segment, in a profile, revolvable like any arc. Non-uniform scaling of
a whole body is a secondary route, and lower priority. Other kernels
support it mostly by converting the stretched surfaces to NURBS.

## The design question

What surface a revolved ellipse arc becomes: a new exact spheroid kind
(every face-pair table then owes it a row), or the rational NURBS
surface of revolution (which the boolean meets as `Nurbs`, behind the
operand edge gate `splice/delete-the-boolean-operand-edge-gate`).

## Done when

The lily's bud can be authored as an ovoid, and wall 4 is retired or
re-aimed at the non-uniform scale it pins.

## Weighed (2026-11-21)

Two designers, one round, agreed (fork log row 298). Corrections to
this row:

- **The bud is a stretched meridian, not a stretched body.** It is
  three partial revolves on a tilted tripod of axes. Stretching a
  revolve along its own axis is revolving the stretched meridian: the
  belly's circular arc becomes an ellipse arc centred on the axis. A
  whole-body stretch of the tripod would not give three ovoid tepals,
  so the "secondary route" above is a different shape, and
  `RigidMotion` refusing a stretch is right.
- **"Spheroid kind or NURBS" is not an either-or.** A revolved ellipse
  arc is a spheroid only when centred on the axis and aligned with it.
  Off the axis it is a quartic surface, and extruded it is an elliptic
  cylinder, so the exact NURBS wall is needed whatever the spheroid
  answer.
- **The operand edge gate cited above was deleted on 2026-10-25**
  (`work/splice/delete-the-boolean-operand-edge-gate.md`, closed). A
  curved NURBS face meets typed spline lanes at the boolean; it is not
  refused outright.
- **The spline segment is not new design.** `docs/PATHS-DESIGN.md`
  specifies the NURBS legs; the profile README's VQ7 records only that
  the segment layer has nothing to lower them to.
- **"Done when" above measures a demo.** The bars the designers set are
  the kernel's, below.

The agreed final state, in three parts that can land in this order:

1. An ellipse-arc segment in the profile, its own kind, lifting to
   `Curve3::Ellipse`; authored as `ellipse_to(Stretched { c, along,
   factor, winding, p })`, the circle about `c` through the tip
   stretched by `factor` along `along`.
2. A `Spheroid` surface kind, minted by the revolve under two margined
   verdicts (centre on the axis; a principal direction along it).
3. The spline leg PATHS already specifies, and the exact rational
   `Nurbs` wall for every sweep with no kind (an ellipse arc off the
   axis or extruded; any spline leg).

Bar for parts 1 and 2: an axis-centred ellipse-arc revolve builds full
and partial, validates at rest, matches 4/3·π·a²·c inside its certified
bound, tessellates within its theorem, writes exact STEP, replays at
every lane scalar, and is cut by a plane; every other door refuses
typed. Bar for part 3: the same list for an egg and an elliptic prism.
Wall 4 is then deleted, not re-aimed.

## Moved (2026-11-22)

From `work/profile/` to OVOID by `git mv`, id kept, when the build was
cut (below). Parked on the last unit of each program.

## Cut (2026-11-22)

Ruled on PR 7289 (2026-11-21). Cut into two programs at the seam
between the analytic lanes and the spline lanes: OVOID holds parts 1
and 2, SPLINEWALL part 3. Each unit's body is its spec.

**OVOID** (`work/ovoid/plan.md`):

1. `a-spheroid-is-a-surface-kind-that-nothing-mints-yet` (H, open)
2. `a-profile-segment-is-an-ellipse-arc` (H, open)
3. `ellipse-to-draws-the-stretched-circle-from-a-point-tip` (H, on 2)
4. `profile-validation-reads-an-ellipse-arcs-pairs-on-the-planar-reader`
   (M, on 2 and on PROFILE's planar-reader build,
   `check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector`)
5. `the-revolve-mints-a-spheroid-from-an-ellipse-arc-centred-on-its-axis`
   (H, on 1 and 4)
6. `a-spheroid-face-tessellates-within-its-own-sizing-theorem` (M, on 5)
7. `a-spheroid-face-measures-its-closed-form-volume-and-area` (M, on 5)
8. `a-spheroid-writes-exact-step-as-a-surface-of-revolution` (E, on 5)
9. `a-plane-cuts-a-spheroid-by-pulling-the-plane-back-through-the-stretch`
   (H, on 5)
10. `the-lily-bud-is-an-ovoid-and-wall-4-is-deleted` (M, on 3, 6 and 8)

**SPLINEWALL** (`work/splinewall/plan.md`):

1. `measure-what-an-exact-nurbs-wall-of-revolution-does-today` (M, open)
2. `a-profile-segment-is-a-rational-spline-and-the-nurbs-legs-lower-to-it`
   (H, on OVOID 2 and the planar-reader build)
3. `the-revolve-mints-the-exact-nurbs-wall-where-no-kind-fits` (H, on 1
   and OVOID 4)
4. `extrude-loft-and-sweep-mint-the-wall-of-an-ellipse-or-spline-segment`
   (H, `design`: the elliptic cylinder; on 1 and OVOID 2)
5. `the-egg-and-the-elliptic-prism-meet-the-bar` (X, 24; re-cut after 1)

**The ellipse pair rows are the planar reader's.** Ev's planar-readings
ruling of the same day (PR 7290) gives profile one planar meeting
reader in `geom-brep`, whose table already routes line × ellipse,
circle × ellipse and ellipse × ellipse. So OVOID writes no pair reader:
unit 4 wires the segment to the reader. That also settles where the
designers' "quartic sign ladder in `geom-brep`, above `profile`" goes:
nowhere, since profile reads it through `geom-brep`. And the table
reads two coplanar ellipses by certified subdivision
(`curve_pair::pose::coplanar_ellipses`), not by a quartic closed form.

Filed beside the cut: ONECOPY's
`an-ellipse-with-equal-semi-axes-is-a-second-circle` and EXCH's
`a-surface-of-revolution-is-refused-at-import`.
