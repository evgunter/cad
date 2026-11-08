---
id: main-red-the-plates-sym-walk-ledger-reads-12-more-forms
kind: issue
title: main is red (slow set): m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row, the plate's Plain/Decision and Early/Decision lines read 12 more forms than pinned
status: open
opened: 2026-11-02
priority: P2
cost: E
---


Found by the ONEHOME finished-body lane (PR 6371, review r1 m7),
measured on `origin/main` at `8ae2b9bfc` in a clean worktree.

`crates/editor-core/tests/m10_sym_profile_interval.rs::the_forms_the_walks_build_are_pinned_per_eps_row`
is in the slow set (`.config/nextest.toml`), so no PR touching other
files runs it. On main it fails at the default ε. The plate's ledger
reads:

- `Plain/Decision calls 1149 forms 20630 frozen 36 digest 41f712de2e74b3fa24e508cff644d66b`
  against the pinned `forms 20618 … digest 5b68ab11cb6a6488e4a842bd68fd8b54`;
- `Early/Decision calls 342 forms 11661 frozen 0 digest 0cd9855b61169dd1282cb87c42c00f67`
  against the pinned `forms 11649 … digest 38b61e2e3311ab58939c93cd540bd52b`.

Every call count, and the other three lines, match the pin. PR 6371
measured the same numbers on its own branch and first blamed itself.
Main's own run shows the move predates it.

The row's docs pin main's measurement "not explained" and name the PRs
that moved it before. What moved it this time is unread: bisect main
between the last commit that touched `PLATE_LEDGER` and `8ae2b9bfc`.
