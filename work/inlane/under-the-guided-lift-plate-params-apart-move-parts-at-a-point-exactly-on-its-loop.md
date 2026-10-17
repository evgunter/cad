---
id: under-the-guided-lift-plate-params-apart-move-parts-at-a-point-exactly-on-its-loop
kind: issue
title: Under the guided lift, plate_param's point_in_loop_arm margin is exactly zero at the nominal, so the revalue apart move flips it and the builds part at decision 6035 of 15539
status: open
opened: 2026-10-17
priority: P3
cost: M
---


Found by the shared replay session unit (`inlane/sym-replay-session`). The revalue instrument now reads the door's session with the guided lift (`editor_core::coincide::with_replay_session`).

- **Move:** Apart (`crates/editor-core/tests/revalue_corpus.rs`, `Move::Apart`; plate_param has one continuous variable).
- **Decision:** #6035, `point_in_loop_arm`. The margin is 0.0 exactly at p0 and 1.9852281229370488e-7 at p1, so the sign flips. After it the two builds decide different predicates. They part at 6,035 of 15,539 decisions, and the apart move compares only that prefix (no laundering in it).
- **Repro:** `REVALUE_APART=1 REVALUE_DETAIL=1 REVALUE_DOC=plate_param cargo nextest run -p editor-core --run-ignored only -E 'test(=revalue_corpus::revalue_census)' --no-capture`

Under the pinned lift the profile did not move with `hole_r`, so this decision never moved either. At the nominal, a point the boolean classifies lies exactly on a loop of the guided profile. The degeneracy is in the document's nominal geometry, a coincidence by value. Read whether it is a held coincidence the decision should take by name, or a classification at an exact boundary.

The gating row `revalue_plate_param` is now pinned under the in-step move (15,539 compared, 7,877 moved, nothing laundered). The apart move's power over this document is lost until this is resolved.
