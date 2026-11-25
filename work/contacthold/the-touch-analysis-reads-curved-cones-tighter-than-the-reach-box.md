---
id: the-touch-analysis-reads-curved-cones-tighter-than-the-reach-box
kind: issue
title: The census touch analysis reads a curved star face only through its reach box; reading curved cones would be tighter (fewer false refusals of true curved rests), and stays owed after the box reading lands
status: closed
opened: 2026-09-29
priority: P3
cost: H
closed: 2026-11-25
---


Filed on Ev's ruling on PR 3422 (2026-09-29). Ev said: "it sounds like
the cone may be tighter and hence something we'd want later; if so,
adding it as a row would be good."

## What

Under C1, a curved face in a touch's star enters the analysis as the
corners of its certified reach box (`face_reach`). A box on its side
of the candidate plane certifies a Rest. A box crossing the plane
refuses, and never decides a crossing. This is sound but loose.

A true curved rest refuses whenever the box pokes through the plane.
Examples:
- a tilted curved seat;
- a conformal cradle, for which no separating plane exists at all.

Reading the curved face's own cone at the touch point, with a
second-order or sagitta-bounded reading of the face over its extent,
would certify those.

## Owed

Neither designer on PR 3422 measured how loose the box is, so the
first step is a measurement:
- how many ordinary curved assemblies refuse under the box reading;
- which of them a cone reading would certify.

The designers agreed on one constraint. A curved face can dip between
its vertices, so any tighter reading still has to bound the face's
whole extent, not only its cone at the point.

## Closed

Closed 2026-11-25 as **obsolete** (its premise is gone), in the CONTACTHOLD cleanup (`docs/doc-ledger/contacthold-leaves-the-tracker.md`), against main at `6003877ad`. The premise was "a curved star face enters the touch analysis as its reach box's corners". It no longer holds. `crates/topo/src/link.rs:64-79`: a link holds planar faces only; a curved face refuses `CurvedUnread::Unreadable`; a sphere or cylinder is read by its carrier (`Round`); a self-touch is read by tangent cones (`FaceSector::tangent`, `second_order`). `rest::judge` (link.rs:4304) reads no `face_reach`. The cone and second-order reading the row wanted now exists, for self-touch only. If two-solid touches on cone, torus or spline faces (now an `Unreadable` refusal) need a lane, file a fresh row for that against link.rs:64-79 (ATREST), rather than keep this C1-era row.
