---
id: the-radial-repairs-catastrophic-cancellation-claim-is-unmeasured
kind: issue
title: CylFrame::radial asserts the projection read cancels catastrophically and nothing in the tree measures where
status: open
opened: 2026-09-20
---


## Finding

`topo::test_support::CylFrame::radial`'s rustdoc justifies reading the
radial direction from the frame's own fields rather than by projecting
a chart point back onto the plane through the axis:

> the projection cancels catastrophically for a tilted frame or a small
> radius, and the direction it yields is then not the one the chart
> names

**Nothing in the tree holds that true, and after PR #2925 nothing in
the tree contains the other read at all.** The last in-tree spelling of
the projection was `crates/topo/src/census.rs`'s `cyl_sheet_b`, folded
onto `radial` in that PR after both call sites of
`cross_description_pair` were dumped arena-key-by-arena-key and came
back **bit-identical** — which is why the fold was safe, and equally
why the claim now has no witness: the two reads agree everywhere the
tree still looks.

So the repair's premise survives only as a sentence in a doc comment,
while the frames it is about (`CylFrame::tilted`, and
`CylFrame::canonical` at small radii — `mate5_cyl_eps_rung` builds one
at 1e-3) are exactly the ones the probe suites use.

## Why this is S-TINT's

It is not a duplication: the duplicate is gone. It is a documented
invariant with no assertion under it, on a door several suites reach.

## What a unit here owes

Measure where the two reads diverge, at the frames the suites actually
build, and then either hold the claim with a row — a tilt or a radius
at which the projection's direction is outside some stated bound of the
frame read's — or delete the justification and keep the repair on the
weaker true ground that a frame read cannot cancel at all.

**No number is stated here on purpose.** The lane that found this had
no measurement of the divergence and declining to guess one is the
point; a row opened with a plausible threshold in it would be re-taken
by the next lane and found wrong, which is this tracker's most
repeated failure.
