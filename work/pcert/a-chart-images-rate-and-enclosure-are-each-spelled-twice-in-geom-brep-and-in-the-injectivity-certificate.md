---
id: a-chart-images-rate-and-enclosure-are-each-spelled-twice-in-geom-brep-and-in-the-injectivity-certificate
kind: issue
title: A chart image's closed forms are spelled twice: Focal::rate beside speed.rs's derivative, and Harmonic::on and Wall::on beside harmonic_span_box
status: open
opened: 2026-11-24
priority: P1
cost: M
---



## Finding

From `embed/cone-image`'s dual review (2026-11-24). A stored chart
image's closed forms live in `crates/geom-brep/src/pcurve_cache.rs` and
`pcurve_cache/speed.rs`, and the injectivity certificate
(`crates/topo/src/injective.rs`) spells some of them again:

1. **A focal image's azimuth rate.** `Focal::rate` encloses
   `±(1 − β²)/(1 − 2β·cos E + β²)` over a parameter piece;
   `pcurve_cache/speed.rs` derives the same derivative for the image's
   speed bound, as a sup over a period. A rate over a piece beside a sup
   over a period, and neither cites the other.
2. **A harmonic image over a span.** `Harmonic::on` (`channel_on`)
   encloses `p0 + pa·cos t + pb·sin t + pl·t` by interval evaluation
   over the span; `harmonic_span_box` (public as
   `Pcurve::closed_form_span_box`) encloses the same quantity by its end
   values and the chord-dip charge, clamped at the amplitude. Two
   enclosures of one quantity, of different tightness.
3. **A wall image over a span.** `Wall::on`, the certificate's own
   enclosure of a spiric's wall image, named with the two above by the
   review. Whether the stored spiric image has a span box of its own to
   read was not checked here.

`embed/cone-image` folded the one pair it had added: `Focal::on` now
reads `Pcurve::closed_form_span_box` and its own hull is gone. These
three were there before it or are its rate.

**Band P1**, by `work/README.md`'s definition: two implementations of
one underlying logic. Nothing reads wrong today (each is sound alone,
and the focal pair was attacked by randomized rows); what entrenches is
that a change to an image's form has to be made in both crates, and
only one of them is checked against the stored row's evaluator. One
reviewer read it P2 for that reason; the README names this shape under
P1.

## What a fix needs

One home per quantity, read at the scalar the caller has:
- a rate door on `Pcurve` (or on `FocalImage`) over a piece, which
  `speed.rs`'s sup and `Focal::rate` both read;
- `Harmonic::on` reading `closed_form_span_box`, as `Focal::on` does,
  with its `rate` beside it; then `Wall::on` the same way.

A reader moved onto the stored box changes its enclosure's width (the
chord-dip box is looser than interval evaluation on a short span and
tighter on a long one), so the certificate's rows are re-run over the
move, and the endpoint census's note for `injective.rs` moves with it.
