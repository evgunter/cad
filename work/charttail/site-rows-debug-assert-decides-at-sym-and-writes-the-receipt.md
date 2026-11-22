---
id: site-rows-debug-assert-decides-at-sym-and-writes-the-receipt
kind: issue
title: site_rows' kept-image debug_assert re-decides a row interval inside the assertion, which at a Sym scalar walks the tier and writes the session's receipt
status: open
opened: 2026-11-22
---


Found by BOXFINISH's sweep for "a check that runs the tier, or writes a
receipt column, after an answer that does not depend on it" (the shape
`boxfinish/decide-path-check` removed from `Decide for Sym<T>`).

`crates/topo/src/pcurves/site.rs`, `site_rows`' `item` closure, the
`(Some(cache), SiteHalf::Existing(he))` arm: a `debug_assert!` whose
condition runs `row_interval(body, he, cache, &carrier, band)` under
`k_stats::detached`. `row_interval` decides through the funnel. At a
`Sym<T>` lane each of those decisions runs `Decide for Sym<T>`, which
counts the decision in the installed session's `SymCounts`
(`count_decision`, `numeric` / `symbolic`) and, where the numeric
channel is not definite, walks the tier's forms (freezes land in
`frozen`). `k_stats::detached` returns the K channels but not the
session's receipt, which is mutated in place, so the receipt a run
reports depends on whether debug assertions are on (`[failure.taxonomy]`
row 5: a check for a kernel bug is a `debug_assert`, and its absence
must not change what a run says). This workspace builds release with
`debug-assertions = true`, so every profile pays it today.

Unmeasured: how many decisions the assertion adds to a receipt on the
corpus (count `SymCounts::decisions` over a `Sym<Interval>` build with
the assertion's body replaced by `true`, against the head).

Shapes of a fix, none chosen: run the check under a door that sets the
session aside (`sym::aside` cuts the decision logs; nothing restores the
counts today), or read the kept image's interval without deciding.
