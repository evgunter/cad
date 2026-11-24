---
id: main-red-predicate-name-coverage-meets-quantitys-decide-and-geom-cores-test-local-row-names
kind: issue
title: Main red: predicate_name_coverage reds on quantity's decide (no roster) and on four row/row_floor literals in geom-core's k_stats_doors.rs
status: closed
opened: 2026-11-23
priority: P0
cost: E
closed: 2026-11-23
pr: 7676
---


Found by GREEN's U17 on probe run 45310731023 (branch `green/census-reds-probe`, which is `origin/main` at `82273ca89b` plus PR 7670's census lines and a comment line in each of the two roster files, added so the slow rows would run). `test-utils` `predicate_name_coverage::every_deciding_crate_runs_a_roster_and_every_referenced_name_is_decided` (`crates/test-utils/tests/predicate_name_coverage.rs`, the final `assert!`) is red on two causes. Neither is the two names `main-red-two-slow-rows-no-row-holds-at-da61db71a` lists; those resolve now.

- **`quantity` decides predicate names and runs no roster.** The census counts the exact real's `decide` method (`crates/quantity/src/exact.rs` `Exact::decide`, and its callers in the same file) as a decide site. It came with SCALAR's `56c900438c` (PR 7291, `scalar/a-number-in-a-unit-is-rounded-once`). The row has derived "a crate with a site runs a roster" from the tree since CIW's PR 7496, so the method turns red now. `Exact::decide` rounds an exact number; it is no predicate-name door. Owed, and a judgement for TOLD (which owns `crates/quantity`) with CIW (the census): rename the method so the census's `decide*` token does not match it, teach the census that a `decide` whose arguments are closures names nothing, or give `quantity` a roster.
- **Four `row`/`row_floor` literals in `crates/geom-core/tests/k_stats_doors.rs`** (near :604, :625, :632, :649): "a predicate literal no crate decides". The test spells its own names in a local `const NAMES: [&str; 2] = ["row", "row_floor"]` and asserts `predicate == Some("row")`. A crate's `tests/` census runs with no carriers (`predicate_census(&tests, &[])`), so the local const is not read as a decision. It came with ONEROUND's `0bfdd819ba` (PR 7442, `oneround/readings-not-levers`). Owed: mark each one a deliberate negative (`// UNDECIDED-NAME: <why>`, since the names are the test's own fixture), or have the test decide them through a call the census reads.

Not bisected beyond `git log -S` on the two spellings.

## 2026-11-23 — both halves fixed (PR 7676)

- **`quantity`'s `Exact::decide`: the census over-matched its own rule.** `predicate_census`'s doc defines a site as a `decide*` token "followed by an argument list whose first argument is the name". Both `quantity` calls hand `decide` a closure (`self.decide(|n, d| …)`), and a closure is never a name. So they are another type's method, not funnel sites; `fn_parameters`' doc already reads a non-`str` argument that way for forwarders.
  - `test_utils::source::funnel_sites` now skips a call whose first argument is a closure, plain or `move`, and still reads any funnel inside that closure. The doc sentence says so, and the selftest is `a_decide_handed_a_closure_is_no_site`.
  - Sweep: a multi-line search over every `crates/*/src` for `decide*(` followed by a closure finds only the two `quantity` calls, so no other crate's census changes. Blind spot: a closure bound to a name first and passed by that name stays an indirect site. That is the census's existing behaviour, and it reds nothing, because a roster reds only consts and unreadable sites.
- **geom-core's `k_stats_doors.rs`.** The two tests spelled their names in a test-local `const NAMES`. A crate's `tests/` census runs with no carriers, so it read no name there, and the census's rule is "spell it as the door does". Each `decide_across` call now spells `["row", "row_floor"]` at the call, which the census reads as names (`a_funnels_literal_tuple_is_read_as_its_names`). The consts are gone.
