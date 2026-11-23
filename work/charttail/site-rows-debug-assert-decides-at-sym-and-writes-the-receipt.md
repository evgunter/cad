---
id: site-rows-debug-assert-decides-at-sym-and-writes-the-receipt
kind: issue
title: A decision taken in debug builds only writes a Sym session's receipt that a release build lacks (six sites, site_rows' kept-image assertion among them)
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

**2026-11-23, the class.** The same shape, a decision through the funnel
that only a debug build takes, at a generic `T: Decide` (so at `Sym<T>`
it counts in the session's `SymCounts` and lands in the re-valuation
record), is at six sites. Read by `boxfinish/decide-path-check`'s fix
pass over every `debug_assertions` mention and every `debug_assert*`
outside test modules and `geom-core/src/sym*`:

1. `crates/topo/src/pcurves/site.rs:1000`, `site_rows` (above).
2. `crates/topo/src/held.rs:104`/`:111`, `Held::check`: returns at once
   unless `cfg!(debug_assertions)`, then
   `k_stats::check_unlogged("held_on_carrier", …)` of the point's
   implicit residual.
3. `crates/topo/src/boolean/shell_witness.rs:446`/`:456`,
   `debug_assert_contacts_undecisive`: `point_in_solid` for every
   recorded contact vertex, behind the same early return (callers
   `boolean/finish.rs:240`, `boolean/ops.rs:7932`).
4. `crates/topo/src/boolean/insert.rs:2741`, under
   `#[cfg(debug_assertions)]`: `sectors::rays_within` (both rays, each
   `decide("bool_sector_within", …)` per sector, `sectors.rs:1216`).
5. `crates/topo/src/boolean/vtxfac.rs:2904`, `pierce_germ_dir`: the same
   `rays_within` read under `#[cfg(debug_assertions)]`.
6. `crates/topo/src/link.rs:2258`, `Cone::reads_tied`:
   `debug_assert_ne!(self.closes(rd), …)`, where `Cone::closes`
   (`:2100`) reads every pair of bounds through `metric::sign_over`,
   `wedge_side` and `ahead*`: O(bounds²) decisions per call.

Near miss: `boolean/vtxfac.rs:1303`, `read_vertex_on_face`'s in-plane
check, forces `declared.sections.carried(..)`, which the path that
reaches it has already forced (`join.rs:3525`); it decides nothing new
while that order holds.

**The sweep's blind spot now.** With a definite numeric sign walking
nothing, a debug-only decision moves `numeric` or `symbolic` and the
re-valuation record, and no `frozen` count: a receipt diff that reads
only the tier's walks does not show it. And a site of the shape of 2 and
3, a callee whose own body is debug-gated, matches no pattern a sweep of
`debug_assert!` conditions spells: it is found only by reading each
`cfg!(debug_assertions)` early return and `#[cfg(debug_assertions)]`
statement, as above, and a new one is found by nothing.
