---
id: the-levered-turn-recourses-are-one-skeleton-spelled-four-ways
kind: issue
title: profile: the four levered-turn recourses repeat one skeleton by hand, and the fillet trim's TurnInBand re-spells LeveredTurn's fields
status: open
opened: 2026-11-21
priority: P3
cost: E
refs: [a-real-junction-on-a-short-leg-refuses-as-tangent]
---

(PROFILE lane, from the review of PR 7260, `profile/junction-turn-names-its-factors`.)

## Finding

The payload of a levered turn that is zero at tolerance is spelled once
(`profile::LeveredTurn`, `crates/profile/src/path.rs`). Its recourses are
not. Four constants in `crates/profile/src/fillet_recourse.rs` each write
the same skeleton by hand: "if the angle is real, LEVER; if smooth,
DECLARATION; [if it doubles back, …;] otherwise move the geometry / cut
at a corner". They are:

- `JUNCTION_TURN_LEVER`: the junction's in-band ending and definite arm.
- `SEAM_TURN_RECOURSE`: the seam's (`EscalatedAt::Seam`, `PathError::SeamTurnWithinTolerance`).
- `FILLET_TURN_INBAND_RECOURSE`: `fillet_corner_turn` and `TurnWithinTolerance` on `TurnLever::ShorterLeg`.
- `FILLET_ANCHOR_TURN_RECOURSE`: `path_corner_turn` and `TurnWithinTolerance` on `TurnLever::AnchorSeparation`.

The wording already drifts. The smooth clause reads "if smooth, declare
it: .tangent() …" in one constant and "if the legs continue smoothly,
construct the tangency (.tangent())" / "if the rays continue smoothly,
…" in others. Only the fillet two name the doubling back. The leg lever
is shared word for word only because PR 7260 aligned it by hand.

Separately, `crates/profile/src/sugar.rs` `ArcTrimRefusal::TurnInBand
{ sin_turn, align, arm }` is a further copy of `LeveredTurn`'s fields
in raw `T`, re-wrapped into a `LeveredTurn` in
`crates/profile/src/path/arc_fillet.rs` (`map_refusal`'s `TurnInBand`
arm). It was not folded in PR 7260 because `ArcTrimRefusal` derives
`Copy`, and `LeveredTurn` holds `Told`, which is boxed and not `Copy`.
Folding it means dropping `Copy` from the trim's refusal and whatever
relies on it.

## Shape

One constructor for the skeleton, taking the lever clause, the
declaration clause and the last resort, so the four constants become
four calls. The doubling-back clause is either common to all four or an
explicit optional argument. Then fold `TurnInBand` onto `LeveredTurn`
(or onto a `T`-generic core of it) once the `Copy` question is
measured.

