---
id: register-equal-allowlist-exempts-a-whole-file-and-hides-test-calls
kind: issue
title: register-equal-allowlist's whole-file skip exempts geom-core's sym.rs for DEFINING the method, and hides thirteen test calls of the door with it
status: open
opened: 2026-09-14
priority: P3
cost: E
---


**Found by SYM-2** (the `sym.rs` file split, PR #2532), on `guard`'s
ground because `scripts/gates/` is `guard`'s territory
(`python3 scripts/work.py territory --files -`). It blocks a cut SYM-2
was asked to take, and SYM-2 reverted that cut rather than edit an
allowlist in a fix pass — the gate's own header says adding a site
"belongs in a spec revision, not in a fix pass".

## What happens

`scripts/gates/register-equal-allowlist.sh` holds two lists of WHOLE
FILES: `CALLER_HOMES` (the ratified constructor sites, which may call
`Real::register_equal`) and `DEFINITION_HOMES` (files that DEFINE or
re-export it, "where a mention is a definition and not a call").
`crates/geom-core/src/sym.rs` is a DEFINITION home — it carries
`Sym::register_equal` and the `Real for Sym<T>` impl that forwards to
it.

`sym.rs` also carries the tier's own `#[cfg(test)] mod tests`, and that
module calls the door **thirteen times** (`a_registered_identity_decides_zero_and_is_counted_apart`,
`a_gated_door_form_does_not_discharge`, `a_lying_registration_is_refused_typed`,
`a_cyclic_registration_is_refused_typed`, `the_span_identity_discharges_and_its_planted_lie_is_refused`
and five more). The whole-file skip covers them, so they have never been
seen by the gate — not by a decision, but because they share a file with
the definition.

Move the test module to `crates/geom-core/src/sym/tests.rs` — a pure
relocation, no call added, no spelling changed — and the gate fires on
all thirteen with "a call to Real::register_equal outside the ratified
constructor sites". Reproduced on `c9a38f5e4` (CI run 34805286468, the
`discipline (evaluation-code)` job), reverted in `c9811f408`.

## Why it is a row rather than a shrug

**The skip is broader than its stated subject.** `DEFINITION_SUBJECT`
says a mention in these files is "a definition and not a call". For
thirteen lines of `sym.rs` that is false: they are calls, they are the
door's own unit tests, and the gate's claim about them is wrong in the
direction that lets something through. Nothing is unsound here — a test
that registers an identity inside `with_session` is exactly how the
refusals (`Contradicted`, `Cyclic`, `Already`) are pinned — but the gate
cannot currently say so, and the same skip would swallow a real new call
site added anywhere in that file.

**It makes file layout load-bearing.** The tier is 3,540 lines in one
file and `work/sym/sym-rs-is-one-file-with-a-347-line-header` exists to
cut that down; today the gate silently rewards keeping the tests inside
the definition file. A structural gate should not have an opinion about
which file the tests live in.

## What is owed

A way for the gate to tell a TEST call from a registrant. The options, for
`guard` to choose:

1. **A third list, `TEST_HOMES`** (or a `#[cfg(test)]`-aware read): a
   file that may call the door only from test code. The gate already
   reads a code-only view and its `lib.sh` neighbours already know about
   cfg(test) narrowing — several selftests turn on it — so the machinery
   is there.
2. **Ratify `crates/geom-core/src/sym/tests.rs` as a caller home** with
   its own subject line saying it is the door's own tests and states no
   axiom. Cheapest, and the clean fixture must then plant it with a call
   like every other home.
3. **Leave it and say so**: keep the whole-file skip, and add to the
   header that a definition home's tests are exempt by construction, so
   the next reader does not take the subject sentence literally.

Whichever is taken, `work/sym/sym-rs-is-one-file-with-a-347-line-header`
can then take the tests cut (1,040 lines out of `sym.rs`).

## Related

`work/sym/sym-header-claims-outrun-the-code` names a second blind spot in
the same gate: `crates/sweep/src/extrude.rs` states the rim identity
through `swept::register_rim_identity`, which the caller arm's
`\.register_equal\(` grep cannot see, so that route is held by
convention. Same gate, same class — a reader of the allowlist cannot tell
what it actually covers — and worth taking together.
