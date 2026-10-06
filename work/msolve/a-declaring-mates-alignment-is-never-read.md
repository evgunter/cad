---
id: a-declaring-mates-alignment-is-never-read
kind: issue
title: a declaring mate's authored Alignment is stored but never read: places compares gauge references by identity, and the at-rest gate reads no alignment
status: parked
opened: 2026-10-03
priority: P3
cost: M
design: true
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
---


Filed by the PLACE orchestrator from the designer pair on `[ev]` #3920 (read by both designers; no row has run it).

## What

A mate between instances on two gauges declares (A11 (2)). Its `Alignment` (frames, primitive, sense, rider) is written at insert and persisted, but nothing reads it: `places` (`crates/editor-core/src/mate/solve.rs`) decides placing by gauge-reference identity, and the at-rest gate (`crates/editor-core/src/assembly.rs`) verifies the declared contact without reading an alignment. So a declaring mate's frames are inert data. That is why the PLACE row `a-part-resting-on-a-gauge-cannot-follow-a-part-edit`'s knob 2 ("refused alike, whatever its frames") saw no difference between frame choices.

## The question

Should a declaring mate hold an alignment at all, or should the gate check it (the declared contact at the declared frames)? Either is a design choice on MSOLVE's and ASSEMBLY's ground; today the field promises a meaning nothing enforces.
