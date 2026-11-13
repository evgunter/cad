---
id: a-half-sphere-reads-vacuous-in-its-own-window-so-a-departure-pair-certifies-on-a-zero-width-rectangle
kind: issue
title: A revolved ball's half-band is presented in its own whole window as the zero-width rectangle u = π and read vacuous, so the departure arm certifies the two half-bands at the root without reading either
status: open
opened: 2026-11-13
priority: P1
cost: M
---


## Finding

From CLEAR's lane clear-pairs, while it traced
`the-certificate-runs-out-of-pairs-on-eight-finished-tour-bodies`.
The ball `revolved_about_y([(0, −1), bulge 1], [(0, 1)], Full)`
(`sweep::test_support`) is two half-bands of one sphere, `1v1` and
`2v1`. The injectivity question holds it at the root: 3 pairs, one
certified across. That certificate rests on a wrong reading.

`departure` presents `1v1` in its whole window (`u` over a turn, `v`
over the latitude range) as `Presentation::Vacuous`, from the sliver
rule in `present_chart` (`crates/topo/src/injective.rs`):
- the face's extent is `u ∈ [0, π]`, so the rectangle starts as
  `x = (0, π)`;
- the meridian at `u = π` is clipped as `XLo` (the face read ABOVE
  it), and the meridian at `u = 0` is read as a line the cell lies
  outside of;
- so `x = (π, π)`, the side's width across is 0, the sliver reading
  decides it `Zero`, and the face is read as holding no point of the
  cell.

The face holds a whole half-sphere in that cell. Either `face_side`
reads both meridians' sides opposite to the extent, or the extent is
on the other sheet; which one was not traced. A vacuous face
certifies any pair at once (`departure`: either side `Vacuous` →
`Certified`), so a body whose half-band truly met another face near
its seam could be read `Holds` here. The ball itself is embedded, so
no wrong verdict is known.

## What a fix needs

Trace which of the two readings is wrong on this ball, and add the
check that a presentation's rectangle that collapses to zero width
from a face's own clips is a contradiction (refuse `Local`), not a
sliver. Pin: the ball's pair certifies through a presented rectangle,
or splits, but never reads `1v1` vacuous in its own window.
