---
id: a-rim-arcs-pad-is-its-widest-certifying-rung
kind: issue
title: A spline x implicit rim arc's pad is its chart window's whole image, so two arcs within about two rungs carry neither
status: open
opened: 2026-11-12
priority: P1
cost: M
---



Filed by the `sectarms/join-reads-spline-implicit` lane; rewritten by
its fix pass with the true cause.

## What

A rim arc's pad (`CarriedComponent::pad`, `spline_implicit::pad_of`)
bounds every point of the chart tube's windows, imaged on the wall,
from the carrier. The windows are the wall pcurve's hull over each of
its spans, padded by `pad_u` and `pad_v`
(`ssi::certify::chart_tube_windows`), and limb 3 holds the section in
them one arc. A pcurve of one span gives one window over the whole
arc.

As first read, the pad imaged each window whole, halved its box three
times and read the parts against 9 carrier samples over the window's
span, so it floored near an eighth of the window's span, not near the
rung: on the carry rows' wall against a rod along `x` of radius `0.07`
(`s = 1`), the rung is `0.0444` m and the pad read `0.254` m. Each of
two arcs' pads then reached across to the other, and a site on either
read on both: a desync. The witness:
`loft_prism_at(&[0.0, 0.2, 0.4])` ∪ `ball_poled_z(0.21, (0.3, 0.95,
0.2))` aborted `JoinDesync { "a site lies on two components of its
section" }` in union, subtract and swapped union, at the default ε and
at 1e-12; on the carry rows' wall, rods of radius `0.07·s` and
`0.10·s` desynced each arc's own witness at `s = 0.01, 1, 10`.

The fix pass halves each window into cells until each cell's image is
no wider than two rungs, each imaged on its own and read against eight
carrier points per cell (`spline_implicit::PAD_CELL_RUNGS`), so the
pad now reads about 1.1 rungs on those walls, and the carry refuses
`CarryRefusal::PadsMeet` where two fitted arcs' carriers stand no
farther apart than both pads and both bands (`spline_implicit::apart`).
The prism and ball now refuse `GermSectionUnbuilt` naming `PadsMeet`
(pads `0.092` m, gap `0.080` m).

What remains is valid input refused:

- **The window is a whole span's hull.** The pad is at least that
  hull's width across the carrier imaged on the wall. For a pcurve of
  one span that bends in the chart, that is the arc's whole bend,
  however narrow the rung.
- **Two arcs within about two rungs carry neither.** On the carry rows'
  wall at `s = 1`, rods of radius `0.09` and `0.095` refuse `PadsMeet`
  (radius `0.09`: arcs `0.180` m apart, pads `0.098` m at the rung
  `0.089` m), and the prism and ball above refuse.
- **The reach is half the arc**: the witness's reach is the farthest
  its window's image stands from it, so for one window about half the
  wall across.

## Remedy

Windows over sub-spans: refine the pcurve and read limb 3's one-arc
proof over the narrower windows (a sub-arc spanning the same end
slices is the arc), or certify a narrower chain after the wide one, so
each window's image is about its rung across. PR 6829's per-box
narrowing toward a node is the same move for node arcs.

## Done when

Two rim arcs 1.2 rungs apart are carried, each point reading on its
own arc alone, and a bending one-span arc's pad stands within two
rungs.
