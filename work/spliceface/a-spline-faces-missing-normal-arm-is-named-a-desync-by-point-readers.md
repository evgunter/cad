---
id: a-spline-faces-missing-normal-arm-is-named-a-desync-by-point-readers
kind: issue
title: Readers of face_outward_normal_at that hold no chart point name a spline face's missing arm (Ok(None)) as a desync or an invariant, not as the kind it is
status: open
opened: 2026-10-31
priority: P3
cost: M
---


Filed by `splice/a-vertex-piercing-a-spline-face-reads-no-outward-normal`'s
lane, from its sweep.

## What

`face_normal::face_outward_normal_at` answers `Ok(None)` for two
different things: a face key that no longer resolves, and a `Nurbs` or
`Approx` face. A spline face has no implicit form, and a point alone
cannot say which sheet to read. Its arm is
`face_normal::spline_outward_normal_at`, which needs the chart point the
caller placed the point at. Several readers hold no chart point, and
they map `Ok(None)` to the stale-key reading. On a spline face that is
a misnamed refusal (fail-loud, but in the wrong category).

These read `Ok(None)` as stale (as of `origin/splice/spline-span-curved-crossing`):
- `boolean::reduce::sheets_part`: `JoinDesync` "a face a crossing reads
  does not resolve";
- `boolean::join::holding_face_normal`: `JoinDesync` "the face holding a
  section chord no longer resolves";
- `boolean::slit` (the edge-in-face gap's `normal_at`): `JoinDesync`
  "an edge lying in a face does not resolve";
- `boolean::fate` (a fate cut's corner): `ClassificationInvariant` "a
  fate cut's corner face has no normal at its vertex";
- `boolean::vtxfac::mint_node`: `ClassificationInvariant` "a node's face
  has no outward normal". A node is an equal cylinder pair's, so this
  one is unreachable for a spline face today.

Of the same class, through the poisoned implicit gradient rather than
`Ok(None)`: `splitting::join::leave_on` reads
`geom_brep::implicit_outward_normal` on the wall and names a poisoned
normal `SectionInvariant` "a null-edge half at a point where its wall
has no normal", which is the cone-apex reading.

Each of these sites holds a face it resolved itself, so its stale arm is
a real invariant. What it lacks is a separate arm for the kind. None of
them is known to be reached by a spline face today.

## What it needs

At each site, a spline face either reads its normal at a chart point
the site places on the face's own sheet (a pcurve, or a meeting), or it
refuses by its kind (`CurvedBooleanUnsupported` or the site's own typed
frontier). Its stale-key arm stays a desync. One way is to split the
door's `Ok(None)` into "stale" and "no arm at a point".
