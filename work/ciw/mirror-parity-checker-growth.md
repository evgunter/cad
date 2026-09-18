---
id: mirror-parity-checker-growth
kind: issue
title: check-ci-mirror-parity.py has grown 64% in three days and nothing measures it
status: open
opened: 2026-09-11
---


## The numbers

`scripts/check-ci-mirror-parity.py`, `wc -l`:

| point | lines |
| --- | --- |
| `eb4bae11` (2026-09-09) | 2983 |
| the env-arm unit | 3593 |
| `64407cb` (2026-09-11, this unit's base) | 4016 |
| unit 4's head (the directory arm) | 4908 |

**+892 lines in this PR, +1925 (64%) in three days.** Claim 10's block is now
roughly 63% of a module docstring that serves twelve-plus claims.

## Why it is an item and not a complaint

Every review lane this quarter has flagged the file's growth and every unit has
answered with the same true sentence: the alternative to widening claim 10 was
minting another claim, which is worse. That answer is correct per unit and says
nothing about the sequence. Six consecutive units have each taken a claim or an
arm here; the seventh will make the same argument.

`work/ciw/mirror-pairs-context-beyond-env.md` names accumulation as the
counter-argument to writing all three of its clauses at once — so the file's
size is already being used as a scheduling input, by lanes, with no measured
basis. Nothing in the repo measures it, no threshold exists, and the growth is
therefore an argument anyone can make in either direction.

## Shapes, none of them taken here

Splitting the file is the obvious one and is not obviously right: the module
docstring's whole design is that the claims share one recogniser and one
tokenizer, and `gate-roster.sh`'s own argument (quoted at the head of this
file) is why it is not a `scripts/gates/*` member. What might be right instead:
moving claim 10's three arms and their tables to a sibling module the check
imports; or a size floor that reds and has to be lowered deliberately, the way
`MIRROR_MARKER_FLOOR` and `CENSUS_FLOOR` are.

Whatever the answer, it is a unit with its own argument and not something to
take under a working-directory item. Candidate for the next slate.

## Provenance

The style review of CIW's third slate, unit 4
(`mirror-pairs-context-beyond-env` clause 1), which took the measurement the
unit itself did not.
