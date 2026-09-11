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

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Moved to S-TINT (2026-09-11)

Moved by `git mv` from `work/tcost/` at S-TINT's opening. Id, title and
body are unchanged; the directory is the claim (`work/README.md`).

**Why it moved.** S-TCOST's board was re-sorted on 2026-09-11 against the
repository going public on 2026-09-03 (`work/tcost/log.md`, the
2026-09-11 seam). That sort found this row is not a cost lever in either
currency — it neither shortens the gate's critical path nor saves a
billed minute, and it was never argued on one. It reached S-TCOST by the
tracker-wide re-home of 2026-09-04, which routed rows by PATH GLOB
(`crates/*/tests/*`, `crates/test-utils/*`) rather than by question.
This program is the question it was always about: whether the suite
asserts what it claims to assert.
