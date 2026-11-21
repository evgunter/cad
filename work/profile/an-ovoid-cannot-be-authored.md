---
id: an-ovoid-cannot-be-authored
kind: issue
title: An ovoid cannot be authored: a profile has no ellipse arc to revolve, and a body cannot be scaled non-uniformly
status: open
opened: 2026-10-25
priority: P1
cost: M
design: true
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
