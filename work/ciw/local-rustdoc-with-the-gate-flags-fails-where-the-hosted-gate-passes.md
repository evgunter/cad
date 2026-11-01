---
id: local-rustdoc-with-the-gate-flags-fails-where-the-hosted-gate-passes
kind: issue
title: the hosted rustdoc gate was red every night from 2026-10-02 on broken intra-doc links across six crates
status: closed
opened: 2026-10-01
closed: 2026-10-15
priority: P2
cost: E
---

The nightly job `rustdoc (gate, every root)` (`scripts/doc-gate.sh`)
was red every night from 2026-10-02 through nightly 61 (run
38687151370, job 116264822459). The cause was rustdoc lint errors in
kernel prose: 78 reported sites over editor-core, geom-core, geom-brep, topo,
sweep, mesh, pncad, profile and viewer, almost all unresolved
intra-doc links to items renamed, deleted, made test-only, or out of
scope, plus a few redundant explicit targets and fn/module
ambiguities. Branch `ciw/rustdoc-gate-red` fixed every one
mechanically (qualify the path, disambiguate, or unlink to code text);
its PR body carries the site list. `scripts/doc-gate.sh --selftest`
followed by `scripts/doc-gate.sh` then passes locally, every pass.

## Why the list kept growing

Each pass is one `cargo doc` invocation, and cargo stops scheduling
new crates after the first one rustdoc rejects ("build failed, waiting
for other jobs to finish"). A red pass therefore reports only the
crates that happened to run before the first failure, and fixing those
uncovers the next ones. The 2026-10-12 note's list was one such
partial reading; the full set came out over five local rounds, four of them run with
`--keep-going`, which the gate itself does not pass.

## The original local/hosted discrepancy is moot

The row was opened because a local run failed on 2026-10-01 text that
the hosted gate passed (job 110459257131 on `0dbfa319` logs `doc-gate
OK`). From 2026-10-02 the hosted gate failed on the same class of
sites as local runs, and on nightly 61 the hosted `--no-default-features`
passes reject the same lines a local run rejects
(`crates/topo/src/boolean/ops.rs`'s two redundant `ValidationError`
targets, `crates/geom-core/src/linalg/svd.rs`'s `Real`). Both sides run
the pinned 1.97.0 toolchain (`rust-toolchain.toml`) and the same
script. Why the one 2026-10-01 hosted run was green was not
established; with the gate now agreeing with itself on both sides and
green, nothing turns on it.

## Filed from here

- `module-docs-on-both-sides-resolve-their-inner-links-in-the-parent`:
  three of the sites were `//!` links that rustdoc resolved in the
  parent module's scope, a latent class that can resolve to the wrong
  item with the gate green.
