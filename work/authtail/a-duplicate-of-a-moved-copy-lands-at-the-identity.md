---
id: a-duplicate-of-a-moved-copy-lands-at-the-identity
kind: issue
title: The duplicate tool places its copy at the identity, so a duplicate of a moved copy lands beside where the body stands in its own space, not beside the copy
status: open
opened: 2026-10-25
priority: P1
cost: E
---


Found by the review of PR 5683 (the mirror tool), which had the same defect and fixed it there. `DocSession::add_duplicate` (`crates/viewer/src/session.rs`) measures the step off the body in its own space (`combine::duplicate_step`). It then places the copy with `place_in_world`, which uses the stand bundle, i.e. the identity. The original's world copy keeps its own placement, so the duplicate lands relative to where the body stands in its OWN space, not relative to where it is drawn.

Measured: a 40 × 20 × 10 mm block whose world copy is pinned at T(0.1, 0, 0) is drawn over x ∈ [0.08, 0.12]. Its duplicate is drawn over x ∈ [0.032, 0.072]: on the −x side of the original, where the rule promises "+x, clear of the original" (`STEP_DIRECTION`, `duplicate_note`).

The mirror's repair is the pattern to reuse here: `beside` in `session.rs` places a new body by the source copy's own bundle as written (`Node::written`). For a viewport pick on a copy, the duplicate's copy would take that copy's placement. For a tree pick, it would take each world placement of the source (`combine::mirror_arrivals`'s rule). Check the row with a test that reads the drawn copy's geometry on a non-identity placement. The suite `crates/viewer/tests/mirror_authoring.rs` (`moved_block`, `x_span`) has the fixture.
