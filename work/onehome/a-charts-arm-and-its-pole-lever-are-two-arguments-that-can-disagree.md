---
id: a-charts-arm-and-its-pole-lever-are-two-arguments-that-can-disagree
kind: issue
title: Chart::charted takes the bundle's Arm and its pole verdict's Lever as two arguments, and read_back passes them disagreeing
status: open
opened: 2026-11-15
priority: P3
cost: E
---

Raised by PR 6988's FULL review (the decided least-turn mint, fork 270),
as a style finding.

**What is there.** `Chart::charted`
(`crates/editor-core/src/mate/value.rs:352`) takes `arm: Arm`, which the
chart stores and levers its span verdicts over, and `lever: Lever<T>`,
which its least turn's pole verdict and conditioning are decided over.
`Chart::of` passes the two as one quantity (the lever is
`Lever::unsourced(arm)`). `read_back` (`value.rs:918`) passes
`Arm::of(0.0, 0.0)` together with `Lever::session_box()`, so there the
chart's span verdicts are levered over nothing while its least turn is
levered over the session box.

**Why it matters.** Two arguments that mean one length can be passed
disagreeing, and one caller does. Each verdict in the chart should be
levered over the same arm.

**What closes it.** `charted` takes one of the two and derives the
other. That needs `read_back` to hold the copy's own reach, which is
`work/onehome/the-gesture-read-back-decides-its-least-turn-over-the-session-box.md`;
the two rows close together.
