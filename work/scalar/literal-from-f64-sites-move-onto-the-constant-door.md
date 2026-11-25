---
id: literal-from-f64-sites-move-onto-the-constant-door
kind: issue
title: The literal from_f64 sites move onto Real::lit, and the bare door closes behind them
status: open
opened: 2026-11-24
priority: P2
cost: M
parent: from-f64-is-split-by-where-the-number-comes-from
---


Found by `from-f64-is-split-by-where-the-number-comes-from`, which built the constant door and left the literal sites where they were. `scripts/gates/from-f64-allowlist.sh` exempts a `from_f64` whose argument is built only from numeric literals, SCREAMING_CASE constant paths, `as f64` and arithmetic: 679 sites in `crates/*/src` and `demos/*/src` production code at the time of filing (631 in the nine kernel crates, 48 in `demos/tour`), which the gate prints on every run.

Each is `T::from_f64(0.5)` and becomes `T::lit(geom_core::lit!(0.5))`: a mechanical sweep, one token rewrite per site, no value moved at any scalar (`Real::lit` is `from_f64` of the same number). Two things are not mechanical:

- A literal that names a generic `const` of its function (`N as f64` over a `const N`) does not compile inside `lit!`: an item sees no outer generic. It takes a `geom_core::Constant` written by hand, or it is a count and takes `Real::count`.
- The spelling is longer at every site. A shorter one is a design choice the sweep should make once (a `lit!` form that names the scalar, constants on `Real` for the common few), before 679 sites take it.

Test code holds several thousand more `from_f64` calls the gate does not walk. The bare door is a trait method, so it closes (a sealed argument, or leaving the public trait) only when they have a spelling too; say which in the sweep.

When this row and the owed entries of `scripts/gates/from-f64-sites.tsv` are done, `Real::from_f64` has no caller but the doors and the scalar impls, and the exemption by shape comes out of the gate.

Cost: M.

## From PR 7721's reviews

- `lit!` declares `struct K` and expands its argument inside `impl Constant for K`, so it shadows a caller's `K` and `Self`: `lit!(K)` over a caller's `const K: f64` does not compile (E0308, it reads the unit struct), and `lit!(Self::HALF)` in an `impl` does not either (E0599). No literal site has either shape today; the tree is full of `K` and `Self::` constants, so the sweep will meet both. Fix the macro's hygiene first.
- `T::lit(geom_core::lit!(0.5))` says "lit" twice and names the crate at every site. Choose the shorter spelling (a macro that names the scalar, or constants on `Real` for `0.5`, `2.0` and the few that make up most of the sites) before it is written 681 times.
