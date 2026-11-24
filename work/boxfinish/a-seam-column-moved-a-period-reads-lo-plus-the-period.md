---
id: a-seam-column-moved-a-period-reads-lo-plus-the-period
kind: issue
title: A seam column the lift moves one period of a closed spline chart is held at lo + (hi − lo), computed, which has width at Interval on a domain not starting at 0
status: closed
opened: 2026-11-15
priority: P3
cost: M
closed: 2026-11-24
pr: 7393
---


Found by `boxfinish/rectangle-runs` (check 7's rectangle certificate
reading the loop's held lines off its lift, `topo::pcurves::Lift::runs`).
The period's statement is weighed in
`an-iso-line-states-its-ends-twice-as-its-map-and-as-stored-ends`
(fork 271, `[ev]` PR 6975, "The period"); this row holds the defect and
what reaches it until that lands.

**What is there.** `topo::pcurves::Lift::read` moves each stored image by
its deck element, and on a spline chart the period it moves by is the
knot domain's length, `hi − lo` (`ChartPeriods::stated`,
`crates/topo/src/pcurves/mint.rs`), added by `Pcurve::translate`
(`Period::times`, `k·(hi − lo)`). A seam column stored at `u = lo` and
lifted one period is held at `lo + (hi − lo)`, computed. At `Interval`
that sum has width on any domain that does not start at 0, so the
column's held value is no point, and the rectangle certificate refuses
the wall ("a NURBS-face boundary line's held value is not exact
structure"); at `f64` the sum is a point, and its rounding is read as
the column, misplacing it by an ulp wherever the sum misses `hi`:
`[0.3, 1.7]` lands on `1.7`, but `[0.4, 1.7]` gives `1.6999999999999997`,
`[0.6, 1.7]` gives `1.7000000000000002` (outside the domain) and
`[0.7, 3.1]` gives `3.1000000000000005` (found by PR 6982's review).
Pinned, all four, by
`topo::props::quad_lane::tests::trim_rectangle_rows::a_seam_column_moved_one_period_is_its_computed_sum`;
at `Interval` the certificate refuses the column naming the period.

**What reaches it today** (head of `boxfinish/rectangle-runs`, every
suite of `topo`, `sweep`, `mesh`, `step-export`, `step-import`, `verbs`,
`stl` at ε = 1e-9, a log of every lifted row with a deck other than the
identity on a face the certificate reads): the closed one-segment loft
walls of `sweep`'s `one_segment_loop` and `degree1_loft_slabs`, whose
wrap strut is lifted one period (`Deck { u: 1 }`), on `[0, 1]` domains,
where `0 + 1` is exact; all at `f64` but
`one_segment_loop::a_lofted_one_segment_section_is_one_spline_wall_with_a_wrap_strut`,
at `Interval`, also on `[0, 1]`. No kernel-built chart has another
domain; an imported closed wall would.

**What would close it:** the period stated by its domain's two ends, so
that a value that is exactly one end moved by one period is the other
end (the fork's recommendation).

## From `boxfinish/iso-image` (the period, 2026-11-22)

The cause is closed there. A spline chart's period is stated by its
domain's two ends (`geom_core::period::Period::of_domain`), and a value
moved by periods goes through `Period::moved`: a value that is exactly
one end, moved the one period towards the other, is the other end, with
no sum formed; every other move adds `k·(hi − lo)`. An iso image's held
value is moved that way (`geom_brep::Pcurve::shift_branch`,
`shift_branch_v`), so a seam column stored at `lo` and lifted one period
is held at `hi` at `f64` and at `Interval`.

The four pinned domains re-pin with the move accounted: `[0.3, 1.7]`,
`[0.4, 1.7]`, `[0.6, 1.7]`, `[0.7, 3.1]` each read the rectangle
`[lo, hi]` at both scalars
(`topo::props::quad_lane::tests::trim_rectangle_rows::a_seam_column_moved_one_period_is_the_domains_other_end`,
`geom-brep` `an_iso_image_holds_its_line::a_period_moved_seam_column_is_its_other_end`).

What is left, and is by design: a column that is NOT at an end, moved by
periods, is held at the computed sum, and at `Interval` the rectangle
certificate still refuses it, naming the move. No construction lifts
such a column: a lift moves a seam, and a seam is at an end.

## Closed

PR 7393 (2026-11-24): the section above. One period up is
`hi + (x − lo)` and one down `lo + (x − hi)` (`Period::moved`), so a
seam column at an end is held at the other end itself at every scalar,
and the four pinned domains read the rectangle `[lo, hi]`.
