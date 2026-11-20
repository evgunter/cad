---
id: the-stackups-tangents-are-checked-against-finite-differences
kind: unit
title: The stackup's tangents are checked against finite differences of the f64 run
status: open
opened: 2026-11-20
priority: P3
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
---

A check that would have caught ATREST's `link-lowest-on-freezes-its-candidates-so-a-dual-sensitivity-through-a-tie-drops-a-term`: run the recipe at plain `f64` with one parameter nudged, take the difference quotient of a measurement, and compare it with the tangent the dual run reports. PR 3724 chose the instrument for the blend (`blend_dual_sensitivity`).

Ev, PR 7155: "the stackup test sounds reasonable to have behind a debug_assert, if i'm understanding correctly?" The orchestrator answered that it costs two more evaluations of the recipe per toleranced parameter, so it reads as a suite row over the dual corpus; whether it also runs in debug builds is Ev's to say. Not specified until that is answered.
