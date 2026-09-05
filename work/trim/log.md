# TRIM log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/trim/plan.md`. A/B band 2500–2599
(`docs/MODEL-AB-LOG.md` owns every live experiment number).

## Opening state (2026-09-03)

Opened on Ev's direction (in-chat, 2026-09-03: "proceed to actually
creating these tracks with their own directories in work/") from the
2026-09 work-track proposal, `docs/WORK-TRACKS-2026-09.md`, whose TRIM section is the
charter this plan restates. Opens when CURVED lands the rim arms. Items re-homed into this
directory at opening, by header edit and `git mv` only (ids unchanged):

- `clearance-window-tightening-needs-chart-boundary` from `work/m10/`
- `interior-iso-curve-de-boor-extractor` from `work/issues/`
- `general-pcurve-face-props-and-tess-refuse` from `work/issues/`

No unit is cut and no branch exists yet. The first dispatch claims its
ordinal from the band above and records it in `docs/MODEL-AB-LOG.md`.

## Opened for dispatch (2026-09-04)

Same orchestrator as CURVED (branch `curved/orchestrator` carries both
programs' state-sync; unit branches stay `trim/<unit>`). The tracks
doc's gate "at CURVED's rim arms" was traced and found to name this
program's own file: the rim arms are `nurbs_iso_derive`'s in
`topo/src/pcurves.rs` (P-2, PR #1177: cap-rim arm widened, wall-seam
arm reverted), and the wall-seam arm's blocker is the de Boor
extractor — item 1 here. Ev confirmed in-chat (2026-09-04) that the
order is the orchestrator's call; the plan's §Opening condition and
§Order are revised accordingly: the extractor opens, the props/tess
lane measures behind it, the clearance-window description runs in
parallel. Riders on the Track Q files (`D36`, `S394`, `S83`, `D305`,
`fitted-magnitude-nan-schedule-parameter`) land with whichever unit
opens their file. First action after this PR: write the extractor's
spec (Fable) and dispatch it under the A/B protocol from band 2500.

## First dispatches (2026-09-04, later)

- **`docs/TRIM-3-SPEC.md` ratified (#1862)** — the chart-boundary
  description (PR-1, `topo` only) and the clearance-window seam (PR-2).
  Refuted on the way: cell dropping alone does not flip the L-cap row
  (the exhibit arm's 9-station lattice is a third consumer site);
  extruded bodies carry no stored pcurves (the description derives via
  `walk_loop`); a box fixes neither planar row; SHELL-3 moves the same
  functions (PR-2 sequenced by announcement; SHELL-3 not dispatched).
  Rulings §9: keep `ClearanceReport::windows`; new `chart_bound.rs`;
  the `WINDOW_TIGHTENING` const stops promising and the exact-region
  recourse gets its own item.
- **`docs/TRIM-1-SPEC.md` ratified (#1865, rulings #1876)** — the de
  Boor collapse extractor. Refuted: `an_interior_column_still_refuses`
  is not the row this unit flips (it is an arc-class row refusing at
  the schedule residual); the wall–seam revert cited a test's name as
  its reason; no `geom` primitive is needed. `Pcurve::IsoLine` gains
  interior columns (no new variant); rational class = weight nets
  separable by structure; riders S394 and
  `fitted-magnitude-nan-schedule-parameter` carried.
- **Block TRIM-B1** drawn branch-side (`trim/b1-block`): byte 43 ⇒
  fable at slot 1. Slot 0 = TRIM-3 PR-1 (Opus) on `trim/3-chart-bound`;
  slot 1 = TRIM-1 (Fable) on `trim/1-de-boor-extractor`; slot 2 =
  TRIM-3 PR-2 (Opus), opens after PR-1 merges with the seam announced
  to SHELL and M10.
- Both spec lanes were starved of the build mutex (1–2.5 h waits under
  load 25–40) and pre-registered their one measurement as the
  implementer's first act; see the CURVED log's operations note.
