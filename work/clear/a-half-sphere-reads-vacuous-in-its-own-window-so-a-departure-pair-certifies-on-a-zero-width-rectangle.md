---
id: a-half-sphere-reads-vacuous-in-its-own-window-so-a-departure-pair-certifies-on-a-zero-width-rectangle
kind: issue
title: A revolved ball's half-band is presented in its own whole window as the zero-width rectangle u = π and read vacuous, so the departure arm certifies the two half-bands at the root without reading either
status: open
opened: 2026-11-13
priority: P0
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

## Re-banded P0 (CLEAR orchestrator, 2026-11-13)

A face read vacuous certifies any pair it is in, so a wrong vacuous
reading can certify a body that is not embedded. The at-rest gate
(P0) rests on the certificate's soundness, and a hole in it inherits
the gate's band. No wrong verdict is known yet.

## Trace (CLEAR lane clear-vacuous, 2026-11-13)

**The extent is wrong; `face_side` is right.** On the ball, both
half-bands carry the same extent, `u ∈ [0, π]`, `v ∈ [−π/2, π/2]`
(`Boundary::extent`, built by `extent_of` in
`crates/topo/src/clearance.rs`). Two halves of one sphere cannot both
lie on `[0, π]`.

`face_side` reads each face's two meridians consistently:
- `2v1`: the line `u = 0` has the face above it and `u = π` has it
  below, so the face is `[0, π]`, which agrees with its extent.
- `1v1`: the line `u = 0` has the face below it and `u = π` has it
  above, so the face is `[π, 2π]`.

Checked against the geometry, `1v1` runs its loop south to north
along the `u = 0` meridian (edge `1v1`, `plus = false`), then north
to south along `u = π`. That is a great circle in `z = 0`, running
counterclockwise about `+z`. Its left side about the outward normal
is the `z > 0` hemisphere. The sphere's chart has `v_ref = axis ×
u_ref = y × x = −z`, so `z > 0` is `u ∈ (π, 2π)`. `1v1` is `[π, 2π]`,
as `face_side` reads it, and its extent is on the other sheet.

**Why the extent is wrong.** `extent_of` takes the hull of the outer
loop's lifted rows, on the premise (stated on `Boundary::extent`) that
"its lift is one continuous chain, so on a periodic axis the hull is
on ONE sheet". `pcurves::loop_lift` says otherwise: "consecutive
curves of the chain meet at every joint but a reset (a pole, an
apex)", and a reset restarts the first channel. `1v1`'s loop is two
meridians, joined only through the two poles, so it is two chains
with nothing between them on `u`. Their hull, `[0, π]`, is the same
for the face and for its complement. It holds `2v1` by chance and
misses `1v1` entirely.

**What the wrong extent does in `present_chart`.**
- Whole window, `u ∈ [0, 2π]`: `within` clips `x` to the extent's
  copy `(0, π)`. The `u = π` line clips `XLo`, so `x = (π, π)`. The
  `u = 0` line goes on the `outside` list. Width 0 reads `Zero` at the
  sliver, giving `Vacuous`, which is the row's finding.
- **The guard the row names does not close the hole.** With a guard
  that refuses `Local` when the face's own clips (extent copy and
  lines) leave an axis no width, the root splits on `u`.
  - Cell `[π, 2π]` holds all of `1v1`. There `sheets` finds no copy
    of `[0, π]` overlapping it by more than `few_ulps`, so
    `Sheets::Missed` returns `Vacuous`.
  - Cell `[0, π]` reads `Vacuous` through `vacuous`, which is right
    there.
  - The pair certifies at depth 1 (receipt: 1 split, 2 certified
    across), still without reading `1v1`.

So every reading `present_chart` takes from the extent is wrong on a
loop whose lift resets: `Sheets::Missed`, the `within` clip (a
SUBSET rectangle, which is unsound for departure even when it does
not collapse), and the sliver. The same holds for
`present_line_corner`, which calls `sheets`.

**What a fix needs is a ruling.** How should the extent read a loop
whose lift resets? The options are below; the rows do not settle it.
1. **Unknown on the first channel.** The extent claims nothing on
   `u`, so `Sheet::Unknown` applies and a meridian has no placement.
   This is sound. Every face whose outer loop passes through a pole or
   an apex then refuses `Structural` on its meridians.
2. **Lift through the reset by orientation.** At a reset the
   collapsed side runs one way: on a positively oriented chart with
   the face to its left, it runs `−u` along the top (the north pole)
   and `+u` along the bottom. Sense and, on a cone, the nappe flip it.
   This places each chain on the face's sheet. Wrap tips (a seam kept
   to the pole, Ev's ruling on PR 6085) are the degenerate `Δ = 0 / 2π`
   case. ONEDOOR's `three-walks-of-a-loops-chart-images` already asks
   whether `chart_window`'s pole and apex rules or `loop_lift` are
   the one walk.
3. **Hull or its complement, chosen by the iso lines' face sides**,
   and `None` where neither agrees, or where any row on a reset loop
   is not an iso line.
