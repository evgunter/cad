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
built."* Its target census, `every_target_form_is_a_document_program`,
first appears in `crates/editor-core/tests/switch_program_vocabulary.rs`
at `7902971`, **2026-09-10 00:27 UTC** — the day BEFORE the cut. The
census the class said must be built already existed and was on main.
(PR 2445 re-anchored it on a `TargetKind::ALL`, which was the real
remaining work — smaller than the class implied and a different shape.)

**`S195`** — classed **H**, *"four mirrored vocabularies … one-census-
or-four is a design call."* Measured by PR 2447: three of its four
claims were stale. Two of the three had been stale since **2026-09-01**
(`70aaee60d`, `592685539`) — ten days before the cut. The third was
discharged by PR 2445 on 2026-09-12, which is AFTER the cut and is
therefore **not** evidence against it; an earlier draft of this finding
used it as such and was wrong.

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
