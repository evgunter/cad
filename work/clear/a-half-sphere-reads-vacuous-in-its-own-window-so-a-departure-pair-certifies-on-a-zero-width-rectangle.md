---
id: a-half-sphere-reads-vacuous-in-its-own-window-so-a-departure-pair-certifies-on-a-zero-width-rectangle
kind: issue
title: A revolved ball's half-band is presented in its own whole window as the zero-width rectangle u = π and read vacuous, so the departure arm certifies the two half-bands at the root without reading either
status: open
opened: 2026-11-13
priority: P0
cost: M
blocked_on: [three-walks-of-a-loops-chart-images-each-claim-to-be-the-one]
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
4. **The stored rows carry the sheet.** Across a reset the hull is
   right only if each chain's stored image already sits on the face's
   sheet. That makes it an invariant of the stored rows (the revolve
   door's, and every door's that writes rows), checked where the lift
   is read.

**Measured (debug, `self_intersection_proves_injectivity`, 44 rows).**
- **The guard alone:** 44 of 44 pass. The ball still certifies, at
  depth 1, through `Sheets::Missed`, so the guard cannot be pinned red
  on this ball by itself.
- **Option 1** (sheet `Unknown` on `u` for any face with a vertex at a
  pole or an apex, a hack): 3 rows go red, each refusing `Uncertified`
  departure.
  - `a_revolved_cone_holds`;
  - `a_two_ball_union_holds_with_its_poles_read`;
  - `a_filleted_corners_ball_holds_against_a_face_at_its_pole`.
- In all of the cone's and the snowman's faces, each face's own
  meridians agree with its extent: the cone's are `[0, π]` and
  `[π, 2π]`, the snowman's `[0, π]` twice, `[π, 2π]` and `[−π, 0]`.
  Their stored rows sit on the face's sheet, so neither holds through
  a wrong `Vacuous`. Option 1 refuses them anyway, which is the cost
  of refusing every reset loop. Only the freshly revolved ball's `1v1`
  carries a row off its sheet: its `u = 0` seam meridian, where the
  sheet needs `u = 2π`.

## Already ruled (Ev, PR 6851): the lift has no reset (CLEAR orchestrator, 2026-11-13)

The question the trace stopped on, how the extent reads a loop whose
lift resets at a pole or an apex, is settled. Ev approved fork row 262
on PR 6851: a loop's position is its lift, `decide_joint` is the one
decision, and "i don't want to keep reset". At a pole, the joint
stores the pole rule's decided face-side element, and
`JointElement::Reset` retires. So a loop's lift is the face's own
boundary, and `extent_of`'s hull of the lifted rows is on the face's
sheet. That is the trace's option 2. No new fork is needed.

ONEDOOR implements it: `work/onedoor/three-walks-of-a-loops-chart-images-each-claim-to-be-the-one.md`,
PR 6900 ("loop lift: one decision for a loop's chart lift, with no
reset"). This row waits on it.

**Once it lands:**
1. Re-run the trace on the ball. `1v1`'s extent should read
   `[π, 2π]`.
2. Build the row's guard. A face's own clips leaving an axis no width
   is a contradiction, and refuses `Local`.
3. Re-check each `Vacuous` path the sweep marked a hit or suspect:
   - `Sheets::Missed`;
   - `vacuous()`;
   - the two slivers;
   - `present_plane_corner`'s sliver.
4. Pin and mutant as the row says.

If the extent is still off its sheet after the lift lands, that is a
finding against the lift unit, and goes on its row.

**Interim:** no wrong verdict is known. The only measured off-sheet
row is the freshly revolved ball's seam meridian, and that body is
embedded. A sound stopgap exists (the trace's option 1: `u` unknown
for a face with a vertex at a pole or apex), but it refuses three
holding pins, among them PR 6892's and PR 6894's. It is not taken,
because the lift fix is in flight.

**The sliver and the band clause.** The sliver reads a strip inside
the band as holding nothing, while DESIGN's finished-body clause says
a reading inside the band refuses as ambiguous. Filed separately
(`the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause`).
