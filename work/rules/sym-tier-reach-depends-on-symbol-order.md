---
id: sym-tier-reach-depends-on-symbol-order
kind: issue
title: The symbolic tier's reach depends on symbol order
status: open
opened: 2026-10-04
priority: P2
---

How many identities the symbolic tier proves on a document depends on the
order of its parameter symbols, not only on the document. Both reviewers of
PR 4011 measured it on `r2_filleted_bracket` in
`crates/editor-core/tests/sym_9_retry_interval.rs:491`
(`sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`), re-keying the
symbol that `param_env_over` (`crates/editor-core/src/analysis.rs:1011`) hands
to `axis_of`:

| symbol keying | registered (without / with the ladder) |
|---|---|
| the old name hash | 156 / 162 |
| `!id` (order reversed) | 156 / 162 |
| `id.rotate_left(17)` or `(32)` | 154 / 160 |
| raw `VarId` (as merged) | 154 / 160 |

The other four documents in the row do not move under any keying. No class of
identities is lost; which two registrations close follows the relative order of
the symbols. Before PR 4011 that order came from a hash of the name; now it
comes from the minted `VarId`, so an unrelated earlier edit can move a pin.

Nothing in `geom-core/src/sym*` states whether reach should be invariant
under relabelling. Either find the order-dependent choice (a normal-form
tie-break, a term order, a cut-off) and make it order-free, or state the
dependence where the tier's reach is described and stop pinning counts that
hold it.
