# FIT — the plan

the camera, the display budget and the pick index: what the viewer decides to show and how much of it

Opened 2026-09-20 by VGEOM's priority-seam cut
(`work/README.md`, Track size).

## The slate

**6 budget points** of dispatchable work, all at P3.

| pri | item | cost | title |
|---|---|---|---|
| P3 | `a-dense-floor-is-priced-off-the-law-from-the-scale-probe` | M | A floor mesh over B/16 breaks `Converged` at the scale probe, so the law runs from the floor |
| P3 | `flat-harmlessness-is-unproven-on-the-nurbs-refinement-lane` | M | The Flat bound is per direction, and `mesh::trimmed`'s certificate refinement is outside it |
| P3 | `mispaired-ids-exempts-the-empty-window` | E | `MispairedIds` skips the zero case; first establish whether a zero-name part with patches is reachable (`NodePick::patch_names`, editor-core) |

Parked on the D10 hold (stage 3 D′,
`work/intent/a-placement-pins-by-written-values-without-a-mate.md`,
`[ev]` PR 5494):
- `a-moved-copys-pick-span-crosses-its-frame-unconverted` (P1, E);
- `one-failed-root-refuses-the-whole-pick-index` (P2, M, design).

Re-read both against the walk D′ leaves once it lands. Each body
says what to check.

## Order

The two budget-probe rows share `scene::fit_delta`'s ladder and the
Flat paragraph that #5600 wrote, so one lane can take both.
`mispaired-ids…` is independent.

## Review posture

The repo-wide tiers (`memories/orchestration-model.md`). The
2026-10-22 sitting used:
- full review for changes that moved a type or a frame order, or added
  a certified door;
- style review for the rest;
- the orchestrator's own read for E rows.

