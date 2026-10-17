---
id: the-solve-reads-a-face-only-of-an-instance
kind: issue
title: The mate solve reads a face only of an instance: a face side, or a needed lever, on a Place copy or a raw construction refuses typed
status: open
opened: 2026-10-11
priority: P1
refs: [a-mate-relates-two-poses]
---


Stage 3 B's named interim (the B4 ruling): the mate solve runs before evaluation and reads geometry only through `MateReach`, which levers parts, so it can resolve a face only of an instance (or a pattern copy of one). Mates whose sides are instances fold as before and bit-equal; a `Copy` or `World` side works on any placement, and a non-instance member's lever reach is taken lazily, only where an angular decision needs it. A face side, or a needed lever, on a `Place` copy or a raw construction refuses typed: `FaceRefusal::NotAnInstance` in `face_base` (`crates/editor-core/src/mate/solve.rs:1128`) and `LeverRefusal::NotAnInstance` in `pair_reach` (`mate/solve.rs:1768`). Split re-anchors a kept face side onto the instance it leaves, and inline refuses a host face side that would land on a non-instance copy (`InlineError::SideLeavesAnInstance`, `crates/editor-core/src/refactor.rs`).

This is no regression: before B every mate member was an instance. C lifts it by moving the fold into the evaluation, per placement in `Doc::upstream` order, where every face is a value (`a-mate-relates-two-poses`).
