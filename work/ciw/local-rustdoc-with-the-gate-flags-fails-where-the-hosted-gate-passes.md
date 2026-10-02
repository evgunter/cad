---
id: local-rustdoc-with-the-gate-flags-fails-where-the-hosted-gate-passes
kind: issue
title: cargo doc with doc-gate.sh's own lint set fails locally on main's text that the hosted rustdoc gate passes; the cause is unexplained
status: open
opened: 2026-10-01
priority: P3
cost: E
---

Found by CLEAVE's mint-doors lane (PR 3720) while checking its own
prose, then confirmed by the CLEAVE orchestrator. It is not fixed
there.

## Measured (local, 2026-10-01, main's own text)

`scripts/doc-gate.sh` defines the lint set
`RUSTDOC_LINTS="-D warnings -A rustdoc::private_intra_doc_links"`
(the `RUSTDOC_LINTS=` line) and runs
`cargo doc --no-deps --document-private-items "$@"` (`doc_pass_with`).
The workspace pass adds `--workspace --all-features`. Run locally with
those flags:

- `-p topo -p sweep --all-features` fails in `crates/topo/src/boolean/join.rs`
  (lines ~181, ~212): `` [`JoinLane::Split`] `` and
  `` [`JoinLane::BoolPlanar`] `` are unresolved. `JoinLane` is imported
  only inside a nested `use` further down the file, at its use site,
  not where the doc is.
- `--workspace` fails in `crates/sweep/src/blend/battery.rs:5`
  (`` [`super::decide`] ``: no item `decide` in module `blend`) and in
  `crates/editor-core/src/mate/solve.rs:444` (`solve_cluster`).

Yet the hosted nightly job `rustdoc (gate, every root)` passed on
`0dbfa319` at 16:07 on 2026-10-01 (job 110459257131), and all three
links are present at `0dbfa319` (`git show 0dbfa319:<path>`).

## What is unexplained

Either the hosted gate runs different flags, toolchain or targets from
what `doc-gate.sh` reads as, or the local run differs in some way
nobody has identified (a different toolchain, a stale lint set, the
private-items pass). Until that is known, a broken intra-doc link in
kernel text may pass the nightly unseen, or a local run of the gate
gives a red that CI does not reproduce. Start by diffing the hosted
job's log (the exact `cargo doc` lines and `rustc -V`) against a local
`scripts/doc-gate.sh` run.
