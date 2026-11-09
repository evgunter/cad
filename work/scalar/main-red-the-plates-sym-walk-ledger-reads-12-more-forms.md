---
id: main-red-the-plates-sym-walk-ledger-reads-12-more-forms
kind: issue
title: main is red (slow set): m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row, the plate's Plain/Decision and Early/Decision lines read 12 more forms than pinned
status: closed
opened: 2026-11-02
priority: P2
cost: E
closed: 2026-11-09
pr: 6663
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

**Still red at main `2fee45b9`** (measured by ONECOPY's
`onecopy/length-rule`, PR 6628, with main's `crates/` checked out):
`the_forms_the_plates_walks_build_are_pinned` reads largest form 38
(pinned 28) and `Plain/Decision calls 1203 forms 53637 frozen 62`
(pinned `forms 51397 frozen 36`), `Plain/Assertion … forms 9172`,
`Early/Decision … forms 42505`, `Early/Assertion … forms 10583`,
`Door/Decision … forms 13458`. PR 6628 moves only the
Plain/Decision, Plain/Assertion and Early/Assertion lines further
(53631, 9114, 10519; a refused direction reads its norm once), and
leaves the plate pin for this row's owner to re-derive.

**Closed by PR 6663** (2026-11-09): bisected to PRs 6568, 6594, 6588 and 6628, every call count and every plate decision unmoved; re-pinned at main `337c944730` at all three ε. The cost 6588 added is filed on BOXES (`work/boxes/cover-readers-moved-the-plates-sym-ledger-undisclosed.md`).
