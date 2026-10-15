---
id: torus-circle-arc-classifies-far-frame-carriers-uncharged
kind: issue
title: The ring lane classifies a torus loop circle by its stored axis and radius with no charge for far coordinates' rounding
status: closed
opened: 2026-10-13
closed: 2026-10-14
branch: germ/torus-circle-arc-far-frames
pr: 5103
priority: P2
cost: M
refs: [plane-torus-section-gaps-decide-far-coordinates-uncharged]
---


## What

`Quadric::torus_circle_arc` (`crates/topo/src/ring_path.rs`, ~:900)
tells a torus loop circle's kind from four readings that charge no
coordinate resolution, unlike every path reading after the
far-frame fix (`ring-path-readings-decide-past-their-resolution-at-far-frames`):

- **`split_ring_torus_meridian`**: `n̂·â` levered at the radius;
- **`split_ring_torus_parallel`**: `‖n̂ × â‖` levered at the radius;
- **`split_ring_torus_parallel_side`**: the circle's radius less `R`;
- **`split_ring_torus_crest`**: the centre's height `(k − c)·â`.

The first three read the carrier's stored axis and radius against the
torus's, with no subtraction of coordinates in the reading itself.
But a circle that a section lane mints from points in a far frame
carries their rounding in its axis and radius: at coordinates ~1e6 that
is ~1e-10 m, past the zero band at ε 1e-12. A crest circle, a parallel
of radius `R`, could then read `parallel_side` decided. It would be read
as an ordinary parallel, by crossings of its touching plane, not
across the cylinder `ρ = R`. A meridian could read as neither kind
(`Ok(None)`, a refusal rather than a wrong answer). The crest height
is `±r` on every valid input, so it is never in the band.

## Why the far-frame fix did not charge them

There, a zero verdict *accepts* a kind (a meridian, a crest). It is not
a stand-down. So charging the margins widens what is accepted. That
needs its own argument: a parallel within the resolution of `R` is
read as a crest. Nothing measured shows a wrong parity from these four.
The far-frame rows' islands carry exact axes and radii.

## To do

Measure first: mint the torus islands' circles through the section
lanes (`plane×torus`) in the far frame (`ring_path::far_frame_rows`'s),
and see whether any classification differs from the frame at the
origin. Charge the readings only if one does, and argue the widened
acceptance.

## Measured (`germ/torus-circle-arc-far-frames`): no classification differs

Priority: P2, the interval and error-propagation band. The question was
whether a far frame could make a circle read as a different kind. It
cannot, so nothing is charged.

`ring_path::far_frame_rows::a_torus_circle_the_section_lane_mints_far_reads_its_kind_at_the_origin`
mints each circle through `geom_brep::plane_torus_section`
(`crates/geom-brep/src/intersect.rs`, ~:2750). The circles are 12
meridians and 20 parallels, the crests `v = ±π/2` among them. Each is
cut by its own plane, stored at the circle's centre or 3.7 units off it
along the plane. The frames are the three of `frames()`, moved to
twelve origins `1e5`–`1e6` off the origin, at scale 1 and `1e3`, with
the torus's axis either way. Each circle's kind, as
`Quadric::torus_circle_arc` (`crates/topo/src/ring_path.rs`, ~:894)
reads it, is compared with the same circle minted at the origin.
Wherever the far section is the same kind as the origin's, **no circle
reads another kind** at ε unset, `1e-6` or `1e-12`. Circles compared at
ε `1e-12`: 2160 meridians, 2592 parallels each side of `R`, and 154
crests each side of the equator plane. At the other two ε: 3456, 2592
and 288.

Why: the section lane takes a circle's classification inputs from
translation-free data, never from a difference of far points. Those
inputs are its axis (the plane's normal for a meridian, the torus's
stored axis for a parallel) and its radius (the torus's stored `r`, or
`R`, or `R ± w` with `r − |h|` decided off zero, so `w` is at least
about `√(band·r)`, far outside the band). The only far-point
differences in that lane are its own gaps `d` and `h`. They decide which
section is minted, not what its circles read as. The crest height is
`h = ±r` on every tangent circle. `sweep::revolve`'s parallels take
their radii from sketch coordinates (`AxisFrame::r`), and its far
placement moves only their centres, so the same holds there.

Two mutants turn the row red. A far-point radius in
`split_ring_torus_parallel_side` (`|(k + u·ρ − c) × â| − R`) goes red
at ε `1e-12`: 54 crests read as parallels or escalate. A shifted
`radius − R` that reads crests as parallels everywhere breaks the
row's floor of each kind.

**What the row found instead** is upstream, and it is filed as
`plane-torus-section-gaps-decide-far-coordinates-uncharged`. At ε
`1e-12` the section lane's own `pt_axis_plane_gap` and `pt_cap_gap`
read the far frame's rounding: 1296 of 3456 far meridian planes come
out as ovals or refusals, and 268 of 576 far crest planes as
concentric pairs, empty, or refusals.

**Blind spot, not measured:** a profile drawn in sketch coordinates
that are themselves far, with a tilted axis. There a crest
parallel's radius (from the arc's endpoint) and the torus's `R` (from
its centre) round differently. That rounding is the sketch's own data,
not a far placement. Measuring it would need a `sweep` row, since
`topo` sits below `sweep`.
