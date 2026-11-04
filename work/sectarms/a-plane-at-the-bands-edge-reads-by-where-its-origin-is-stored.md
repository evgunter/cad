---
id: a-plane-at-the-bands-edge-reads-by-where-its-origin-is-stored
kind: issue
title: A plane one ε off a dome's top reads Unaccounted with its origin stored at the foot, a touch with it stored far along
status: open
priority: P3
cost: M
opened: 2026-11-03
---


Found by PR 6479's dual review (R1), while it built the row that pins
the pivot's padded band
(`crates/geom-brep/src/pair_section/spline_implicit/sweep/rows.rs`,
`a_plane_stored_far_along_itself_reads_the_touch_its_pivot_pads`).

## What happens

The fixture: the dome `z = −(x−½)² − (y−½)²` (3×3 biquadratic), at
ε 1e-9, against the plane of normal `(0.1, 0.06, 1)`, normalised, placed
`h` above the dome's tangent point `(0.55, 0.53, −0.0034)`. The plane's
origin is stored `far` along the plane.

`pair_section(.., Some(SsiLane::marched()), band)` reads as below, on
the PR 6479 fix-pass head. "shift" is the pivot's certified distance
from the stored plane, which pads the band.

| h | far 0 | far 1e5 | far 1e6 | far 1e7 |
|---|---|---|---|---|
| 0.5 ε | Point | Point | Point | Point |
| 1 ε | **Unaccounted** | Point | Point | Point |
| 1.5 ε | Unaccounted | Unaccounted | Unaccounted | **Point** |
| 2 ε, 3 ε | Unaccounted | Unaccounted | Unaccounted | Unaccounted |
| shift (m) | 6e-18 | 9e-12 | 8.7e-11 | 7e-10 |

Unpadded (`band.padded(0.0)`), the cells read the same, except that
1.5 ε at 1e7 reads `Unaccounted`.

So one plane reads a touch or refuses depending on where its origin is
stored, in two ways:
- the shift pads the band by up to 0.7 ε at 1e7 m, which decides the
  touch at 1.5 ε;
- at 1 ε the stored origin's own rounding (an ulp of 1e5 m is about
  1.5e-11 m) moves the stored plane, and the reading flips with no
  padding at all.

R1 (PR 6479's dual review) found it at 1 ε, with the origin stored at
the foot: the touch reads `Unaccounted` there, on the base too. The
module docs of `crates/geom-brep/src/pair_section/spline_implicit.rs`
said "where its origin is stored changes nothing", which is false at
the band's edge. PR 6479 corrected the sentence.

## What closes it

A reading at the band's edge that does not depend on where the origin is
stored: the touch decided on the pivot plane whatever the shift, or
refused alike at every offset. Add a row at the foot and at the far
offsets.
