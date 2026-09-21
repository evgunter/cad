---
id: an-inside-out-part-passes-tier-3-because-only-the-body-total-volume-is-pinned
kind: issue
title: A multi-solid body whose one solid is inside-out (negative signed volume) passes tier 3 when the body's total volume is positive — validate pins only the total, never per solid
status: open
opened: 2026-09-16
refs: [2767]
priority: P0
cost: D
---

Found by BOOL-4's R2 review (PR 2767) as a side observation while
verifying the per-solid at-infinity sign, pre-existing, and filed by
the S-BOOL orchestrator on TOPO's slate (`crates/topo/src/validate.rs`
is TOPO's path). Tier 3's `NegativeVolume` reads the body's total
signed volume (Σ over every solid); `classify_shells` is never called
from `validate.rs`. A reverted (inside-out) part beside a larger
ordinary solid therefore certifies, and BOOL-4's per-solid
point-in-solid entry — which reads the SELECTION's sign — would answer
the complement's material correctly while the certifying door never
noticed the part was inverted. The fix is a per-solid volume sign at
tier 3 (each solid's closed-form volume definitely positive, decided
under the run band), refusing typed with the solid named. Measured,
not acted on; difficulty S.
