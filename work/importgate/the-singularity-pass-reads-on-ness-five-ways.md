---
id: the-singularity-pass-reads-on-ness-five-ways
kind: issue
title: the singularity pass says a point lies on something five ways
status: open
opened: 2026-11-18
priority: P3
cost: M
---


Found by the review of PR 7117 (`importgate/singularity-meet`), Q1.

## What

`crates/step-import/src/normalize/singularity.rs` decides whether a point
lies on (or at) something through five readings:

- `Read::on`: `FileBand::read` Zero under `STEP_SINGULARITY_POINT`
  (`strut_tip`'s singular point at a vertex);
- `Read::not_apart`: `FileBand::read` not Apart (`interior`'s centroid and
  vertex azimuths);
- `on_boundary`: `geom_brep::chart_singularity::on_boundary` (a boundary
  landing at a singular point, `first_hit`);
- `curve_pair::Span::read`: a singular point, or a shared run's end, on
  the ray's span (`first_hit`, `landings`);
- `meet`'s own span-end chord (`curve_pair_span_end_chord`): a meeting
  at an edge's end, which is a vertex landing.

Each is margined and each is at the pass's band, but they are different
named decisions over the same question, so their records and their
refusals do not line up, and one point can read on by one and in band by
another.

## Fix

Pick one reading for "this point is that vertex / on that carrier" at
the pass's band (the kernel's point-on-span reading is the natural one)
and route the others through it, or state per site why the question
differs.
