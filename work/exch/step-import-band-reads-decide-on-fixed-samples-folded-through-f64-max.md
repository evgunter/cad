---
id: step-import-band-reads-decide-on-fixed-samples-folded-through-f64-max
kind: issue
title: step-import decides edge-on-description band reads on three or five fixed samples, folded through f64::max
status: open
opened: 2026-11-20
priority: P3
cost: M
---


Found by EXCH unit B's sweep (PR 7170, reviewer 2's m3): the pattern
"a certified bound read on fixed samples and folded with
`fold(0.0, f64::max)`" outside `recognize_curve.rs`. Two defects
share each hit: a fixed station set is a witness, not a sup (the
curve can leave the band between stations), and `f64::max` drops a
NaN, so a poisoned station reads as zero.

Hits (`crates/step-import/src/`), by name:

- `build.rs`, `on_description`: three stations (ends, middle) of the
  printed carrier against the built one decide
  `STEP_EDGE_ON_DESCRIPTION` (`agree_at`). **This class.**
- `build.rs`, `cap_row`: three stations of the printed edge against the
  wall's row decide the same name. **This class.**
- `build.rs`, `rebuilt_in_surface`: five stations of a built section
  against its end-to-end chord decide whether the section is that
  segment (`STEP_EDGE_ON_DESCRIPTION`). **This class**; its direction
  is also on `a-stored-direction-is-taken-from-a-format-gate-with-no-length-decision`.
- `build.rs`, `held_gap`: five stations — the `CarrierBuilt` move, a
  reported measurement ("measured as read"), not a decision. NaN is
  still dropped.
- `build.rs`, `deferred_carrier`: the refused gap's report on a path
  that refuses anyway. NaN is dropped from the report.
- `build.rs`, `cap_row`'s `off_plane`: a max over the row's CONTROLS,
  which bound the row (convex hull) — sound; not this class.
- `reach.rs`, `axes_apart`, and `snap.rs`, the plane gap: maxima over a
  segment's two ends of a quantity convex along it (a distance from a
  line; `|n·q − c|`) — exact at the ends for the first; the second's
  outer `| · − r|` is not convex, so it is a witness there. Not
  decided here.
- `normalize/singularity.rs`, the reach bound: a sum of upper bounds
  in to-nearest arithmetic, folded; an ulp-level rounding gap of the
  `curve-recognition-roots-its-certified-sups-to-nearest` kind.

What would close it: read each decided gap as a sup (a hull or
composite, as the recognizers do), or state at each site why the
stations bound it; fold with a NaN-propagating max.
