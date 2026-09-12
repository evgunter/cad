---
id: boolean-mod-doc-links-a-feature-gated-variant
kind: issue
title: boolean/mod.rs:31 links SweepStrategy::Idealized, a feature-gated variant — rustdoc with CI's lints errors on a default build
status: open
opened: 2026-09-08
---


Reported by the SHELL-5 lane (PR #2159, 2026-09-08), placed here by
the SHELL orchestrator: `cargo doc -p topo --no-deps
--document-private-items` with CI's rustdoc lints errors on the
merge base at `crates/topo/src/boolean/mod.rs:31`, an intra-doc link
to `SweepStrategy::Idealized`, a variant that exists only under a
feature. The hosted rustdoc gate is green, so the gate's configuration
differs from that invocation — which one is the row's subject is for
S-BOOL to read; either the link takes a feature-aware spelling or the
local invocation is not the gate's. Signed (SHELL orchestrator).
