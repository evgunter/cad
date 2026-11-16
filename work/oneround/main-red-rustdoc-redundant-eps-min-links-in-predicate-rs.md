---
id: main-red-rustdoc-redundant-eps-min-links-in-predicate-rs
kind: issue
title: main red: rustdoc's --no-default-features pass rejects three redundant explicit eps_min links in geom-core predicate.rs, since PR 6380
status: open
opened: 2026-11-09
priority: P0
---

Nightly run 42567046151 (sha `836426e07e`), job `rustdoc (gate, every root)`, the `crates/geom-core/Cargo.toml` pass at `--no-default-features`: three `redundant explicit link target` errors in `crates/geom-core/src/predicate.rs`, each spelled `[`eps_min`](crate::tolerance::eps_min)` (lines 503, 516 and 622, in the `Lever` docs). PR 6380 (`onehome/run-owns-rounding`, merge `b2cc80453b`) added all three. rustdoc reports each target as redundant, meaning `eps_min` already resolves there. Drop the explicit targets, then run `scripts/doc-gate.sh` to confirm the bare form resolves in every pass.

Not BOXES'.

(BOXES nightly-reds triage, 2026-11-09)
