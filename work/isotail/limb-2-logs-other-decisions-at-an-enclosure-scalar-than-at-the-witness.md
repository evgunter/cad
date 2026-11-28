---
id: limb-2-logs-other-decisions-at-an-enclosure-scalar-than-at-the-witness
kind: issue
title: Limb 2 logs other decisions at an enclosure scalar than at the f64 witness
status: open
opened: 2026-11-24
priority: P1
cost: M
---

Found by reading, in the sweep PR 7716 ran for checks logged at one
scalar and not another; reproduced on that PR's head by its review's
parity run (below). Filed by BOXFINISH's witness-premises unit and
moved here: `crates/geom-brep/src/edge_nurbs.rs` is ISOTAIL's and
SELECT's ground, and rung 3's certificate of an edge on its carrier is
an at-rest certificate, ISOTAIL's charter, not a selector.

`limb2` (`crates/geom-brep/src/edge_nurbs.rs:2303`) branches on the
scalar's witness kind, and on one arm the two kinds state different
decisions on the log:

- **`Offset::Miss { bound: Ok(_), .. }`.** A point scalar decides
  `ssi_hull_value` (`measured_miss`, `edge_nurbs.rs:2543`) and, when
  that is in the band, `ssi_hull_sup` (`upper_bound`,
  `edge_nurbs.rs:2574`): two decisions. An exact-witness scalar takes
  `limb2_enclosure_as`, which states one (`edge_nurbs.rs:2350`).
- **`Offset::Bound(_)` does not differ.** It is only ever built with
  `floor: 0.0` (`pcurve_cache/projected.rs:2892`–`2895`,
  `:2940`–`:2943`), so `limb2_enclosure_as` names `ssi_hull_sup` there,
  as the point arm does.

Both doors go through `dihedral::decide_reported`
(`crates/geom-brep/src/dihedral.rs:259`), the logged funnel, and the
verdicts are choices.

## Reproduced

An `f64` build and an `Interval` build of one document at one point,
their verdict vectors compared (`editor-core`'s
`certifying_vector`), on PR 7716's head merged with main `3ff862c423`:

| document | first parting, `f64` against `Interval` | `pcurve_projected_lever`, `f64`:`Interval` |
|---|---|---|
| `tube_arc` | row 0, #213: `ssi_hull_sup` (Zero) against `pcurve_projected_lever` (Positive) | 8:12 |
| `hollow_tube_elbow` | row 0, #378: the same pair | 16:24 |
| `cone_on_sphere` | row 3, #135: the same pair | 8:10 |

No other count differs on those three. So what parts them here is the
number of `pcurve_projected_lever` rounds before `ssi_hull_sup`, more
at `Interval` (the class row's trace: the lever decisions run per
round until a read clears, which outward rounding moves; and the
parts are halved on a spread reading that takes "spread" where it is
undecided, `pcurve_cache/projected.rs:3165`–`3204`), not the
`Miss` arm above. No run has shown the `Miss` arm's two-against-one.

Under today's certificate (equal vectors) a leaf of any of these
documents at its own witness cannot certify.

## What would close it

Each of the two, separately: the lever rounds counted alike at both
kinds or taken off the log on a ledger row that argues it; and the
`Miss` arm stating the same decisions at both kinds, with a row that
reaches it. The class is
`the-two-lanes-decision-logs-differ-at-one-point-with-the-build-the-same`
(BOXFINISH).
