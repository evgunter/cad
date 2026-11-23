---
id: a-real-junction-on-a-short-leg-refuses-as-tangent
kind: issue
title: JunctionTangent and SeamTangent name tangency at a real angle on a short leg: their levered turn margin cannot tell a short arm from a small turn
status: closed
opened: 2026-11-06
priority: P1
cost: M
closed: 2026-11-22
pr: 7260
branch: profile/junction-turn-names-its-factors
---

## Finding

`seg::junction_turn` (`crates/profile/src/seg.rs:1230`, `path_junction_turn`)
classifies `Margin::levered(sin φ, arm)`, with arm being the shorter leg's arm. That is
the same product the fillet turn gate had
(`work/round/the-levered-turn-margin-conflates-a-short-arm-with-a-small-turn.md`,
repaired for the fillet door by giving `PathNoCornerReason` a
`TurnWithinTolerance { sin_turn, align, lever, lever_of }` arm). At a
junction the definite Zero arm still names a geometric fact that the input does not have:

Measured at `Tol::witness()` (ε = 1e-9), a 64° turn after a straight
leg of `k·ε`:

- `Open.at((0,0)).line_to((0.3ε, 0)).toward(cos 64°, sin 64°)` and
  the same with `.line_to(...)` at 64° refuse
  `JunctionTangent { margin: 2.70e-10, arm: 3e-10 }`, "this junction
  is tangent at any precision you could care about". It is not
  tangent: it turns 64°. The same happens at `k = 0.9`. At `k = 3` the gate escalates in band.
  At `k = 50` the junction builds.
- The closing seam behaves the same way: a closing leg of `0.3ε`/`0.9ε` arriving at
  `Start` at 64° off the entry's first side refuses
  `SeamTangent { margin: -2.70e-10 }`, "the SEAM arrives tangent to
  the entry's first side". With `Start.arrives_tangent()` declared,
  `SeamArrivalLeverTooShort` answers first, which is honest. The declared arm
  at `k = 50` refuses `SeamArrivalOffDirection`, which is also honest.

Both definite arms carry the margin (and `JunctionTangent` the arm),
so the payload already holds both factors. The class name and the sentence
are what assert the tangency (`PathError::JunctionTangent`'s Display, `crates/profile/src/path.rs:1769`; `SeamTangent`'s producer, `crates/profile/src/path.rs:3621`). The recourse, `JUNCTION_TANGENT_LEVER`
in `crates/profile/src/validate.rs:217`, is shared with the in-band ending of
`path_junction_turn` (D4 ¶1 (iv)), so a repair changes both arms
together. The kind name `junction_tangent` is also a Python tag
(`crates/pncad-py/src/tags.rs:179` `path_error_tag`;
`crates/pncad-py/tests/test_paths.py:320` `test_the_junction_checks`).

## Shape question

Choose between renaming or splitting the class, or keeping the kind and rewriting its sentence
to say "the turn at this junction is zero at tolerance" with both
factors and a "lengthen the shorter leg" lever. This binds the Python tag and
the shared recourse constant, so it is a decision for the owner. The neighbouring
`work/profile/a-zero-length-line-leg-refuses-as-a-tangent-junction.md`
covers the zero-length case, which is a different defect: there the stand-in ray is the problem.
The driving probe is in the ROUND PR that filed this row (the
sibling sweep of `round/turn-gate-names-its-factors`).

## Re-banded P2 → P1 (2026-11-06, ROUND orchestrator)

PR 6544's review drove `JunctionTangent` to refuse "tangent at any
precision" at a 64° junction on legs of 1.05ε and 1.1ε. So legs of at least ε
reach it without any leg-length gate answering first. That is the property that
made the fillet turn-gate row P0.

## Closed (2026-11-22, PR 7260)

The junction and seam arms follow the fillet door's repair: the kinds
are `JunctionTurnWithinTolerance` and `SeamTurnWithinTolerance`, both
carrying `profile::LeveredTurn { sin_turn, align, lever, lever_of }`,
the payload the fillet door's `TurnWithinTolerance` now holds too. The
sentence states the turn's sine and cosine and the arm it was levered
over, and offers lengthening the shorter leg first. The seam's in-band
ending has the seam's own recourse. No decision moved. The Python tags
are `junction_turn_within_tolerance` and `seam_turn_within_tolerance`.
`docs/PATHS-DESIGN.md` §4 item 1 states the gate as the margin
`sin φ · arm` (Ev's sign-off on the PR, 2026-11-21).

Left on their own rows: the cusp twin
(`a-real-near-reversal-on-a-short-leg-refuses-as-a-cusp`), a declared
seam arrival read over the first leg
(`a-declared-seam-arrival-reads-its-turn-over-the-first-leg-and-names-it-the-arriving-legs`),
and the recourse sentences' one skeleton
(`the-levered-turn-recourses-are-one-skeleton-spelled-four-ways`).
