---
id: k-report-bit-identity-claim-has-no-citation
kind: issue
title: K-REPORT's decisions-bit-identical-to-f64 claim can now cite the differential that checks it
status: closed
opened: 2026-09-15
priority: P3
cost: E
closed: 2026-10-26
---


## What

`docs/K-REPORT.md`'s collection-method section describes the `Probe`
run as recording `(predicate, margin, band, outcome)` *"with decisions
bit-identical to"* the f64 lane. Until SUITE/D114 that was an unchecked
claim — the report's own subject rests on it, and the three probe rows
beside it asserted one-sided greenness only.

It is now checked:
`crates/editor-core/tests/m4_pr8_k_probe.rs`'s
`probe_agrees_with_f64_bit_for_bit_save_the_pinned_key_splits` evaluates the whole
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

## Closed (2026-10-26, tracker sweep)

**Fixed in this sweep.** `docs/K-REPORT.md`'s Methodology bullet
(`:24-35`) now cites
`crates/editor-core/tests/m4_pr8_k_probe.rs::probe_agrees_with_f64_bit_for_bit_save_the_pinned_key_splits`.
It keeps the two limits this row asked for: the differential does not
read carriers, surface geometry or pcurves, and it does not cover the
demo scenes or `cup`/`vessel`. The two sibling sentences, in
`crates/geom-core/src/k_stats.rs` and `crates/editor-core/src/eval/memo.rs`,
are other programs' ground and were not edited.

That differential is one of the two rows recorded as red on main in
`work/green/main-reds-editor-core-slow-set-and-the-k-probe-sweep`. The
citation names the check. Whether the check passes today is that row's
question.
