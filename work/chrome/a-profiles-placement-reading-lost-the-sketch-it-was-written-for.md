---
id: a-profiles-placement-reading-lost-the-sketch-it-was-written-for
kind: issue
title: world.rs's Node::Profile arm of built_on_copy and placement_reading answers for a sketch no viewer act now authors
status: open
opened: 2026-11-22
priority: P3
cost: M
---

Found by VSEAM's review of PR 7466 (`vseam/sketch-on-face-retires`, 2026-11-22), which retired the fused `SessionOp::SketchOnFace`. Filed by the VSEAM orchestrator.

`crates/viewer/src/world.rs`'s `built_on_copy` / `placement_reading` has a `Node::Profile` arm. It was written for a profile sketched on a placed copy, which is what the fused act authored: the profile drawn on its own plane, its extrude placed on the copy's face, then fused. That act is gone. The viewer's only on-face sketch is now a profile drawn on the face's chart (`SessionOp::DrawOnFace`, a `DrawnOn` frame), and by `built_on_copy`'s `from_nothing` rule (Ev, PR 6917) such a profile is not "on the copy".

What the arm now mostly yields, according to the review's mutant (`copy_downstream_of` returning `None` flags only it), is the bracket's own original profile, read as "on the placed copy … belongs to that copy alone". It reads that way only because a fillet of the copy sits downstream of it. The reviewer judged that answer arguably wrong, and a defect wearing documentation (unsure). `gesture_placements.rs`'s `on_this_copy_at_depth_and_through_member_copies` profile loop now pins it.

Two things need deciding:
- whether the Profile arm still has a case to answer;
- if it does not, whether that loop's expectation is right.
