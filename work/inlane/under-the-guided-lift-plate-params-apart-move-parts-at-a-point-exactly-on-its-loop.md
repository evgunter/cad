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

- **Move:** Apart (`crates/editor-core/tests/revalue_corpus.rs`, `Move::Apart`). Every one of plate_param's 41 continuous variables moves by its own multiple of 1e-7; every authored length is a free variable, not only `hole_r`.
- **Decision:** #6035, `point_in_loop_arm`. The margin is 0.0 exactly at p0 and 1.9852281229370488e-7 at p1, so the sign flips. After it the two builds decide different predicates: they agree through #6035 and first differ at #6036, of 15,539. The apart move compares only those 6,035 (no laundering in them).
- **Repro:** `REVALUE_APART=1 REVALUE_DETAIL=1 REVALUE_DOC=plate_param cargo nextest run -p editor-core --run-ignored only -E 'test(=revalue_corpus::revalue_census)' --no-capture`

Under the pinned lift the profile did not move with its variables, so this decision never moved either. At the nominal, a point the boolean classifies lies exactly on a loop of the guided profile. The degeneracy is in the document's nominal geometry, a coincidence by value. Read whether it is a held coincidence the decision should take by name, or a classification at an exact boundary.

The gating row `revalue_plate_param` is now pinned under the in-step move (15,539 compared, 7,877 moved, nothing laundered). The apart move's power over this document is lost until this is resolved.

## A second instance: declared_tangency (NOTHING PINS IT)

It is the same class: a decision exactly 0 at the nominal that flips under the move.

- **Row:** `revalue_declared_tangency`, Apart move, guided lift.
- **Decisions:** #2874 and #2961, both `pcurve_chart_azimuth_frame`. Each margin is 0.0 exactly at p0 and −1.5416663312218278e-7 at p1, so the sign flips.
- **Unlike plate_param, the builds do not part:** all 2,965 decisions compare, and nothing re-values away after the flips (`after_flip` is empty).
- **Repro:** `REVALUE_DETAIL=1 REVALUE_DOC=declared_tangency cargo nextest run -p editor-core --run-ignored only -E 'test(=revalue_corpus::revalue_census)' --no-capture` (the census line lists `flips`).

The row allows these flips today. A row with no `flips` entry asserts nothing about flips that leave the re-valued margins in agreement. So nothing pins this instance, and it goes away, or moves, unseen.
