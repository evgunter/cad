---
id: the-line-conic-arm-spells-its-heights-about-the-conics-centre-by-hand
kind: issue
title: The line × conic arm of meet spells (c − o)·n by hand five times, around the one spelling plane_offset, so its heights round at the line origin's ulp
status: open
opened: 2026-11-22
---


Filed by GERM's `germ/plane-pose-far-frame` lane, from its sweep for
offsets formed about a stored origin far from the geometry.

## Finding

`geom::plane_offset` (`crates/geom/src/frame.rs`) is the one spelling of
`(q − o)·n`, and it is now posed with error-free differences and a
compensated sum (`Real::difference_dot`), so it keeps its own digits
however far `o` lies from `q`. `curve_pair::pose::line_conic`
(`crates/geom-brep/src/curve_pair/pose.rs`) spells the same formula by
hand, about the line's stored origin `o`, which is free along the line
and can lie far from the span:

- `c = (k.frame().origin() − o).dot(u)` (the conic's extent window);
- `height = (o + u·at.mid() − k.frame().origin()).dot(n)` (the tilt
  route's offset);
- `(k.frame().origin() − o).dot(n) / den` and
  `(point − k.frame().origin()).dot(n)` (the crossing's seed);
- `c0 = (k.frame().origin() − o).dot(m)` and
  `(k.frame().origin() − o).dot(v)` (the level pair's harmonic and slant).

Each rounds at `ulp(|o|)`. The window's projection is charged
(`rounding_charge(ends(...))`); the heights are not.

## Settles when

The arm reads each through `geom::plane_offset` (or a re-posed point
form where it reads a point of the line), its window's charge dropped,
with a far-origin row on the line × conic route at ε 1e-12.
