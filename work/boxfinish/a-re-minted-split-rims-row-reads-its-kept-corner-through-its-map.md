---
id: a-re-minted-split-rims-row-reads-its-kept-corner-through-its-map
kind: issue
title: A split rim's re-minted row reads its kept chart corner through its measured map, so a split Interval loft refuses check 7
status: closed
opened: 2026-11-14
priority: P1
cost: M
closed: 2026-11-15
pr: 6982
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

## Corrected (2026-11-15, fork 271): the cut refuses first

Measured by a designer's probe on
`a_split_rims_cut_is_never_read_as_a_chart_end`: the as-split
`Interval` body, finished BEFORE `mint_pcurves` re-mints anything, with
both kept corners reading exactly 0 and 1, already refuses "a
NURBS-face pcurve endpoint is not exact structure"; `f64` finishes. The
rectangle certificate applies its exactness test to both ends of every
image (`quad_lane::nurbs_face`, the `exact` closure), the cut included,
and a cut's moving coordinate is the map read at a computed parameter.

So this row's title names half the cause, and restoring the kept
corner alone closes nothing. What closes it is the certificate reading
runs and their meets, where a cut is a joint inside one run and is
never read as a point (unit 1 of
`an-iso-line-states-its-ends-twice-as-its-map-and-as-stored-ends`).


## Closed (2026-11-15, PR 6982)

Check 7's rectangle certificate reads no image's end. A NURBS face's
trim is the lines its loop holds (`geom_brep::Pcurve::held_line`,
`topo::pcurves::Lift::runs`): four runs, each of pieces that state one
exact value and run one decided way, walked once round
(`TrimRectangle::walked`). A cut between two images on one line is
never asked to be exact, and a corner is the meet of two lines
(`Lift::joint_point`); how far an image's end stands from its joint's
point is padded (`joint_defect`), with the argument and its five
assumptions stated once on `nurbs_face`. So the re-minted split
`Interval` loft finishes: split `Interval` lofts and five re-minted
`f64` ones that main refused certify (the review counted 25 and 5),
each holding its analytic volume
(`a_split_and_reminted_loft_holds_its_volume.rs`,
`split_lofts_hold_their_volumes_at_interval`).

Neither question the row left unmeasured was needed: the mint is as it
was, and the measured-feet arm still states no ends.

Left, each with a row: a seam column moved one period reads
`lo + (hi − lo)`, which is no single number
(`a-seam-column-moved-a-period-reads-lo-plus-the-period`);
`clearance::iso_bounded_domain` still reads image ends through the map
with a hand ulp slack
(`clearances-free-form-domain-test-reads-image-ends-with-a-hand-ulp-slack`);
the readers that move onto `Lift::joint_point` and the one held-line
home with the image's representation
(`an-iso-line-states-its-ends-twice-as-its-map-and-as-stored-ends`,
with Ev on `[ev]` PR 6975).
