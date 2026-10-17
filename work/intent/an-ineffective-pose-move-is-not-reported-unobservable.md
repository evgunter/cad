---
id: an-ineffective-pose-move-is-not-reported-unobservable
kind: issue
title: An ineffective pose move is admitted but not reported unobservable
status: open
opened: 2026-10-17
---

`docs/INTENT-STAGE3-SPEC.md:111` says that a move whose result does
not change with its number at these values is admitted, and that "the
unobservable-value report names it, as it names a round pin's spin".
Two examples are a plane slid along a direction lying in it and an
axis spun about a parallel line. `docs/DESIGN.md:1682` states the
same rule for placements and pose moves.

A′ (PR 5243) admits such a move: `PoseDef::Slide` and `PoseDef::Spin`
are checked only by kind, in `crates/editor-core/src/edit.rs`
(`mint_pose`) and in `crates/editor-core/src/persist/check.rs`
(`first_pose_fault`). But no unobservable-value report exists to name
the move: `rg -i unobservable crates/` finds nothing. Until it is
built, a slide of a plane along its own x reads a length that moves
nothing and says so nowhere.

Raised as a style note in the dual review of PR 5243.

The report is not A′'s to build. The spec names it beside a round
pin's spin, which C (`a-mate-relates-two-poses`) owns. The work is to
build it once and have it name both the placements and the pose moves
whose values cannot be observed.
