---
id: an-all-tier-diff-with-no-crate-seed-runs-no-slow-row
kind: issue
title: An all-tier diff that changes no crate (ci.yml, Cargo.toml, the toolchain) runs every gate row except the slow set
status: closed
opened: 2026-10-14
closed: 2026-10-15
priority: P3
cost: E
branch: ciw/readable-reds
pr: 5206
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

## Closed

ci.yml's slow-set step now runs when `seeds != ''` or `tier == 'all'`.
With no seed, its package expression is `all()`, so the whole slow set
runs, and the step prints which set it took. `scripts/ci-filter.py` is
unchanged. An `all`-tier diff that does seed crates still runs only their
slow tests, as before.

## 2026-10-16: the whole-slow-set run is withdrawn

Ev ruled on #5224: "cool. let's skip the slow tests then". No failure had
motivated running the whole slow set on a seedless `all`-tier diff, and it
was a costly rule. ci.yml's slow-set step now runs only when `seeds != ''`,
over the slow tests of the seeded crates, on every tier. A workspace-level
file adds no slow tests. When TIER=all seeds no crate, the `test` job writes
one line to the step summary saying the slow set did not run and that the
nightly runs it whole, so the PR does not read as fully gated. The
failing-test reporting stays. Landed with
`a-slow-set-edit-to-nextest-toml-runs-the-whole-suite`.
