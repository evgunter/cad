---
id: the-scientific-arm-rounds-out-of-the-type
kind: issue
title: the render's scientific arm rounds out of the type at the top of f64
status: closed
opened: 2026-09-12
closed: 2026-09-16
refs: [render-mm-overflows-to-inf-for-a-delta-the-door-accepts, the-fields-door-has-no-width-bound-at-all]
---



(VIEW) `crates/viewer/src/readout.rs`'s `number` searches decimal
spellings for one that reads back as the value, and falls through to
`format!("{value:.3e}")` when none fits. **That fallback is not itself
held to reading back, and it rounds.** Within half a unit in the fourth
significant figure of `f64::MAX` the rounding leaves the type:
`number(f64::MAX)` is `1.798e308`, and `"1.798e308".parse::<f64>()` is
`inf`. So the module's own rule — a rendered number reads back as the
value it renders — is false at exactly one place, and the place is the
one the fallback exists to serve.

Pre-existing rather than introduced: `DisplayTolerance::render_mm` had
the same `{:.3e}` fallback before the four render sites were routed
through one door, and it was worse there, because `render_mm` multiplies
by `1.0e3` first — a δ within three decades of `f64::MAX` already
overflowed to `inf` before any spelling was chosen.

## Why it is filed rather than fixed

The truthful spelling is the exact one, `format!("{value:e}")` —
`1.7976931348623157e308`, twenty-two characters. `readout::MAX_CHARS` is
ten, and it is not decoration: `crates/viewer/src/pane/view.rs`'s
`FIELD_WIDTH` is measured against it by `the_field_shows_the_longest_
render`, and a render the box cannot show is clipped, which is a silent
misread too. So the unit that found this kept the width as the guarantee
and made the top of the type the carve-out, on the argument that no
length this chrome shows is within three hundred decades of it — a
camera distance is `scene_radius` times a constant, a probed bound is a
bounded number of doublings from a field's current value, and a δ that
large does not survive the millimetre conversion.

`the_top_of_the_type_is_the_one_value_that_does_not_read_back`
(`crates/viewer/src/readout.rs`) pinned the exception so it was met
rather than rediscovered, and `number`'s doc stated it.

## What a repair would have to answer

Not "which arm wins" but **whether a render owes a width bound at all**.
The bound is a contract with one widget; every other caller is a label
that wraps. A third arm — exact scientific when the four-figure one does
not read back — makes the rule true everywhere and makes
`MAX_CHARS`'s own claim false, which is the same carve-out moved rather
than removed. The other shape is a width that is the CALLER's, which the
unit that wrote `number` rejected for a reason that would have to be
re-argued: the tolerance and the fallback spelling are two halves of one
choice, and a caller free to name a width is free to name one its
fallback cannot honour.

## Closed — the third arm, because the carve-out's own argument is false

`number` now falls through to the exact spelling exactly where the
four-figure one does not read back, and that is only the band at the top
of `f64`. Over `1.0e-320` to `1.0e300` stepped by 1.05 — 65,000 renders
— **not one text changes**; below the band the four-figure arm still
wins everywhere, and `NaN` and `inf` are spelled identically by both
arms. `the_four_figure_arm_still_carries_everything_below_the_band` is
the row that holds that, and it reds when the exact arm is made
unconditional.

**The load-bearing argument was measured and it fails on all three
producers.** The carve-out rested on *"no length this chrome shows is
within three hundred decades of it"*. Each producer's guard is
`is_finite()` and each then multiplies UP toward the top of the type, so
the real headroom is three to five decades, not three hundred:

- **δ.** `DisplayTolerance::new` accepts any finite δ > 0;
  `render_mm` is `number(δ * 1.0e3)`. Every δ in
  `[1.7975e305, 1.7976931348623156e305]` survives the conversion and
  lands **inside the band**. Above that the product is `inf` — filed as
  `render-mm-overflows-to-inf-for-a-delta-the-door-accepts`.
- **Camera.** `mm(max_distance)` is `scene_radius * MAX_DISTANCE_FACTOR
  * 1000` — `scene_radius * 1.0e5`. `Camera::new` checks `finite` and
  `>= MIN_SCENE_RADIUS` (`f64::MIN_POSITIVE`) and has **no upper bound**,
  so `scene_radius = 1.7975e303` lands in the band.
- **Probe.** A bound is `origin ± seed · 2^11`, then `/ unit.factor()`
  with the smallest factor `MILLI`. The `f64` `number_field` sites carry
  no `.range()`, so `origin` is whatever a user typed.

**It was never one value either.** The band is
`[1.7975000000000001e308, f64::MAX]` — about 9.7·10¹¹ `f64` values, each
sign. The old row's name and its comment *"an exception rather than a
region"* were both false of the tree when they were written.

**Which of the item's two shapes, and why.** The third arm, not the
caller-named width: the rejected reason still holds untouched, because a
caller still names no width and `REL_TOLERANCE` is still the module's.
What the third arm costs is `MAX_CHARS`'s consequence sentence, and
that sentence was already false of the crate's other two number
renders — `widgets::number_text` (filed as
`the-fields-door-has-no-width-bound-at-all`) and `props::render_number`,
which is `{:?}` and spells `f64::MAX` in twenty-two characters in a
field today. `MAX_CHARS`'s own doc already said it is *"the rule's own
number and not a field width"* and *"what ENDS the search"*; both stay
exactly true.

**And the module already carried the argument.** `reads_back`'s doc:
*"a wide text that does [name this value] is not improved by replacing it
with a narrow one that does not."* The old fallback did precisely that.
The two sentences could not both be right.
