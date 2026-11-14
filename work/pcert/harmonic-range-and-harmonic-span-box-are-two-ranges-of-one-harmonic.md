---
id: harmonic-range-and-harmonic-span-box-are-two-ranges-of-one-harmonic
kind: issue
title: harmonic_range and harmonic_span_box are two ranges of one harmonic over a span in one module tree
status: open
opened: 2026-11-14
---

Filed by TESSLANE's `one-chord-rule-over-each-boundary-images-speed-bound`,
whose fix pass found it. Not scheduled.

## What

`pcurve_cache::speed::harmonic_range` (read by
`mesh::sizing::sphere_face_widest_cos` and by the projected conic's lever
arms) is the range of `c + a·cos t + b·sin t` over `[t₀, t₁]`: its ends,
and `c ± √(a² + b²)` where the span may hold an extremum's phase
(`geom::periodic_window_may_hold`). `pcurve_cache::harmonic_span_box`
encloses the same function per channel with a chord-dip charge
(`implicit::chord_dip_charge`) instead of the phase test. Two ranges of
one harmonic over a span, in one module tree, with different rounding
and different tightness.

## Fix

One range, read by both, or a statement of what each needs that the
other does not give.

## 2026-11-14 — a third reading, and the interval helpers (from the chord rule's second review)

`pcurve_cache::speed::harmonic_enclosure` is a third range of the same harmonic: `harmonic_range`'s reading with each value enclosed in interval arithmetic (the ends through `props::taylor::trig_range`), read by the projected conic's lever arms. `mesh::sizing::sphere_face_widest_cos` still reads the plain `f64` `harmonic_range`. Beside them, `speed.rs`' private interval helpers `dot`, `hyp`, `root_down` and `root_up` sit beside `patch_bound.rs`' interval `dot` (a method on its vector type): the same vector arithmetic in two places.
