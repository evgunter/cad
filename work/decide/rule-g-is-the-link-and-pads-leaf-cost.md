---
id: rule-g-is-the-link-and-pads-leaf-cost
kind: issue
title: rule G is the link's and the pad's leaf cost: shutting it takes the pad's release leaf 73.8s to 17.2s and the link's 8.5s to 3.8s, where shutting the decision read moves neither
status: open
priority: P2
opened: 2026-09-25
refs: [decision-read-triples-the-plate-pin-suites-wall-time, DECIDE-6]
---


## What was measured (DECIDE-6, 2026-09-25)

DECIDE-3 added rule G and the decision read in one unit, and
`decision-read-triples-the-plate-pin-suites-wall-time` put the plate
pin suite's 143 s → 535 s (dev) on the read. DECIDE-6 measured the
read and found it under half a percent of every replay (that row's
"Where the read's cost is (DECIDE-6)"). The same replays with rule G
shut (`SymRules::without_canonical_root`) are where the time moves.

On the release leaf instrument
(`m10_10_evidence_interval::m10_10_leaf_cost_with_and_without_the_algebra`,
one attempt per rung, `CAD_M10_10_COLUMNS=rules`):

| document | shipped (fastest of 3) | read shut (fastest of 3) | rule G shut (1 take) |
| --- | --- | --- | --- |
| R2 rounded pad (`1e2·ε`) | 73.83 s `[890, 6, 150, 907]` | 74.53 s `[894, 0, 150, 909]` | 17.15 s `[850, 6, 128, 969]` |
| R2 link (`1e1·ε`) | 8.52 s `[545, 0, 108, 449]` | 8.36 s `[545, 0, 108, 449]` | 3.83 s `[505, 0, 110, 487]` |
| R2 filleted bracket (`1e1·ε`) | 1.98 s `[1104, 7, 144, 766]` | 2.01 s `[1106, 0, 144, 771]` | not taken |

In dev (`decide_6_read_cost_interval::decide_6_where_the_reads_cost_is`,
ε = 1e-9, one take each):

| leaf | shipped | read shut | rule G shut | both shut |
| --- | --- | --- | --- | --- |
| link at `4.930e2·ε` | 34.72 s | 35.01 s | 19.06 s | 18.76 s |
| bracket at `3.870e2·ε` | 12.34 s | 12.59 s | 12.93 s | 12.90 s |
| plate at 0.2630 | 2.34 s | 2.30 s | 2.53 s | 2.58 s |
| annulus at 0.8416 | 2.45 s | 2.63 s | 2.48 s | 2.37 s |

`m10_10_pins_interval` itself is 337.83 s in dev at DECIDE-6's head
(`--test-threads 2`); its ceiling row replays the link three times and
the pad twice.

## What is not known

Where inside rule G the time goes: the mint site's quotient split and
side condition, the magnitude door, the exact quotient, or the larger
forms the canonical atoms leave downstream. The profile
(`geom_core::sym::profile`) already clocks each walk; a per-rule clock
at rule G's mint site is the next measurement. The pad's dev leaf with
rule G shut was not taken (the pad is measured on the release
instrument only on a four-core box).

## Home

`crates/geom-core/src/sym/root.rs` (rule G's mint site),
`crates/geom-core/src/sym.rs`'s `combine`.
