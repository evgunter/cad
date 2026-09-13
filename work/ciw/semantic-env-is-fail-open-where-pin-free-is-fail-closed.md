---
id: semantic-env-is-fail-open-where-pin-free-is-fail-closed
kind: issue
title: SEMANTIC_ENV admits an unlisted variable silently where PIN_FREE refuses an undeclared literal
status: open
opened: 2026-09-11
---


## What

Two tables in `scripts/check-ci-mirror-parity.py` answer the same shape of
question about a name the file has not been told about, and they answer it in
opposite directions.

- **`PIN_FREE` is fail-CLOSED.** Claim 11 reds on any version literal in the
  local tree that is neither a live pin nor a declared non-pin. Its own header
  says so out loud: *"that table is the list of literals this tree is allowed
  to carry, and it runs that way round on purpose, so a new literal is an error
  until someone says what it is."*
- **`SEMANTIC_ENV` is fail-OPEN.** A semantics-bearing variable whose name is
  not on the allowlist is invisible to claim 10's env arm. Set it on one half
  only and nothing says a word.

The allowlist's argument for being an allowlist is good and is not what this
item disputes: the hosted half's `env:` blocks carry ~65 names and a deny-list
would be a roster of everything GitHub does. What has no mechanism is the
sentence that closes it — *"the day one is set, it is added here in the diff
that sets it"* — which is an invariant held by convention, two hundred lines
from a table that refuses to hold the same kind of invariant by convention.

## The population today is empty, and that is the point

Measured on `main` 2026-09-11, twice (unit 4's lane and its correctness
review): `RUST_MIN_STACK` and `PROPTEST_CASES` appear in neither half, and the
only `NEXTEST_*` name in the tree is `NEXTEST_VERSION`, a pin claim 11 reads
rather than a knob on what a run does. So there is nothing to fix and nothing
to red — which is exactly when the convention is cheapest to replace with a
mechanism, and exactly when nobody does.

## Shapes, none of them taken here

A `CAD_`-style namespace rule already covers this kernel's own knobs. What is
open is the rest: a fail-closed roster of the names each half's `env:` blocks
may carry (the plumbing enumerated once, like `PIN_FREE`), or a narrower rule
over names that reach `cargo` — or a ruling that the convention is the right
answer and the sentence should say it is one.

## Provenance

The correctness review of CIW's third slate, unit 4
(`mirror-pairs-context-beyond-env`), which re-took the clause (3) measurement
and filed the asymmetry the measurement sits inside.
