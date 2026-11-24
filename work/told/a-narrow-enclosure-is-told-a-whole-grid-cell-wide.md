---
id: a-narrow-enclosure-is-told-a-whole-grid-cell-wide
kind: issue
title: A narrow enclosure is told a whole grid cell wide
status: open
opened: 2026-11-20
priority: P3
cost: E
design: true
---


Found building the told quantity (PR 7159) and confirmed by both of its
reviews.

`crates/geom-core/README.md`, "A number told to a person", says an
enclosure reads `[lo, hi]` with each end rounded outward. On the grid
that makes an enclosure narrower than a grid step read one whole step
wide: an `Interval` a few ulps across about 3e-6 m is told
`[2.9999e-6, 3.0001e-6] m`
(`geom_brep::edge_nurbs::tests::two_ends_off_a_plane_report_the_worse_miss_in_either_order`),
and `Sym<Interval>` of `0.5/3`, one ulp wide, reads `[0.1666, 0.1667]`.
The spelled ends do contain the enclosure, so nothing false is said, but
a reader takes the printed width for the enclosure's.

Where the sentence states an order (`Told::beside`), two enclosures that
do not meet are spelled finer until they read apart, so the compared
pair does not depend on this. A lone enclosure, and two that overlap,
read at the grid.

**The question.** Whether a narrow enclosure should read as its width
warrants: one number where its ends agree to the grid ("3e-6 m"), the
ends spelled to the first precision at which they differ, or the rule as
it stands.
