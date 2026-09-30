---
id: k-report-bit-identity-claim-has-no-citation
kind: issue
title: K-REPORT's decisions-bit-identical-to-f64 claim can now cite the differential that checks it
status: open
opened: 2026-09-15
priority: P3
cost: E
---


## What

`docs/K-REPORT.md`'s collection-method section describes the `Probe`
run as recording `(predicate, margin, band, outcome)` *"with decisions
bit-identical to"* the f64 lane. Until SUITE/D114 that was an unchecked
claim — the report's own subject rests on it, and the three probe rows
beside it asserted one-sided greenness only.

It is now checked:
`crates/editor-core/tests/m4_pr8_k_probe.rs`'s
`probe_agrees_with_f64_bit_for_bit_over_the_corpus` evaluates the whole
Band 4 corpus at `Probe` and at `f64` and compares the two bit for bit
— per-node value channel, mass properties, content keys, name tables,
verdict and escalation logs — once per ε row `k_probe_sweep.sh` dumps
(1e-6, 1e-9, 1e-12).

## Fix

Cite the differential where the claim is made, so a reader of the
report can tell a checked premise from an argued one. The same sentence
appears in `crates/geom-core/src/k_stats.rs`'s module docs (PROPS'
ground) and at `ContentBits for Probe` in
`crates/editor-core/src/eval/memo.rs` (EDIT's); both are true and both
would read better with the citation, but the report is the document
whose conclusions rest on it.

**What the citation should not overstate.** The differential does not
read curve carriers, surface geometry or pcurves, and it covers only
the registered corpus — not the demo scenes the same sweep dumps, and
not `cup`/`vessel`.

Filed by SUITE/D114.
