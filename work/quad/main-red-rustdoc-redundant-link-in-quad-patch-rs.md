---
id: main-red-rustdoc-redundant-link-in-quad-patch-rs
kind: issue
title: main red: rustdoc's --no-default-features pass rejects a redundant explicit link in geom-brep props/quad/patch.rs
status: open
opened: 2026-11-09
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), job `rustdoc (gate, every root)`, the `crates/geom-brep/Cargo.toml` pass at `--no-default-features`: `redundant explicit link target` at `crates/geom-brep/src/props/quad/patch.rs:120` (`[`knot_aligned_cuts`](super::cells::knot_aligned_cuts)`).

Red since before 11-02 (run 41393708363), so it is not BOXES'.

(BOXES nightly-reds triage, 2026-11-09)
