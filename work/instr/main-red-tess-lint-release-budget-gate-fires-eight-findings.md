---
id: main-red-tess-lint-release-budget-gate-fires-eight-findings
kind: issue
title: main red: the nightly k-lint (release-budget) tess-lint gate fires 8 findings (klein_loop triangles 1.09x, five re-keyed twisted/spring faces)
status: closed
opened: 2026-11-09
closed: 2026-11-10
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), job `k-lint (release-budget)`: `tess-lint: GATE FAILED — 8 finding(s)` against `docs/tess-budget-data/tess-budget-baseline.csv` (cut at `4809ce56162e`):

- `klein/klein_loop`: triangles 80378 → 87512 (1.09x), and its face 2 re-keyed (nv 198 → 208);
- face 2 re-keyed in `twisted_tube` (nv 544 → 520), `twisted_duct` (nv 682 → 652), `twisted_duct_shadow_z` and `twisted_duct_shadow_y` (nv 482 → 461);
- face 2 re-keyed in `projectbox/spring` and `projectbox/cutaway_spring` (nu 2 → 1).

No open row covers it: `work/green/tess-lint-gate-red-on-main-23-findings.md` and `work/instr/tess-gate-red-beyond-twopeg-since-the-10-02-cut.md` are closed.

The gate was already red before any of BOXES' cover merges: the 11-02 nightly (run 41393708363, before PR 6336) fired 6 of these findings (klein face 2, twisted_duct, both twisted_duct_shadow rows, both springs). The 11-07 nightly (run 42342466395) fired the same 8 as tonight. So the klein triangle growth and the `twisted_tube` row arrived between 11-03 and 11-07, a window with no BOXES merge.

Recourse is the gate's own: read each re-keyed face in the model first, then re-cut the baseline with `demo-tour tess-budget` and say why.

(BOXES nightly-reds triage, 2026-11-09)

## Closed (2026-11-10)

Reproduced on main `35e3f16882`: the same 8 findings, with the same
values as nightly 88. Each finding was read in the model, then the
baseline was re-cut. Nothing here is a defect.

The causes come from three release sweeps (`--sizing-only`) at merge
boundaries, compared column by column with the old baseline:
- at `060cdc5a13^1` the sweep equals the old baseline;
- at `060cdc5a13` it equals the sweep at `5604b7c88b^1`;
- at `5604b7c88b` it equals main on every sizing and identity column
  of all 1677 rows.

So every sizing move is one of two merged PRs. Both were deliberate
frame changes, and both PR bodies list these scenes as expected to move:

- **PR 6138** (`060cdc5a13`, merged 10-31). The sweep frame became the
  rotation-minimizing frame by double reflection, and a sweep's v
  became the path parameter `i/(n−1)`. Its findings:
  - `twisted_duct` and both shadows (nv 682 → 628; nu 3 → 2): the
    square no longer carries the old frame's extra spin, so `muv` and
    `mvv` fall.
  - `projectbox/spring` and `cutaway_spring` (nu 2 → 1): the coil's
    square wire stops spinning about the helix. `mvv` drops from 1415
    to 265 and the walls go from about 7700 triangles to 2696.
  - `klein_loop` face 2 (nv 198 → 208): the loop is now `sweep_body`
    on `exact_spine` (rational quarter arcs), no longer the
    hand-built loft on an interpolated spine. Its scene total moved
    1.033x.
- **PR 6465** (`5604b7c88b`, merged 11-03). A sweep's start frame is
  the least turn of a written frame (`turned_onto`), replacing the
  retired `path_start_frame` ladder. Its findings:
  - `twisted_tube` (nv 544 → 520): its loft stations are now the
    sweep's own carry (`tube_places`).
  - `twisted_duct` and its shadows: they start from a different roll.
  - the springs: the square starts at another roll, so faces 2 and 4
    trade their readings.
  - the klein triangle growth (80378 → 87512, 1.09x): the loop's start
    frame is now the world frame, so its circles' seams lie in the
    spine's plane `y = 0`. Each half-wall now runs from the inside of
    the bend to the outside. `NurbsCellGrid::band_schedule` sizes a
    band to its worst column (`row_bound`), so both halves are now
    sized to the bend's worst cells, where before one half was not.
    `span_opt_cells` over the four walls moves only 37946 → 38152,
    while `grid_cells` moves 35769 → 38982. The growth is the banding
    forfeit the meter already reports, not a sizing change.

Also folded in, with no finding:
- the lily leaves and `s_duct`: bound columns moved in their last
  digits (PR 6138's v law, and PR 6465's half-turn start roll on the
  centred square of `s_duct`).
- the teapot's `name` column: PR 6341 (11-02) names loft walls by a
  minted `SectionId`.

The re-cut commit (`ea6e660458`) names each finding. The censuses in
`tools/tess-lint/tests/baseline_census.rs` were re-pinned to the new
readings. The swap census's second alarm now reads the springs' faces
3 and 5, which is 0.4% apart and inside the tolerance.
