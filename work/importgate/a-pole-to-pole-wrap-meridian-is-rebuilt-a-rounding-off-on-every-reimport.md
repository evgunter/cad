---
id: a-pole-to-pole-wrap-meridian-is-rebuilt-a-rounding-off-on-every-reimport
kind: issue
title: A pole-to-pole wrap meridian is rebuilt from its printed middle on every import and walks by an ulp: a tilted ball is no byte fixed point
status: open
opened: 2026-11-25
---


Found by the dual review of `lathe/import-wrap-tip` (PR 7774, R2's
MINOR-1), reproduced there. It is distinct from
`a-free-vertexs-foot-creeps-on-every-reimport.md`, which moves a
vertex, not a carrier.

## What

Two shapes, neither of them a byte fixed point on re-export. Census and
volume hold in both.

- **A tilted ball.** Revolved about the sketch y axis, then turned 0.7
  rad about `(1, 2, 3)`: one face, one wrap edge from pole to pole.
  Export, then import and re-export over and over. The wrap meridian's
  `CIRCLE` frame (its normal and its reference direction) moves by an ulp
  or two in some component on every hop and does not settle: a walk.
  R2's tilted ball is period 2 in the circle's reference direction.
- **A tilted cone.** R2's cone, revolved about the axis `(sin 0.3,
  cos 0.3)` through `(0.1, 0.2)`: its seam generator's `LINE` direction
  alternates every hop, period 2. (The lane's own probe, a cone turned
  0.7 rad about `(1, 2, 3)`, settled after one hop, so the cone is not
  always a cycle.)

On coordinate axes both are byte-stable.

The cause is read for the ball only: both ends of its wrap edge stand on
the axis, so `build.rs`'s `deferred_carrier` builds "the generator
through the printed middle's own foot on the surface", the azimuth being
free. The middle is read off the printed circle and the meridian rebuilt
through its foot, which is not idempotent in f64, so each hop's print
chooses the next hop's model a rounding away. That does not explain the
cone, whose seam runs from a rim vertex, which names its generator. The
cone's cause is open: a generator rebuilt through a built end is
evidently not idempotent either, on some axes.

## What would close it

A generator rebuilt through a point of a printed generator returns that
generator. For instance: where the printed carrier already reads as the
surface's generator at the file's band, keep its datum (a canonical sign
and rounding for the normal and reference, read from the printed
values), not a rebuild through its middle. Then a tilted ball's and a
tilted cone's re-export are byte-stable, and `a_revolved_pole_or_apex_imports_as_built`
can hold a tilted case's text.
