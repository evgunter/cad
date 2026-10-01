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

`crates/geom-brep/src/props/quad.rs`. Three producers build
`Collapse::Over(lo_or_refuse(b), hi_or_refuse(b))` from a bracket that
`lo_or_refuse`/`hi_or_refuse` turn into NaN ends when it is refused, by
design:

- `piece_monotone`, from the block box (`block_box`), feeding `rate`;
- `trimmed_patch_face_rounds`, from the trim box (`trim_box(chords)`),
  feeding `s_hull`, `h_su`/`h_sv` and so the whole-box `sup_f` and
  `sup_g`, the levers of the area's gap, vertex-connector and sliver
  pads;
- `trimmed_patch_face_rounds` again, per chord, from the chord's lune
  hull (`c.hull`), feeding `lune_f`/`lune_g`, the lune pads.

Then:
- `PatchGrid::collapse_1d`'s three `Collapse::Over` arms (reached through
  `PatchGrid::vec`) then read those ends
  as a window: `range_hull` (`kv.span_range(lo, hi)`), `raw_range_hull`
  (`raw_span`, whose `knots[i] <= t` is false for NaN, so it answers
  `degree`, the first span) and the `Dir::Const` arm
  (`Dir::const_index`). None of them asks whether the window is
  ordered.

So a refused bracket does not refuse what is built over it. In
`piece_monotone`, `grid_vec` returns the first span's derivative hull,
`norm_lo` takes a finite lower bound from it, and
`Margin::metered(span, InfSpeed::new(rate))` classifies a
definitely-apart claim on a rate that belongs to another region. In
`trimmed_patch_face_rounds` the first span's hulls stand in for the trim
box's or the lune's, so `sup_f`, `sup_g`, `lune_f` and `lune_g` can be
SMALLER than the region's — an area pad that under-states what it pads,
which is the unsound direction for a certified area. The
neighbouring row
`trim-piece-monotone-span-fold-drops-a-refused-window` reasons that
"only a refused `rate` (from the block box) keeps that from reading as
definitely monotone"; this is why that backstop does not hold.

## Why it is latent today

Each bracket is a hull of the trim's chord points or of sums and
products of certified brackets over them. A refused one needs a
non-finite chord point or control point, which the props doors refuse
earlier (the neighbouring row's argument for the block box), or, for
the trim box, no chords at all (`trim_box` starts refused and is only
seeded by a chord). No fixture this lane knows of reaches any of the
three. The pads are the sharper case if one is ever reached: there a
first-span hull is not a wrong rate behind a decision but a smaller
number added to a certified bound.

## Repair

Refuse a NaN or inverted window where the window is read: `range_hull`,
`raw_range_hull` and the `Dir::Const` arm return `Interval::refused()`
unless `lo <= hi` (false for either NaN), as `NurbsBoxes::cells` does.
A refused hull then refuses `rate`, `sup_f`/`sup_g` and the lune terms
through the arithmetic over it, and all three producers are covered.
The class's root is `span_range`'s first-span tie-break for NaN, filed
on the NURBS slate as
`span-locator-lands-a-nan-parameter-on-the-first-span`.
`KnotVector::span_range` documents the first-span tie-break as its
contract (`crates/geom-core/src/spline/knots.rs`), so a door there would
move every caller; the per-reader check is the local fix.
