---
id: viewer-readme-rustdoc-posture-describes-the-per-pr-skip-mode-pass
kind: issue
title: crates/viewer/README.md's Rustdoc posture bullet argues a per-PR skip-mode rustdoc pass that no PR runs since the latency cut
status: open
opened: 2026-09-28
priority: P4
cost: E
---


Found by the 2026-09-28 tracker sweep that closed MIRROR
(`docs/doc-ledger/mirror-and-blind-leave-the-tracker.md`).

`crates/viewer/README.md` §*Rustdoc posture* (from `:2022`) argues which
rustdoc pass reads the renderer-free half on a PR — the skip mode keyed
on `run_viewer_toolkit` / `VIEWER_TOOLKIT_SEEDS` against a closure-keyed
scope, and the open hole for `Refusal::NoSuchParam`'s link into
`editor_core`. Since the CI-latency cut (`work/ciw/latency-cut.md`) no PR
runs rustdoc at all: `nightly.yml`'s `rustdoc (gate, every root)` runs
`scripts/doc-gate.sh` without skip mode, so the all-features pass reads
the whole crate with the link lint live once a day. The sweep re-pointed
the bullet's dangling citation only; the argument around it still
describes the per-PR pass as live.

Re-word the bullet to the nightly reading (a clause re-worded because the
CI it describes moved, not a new decision), keeping the sweep rule and the
"what the rule cannot match" paragraph, which are still true.
