---
id: import-mints-the-wrap-tip-at-a-chart-singularity
kind: issue
title: import's pole form flips to the wrap tip: geom::pole_form mints one wrap edge to each singularity, DegenerateApexCone's two-half form retires, OCC's seamed cap and cone import as written
status: dispatched
opened: 2026-10-29
priority: P0
cost: E
---


Ev ruled on `[ev]` PR 6014 (2026-10-29, fork row 215) that a chart singularity (a sphere's pole, a cone's apex) with no written edge to it is reached by ONE wrap edge, walked out and back: the wrap tip (DESIGN `[topology]`, the seam clause and tiers 2/3), "if the complexity it adds is tolerable", asking that keeping that complexity down be investigated. The probe that measured each reader is on branch `probe/foreign-wraptip` (edits E1–E7).

PR 6007 built import's door minting the two-half form from `geom::pole_form`, its one home. Owed: `pole_form` mints the wrap tip; the three dispositions keep their detection (interior, inside an edge, a valence-1 vertex) and mint one wrap edge; a valence-1 vertex on one meridian (OCC's `seamed.step`, `freecad/cone_apex.step`) imports as stated; `EdgeFreeSphere` mints one vertex at each pole, one meridian, one face. Lands after the tier rules admit a tip and revolve builds one band, so import and native construction agree.

Signed: (FOREIGN orchestrator)

- 2026-11-02 — moved from FOREIGN to IMPORTGATE by `git mv` when FOREIGN merged into IMPORTGATE, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and FOREIGN had fallen far under it).

## Evidence from CLEAR (2026-11-13, `clear/seam-meridians`)

Until the wrap tip lands, the two-half form must store its two
meridians in one plane. The at-rest gate's injectivity check lists a
second meridian under the plane built from the first only where their
stored centres and axes are one value (`one_plane` in
`crates/topo/src/injective.rs`). Otherwise a pole refuses `Departure`.

- `EdgeFreeSphere` mints both meridians through
  `geom::pole_form::sphere_meridian` at `azimuths(d)`. The normal is
  `axis × ±d`, which is exactly negated, and the centre and reference
  are shared. That pair is one plane.
- The strut twin that `normalize/singularity.rs` mints is
  `geom::pole_form::half_turn_curve` of the file's seam. It turns the
  normal by `2(n·â)â − n`. Where `n·â` rounds to a nonzero value (an
  axis that is not a coordinate axis), the twin's axis is a rounding
  off `−n`, so the twin is not in the seam's plane. Read, not measured.
  For a circle whose normal is square to the axis, `azimuths`' exact
  negation is the spelling that keeps the pair in one plane.

- 2026-11-25 — moved from IMPORTGATE to LATHE by `git mv` (LATHE orchestrator), id and body unchanged, re-banded P0. The one-band build (PR 7414) regenerates `step-export/tests/fixtures/ball.step` as one face, which step-import refuses ("a pole-form meridian ends at a vertex the face's loop passes more than once"). That reddens five step-import rows: three in the ci profile (`review_probes::a1_*`, `records_follow_the_file::a_pinch_is_refused_at_tier_3_on_one_record_or_two`) and the slow roundtrip pair. So this row gates the one-band landing like its other consumer gaps, and lands into the one-band stack (Ev's ruling on `[ev]` PR 6014 and the landing order logged on LATHE 2026-11-22). The probe branch `probe/foreign-wraptip` (E1–E7) is the starting point.
