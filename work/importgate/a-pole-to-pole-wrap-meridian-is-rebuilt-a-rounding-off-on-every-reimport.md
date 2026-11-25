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

A ball revolved about the sketch y axis, then turned 0.7 rad about
`(1, 2, 3)`: one face, one wrap edge from pole to pole. Export, then
import and re-export over and over. The wrap meridian's `CIRCLE` frame
(its normal and its reference direction) moves by an ulp or two in some
component on every hop and does not settle. Census and volume hold. On
coordinate axes it is byte-stable, and so is a tilted cone, whose seam
runs from a rim vertex.

The cause, read: both ends of a ball's wrap edge stand on the axis, so
`build.rs`'s `deferred_carrier` builds "the generator through the
printed middle's own foot on the surface", the azimuth being free. The
middle is read off the printed circle, and the meridian is rebuilt
through its foot. Rebuilding a printed meridian is not idempotent in
f64, so each hop's print chooses the next hop's model a rounding away.
The reviewer saw period 2 on one shape; the probe here saw a walk.

## What would close it

A generator rebuilt through a point of a printed generator returns that
generator. For instance: where the printed carrier already reads as the
surface's generator at the file's band, keep its datum (a canonical sign
and rounding for the normal and reference, read from the printed
values), not a rebuild through its middle. Then a tilted ball's
re-export is byte-stable, and `a_revolved_pole_or_apex_imports_as_built`
can hold a tilted case's text.
