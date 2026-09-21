---
id: an-items-stated-sweep-pattern-may-not-match-its-own-instance
kind: issue
title: an item's stated sweep pattern can fail to match the instance it was derived from, so a lane re-running it faithfully finds nothing
status: open
opened: 2026-09-16
priority: P3
cost: D
---


## What

`docs/prompts/implementer-discipline.md` §5 tells a lane to say what
pattern it swept with and what that pattern could not match, and §5's
second half tells the next lane to grep for **the shape, not the
symbol**. Neither half asks the pattern to be checked against the
instance the item was written from — and an item that states a pattern
its own defect does not match sends the next lane looking with the
wrong instrument.

**The measured instance.** `work/instr/baseline-census-partition-assert-cannot-fail.md`
described its sweep as covering *"every `assert` in `tools/*/tests` and
`tools/*/src` whose operands are `const` items or are derived from them
alone."* INSTR unit 4 re-ran exactly that, in two scripted passes —
operands that are `const` items, then locals derived from consts via a
fixpoint over `let` bindings in the enclosing `fn` — over all 509
assert macros in the 20 files under those two globs.

**Both passes returned the item's own instance not at all.** The
assertion the item exists to describe is
`constant.len() + discriminating.len() == IDENTITY_COLUMNS.len()`, and
`constant`/`discriminating` descend from `parse(BASELINE)` at runtime.
They are not consts and are not derived from consts: the defect is a
**tautology** — a predicate forced by how its own operands are
constructed — not a compile-time constant. It surfaced only under a
third pass written for that shape (`len() + len()` against a third
length, complementary `.filter` pairs over one source,
`assert!(x.iter().all(P))` where `x = ….filter(P)`).

## Why it is a class and not a lane's mistake

The item was not careless: "operands fixed at compile time" is the
phrase the discipline itself uses for this family (*"a predicate over
things fixed at compile time"*), and two of the three precedents the
item cites really are that shape. The misdescription is what happens
when a finding is written from one instance and generalised in the
vocabulary nearest to hand. The cost is asymmetric and silent: a lane
that runs the stated pattern gets a **clean result over the very defect
it was dispatched to fix**, and a clean sweep reads as a negative
result rather than as a wrong instrument.

`work/meta/doc-citations-no-gate-checks-rot-silently.md` is the
neighbouring shape for citations; this is the same failure for
patterns, and unlike a citation a pattern has no referent for anything
to resolve.

## Shapes a fix could take

1. **A discipline clause**: an item stating a sweep pattern says which
   of its own hits that pattern matched, so a pattern that cannot find
   the instance it came from is visible at writing time rather than at
   the next dispatch. Cheapest, and it is a sentence in
   `docs/prompts/implementer-discipline.md` §5.
2. **A reviewer question**, in `docs/prompts/reviewer-style-lane.md`'s
   Q6 register, since a stated sweep is a claim resting on a
   measurement and this is the case where the measurement did not
   measure what it names.
3. **Nothing mechanical.** There is no referent to resolve and no way
   to execute a prose pattern, so a guard of the `lint`-style shape is
   not available here; if that is the answer it belongs written down,
   because the absence is not obvious.

**Both `docs/prompts/` files are this program's `paths` and a change to
either waits for Ev** (CLAUDE.md: the standing discipline handed to
every lane by path), so shapes 1 and 2 are `[ev]` conversations, not
self-merges.

## Was

Found by INSTR unit 4's fix pass (PR 2736) and routed here at
adjudication rather than filed by the lane, because `docs/prompts/*` is
META's territory and the finding is about the discipline rather than
about the census. The unit's own item is being corrected in that PR;
this row is the class the correction is an instance of.
