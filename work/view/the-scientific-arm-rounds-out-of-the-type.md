---
id: the-scientific-arm-rounds-out-of-the-type
kind: issue
title: the render's scientific arm rounds out of the type at the top of f64
status: open
opened: 2026-09-12
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
(`crates/viewer/src/readout.rs`) pins the exception so it is met rather
than rediscovered, and `number`'s doc states it.

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
