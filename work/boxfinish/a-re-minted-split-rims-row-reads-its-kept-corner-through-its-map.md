---
id: a-re-minted-split-rims-row-reads-its-kept-corner-through-its-map
kind: issue
title: A split rim's re-minted row reads its kept chart corner through its measured map, so a split Interval loft refuses check 7
status: open
opened: 2026-11-14
---


Found by `boxfinish/chart-ends`'s fix pass (PR 6959). Pinned by
`sweep/tests/a_lofts_chart_ends_read_as_placed.rs::a_split_rims_cut_is_never_read_as_a_chart_end`.

**Measured** (`Interval`, the point loft of three `quad(1.2)` sections):
- Split the bottom cap's slanted-side rim at a third of its span
  (`Body::split_edge`). The children carry the parent's row through
  `Pcurve::split_at`, so each reads its kept corner exactly (`[0, 0]`,
  `[1, 1]`) and its cut through the map.
- Then `topo::mint_pcurves` clears the rows and derives them again.
  `topo::pcurves::nurbs_iso_derive`'s boundary schedule offers each child
  the whole row (its probe misses, as it should), and the measured-feet
  arm takes it. That arm states no ends.
- The second piece's kept corner then reads `[0.9999999999999994,
  1.000000000000001]`.
- `topo::finish` refuses: `VolumeUncomputable { face 3v1,
  QuadratureUnsupported { "…not exact structure…" } }`.
- At `f64` the same body finishes and its volume is the prism's 8.4.

**What is known at the mint.** The child's outer end is the parent's
chart corner; the mint pass does not know it. The carried row did, and
the clear-and-derive drops it.

**Not measured:** whether the measured arm could be offered a candidate
with one stated boundary end (the chart side at the kept end, the
measured foot at the cut), judged by the same probe and certified
by checks 3 and 4, without moving the `f64` split rows'
re-minted maps; or whether `mint_pcurves` should keep a carried row
that still certifies.
