---
id: an-iso-line-states-its-ends-twice-as-its-map-and-as-stored-ends
kind: issue
title: An IsoLine states where its ends are twice, as its map and as stored ends, kept in step by hand and certified by different checks
status: review
opened: 2026-11-14
priority: P1
cost: M
refs: [6959]
branch: boxfinish/iso-image
pr: 7393
---


Raised by the review of PR 6959 (the chart-ends unit,
`check-7s-quadrature-refuses-interval-nurbs-walls-with-inexact-pcurve-ends`),
2026-11-14, as its main style finding. That PR lands with the hole
below closed inside the representation it built; this row is the
representation.

**What is there.** `Pcurve::IsoLine` holds a map `p0 + pl·t` over its
edge's parameter and, since PR 6959, `ends: Option<[Point2; 2]>`, the
chart points at the two ends of the span it is stored on. The PR
measured why an end has to be stored: an edge's own `t1` is a computed
length, an enclosure a few ulps wide at `Interval` and box-wide over a
parameter box, and no arithmetic re-reading of `p0 + pl·t1` recovers an
exact chart 0 or 1 from it.

**What is wrong with it.**
- The two say one thing and are kept in step by hand at about ten sites
  (`split_at`, `with_end_at`, `over_span`, the re-anchor, `map_chart`,
  `shift_chart_v`, the lift). One stale end was found by two rows
  breaking.
- They are certified by different checks: the ends by check 3's end
  stations, the map by check 4's envelope. The review demonstrated the
  drift: an image whose stated end sat ε/2 off its map certified with
  `envelope = 0`, and the rectangle certificate's pad is that envelope.
- `Option` conflates four states: placed by the mint; re-read through
  the map after a span changed; selected in band by the importer; none.
  `None` cannot tell "nothing to state" from "forgot".
- Both are functions of the same two placed points and the span.

**The question.** What does a chart image that is a straight row or
column of its chart store, so that an end its construction placed on
the chart's edge reads as exact at every scalar, and nothing is stated
twice? Whoever weighs it should look at what the quad lane's rectangle
certificate actually needs (that the trim IS the chart's rectangle,
which may be a discrete fact and not two points), and at
`docs/DESIGN.md` `[edges]`: for a `Chart` edge the pcurve is the
description, not a cache.

**Waits on it:** `an-iso-arc-rims-end-reads-through-its-map-at-interval`
(filed by PR 6959), so that `IsoArc` does not get a second copy of the
same shape.

## From PR 6959's delta review (2026-11-14)

- **`StoredSpan` names some edge's span, not this image's.** It is `Copy`, so another edge's `edge_span()`, or a span copied before a split, still compiles against an image; `LiftedRow { params }` is a pub field. No caller pairs them wrongly today. Whatever the representation becomes should make that pairing unwritable.
- **A stated end is allowed off its map by up to the band**, and the envelope now pads for it. That term is nonzero at `Interval` even for a minted image (about 1e-15 × the chart's stretch), where one description would make it zero by construction.

## Weighed (fork 271, 2026-11-15): three designers agree after two rounds

**What an image stores.** A boundary image that is a row or column of
its chart (straight or arc) stores the chart line it holds fixed, as
one value, exact where a mint placed it, and one map along that line
over the edge's parameter. It stores no ends. `Pcurve::IsoLine`'s
`ends`, `Pcurve::span_ends` and `StoredSpan` go.

**Where a corner is read.** A joint's chart point is the loop's lift's
(`topo::pcurves::Lift`): where a row meets a column it is the meet of
the two held values, exact wherever both are; between two images on one
line (a split's cut) and anywhere else it is the image's value there.
Every reader that wants a corner reads it there: the rectangle
certificate, `chart_region`'s loop polygon, the spline trim,
`boolean::spline_known`, `replace_face::iso_image_row`.

**What the rectangle certificate reads.** The loop is iso images only;
grouped into runs on one held line there are four runs, alternating row
and column; each run's pieces run one way, decided; the four meets are
points. No end of any image is read, so a cut is never asked to be
exact.

**The period.** A spline chart closed in a direction states its period
by its domain's two ends. Moving a value that is exactly one end by one
period returns the other end, by definition; any other move adds
`k·(hi − lo)`. Without it a seam column lifted one period is
`lo + (hi − lo)`, which at `Interval` has width on any domain not
starting at 0. No kernel-built chart is affected (their domains are
`[0, 1]`); an imported closed wall would be. This is a defect under PR
6959's stored ends too.

**What would change it:** a door that re-parameterises a spline chart's
domain while keeping its stored images. The designers found none
(reflection is the identity on a spline chart; `mirror_v` runs on
planes; the offset's `v` shift on cones).

**The units, in order.**
1. The certificate reads four runs and their meets. Needed under any
   representation, and builds on today's: a held value is the fixed
   coordinate of an image's map. Closes
   `a-re-minted-split-rims-row-reads-its-kept-corner-through-its-map`.
2. The image type (the held line and one map; `ends` and `StoredSpan`
   deleted; the readers moved to the lift's joint point), with the
   period stated by its ends. Waits on Ev.
3. The arc image, shown on an `Interval` arc loft once one builds
   (`an-iso-arc-rims-end-reads-through-its-map-at-interval`).

**Left to the implementer:** whether the straight and arc images are
one variant or two sharing the held-line type.

**Acceptance, when unit 2 lands.** `sweep`
`split_edge_loft_charts::a_split_on_a_spline_chart_carries_the_parents_image`
goes back to its first contract: a split child holds the parent's
image, the same `Debug` bytes, with only the interval moved. PR 6959
made a child's stated end at the cut differ from its parent's, so the
row was re-stated in three parts (the map bit for bit, the kept end,
the cut's end as the map's value); with no ends to state, those three
are one comparison again.

**Readers that move onto the lift's joint point, when unit 2 lands**
(named by PR 6982 and its review, 2026-11-15). PR 6982 builds
`Lift::joint_point`: the meet where a row meets a column, the entering
image's point everywhere else. It moves no other reader. These still
read an image's end and are this row's:
- `topo::chart_region::pcurve_entry`;
- `topo::spline_trim::image_pieces`;
- `topo::boolean::spline_known` (the shape exactly: an inexact corner
  is not found as a corner);
- `topo::replace_face::iso_image_row`.

And the held line has more than one reader, which disagree on what
they accept: `LiftedRow::held_line` (`IsoLine` and `IsoArc`, both
axes), `geom_brep::description::row_of` (`IsoLine` both axes, `IsoArc`
in `u` only) and `replace_face::iso_image_row` (`IsoLine` only). PR
6982's fix pass gives the reading one home on `Pcurve`; whichever of
the older two it cannot move without changing behaviour is this row's
too.


## Ratified (2026-11-15, `[ev]` PR 6975)

Ev: "i'm not very sure about the considerations here, but i think this
makes sense". The recommendation stands as written above: an image
stores the line it holds fixed, one value, and its map along it, and no
ends; a corner is the meet of two held lines, read from the lift; a
closed chart's period is held as its domain's two ends. PR 6982 has
since built the certificate's half (it reads four runs and their meets,
`Pcurve::held_line`, `Lift::joint_point`). What is left is the
representation: the image type itself, the period, and the readers
PR 6982 named to move onto `Lift::joint_point`
(`chart_region::pcurve_entry`, `spline_trim::image_pieces`,
`boolean::spline_known`, `replace_face::iso_image_row`), then the arc
image (`an-iso-arc-rims-end-reads-through-its-map-at-interval`).
