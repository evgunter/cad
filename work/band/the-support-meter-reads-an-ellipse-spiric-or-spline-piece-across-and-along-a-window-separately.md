---
id: the-support-meter-reads-an-ellipse-spiric-or-spline-piece-across-and-along-a-window-separately
kind: issue
title: blend: the support-boundary meter reads an ellipse, spiric or NURBS piece by its boxed ranges across a link's strip and along each window half-space separately, so a tilted bore's cap ring beside a strip refuses though it clears
status: open
opened: 2026-11-13
priority: P2
cost: M
---


Filed by the band-meter lane (2026-11-13), from the reviews of PR 6887,
the sibling its sweep missed.

`StripLevel::wedge_margin` (`crates/sweep/src/blend/surgery.rs`) clips
a line or circle piece to each wedge of a link's window
(`StripLevel::within`, through `Clippable::of`) and reads the level over
the parts inside. Any other carrier — an ellipse, a spiric or a NURBS
piece — has no `Clippable`, and `wedge_margin` returns its whole read:
the piece's boxed range across the strip (`boxed_reach`) and its range
along each window half-space, each read separately. A piece that
passes a window's edge reads neither range clear, as circles did
before PR 6887.

Witness (demonstrated by review): the rounded plate `rect(1, 0.6,
[0.3; 4])`, 0.5 thick, with only its outline rims filleted at 0.05,
and a bore of radius 0.1 at (0.849, 0.25) tilted 0.02 rad, so its cap
rings are ellipses. It refuses `RingClearance` at −0.0040, though the
ring clears the right link's strip by about 0.001. Tilts 0.05 and 0.1
refuse at −0.00013 and −0.0082. The untilted bore, whose rings are
circles, builds.

The fix is the same clip for a conic: an ellipse against a half-plane
is an arc of its parameter, `(p − o)·u = k + A·cos(t − φ)` over the
ellipse's frame as for the circle, so `arc_within` generalises with the
ellipse's own `A` and `φ`, and `piece_along` / `piece_distance`'s
ellipse arms read the parts (the distance arm is a bound there, not the
extremes). A spiric or NURBS piece needs a certified subdivision.
