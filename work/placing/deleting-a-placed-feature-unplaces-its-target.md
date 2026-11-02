---
id: deleting-a-placed-feature-unplaces-its-target
kind: issue
title: Add a fillet then delete it and the body leaves the world: the add gesture re-points the target's placement, the delete cascades it
status: parked
opened: 2026-10-08
priority: P1
cost: M
branch: placing/gesture-placements
pr: 6280
blocked_on: [a-mate-on-a-taken-over-copys-face-goes-stale]
design: true
---

Found by the review of INTENT stage 2 C (PR #4359, MINOR-4); filed on
the orchestrator's ruling.

Adding a fillet (any feature gesture) re-points the target body's world
placement to the fillet in the same action (`feature_over` in
`crates/viewer/src/session.rs`). Deleting the fillet
(`Session::delete_node` → `cascade_delete_order`) deletes the fillet's
placement with it, so the pre-fillet body is left unplaced and the part
vanishes from the viewport. Before C, A10's `on_delete` put the
target back in the root list.

Cascading is right for the kernel: re-pointing on delete would infer a
re-point, which DM6 forbids, and keeping the placement would strand it.
The gesture pair add → delete is simply no longer an identity on the
world. The fix is the gesture's: the viewer's delete of a feature could
explicitly re-point the feature's placements back to its target, as the
add gesture re-pointed them forward, in the same action.

`story_authoring`'s rook row was restated in C to expect the rook to
leave the world when its merlon block is deleted.

## Priced (2026-10-11)

P1 M. A GUI defect Ev would meet in ordinary use (add a feature, delete it, the body leaves the viewport). The fix is local to the gesture, so it is not parked behind INTENT stage 3, though stage 3's `a-placement-is-the-bundle-of-mates` rewrites the placements it re-points.

## Parked (2026-11-01, PLACING orchestrator)

Parked on the row in `blocked_on`, a live lane rewriting the same gesture code; the reason is in `work/placing/log.md` (2026-11-01, track claimed).

## Unparked and dispatched (2026-11-01, PLACING orchestrator)

`a-new-body-lands-unplaced` closed in PR 6269: no gesture writes a world `Place` on the person's behalf, and one `Takeover` re-points a copy's source. The lane `placing/gesture-placements` re-reads this row against that code first.

## Review of PR 6280 (2026-11-02, single FULL) — not mergeable; parked on DOORS with a design question

The review kept its probe rows on `placing/gesture-placements-review-6280`, in `crates/viewer/tests/gesture_placements.rs`. Three of them fail on the PR head:

- **M1, sure** (`review_copy_of_copy`): box B, its world copy C, and a transform T of C. Adding a fillet to T and then deleting the fillet also deletes T, and the product falls back to C.
- **m1, sure** (`review_deleting_a_duplicate_hands_nothing_back`): deleting a duplicate re-points readers that no takeover ever re-pointed.
- **m2** (`review_every_gesture_round_trips`): for mirror, sketch-on-face and a fused pattern, deleting the node the gesture made leaves the gesture's other nodes standing and drawn.

M1 and m1 share one cause, and it is a class, not an instance. `taken_body` is a second, hand-written census of "which node kinds took a body over", inferred from node SHAPE rather than from what a gesture did. So any `Place` that stands in its source's own space reads as a takeover, whatever the history. The hand-back the code performs is therefore a splice of every seen-path reader, not the inverse of a takeover.

The PR's own words ("the inverse of the takeover that re-pointed it") describe a fact the document does not hold. The review also found the takeover's reader list ("a copy's source, a `⊔` operand, the product slot") restated in four places.

**Why parked, not a fix pass.** DOORS' `doors/take-over-rebinds` moves the forward re-point into the kernel (`Recording::take_over`), which also writes `DocEdit::Rebind` for the readers' references (m3). A hand-back that inverts only the re-point would leave those references rebound to the deleted feature's names, so the inverse owes a rebind as well. Where the inverse lives, and what it reads to know what was taken over, are questions about that door: recorded at the takeover, derived, or the kernel's own inverse beside `take_over`. They have several viable answers. They go to designers once the door has landed and its shape is fixed. Until then the PR goes back to draft, with its branch and the review's rows kept. The class (a takeover inferred from shape) is logged on `work/placing/log.md`.
