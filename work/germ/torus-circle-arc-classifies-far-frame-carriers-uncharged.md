---
id: torus-circle-arc-classifies-far-frame-carriers-uncharged
kind: issue
title: The ring lane classifies a torus loop circle by its stored axis and radius with no charge for far coordinates' rounding
status: open
opened: 2026-10-13
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
