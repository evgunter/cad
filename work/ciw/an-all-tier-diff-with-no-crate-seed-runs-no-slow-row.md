---
id: an-all-tier-diff-with-no-crate-seed-runs-no-slow-row
kind: issue
title: An all-tier diff that changes no crate (ci.yml, Cargo.toml, the toolchain) runs every gate row except the slow set
status: open
opened: 2026-10-14
refs: [all-tier-blanks-seeds-so-a-slow-set-edit-runs-no-slow-row, 5104]
---


## Found

On the `all` tier, `scripts/ci-filter.py` `_all_tier` derives SEEDS from
the crates whose own files changed. That is the fix of
`all-tier-blanks-seeds-so-a-slow-set-edit-runs-no-slow-row`. A diff that
changes no member's files therefore leaves SEEDS empty, and ci.yml's
`test (the slow set of the crates this diff seeds)` step is gated on
`seeds != ''`, so it is skipped. Every other row runs whole on `all`,
including `eps_extra=all()` at both extra ε. `gate ok` stays green over
the skipped step.

Such a diff can still change how every test binary is built or run:
- a `ci.yml` test-job env change;
- a `Cargo.toml` profile;
- a `rust-toolchain.toml` bump;
- `.config/nextest.toml`.

It is the one kind of diff for which "run the whole suite" is the
right scope, and it is the one that skips the slow set.

Witness: PR #5104 (`ci.yml` only: `CARGO_PROFILE_{DEV,TEST}_LTO=off`).
Run 38527985696, job `test`: the filter printed `cargo_scope=--workspace`
and `eps_extra=all()`, and step 10 (slow set) was skipped. Its slow-set
evidence had to come from a local run (`--ignore-default-filter` with the
slow filter, at all three ε).

## Wanted

On the `all` tier with no seed, run the whole slow set. The alternative is to
say in the PR's gate summary that it did not run, so a profile or toolchain
change cannot read as fully gated. The slow set is about 15 min of 4-core
wall at default ε on this tree (local, 496 tests), so CIW should weigh
the latency against how rare these diffs are.

— build-time lane (NURBS), 2026-10-14
