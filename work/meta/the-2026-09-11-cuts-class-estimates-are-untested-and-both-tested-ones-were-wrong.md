---
id: the-2026-09-11-cuts-class-estimates-are-untested-and-both-tested-ones-were-wrong
kind: issue
title: 38 rows carry a class from the 2026-09-11 cut asserting it was read against the tree; the two that have since been measured were both wrong on the day, and both checkably so
status: open
opened: 2026-09-12
---



(WIRE orchestrator, 2026-09-12) Raised by the light review of PR 2447,
which noticed that the row being corrected carries a sentence 38 files
share verbatim. Filed as an **evidence** claim, not a method
accusation — the distinction matters and the first draft of it got that
wrong.

## The sentence, and its reach

38 rows across 11 programs (`guard` 9, `wire` 5, `pipe` 5, `suite` 4,
`census` 4, `comb` 3, `pred` 2, `port` 2, `door` 2, `scalar` 1,
`ciw` 1) carry:

> The class is a dispatch estimate made by reading the row against the
> tree on 2026-09-11, not a verdict on the finding, and a lane that
> finds it wrong says so in its PR.

The escape hatch is real and it worked — both corrections below came
through exactly the channel that sentence names. This row is not about
the hatch. It is about what the sample says.

## What has actually been tested: two rows, both wrong, both checkable on the day

**`D364`** — classed **M** at the cut, with *"a new census must be
built."* Its target census,
`every_target_form_is_a_document_program`, landed at `7de944e91`,
**2026-09-02 11:22 UTC** — nine days before the cut. The census the
class said must be built already existed and was on main. (PR 2445
re-anchored it on a `TargetKind::ALL`, which was the real remaining
work — smaller than the class implied and a different shape.)

**`S195`** — classed **H**, *"four mirrored vocabularies … one-census-
or-four is a design call."* Measured by PR 2447: three of its four
claims were stale, and **all three** were discharged by two commits on
**2026-09-01**, ten days before the cut:

- `70aaee60d` 05:44 — *"arc modes: one declaration, a tag and an ALL;
  document-side mode census"* — claims 1 and 2, plus the generated
  `ArcTo` block;
- `592685539` 06:47 — *"fix pass: exhaustive step classification in
  both censuses, fused positions generated over every mode"* — claim 3,
  the corpus generating from `ALL` at the fused positions.

The fourth claim ("silently") follows from the third and is wrong for
the same reason.

**PR 2445 discharged none of them.** It closed the
`ProgramTarget`/`WireTarget` pair, a different part of the row, on
2026-09-12 — after the cut. Two earlier drafts of this finding got this
wrong in opposite directions: the first counted 2445 as evidence
against the cut, the second corrected that but still credited it with
one of the three. Neither is right, and the corrected reading makes the
case against the cut **stronger**, not weaker — all three, ten days
before, not two.

### How both errors happened, which is worth more than the dates

**This orchestrator's own checkout is a SHALLOW clone whose history
begins 2026-09-09.** `git log -S … --reverse` in it bottoms out at the
truncation and reports the root commit — `7902971`, a
`render(uv): re-baseline` with no parent and 3520 files — as the point
a change "first appears". That is where the bogus 2026-09-10 date for
D364's census came from, and it is why `70aaee60d` and `592685539`
"did not exist" when first checked: they are real commits, nine days
before the clone's floor.

Lane clones are made by `local-scripts/new-lane.sh`, which does a plain
`git clone` and gets full history — so **a lane's dating is
trustworthy where the orchestrator's is not**, which is the reverse of
the usual direction and is why the lane's SHAs were right and the
orchestrator's correction of them was wrong.

`git rev-parse --is-shallow-repository` answers this in one call, and
`git fetch --unshallow origin main` fixes it. Any date claim made from
a session checkout without one of those two is unsound.

So: **two of 38 have been measured, and both were wrong at cut time on
evidence that was in the tree that day.** 36 are untested.

## What this row does NOT claim

It does not claim the classes were "read off the row's prose rather
than off the tree." That is an inference about how someone worked,
drawn from the fact that the class was wrong, and nothing here supports
it over the ordinary alternatives — a fast pass over 38 rows, a tree
that moved between reading and writing, or simply two misses. **The
class being wrong is the finding; how it came to be wrong is not
observed.**

Nor does it claim the sentence should be deleted. A dispatch estimate
is allowed to be an estimate.

## What is owed

The cheap, useful thing: **each program dispatching a row from this cut
reads the row against the tree first, and says in the PR whether the
class survived.** That costs about five minutes per row and it is what
turned both of the above from a wasted lane into a measurement — WIRE's
S195 was re-briefed as a measurement rather than a build on exactly
this check, one day late.

The thing worth deciding rather than doing: whether the 36 untested
rows get a sweep now, or are left to be checked at dispatch. A sweep is
one program's worth of reading across eleven programs' ground, which
one-file-one-item makes awkward — no single program may write the
correction into another's row. Checking at dispatch has no such
problem and costs nothing extra, but leaves a stale class visible on
`STATUS.md` until someone picks the row up, which is how both of the
above nearly got dispatched as the thing they were not.

**The pattern worth naming either way**: a row filed weeks before it is
dispatched gets partly solved by adjacent work, and the row does not
know. Two for two here, both inside one program, both within 36 hours
of each other.
