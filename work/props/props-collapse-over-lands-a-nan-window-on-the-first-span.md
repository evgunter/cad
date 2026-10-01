---
id: props-collapse-over-lands-a-nan-window-on-the-first-span
kind: issue
title: props/quad.rs: Collapse::Over hands a refused bracket's NaN ends to range_hull/raw_range_hull/const_index, which land them on the first span and return a certified hull of a region nobody asked about
status: open
opened: 2026-10-01
---


Found by the SSI chart-tube lane's §5 sweep (PR "SSI: the chart speed
minted once per axis, and the certificate records the pad it proved"),
which closed the same shape in `ssi/enclose.rs`'s `NurbsBoxes::cells`:
a window whose end is NaN is clamped or located as if it named a
region, and `span_range`'s documented tie-break lands a NaN end on the
FIRST span, so the hull that comes back is certified and is the first
span's.

## The sites

`crates/geom-brep/src/props/quad.rs`:

- `piece_monotone` builds `Collapse::Over(lo_or_refuse(bu),
  hi_or_refuse(bu))` (and the same for `bv`) from the block box.
  `lo_or_refuse`/`hi_or_refuse` answer NaN for a refused bracket by
  design.
- `PatchGrid::collapse_1d`'s three `Collapse::Over` arms (reached through
  `PatchGrid::vec`) then read those ends
  as a window: `range_hull` (`kv.span_range(lo, hi)`), `raw_range_hull`
  (`raw_span`, whose `knots[i] <= t` is false for NaN, so it answers
  `degree`, the first span) and the `Dir::Const` arm
  (`Dir::const_index`). None of them asks whether the window is
  ordered.

So a refused block box does not refuse `rate`: `grid_vec` returns the
first span's derivative hull, `norm_lo` takes a finite lower bound from
it, and `Margin::metered(span, InfSpeed::new(rate))` classifies a
definitely-apart claim on a rate that belongs to another region. The
neighbouring row
`trim-piece-monotone-span-fold-drops-a-refused-window` reasons that
"only a refused `rate` (from the block box) keeps that from reading as
definitely monotone"; this is why that backstop does not hold.

## Why it is latent today

A refused block box needs a refused projection of finite certified
inputs, which the props doors refuse earlier (the neighbouring row's
argument). No fixture this lane knows of reaches it.

## Repair

Refuse a NaN or inverted window where the window is read: `range_hull`,
`raw_range_hull` and the `Dir::Const` arm return `Interval::refused()`
unless `lo <= hi` (false for either NaN), as `NurbsBoxes::cells` does.
`KnotVector::span_range` documents the first-span tie-break as its
contract (`crates/geom-core/src/spline/knots.rs`), so a door there would
move every caller; the per-reader check is the local fix.
