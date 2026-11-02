---
id: the-mate-panel-names-no-slide-a-plane-leaves
kind: issue
title: The mate panel offers every slide and refuses by count, naming none of the two a plane leaves
status: open
opened: 2026-10-26
priority: P2
cost: M
rides_with: mate-tool-commits-one-mate
---


Found by PR 5742 (the value chart reads a written orientation), deviation 4.

Under the value chart, which two target axes a plane mate leaves as slides depends on the mate's orientation: the plane fixes the slide along the axis the orientation puts the copy's mated direction on, and leaves the other two. The mate panel (`crates/viewer/src/matetool.rs`, `MateFreedom::slides_left_by` and `MateFreedom::left_by`) can't know which axis that is before the gesture snaps. So it offers all three slides and refuses a choice by count (`NotAFreedom`): more slides than the kind leaves.

So a person can still name `Slide(Y)` on a wall whose mate fixes `y`. They learn of it only as the chart's `SlidesDoNotSpan` refusal after the commit. Nothing on the panel tells them which two slides the plane leaves.

**Owed.** Once the panel has both picks, it should read the orientation the snap would write (`crates/viewer/src/session/snap.rs`, `snapped`) and name the two slides the plane leaves (the one an axis mate leaves). It should offer those and grey the third, with its reason.

**Done when** a row picks a wall face, sees the panel offer the two slides across the wall's normal, and sees the fixed one refused at the panel, not at evaluation.

## Rides with the bundle editor (2026-11-01, PLACING orchestrator)

The decision on `mate-tool-commits-one-mate` derives the value rows from the kernel: the panel lists exactly the freedoms the draft's mates leave, with the door's verdict live before Apply. That is this row's "done when", so it closes with that unit.
