---
id: the-ledger-census-keys-doors-on-spelling
kind: issue
title: flagged_census resolves a forwarding wrapper by identifier name: shadowing launders a row, a wrapper reached by fn pointer or alias is silent, doors are keyed by bare fn name
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Filed from PR 6603's FULL review, after its MINOR (same-line dedup) was fixed in the PR.

- **Shadowing launders a row.** `fn w(row: &'static str) { let row = pick(); check_unlogged(.., row) }`, or a closure `|row|` param, reads as forwarding the parameter. So the caller's literal is checked, not the row actually passed.
- **A wrapper reached only through a fn pointer** (`let f = w; f("BAD")`), a renamed import or a macro yields no site at all. Before 6603 such a wrapper was a red site citing ""; now it is silent.
- **Doors are keyed by bare fn name** (`known: Vec<(String, usize)>`). A wrapper with a common name (`new`, `check`) would make every same-named call a door read. That's loud, but it's keyed on spelling.
- **The self-test asserts `contains`, not the exact site set.** It covers neither a wrapper whose caller passes a non-literal/const, nor shadowing.
- **The module header** (flagged_census.rs:1-20) still describes the census as decide_flagged-only. The `prev_ok` comment (~:201) describes the old exclusion mechanism.
