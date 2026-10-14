---
id: annulus-rim-host-outer-boundary-is-metered-only-by-the-sampled-screen
kind: issue
title: blend: an annulus rim's host outer boundary is metered only by predicate 2's sampled screen, which overestimates an off-sample gap
status: closed
opened: 2026-09-25
priority: P1
cost: D
closed: 2026-10-01
pr: 3715
branch: band/annulus-host-outer-metered
---


## Finding (the code's own statement; not re-measured here)

`ring_clearance_pass` (`crates/sweep/src/blend/surgery.rs`) meters a
LADDER rim's host outer boundary in closed form, but leaves an ANNULUS
rim's host outer boundary to predicate 2's sampled boundary-pair
screen. Its comment argues that is exact for coaxial latitude circles
and says it stops holding for a boundary whose closest approach falls
between samples — "a non-coaxial ring, or a trimmed wall. Such bodies
exist and are rowed (an off-axis bore's cap,
`review_ring_clearance_r1_probes`), so this is a live gap on the
boundary question". A sampled gap is never smaller than the true one,
so the screen can PASS a trim circle that reaches the boundary between
samples, and nothing exact backs it.

Found while sweeping for carves that move a face boundary without
metering the rest of that face (the ruled cut-off's cap gap,
`ruled-cut-off-leaves-a-cap-ring-inside-the-removed-sliver`).

## What the taker owes

Measure it: an annulus rim whose host's outer boundary carries a line
or circle edge whose closest approach to the trim circle falls between
the screen's `CHAIN_SAMPLES` stations, at a radius where the true gap is
negative and the sampled one positive. If the carve returns a body,
add the closed-form walk the ladder already has (restricted to the
boundary edges the trim does NOT replace).

**Sibling, same screen.** The LADDER walk skips an outer-boundary edge
whose carrier is neither a line nor a circle (or is uncertified), and
leaves that pair to the same sampled screen alone — so a NURBS boundary
edge of a ladder host carries the same one-sided gap.

## Measured (`band/annulus-host-outer-metered`)

The finding holds, and on more than the host. On main before this
branch, each of these passed predicate 2's screen and carved a
tier-3-valid body whose band spans the cut (rows in
`crates/sweep/tests/band_annulus_host_boundary.rs`):

- a revolved washer notched from the bore side, the outer rim's plane
  host carrying the notch's corners past its trim at `−0.074`;
- the same washer notched from outside, the bore rim's host carrying
  the notch inside its trim at `−0.0115` (the trim lies OUTSIDE that
  rim, so the near reach decides, not the far);
- a cone–cylinder shaft whose cylinder HOST a 45° cut leaves an ellipse
  `0.0064` past its trim;
- the same shaft with the cylinder as the rim's MATE: the mate's outer
  boundary was metered by the screen alone too, on ladder and annulus
  rims both.

The ladder walk's skipped carriers are reachable: a tilted cut leaves a
cylinder support an `Ellipse` edge, and a STEP import can leave a plane
support a NURBS one.

## Closed by (`band/annulus-host-outer-metered`)

`ring_clearance_pass`'s arm (b) calls `support_boundary_clearance`,
which walks every distinct host AND mate face of every closed rim,
ladder or annulus, and meters each outer-cycle edge the carve does not
replace (the rim's arcs, and the seam or meridian each support drops
into a rim vertex) against that face's trim under
`fillet3_ring_clearance`: the distance from the trim centre on a plane,
the height along the axis on a cylinder, cone or sphere, decided on the
trim's far side from the rim. Lines and circles go through
`piece_distance` / `piece_along`; an ellipse, spiric or NURBS edge
through `boxed_reach`, a certified bound (the piece's box, and an
ellipse's exact height range), and a refusal it decides carries
`bounded: true`
(`support-boundary-meter-bounds-other-carriers-by-the-whole-carrier`);
an uncertified edge refuses typed. Nothing is skipped.
