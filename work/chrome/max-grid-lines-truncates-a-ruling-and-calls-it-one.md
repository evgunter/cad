---
id: max-grid-lines-truncates-a-ruling-and-calls-it-one
kind: issue
title: MAX_GRID_LINES returns a truncated ruling in the shape of a complete one
status: dispatched
opened: 2026-09-15
priority: P3
cost: E
branch: chrome/datum-honesty
---


## Finding

`crates/viewer/src/datums.rs`, `rule_patch`'s `rule` closure:

```rust
let count = ((last - first) as usize)
    .saturating_add(1)
    .min(MAX_GRID_LINES);
```

**`MAX_GRID_LINES` is 512, not 96** — measured against the tree on
2026-09-21. The defect is unchanged; the reachability arithmetic below
was taken at the wrong number and is re-derived at the end of this
row. When the patch genuinely holds more lattice
lines than the cap, the closure rules that many of them and returns — and the
caller receives a ruling indistinguishable from a complete one. There
is no flag, no count, no shortened patch: a partial grid in the shape
of a whole one, which is the class the sweep that closed
`inclusive-rule-range-draws-a-line-on-a-nan-count` and
`metres-per-pixel-swallows-a-nan-depth` was sent to close, at a site
that sweep read as a backstop and left standing.

The const's own doc says why it reads as a backstop: *"Not a budget
the design expects to spend"*. That argument is
about REACHABILITY and not about honesty: the cap fires exactly when
the arithmetic has gone somewhere the design did not plan for, which
is the case where the caller most needs to know the answer is partial.

## What the disposition might be

Three shapes, and choosing between them is the work:

- **Refuse.** A patch that cannot be ruled inside the backstop is a
  patch this view cannot rule, and the module's answer everywhere else
  to "I cannot compute this" is to draw nothing. Consistent, and
  costs a grid at exactly the views where a coarse grid might still
  orient a reader.
- **Rule what fits and say so**, by shrinking the patch to what 96
  lines cover rather than truncating the line list — the drawing is
  then complete for a smaller patch, which is a true statement.
- **Keep it and argue it at the site**, the way `grid_pitch`'s
  rustdoc argues its refusal: name the consumer, weigh a truncated
  grid against no grid. The project's standard is not "never
  substitute"; it is "weigh the two failures where you do".

## What is asserted about it today

Nothing. No row in `crates/viewer/tests/datum_draw.rs` drove a patch
past the cap in any direction, so it had never been observed firing.

## Fence

`crates/viewer/src/datums.rs` — CHROME's and VIEW's by the
territories table; the change is CHROME's alone.

## Reachability, re-derived at 512

Measured by driving `datums::draws` at a plane seen half a degree
above its own surface — an orbit position, not a pathology — with the
window width varied and everything else the suite's own `view_at`:

| window | lines ruled across |
| --- | --- |
| 1280 px | 109 |
| 3840 px | 323 |
| 7680 px | 512, capped (643 asked for) |

**The closed form**, which is what belongs in the const's doc rather
than three data points. Across the view: the region is cut off at the
depth where a cell spans `MIN_CELL_PX`, and the frustum is
`width_px / 2` pixels wide to each side of the view direction at any
depth, so the region holds about `width_px / (2 * MIN_CELL_PX)` cells
across and the cap is crossed at about `2 * MIN_CELL_PX *
MAX_GRID_LINES` pixels — 12288 nominal, about half that measured
because the grazing region is wider than the head-on one at the same
depth. Along the view the region runs to the cut-off itself,
`1 / (MIN_CELL_PX * metres_per_pixel_at_one_metre)` metres from the
eye, which is why a TALL pane reaches the cap down the other axis.

So the cap is crossed from an ordinary seat at a window wider than
about 6100 pixels — an 8K display, which exists — and a tall pane
crosses it the other way at about 6900. It is NOT dead at every
ordinary view, and the const's own doc was already carrying the
right order of magnitude for the grazing case (*"around 150 on a
1280-pixel window"*) while the row's 26 came from the head-on one.

The const's doc said so too, and that sentence has been rewritten:
*"Not a budget the design expects to spend… a backstop for the
arithmetic going wrong at an extreme"* is what let the previous sweep
read this site as unreachable and leave it standing, and leaving it
in place beside the measurement above would have left the next reader
the same trap.

## Disposition: shrink the patch

The second of the three. `rule_patch` now bounds BOTH directions
once, up front, and shrinks each index span to the cap before either
family is ruled (`capped_span`) — so a capped drawing is a complete
ruling of a smaller rectangle, and the family crossing the capped
direction is shortened with it. The cap is now a bound on the PATCH,
which is a true statement the caller can see the edges of, rather
than a bound on the line list, which was not.

**Shrunk toward the AIM, not toward the region's midpoint.** The
first cut of this centred the surviving span on the region, on the
premise that the region is centred on what the camera is pointed at.
Measured, it is not, and never is in the grazing view that is the
only one reaching the cap: the region's near edge is where the bottom
of the window lands and its far edge is the cut-off toward the
horizon. Centred on that midpoint, the surviving patch walks away
from the aim as the window grows — past a pane about 6900 px tall it
is a complete, closing rectangle floating in front of a reader with
nothing where they are looking, and it worsens monotonically. That
is this row's own defect pointed a different way, and it was a
regression against the truncation it replaced, which kept the span
from the near end and did cover the aim. The aim is the point the
rest of the module is organised around — `grid` reads the pitch there
— so it is what the patch is shrunk toward, clamped to stay inside
the region. **When the aim is outside the region**, which is any
camera pointed past the plane, the patch sits at the end of the
region nearest it: the patch never leaves the region, so a ruling
always describes plane the window can see.

Refusing was rejected for the reason the row already names: it costs a
grid at exactly the views where a coarse grid still orients a reader,
and shrinking leaves the caller holding a true statement AND a grid.
Arguing it at the site was rejected because the argument would have to
be that a one-sided silent truncation is legible, and it is not —
under the old code the ruled family stopped at one bound while the
crossing family ran on to another, which is the shape the new row
measures.

One thing moved with it, stated as an invariant at the code: the
outward rounding of each direction's bounds now happens once rather
than once per family. That is a restructure, not a behaviour change
— the old closure took the OTHER direction's bounds as its `lo`/`hi`
and checked all four, so a non-finite bound already refused both
families and its comment said so. What did change behaviour is the
`last < first` arm, which now refuses the whole patch; it is
unreachable from `seen_region`, which returns `min`/`max` pairs. The
finiteness arm, by contrast, **is** reachable and the suite reaches
it: `a_datum_at_the_end_of_the_number_line_rules_no_line_at_infinity`
hands `rule_patch` a finite bound of `-1.797e308` against a pitch of
`0.02`, and `(lo / pitch).floor()` is `-inf`.

## What is asserted about it now

`a_patch_past_the_grid_backstop_is_shrunk_rather_than_truncated` in
`crates/viewer/tests/datum_draw.rs`. It reads the drawing back and
asserts five things: the ruling is not empty, the two families rule
one rectangle, **the rectangle contains the looked-at point**, the
rectangle stays inside the plane the window can see, and the ruling
tracks the window while the backstop is slack but stops growing once
it bites. Each is separately falsifiable — refusal reds the first,
truncation the second, the region-centred shrink the third, dropping
the clamp the fourth, removing the cap the fifth.
