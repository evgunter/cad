---
id: main-red-revalue-corpus-carrier-and-witness-on-surface-drift-since-7411
kind: issue
title: Main red: ten revalue_corpus rows fail since PR 7411 (germ/plane-pose-far-frame): the re-valued DAG drifts from Sym's build on carrier_on_surface_{1,2} and witness_on_surface_{1,2}
status: closed
opened: 2026-11-23
priority: P1
cost: M
closed: 2026-11-23
pr: 7678
---


Found by DOCTAIL's 7396 lane (2026-11-23) while running `editor-core`'s `revalue_corpus` locally. Filed by the DOCTAIL orchestrator. It was not reproduced by CI, because the push CI on main is queued.

## What

Ten `editor-core::all` `revalue_corpus::*` rows fail on origin/main, identically at ε 1e-6, 1e-9 and 1e-12. A re-valued DAG drifts from `Sym`'s build on `carrier_on_surface_{1,2}` / `witness_on_surface_{1,2}`. One of the ten is `revalue_a_chart_on_a_cap_tilted_at_zero` (from DOCTAIL's PR 7434); it is red for the same reason, not its own.

**Bisected on a clean main:** green at `1883da3` (#7437), red at `3547fe5`, which is **PR 7411, `germ/plane-pose-far-frame`** ("germ: the plane pose's offset is posed near its geometry; the ring path reads cross uncharged").

## Owed

The lane that caused the red owes the fix (`memories/agent-lane-operations.md`): GERM's, or whoever holds PR 7411's follow-up. Either the re-valuation reads the plane pose's new near-geometry offset the way `Sym`'s build does, or the build and the revalue instrument agree on one spelling.

Reproduce: `cargo nextest run -p editor-core --test all -E 'test(revalue_corpus::)'` on main.

## Reproduced and bisected on hosted CI (2026-11-23, GREEN U18)

`revalue_corpus::revalue_boss_union_apart_to_its_parting` was checked on
throwaway probe branches, all deleted since. It passes on `dc263f8553`
(run 45323490953), `698c3b1949` and `1883da3554` (run 45329987319). It
fails on `3547fe5764` (PR 7411, run 45330006738) and on every later probe
through main `a433ff83d6` (run 45312452002). Each failure is the same:
`drifted: 228`, with `carrier_on_surface_{1,2}` and
`witness_on_surface_{1,2}` laundered. Nightly 45117679717 (`dc263f8553`)
passed it at every ε.

**The mechanism, read from 7411's diff and not run:**
`Real for Sym<T>`'s `difference_dot` (`crates/geom-core/src/sym.rs`)
keeps `T`'s posed value. For `f64` that is the compensated
`two_sum`/`two_product` reading in `crates/geom-core/src/real.rs`. It
records `plain_difference_dot`'s node, though, so the DAG holds
`(q − o)·n` while the build's value is the compensated one. Re-valuing
the DAG at `p0` evaluates the plain formula, which is not the build's
bits wherever the two readings differ in the last places: every
`*_on_surface_*` residual 7411 moved onto `difference_dot`. Either the
DAG records an op that re-values the way `T` reads it, or the
instrument treats the node as `T`'s. Choosing between those is GERM's
design call (or SYM's, for the op set).
## Closed onto SYM (2026-11-23, GREEN U19, PR 7678)

**Bisect confirmed on CI** (probe run 45312428995): every row is green on
`1883da3` and red on `3547fe5`, at the default ε. On main `a433ff83d6`
all ten are red at default, 1e-6 and 1e-12 (probe run 45308109008).
`m10_p_lift::at_sym_a_fixed_box_is_the_build` is red for the same
reason.

**The instrument is right, and no expectation was changed.**
`Sym::difference_dot`, which `geom::plane_offset` reads since 7411, takes
its value from `f64`'s `Dot2` and its node from the plain formula. A
re-valued node is therefore not the computation of the value it stands
for. `Sym::harmonic_belly` (PR 7553) has the same shape. Fixing it is a
node-shape choice in SYM's ground. It is filed with the options as
`work/sym/a-sym-posed-reading-carries-a-value-its-node-does-not-compute.md`
(P1, `design: true`), and that row carries this one's evidence. The rows
are left red: a `pinned_red` stop on the drift assertion would silence
every later check in each row.
