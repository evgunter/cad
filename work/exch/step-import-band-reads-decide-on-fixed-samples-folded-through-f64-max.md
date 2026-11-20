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

Hits (`crates/step-import/src/`), by name and line as of EXCH unit B's
merge base (lines drift; the name is the citation):

- `build.rs` `on_description` (~1351): three stations (ends, middle) of
  the printed carrier against the built one decide
  `STEP_EDGE_ON_DESCRIPTION` (`agree_at`). **This class.**
- `build.rs` `cap_row` (~1562): three stations of the printed edge
  against the wall's row decide the same name. **This class.**
- `build.rs` `rebuilt_in_surface` (~2425): five stations of a built
  section against its end-to-end chord decide whether the section is
  that segment (`STEP_EDGE_ON_DESCRIPTION`). **This class**; its
  direction is also on
  `a-stored-direction-is-taken-from-a-format-gate-with-no-length-decision`.
- `build.rs` `held_gap` (~1331): five stations — the `CarrierBuilt`
  move, a reported measurement ("measured as read"), not a decision.
  NaN is still dropped.
- `build.rs` `deferred_carrier` (~1144): the refused gap's report on a
  path that refuses anyway. NaN is dropped from the report.
- `build.rs` `cap_row`'s `off_plane` (~1512): a max over the row's
  CONTROLS, which bound the row (convex hull) — sound; not this class.
- `reach.rs` `axes_apart` (~76) and `snap.rs` the plane gap (~412):
  maxima over a segment's two ends of a quantity convex along it (a
  distance from a line; `|n·q − c|`) — exact at the ends for the first;
  the second's outer `| · − r|` is not convex, so it is a witness there.
  Not decided here.
- `normalize/singularity.rs` `reach_on` (~603): a sum of upper bounds
  in to-nearest arithmetic, folded; an ulp-level rounding gap of the
  `curve-recognition-roots-its-certified-sups-to-nearest` kind.

**The blind spot of the `fold(0.0, f64::max)` pattern**, swept by a
second pass (`x = x.max(…)` accumulators; reviewer 2's hits on PR 7170):

- `entities.rs` `in_box_displacement` (~2736):
  `farthest = farthest.max((…).norm())` — a NaN norm is dropped as 0, so
  a poisoned placement reads as no displacement. **NaN half of this
  class.**
- `one_record.rs` (~387): `reach = reach.max(r)` over a record's
  levels — NaN dropped. **NaN half.**
- `recognize.rs` (~479): `worst = worst.max(cell + slack)` — the cell
  above is NaN-propagated by hand (`if r.is_nan()`), and the fold then
  drops it. **NaN half**; unit A's file (`exch/promote-torus-sphere-cone`).

What would close it: read each decided gap as a sup (a hull or
composite, as the recognizers do), or state at each site why the
stations bound it; fold with a NaN-propagating max.
