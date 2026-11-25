---
id: a-hop-whose-axis-changes-sign-inside-the-box-gets-no-flip-verdict
kind: issue
title: A document whose axis hop changes sign inside its box gets no flip verdict: the drive spends its leaf budget bisecting toward zero and says Budget on the far side
status: open
opened: 2026-11-23
priority: P2
cost: M
---

Found by the review of BOXFINISH's expression-question unit (PR 7488,
both reviewers), on the fixture `riser` in
`crates/editor-core/tests/m10_3_driver_interval.rs`.

**The document.** A quadrilateral `(0,0) → (1,0) → (1,height) → (−1,2)`
extruded, `height` a parameter over `0.25 ± 0.3`. The side
`(1,0) → (1,height)` is a hop along `y`: its chord's `x` is zero as an
expression, so the profile's direction mint (`crates/profile/src/seg.rs`
`axis_or`) stores the axis `(0, ±1)` and signs it by one reading of
`height` (`segment_direction_axis`, audit row F43). Below zero the side
points down: a different, valid outline.

**What the drive says** (`editor_core::drive`, measured on the PR's
head, default ε):

| leaf budget | certified | at and below zero | above 0.5 |
|---|---|---|---|
| 64 | `height` in [0.032, 0.498], 19 leaves | 41 leaves, all `Budget` | 1 `Budget`, 3 `FlipCrossing` |
| 1024 | [0.006, 0.49998], 114 leaves | 904 leaves, all `Budget` | 1 `Budget`, 5 `FlipCrossing` |

The flips above 0.5 are a pair's gap reading, not this hop. Every leaf
wholly below zero is refused for the budget, as are the ones that
straddle it: 904 of 1024 leaves are spent on a region whose answer is
one sentence, "the side points the other way here".

**Why.** The sign reading is taken outside the verdict log (the `f64`
witness stores the quotient and never takes it), so a leaf whose box
straddles zero fails with an empty escalation log and the driver's
error arm bisects it (`_ => LeafVerdict::Bisect`). A leaf wholly below
zero builds a different outline from the witness's, and nothing
compares the two as a flip: the witness holds no decision for this
reading to flip against.

**What would close it.** A verdict for "the direction along this axis
is the other way on this side": either the reading is recorded where
the witness can hold it (it has no door today,
`work/onecopy/a-least-band-reading-off-the-log-whose-escalation-refuses-has-no-door.md`),
or the driver reads the refusal's site and gate off the node's error.

The row `an_undecided_axis_sign_reaches_the_driver_as_a_cue_to_bisect`
pins today's answer (`Budget` at and below zero) and names this file.
