---
id: mirror-pairs-context-beyond-env
kind: issue
title: a mirrored pair's working directory and action inputs are still compared by nothing
status: open
opened: 2026-09-10
---


## What

Claim 10 now compares the two halves of a `HOSTED MIRROR` pair on two
token classes: cargo flags (`SEMANTIC_FLAGS`) and environment variables
(`SEMANTIC_ENV`). A pair's execution context is bigger than both, and
the rest of it is compared by nothing:

1. **The working directory.** `.github/workflows/ci.yml`'s
   `oracle-certify / certify against the oracle` step carries
   `working-directory: interval-transcendentals`; its local half
   (`local-scripts/ci-local.sh`, `oracle_certify` at :934) spells the
   same fact as `(cd interval-transcendentals && …)`. Two spellings of
   one fact, exactly like a prefix and an `env:` block — and change
   either one and the pair stays green. The recogniser already lists
   `working-directory` in `STEP_KEYS` and discards its value.
   **Three pairs carry one**, measured: this one, `interval-backend /
   tests (default features — the oracle stack must stay out)` and `fmt /
   rustfmt (benches — its own cargo root)`. Two cargo roots the
   workspace excludes, which is exactly the population where a wrong
   directory runs a different check under the same row name.
2. **`with:` inputs to a shared action.** A pair whose hosted half is a
   `uses:` step has no `run:` at all, so claim 10 reads nothing on that
   side and its behaviour is decided in a `with:` block. **The
   population is zero today** — measured over every marker: no mirrored
   pair cites a `uses:`-only step — so this is a shape to refuse, not a
   comparison to write. A `Bail` costs a line; a comparison costs a
   vocabulary.
3. **A semantics-bearing variable outside `SEMANTIC_ENV`.** The
   allowlist is `RUSTFLAGS`, `RUSTDOCFLAGS` and the `CAD_` namespace,
   argued at the table. A name outside it — `RUST_MIN_STACK`,
   `PROPTEST_CASES`, a `NEXTEST_*` knob — set on one half only is
   invisible, and adding one is what would make it matter.

## Why it is worth an item

It is the same shape as `mirror-pairs-env-divergence-unchecked`, one
axis further out, and that item's argument holds unchanged: a roster
claim cannot say whether two paired rows run the same check, and neither
can a claim that stops at argv and environment. (1) is measured and
live; (2) and (3) are populations, not instances.

The counter-argument to writing all three at once, and the reason this
is an item rather than a diff: `scripts/check-ci-mirror-parity.py` is
~3000 lines and has taken a claim or an arm in each of the last four
units. Every review lane this quarter has flagged its growth. (1) is
cheap and has a live subject; (2) and (3) may be better answered by
saying at the claim what is out of scope than by widening it again.

## Provenance

Disclosed by CIW's `mirror-pairs-env-divergence-unchecked` unit, which
built the env arm and stated what it cannot see at the claim and in the
module docstring.
