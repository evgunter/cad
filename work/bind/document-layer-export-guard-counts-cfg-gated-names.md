---
id: document-layer-export-guard-counts-cfg-gated-names
kind: issue
title: The document-layer export guard counts cfg-gated names; its profile-layer sibling does not
status: open
opened: 2026-09-14
priority: P4
cost: E
---

## What

`crates/pncad/tests/all.rs` carries two guards with one purpose —
every root export of a curated layer is either carried by the façade
or listed `NOT_CARRIED`/its sibling list — and they read their layer's
root through DIFFERENT views:

- the document layer's (`every_document_layer_root_export_is_carried_or_listed`)
  scans `module_pub_use_names(&code_and_literals(&src))`, raw text, so
  a `#[cfg(feature = "interval")] pub use …` statement's names count;
- the profile layer's sibling scans through `code_without_cfg_gated`,
  whose own documentation argues the opposite rule: "An item behind a
  `#[cfg]` is not that surface: no consumer's build graph turns the
  feature on, so the name does not exist one hop past the façade, and
  asking the façade to carry it would advertise something
  unreachable."

Both arguments are defensible and they are not the same argument. The
`interval` feature is NOT the profile layer's `test-support` case —
CI's interval lane is a shipped configuration and `pncad/interval`
forwards to `editor-core/interval`, so a consumer's build graph really
can turn it on, and counting those names is probably right. What is
wrong is that the tree states both rules without saying which applies
where, so a reader cannot tell a deliberate asymmetry from a drift.

## Why it matters now

DOCM-9 added seven `interval`-gated root exports to `editor-core`
(`work/lib/certified-range-has-no-python-door`) and the guard red the
build until they were listed — correctly, on the document-layer rule.
Had those same names landed one layer over, the sibling guard would
have said nothing. The next lane to add a gated export to the profile
layer gets no decision forced on it and will not know that.

## What would settle it

Either give `code_without_cfg_gated`'s header the carve-out it is
missing (a feature a shipped configuration enables is part of the
surface; `test-support` is not), and apply the same view to both
guards; or state at the document-layer guard why it deliberately reads
raw text where its sibling does not. Both guards are `meta`'s shape of
instrument, but the surface they guard is the façade's, which is LIB's.

Found by DOCM-9's fix pass, reported by its R2 review lane (MINOR-4).
