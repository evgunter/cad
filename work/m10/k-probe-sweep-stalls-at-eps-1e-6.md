---
id: k-probe-sweep-stalls-at-eps-1e-6
kind: issue
title: k_probe_sweep.sh cannot complete at eps = 1e-6: the E6 driver probe row panics 'nothing certified, nothing to sample'
status: open
opened: 2026-08-30
github: 1304
refs: [1296]
---

## From GitHub issue 1304

Opened 2026-08-30; 0 comments.

(S-CERT orchestrator) Found by CERT-4's lane while trying to produce a k-lint verdict for its branch; **verified identical at merge base `ad2f9757`** — pre-existing, not that unit's change.

At tolerance row `eps = 1e-6`, `k_probe_sweep.sh`'s "E6 driver" row panics at `m10_3_driver_k_probe_interval.rs:117` (`nothing certified, nothing to sample`): the slab fixture certifies nothing at that tolerance, and the probe treats an empty certified set as a panic rather than a reportable outcome. Consequence: the k-lint sweep cannot complete AT ALL at that eps, so no branch can produce a k-lint verdict there — a silent coverage hole in the sweep's matrix rather than a red anybody sees.

Two candidate shapes, whoever takes it decides: the probe row reports an empty-certified outcome honestly (a census-style row that can say "0 certified at this eps" without dying), or the fixture is re-cut so the row exercises what it means to at every eps it claims. Per `memories/test-suite-cost.md`, ask which SHAPE the row is before reaching for a seed.

The fixture and row are M10-3's ground — (M10 orchestrator) flagged for ownership; S-CERT is a reporter here, not a claimant.

## Home

`work/m10/` — the fixture and row are M10-3's ground (`crates/editor-core/tests/m10*`), which the issue states explicitly; S-CERT filed it as a reporter.
