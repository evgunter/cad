# HELD — plan

Cut from DOORS on 2026-11-01. Every row came over with its id and body
unchanged. The four held-pick rows were parked on D′3 (PR 5636) until
the cut and were unparked at it, so re-read each against D′3's
placements and tools before writing its spec.

## The slate

**23 budget points** against 30.

| pri | item | cost |
|---|---|---|
| P2 | `the-viewer-authors-no-pick` | M |
| P2 | `the-viewers-numeric-gestures-read-a-frame-back-off-geometry` | M |
| P2 | `moves-of-drops-the-third-angle-near-gimbal-lock` | E |
| P2 | `a-seated-tools-held-node-is-drawn-nowhere` | M |
| P3 | `a-held-face-stays-marked-while-its-form-is-collapsed` | M (rides the seated-tool row) |
| P3 | `committed-nodes-do-not-light-the-faces-they-reference` | M |
| P3 | `the-drafts-keep-held-slot-ids-across-a-document-replacement` | E |
| P3 | `a-last-leg-no-close-can-follow-is-dropped` | M |
| P4 | `the-mate-and-blend-commit-rows-restate-tool-commit-row` | M |

## Order

1. `moves-of-drops-the-third-angle-near-gimbal-lock` (E): the fix is
   written in the row.
2. `the-viewers-numeric-gestures-read-a-frame-back-off-geometry` and
   `the-viewer-authors-no-pick`: both are a gesture writing something
   other than what the author picked or typed.
3. The held picks: `a-seated-tools-held-node-is-drawn-nowhere` with its
   passenger, then `committed-nodes-do-not-light-the-faces-they-reference`
   (re-state it against D′3's placements, where mates read only
   geometry) and `the-drafts-keep-held-slot-ids-across-a-document-replacement`.
   DOORS' P0 `a-mate-on-a-taken-over-copys-face-goes-stale` replaces
   `Renamed::through` with the takeover's rebind map, which re-seats held
   picks. Land this group after it, or say on DOORS' log which lands first.
4. `a-last-leg-no-close-can-follow-is-dropped`: half of it waits on
   PROFILETAIL's `a-close-from-a-tip-on-its-start-is-refused-as-a-tangent-seam`,
   and the other half is a design question about what the preview is.
5. `the-mate-and-blend-commit-rows-restate-tool-commit-row`: re-read it
   against D′3's mate tool.

## Lessons carried from DOORS

DOORS' `plan.md` carries AUTHOR's and AUTHTAIL's lessons, and they hold
here unchanged:

- units mint duplicates, so spec each one against that;
- re-verify every fix pass;
- test a gesture that adds or moves a body on a non-identity placement;
- run ε-sensitive sketch rows at `CAD_TOLERANCE_EPS=1e-6` and `1e-12`.

## Review posture

DOORS', unchanged: a style review plus a second correctness reviewer
on any unit whose failure is a confident wrong picture; E rows take
the orchestrator's read.

## Exit shape

Every row merged or re-homed with a reason. No exit criteria are set,
so no walk is owed.
