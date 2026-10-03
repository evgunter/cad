---
id: trim-walk-chord-lengths-are-rooted-to-nearest
kind: issue
title: The trim walk's chord lengths are f64 roots to nearest, read as a pad, a refusal threshold and a direction
status: open
opened: 2026-10-01
---


## The finding

Found by `linalg/certification-gains-a-sqrt-door`'s sweep for roots
taken on bracket endpoints. That branch retired `props/quad.rs`'s
`sqrt_enclosure` and `norm_lo` into `Certification::sqrt`. Four more
roots in the same file are `f64` Euclidean lengths of differences of
chord endpoints. Each is computed to nearest throughout: the
differences, the squares, the sum and the root.

- `lune_area`: `len`, the chord direction the block is projected on.
  If `len` is short of the true length, `(du, dv)` is longer than unit,
  and `along · across` overstates the area. That is the safe side. If
  `len` is long, the area is understated.
- `piece_monotone`: `len`, the same direction. The certified `rate` is
  a lower bound along `(du, dv)`, so a direction longer than unit
  scales it up.
- The closing check in the trim-walk assembly (`TRIM_OPEN_WALK`):
  `d = √(gu² + gv²)`, refused when `d > slack + slack`. A `d` rounded
  below the true gap can pass a gap that is over the threshold by an
  ulp.
- The same assembly's `vertex_pad += chord_len * c.slack`: a pad, which
  is an upper bound. `chord_len`, the product and the sum all round to
  nearest.

Each error is a few ulps relative to a chord length. None of them
moves a classification on today's corpus as far as the sweep could
see. The question for the owner is which of these are structure, where
any direction is sound, and which are bounds. The bounds want the
interval spelling: `(Interval::point(b.0) - Interval::point(a.0)).sqr()
+ …`, then `.sqrt()`, read with `.mag()` for a pad and `.lo()` for a
floor.
