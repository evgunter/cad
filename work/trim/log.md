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
