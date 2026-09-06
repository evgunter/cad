---
id: mate-dangling-head-is-a-catch-all-that-reports-a-false-cause
kind: issue
title: MateFault::DanglingHead is the catch-all for every pattern-rule failure, and the doc's stated mitigation is unavailable exactly when it is needed
status: parked
opened: 2026-09-04
blocked_on: [MSOLVE-3]
---


Measured by FIX's `unit-admits-non-finite-direction-norm` lane
(PR 1738) at its fix pass, after that unit's style review flagged the
symptom. Filed by the FIX orchestrator, homed here rather than on a
program's slate. **Claimant corrected 2026-09-04:** routed at S-MATE,
which has since CLOSED; `crates/editor-core/src/mate*` is now DOCM's
territory glob. The fix reverses a documented design decision and is
the owner's to rule on, not FIX's to land.

## The decision as written

`crates/editor-core/src/mate/solve.rs:198-208`, `derived_offset`'s
`# Errors`:

> A head whose derived pose does not exist resolves to no member of the
> vocabulary and refuses `MateFault::DanglingHead` — an index at or
> beyond the count, a rule whose slots do not evaluate, **a degenerate
> direction**, an explicit-rule pattern (whose count spelling the
> pattern node itself refuses). **The pattern node's own evaluation
> names the underlying cause in its own voice; this door's job is only
> to refuse rather than guess a pose.**

The catch-all is deliberate, and the second sentence is its
justification: the door may report a symptom because the real cause
surfaces elsewhere.

## The measurement: the justification does not hold

A mate to a patterned instance with a `1e200` direction:

```
mate fault:           DanglingHead { mate: 3, side: A, head: 1 }
node 0 (instantiate): Failed: Mate(DanglingHead ..)
node 1 (the pattern): Poisoned { through: node 0 }
node 3 (the mate):    Failed: Mate(DanglingHead ..)
```

**The mate fault poisons the document, so the pattern node never
evaluates its own direction.** The cause the doc promises will appear
"in its own voice" appears nowhere at all. The user is told the mate's
head pattern is dangling, about a pattern that exists and is
well-formed apart from one slot value.

The compensation the design rests on is unavailable in exactly the
case that needs it — the poisoning is *caused by* the very refusal
whose honesty depends on it.

## Scope: this predates the unit that found it

`work/fix/unit-admits-non-finite-direction-norm.md` added a typed
non-finite refusal at `editor-core`'s `unit()`, which reaches this door
and falls into the `_ => dangling()` arm. That is one more member of a
bucket that was **already mis-labelling the decided-zero case** — the
docstring lists "a degenerate direction" explicitly. Nothing about this
is new with that unit; it is one more instance of a standing decision.

This is `memories/refusal-text-is-not-cause.md` at the reporting layer:
the right outcome (a refusal) reached through a predicate about
something else (a dangling head), with the payload and the raising site
disagreeing about what happened.

## The proposal, from the lane that measured it

> **One variant, no new vocabulary:**
> `MateFault::PatternRule { mate, side, pattern, error: Box<NodeErrorKind> }`,
> replacing the `_ => dangling()` arm. It carries the evaluation
> layer's own typed refusal verbatim — including the role word — so the
> mate road reports the cause instead of a symptom, invents no new
> words, and closes the catch-all so no *future* `NodeErrorKind` can
> silently become a dangling head either.

Cost, stated plainly: it reverses the documented decision for the
degenerate case too, so it wants S-MATE's assent. The genuine
`DanglingHead` causes — an index at or beyond the count, an
explicit-rule pattern — keep the variant they have; only the arm that
swallows a typed evaluation refusal moves.

The alternative is to keep the catch-all and accept that this class of
mate refusal names a symptom. That is a coherent answer; it just is not
the one the current doc argues for, because the argument it makes is
false.

## Interim state, landed on PR 1738

The unit did **not** add the variant, and did not merely extend the
`# Errors` list either — extending it would have left the disproved
sentence standing, which tells the next reader the compensation exists.
The rationale is corrected in place to record that the pattern node's
voice is unavailable when the mate fault poisons the document, citing
this file. So the doc is honest about a gap rather than confident about
a mitigation, and nothing here is silently wrong.

## Home

`work/msolve/` — S-MATE's successor, opened 2026-09-04 for exactly this
residue.

## Ruled (MSOLVE orchestrator, 2026-09-05)

The proposal above is ruled in by S-MATE's successor; the unit is
`MSOLVE-3`, on which this row is parked.
