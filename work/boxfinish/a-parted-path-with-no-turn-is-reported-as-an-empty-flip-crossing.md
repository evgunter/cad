---
id: a-parted-path-with-no-turn-is-reported-as-an-empty-flip-crossing
kind: issue
title: A leaf whose path parts from its witness with no sign turned is reported as a flip crossing that names no predicate
status: open
opened: 2026-11-24
priority: P1
cost: M
design: true
---


Found by the lane repairing
`a-witness-records-curved-premise-checks-the-interval-replay-skips-so-no-curved-leaf-certifies`
(2026-11-24), measuring why a curved leaf's refusal carried no
evidence.

**What happens.** `editor_core::drive`'s `diff_verdicts` keeps only
turns: `first_turn` answers `Parted` when the leaf and its witness
name different predicates at one index with no sign turned, and
`.flip()` drops it. `classify_replay` then wraps the empty `FlipSet`
as `FlipCrossing`. So a leaf whose path parted is reported as a flip
that names no predicate, and counted as refused mass.

**Three states end in empty evidence today** (the lane's reading of
the code; the second is measured, on the plate):
- the choices part with no turn (`FirstTurn::Parted`);
- the choices agree end to end and the check sequences part (the
  plate, before its repair: fifteen premise checks logged three times
  by the witness and twice by the leaf);
- the choices agree and one check has another definite sign.
`first_turn` reads choices only (`Path::steps` filters
`Role::Choice`), so the third, a decided difference by any reading,
cannot be named either.

**Why it is a design question.** `docs/ERROR-DESIGN.md` E6 enumerates
the drive's refusal reasons and decides this case: "a leaf definite on
a *different* verdict vector is refused mass (`FlipCrossing`, flipped
predicates named)", and only an indeterminate decision bisects. And
`FlipEvidence::is_empty`'s doc states today's behaviour as intended
("an empty evidence set is possible and is honest rather than a
bug"). Whether a parted path is refused mass, or a leaf not proven
that a narrower box may re-align, and what its evidence names, changes
what E6 decides. Designers weigh it before anything is built.

The repair of the plate's row (the rounding lane's lifted pass is
unlogged) removes the one measured cause and leaves this
classification as it is. (BOXFINISH orchestrator)
