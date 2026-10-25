---
id: a-duplicate-of-a-moved-copy-lands-at-the-identity
kind: issue
title: The duplicate tool places its copy at the identity, so a duplicate of a moved copy lands beside where the body stands in its own space, not beside the copy
status: closed
opened: 2026-10-25
priority: P0
cost: E
closed: 2026-10-25
pr: 5693
---


Found by the review of PR 5683 (the mirror tool), which had the same defect and fixed it there. `DocSession::add_duplicate` (`crates/viewer/src/session.rs`) measures the step off the body in its own space (`combine::duplicate_step`). It then places the copy with `place_in_world`, which uses the stand bundle, i.e. the identity. The original's world copy keeps its own placement, so the duplicate lands relative to where the body stands in its OWN space, not relative to where it is drawn.

Measured: a 40 × 20 × 10 mm block whose world copy is pinned at T(0.1, 0, 0) is drawn over x ∈ [0.08, 0.12]. Its duplicate is drawn over x ∈ [0.032, 0.072]: on the −x side of the original, where the rule promises "+x, clear of the original" (`STEP_DIRECTION`, `duplicate_note`).

**Repair (decided by the mirror arrival fork's designer pair, 2026-10-25; both designers, likely).** A duplicate is a second, independent placement of the same source, written where the user sees the original and stepped clear: values on every freedom at the clicked copy's evaluated motion, plus the step, with no mates. Do NOT reuse the mirror's first-pass `beside`, which re-authored the copy's bundle: it refuses a mated copy (`NotOnTheCopy`), a fully mated bundle has no free slide for the step, and it was deleted. A duplicate is meant not to follow the original (`[repetition]`: "an irregular family is its members written out, each saying where it is"). Write the frame the way the free-move writes one (`moved_frame` / `FrameMove`). Test it on a non-identity placement, and on a mated copy, by reading the drawn copy's extent. After G (`patterns-are-index-variables`), today's spelling (a `Pattern` of two, a `Part`, a placement) becomes that one placement. Re-banded P0 at the mirror's merge: a normal verb with a live wrong answer.

## Closed

PR 5693. The duplicate's `Part` is placed into the clicked copy's target space by values at that copy's evaluated motion (`combine::copy_motion`, `moves_of`), with no mate; the step stays the pattern's, so it runs along the copy's own x. `SessionOp::Duplicate` carries the clicked copy (`beside`); a tree pick of the body places one duplicate beside each drawn copy. Rows: `crates/viewer/tests/duplicate_beside.rs` (moved, mated, quarter-turned and general-pose copies, two copies, and the `NotACopy` refusal).
