---
id: a-body-moved-by-a-boxed-turn-refuses-its-edges-and-pcurves-at-first-order-in-the-box
kind: issue
title: A body moved by a boxed turn refuses its edge certificates and pcurves at first order in the box, past check 1
status: open
opened: 2026-11-21
priority: P2
cost: M
---

Filed by ONEHOME (2026-11-21) from the measurement on
`work/onehome/check-1-reads-a-turning-frames-length-off-its-bracket.md`.
It stands behind that row: with check 1's frame half gone
(`carriers-hold-their-frame-as-unitvec3-and-rigidmotion-and-check-1s-frame-half-goes`),
the moved body still does not finish, at every width check 1 refused.

**The case.** A disc of radius r = 5 mm and depth r, built at `Interval`
on `SketchPlane::xy()`, moved by `topo::transform_rigid` with
`RigidMotion::rotation(y, 30° ± δ)`, then `topo::finish`ed. A second
copy, translated 3r, finishes, and the two are united. The probe is
`crates/sweep/tests/check1_turned_probe.rs` on the unmerged branch `onehome/check1-turned-probe` (`e487c39388`, on main `6abbae4ae5`), row `check1_turned_disc_behind`.

**What the finish reads besides check 1's datums:**

| δ | edge certificates | pcurves | union |
|---|---|---|---|
| 1° … 1e-5° | 3× `carrier_endpoint_start` | 2× `bool_pierce_normal_on_chart`; 2× each of `pcurve_cylinder_chart_incident`, `pcurve_line_chart_incident`, `pcurve_plane_chart_incident` and `pcurve_point_chart_incident`; 4× `pcurve_row_interval` | not reached |
| 1e-6° (check 1 passes) | 2× `ssi_hull_sup` (`AnalyticRung3` stalled), 1× `pcurve_line_chart_incident` | `pcurve_cylinder_chart_incident`, `pcurve_envelope`, 2× `pcurve_line_chart_incident` | not reached |
| ≤ 1e-7° | none | none | builds, volume `[7.853981e-7, 7.853982e-7]` against the two discs' `2π·r³ = 7.853982e-7` |

The edge certificates' `carrier_endpoint_start` margin is first order
in the box. Its widest bracket runs `[0, 3.37e-4]` at 1°, and ten times
less each decade to `[0, 3.37e-9]` at 1e-5°. The pcurve findings move
with it. It reads the same on `onehome/carrier-frames` at `6762df403e`
(PR 7131).

**Through the document**, BOXFINISH's M3 probe (`boxfinish/probe-switch-off`
`bba45062`, main `4c7404c2`, not re-measured here) read further refusals
past check 1 on a union of the turned copy:
- `VolumeUncomputable` on `props_cylinder_circle_off_axis` at 1e-6°;
- `FlipCrossing` and `PredicateDivergence` (`pcurve_cylinder_chart_axial`, 0 against 32) at 1e-8° and 1e-9°.

The check-1 row's 2026-11-14 section records them.

**Why it matters.** A later finish of a body a boxed turn moved is where
a document meets check 1 (the check-1 row). Removing check 1's frame
half is not enough for such a body to finish: it needs these readings
to decide too, or a later finish to carry the facts the finished input
held (`a-later-finish-could-carry-facts-whose-data-is-bit-identical-to-a-finished-inputs`).
P2: the switch deletion is not blocked on it.
