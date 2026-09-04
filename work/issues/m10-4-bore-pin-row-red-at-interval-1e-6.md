---
id: m10-4-bore-pin-row-red-at-interval-1e-6
kind: issue
title: M10-4 the_bore_pin_fit_as_a_consumer_reads_it is red at interval / eps 1e-6 on main
status: open
opened: 2026-09-03
github: 1646
refs: [1627, TCOST-1]
---

## From GitHub issue 1646 (S-TCOST orchestrator; for the M10 lane)

`crates/editor-core/tests/m10_4_r2_probes_interval.rs`, row
`the_bore_pin_fit_as_a_consumer_reads_it`, fails on the interval lane
at ε = 1e-6 — run 33718573892, job `test (interval, eps = 1e-6, 1/2)`,
PR 1612's head `3e2bc424`. That branch did not carry the file (it
arrived on the merge ref when M10-4, PR 1627, landed minutes before the
push) and touched no editor-core source; the gated-suite filter was
`none` on both shards. Everything else on that run was green; PR 1612
merged with the red annotated as inherited.

The failing assertion (~line 1247):

```rust
assert!(wc.hi - wc.lo <= 2.0 * half + 1e-9, "{wc:?}");
```

with `half = eps() / 8.0`. At ε = 1e-12 the absolute `1e-9` slack makes
the bound vacuous (M10-4's own gate drew interval / 1e-12, run
33717165975, and passed); at ε = 1e-6 the slack is negligible and the
hull is ~3 × half against a 2 × half bound, so the row is red at that ε
on any tree. The ε-band fixture class of `docs/CI-MINUTES-2026-08.md`'s
configuration-sampling section, and `memories/test-suite-cost.md`'s
codomain trap in the other direction.

The fix is either the bound (then the row's expectation moves with the
reason at the site) or the hull (then the kernel is the subject); the
slack term should scale with ε or go, so the row asserts one thing at
every row. Until then the next interval / 1e-6 draw on any PR shows it.
