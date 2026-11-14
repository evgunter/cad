---
id: the-copysign-census-counts-f64-typed-sites-a-sym-scalar-never-reaches
kind: issue
title: The copysign census counts sites that are f64 by type, so a mesh-only change reds a geom-core slow row and the f64-only entries are held by hand
status: open
opened: 2026-11-11
priority: P3
cost: M
---


Found while closing `main-red-the-copysign-census-names-a-walk-rs-site-since-the-horn-cusp` (PR 6838).

**The class.** `geom-core`'s `sym_rule_f_rows::the_copysign_mint_sites_the_tree_holds_are_these` (`crates/geom-core/tests/sym_rule_f_rows.rs:692`) counts every `.copysign(` / `.unit_sign(` in shipped source by text, whatever the receiver's type. A site whose scalar is `f64` by type mints no `Sym` atom and cannot reach a tier decision, but it counts all the same. Two things follow:

- A change confined to a crate that never instantiates `Sym` reds a `geom-core` slow-set row. `crates/mesh/src/walk.rs:1439` (`loop_polygon`, `Body<f64>`, `Chart<f64>::is_horn`) did this at PR 6806.
- The "`f64` only" disposition of such sites is kept by hand in `crates/geom-core/src/sym/manifest.rs`'s header prose, which grows by one entry per PR: `linalg/svd.rs`'s Householder (`manifest.rs:61`), `geom/src/curves/banded.rs`'s dropped `L` zero (`manifest.rs:62`), and `mesh/src/walk.rs`'s horn cusp (`manifest.rs:109`). Nothing in the census tells an `f64`-typed site from a generic one, so nothing checks that disposition either.

Not fixed here. The slow-set class (no PR gate selects these censuses) is separate, on green's `a-src-change-that-moves-a-decide-site-skips-its-crates-recourse-roster`.
