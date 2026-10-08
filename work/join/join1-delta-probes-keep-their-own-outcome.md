---
id: join1-delta-probes-keep-their-own-outcome
kind: issue
title: join1_delta_probes judges SOUND by its own outcome, weaker than common::differential::outcome
status: open
opened: 2026-10-04
priority: P4
cost: E
branch: join/battery-hygiene
---


Found by PR 4031's review (S7, NOTE-3).

## What

`crates/sweep/tests/join1_delta_probes.rs` keeps an `outcome` of its
own beside `crates/sweep/tests/common/differential.rs` `outcome`. Its
SOUND requires neither the at-rest certificate nor a legal operand, and
judges the volume to 1e-6 against a 4 096-chord oracle. PR 4031's
headline ("477 lines move to OK SOUND", `join1_delta_arc_battery`) was
judged by it; the review re-judged all 477 through the differential
checks, with Richardson-extrapolated volumes, and every one is SOUND
(`review_4031_probes.rs` `r4031_arc_battery_differential`, on the
review branch). So the drift has cost nothing yet, but a battery line
reading SOUND here is weaker than the same word elsewhere.

## The shape of a fix

Route the battery through `common::differential::outcome`. Its arc
oracle has to reach 1e-7, so take the review's extrapolated chord areas
or the closed forms (segments and lenses of the shapes' circles). Then
delete the local `outcome`. The battery's lines move in their wording,
and a main-vs-head diff has to be re-baselined across that change.
