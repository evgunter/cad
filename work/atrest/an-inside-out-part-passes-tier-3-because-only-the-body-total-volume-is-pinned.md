---
id: an-inside-out-part-passes-tier-3-because-only-the-body-total-volume-is-pinned
kind: issue
title: A multi-solid body whose one solid is inside-out (negative signed volume) passes tier 3 when the body's total volume is positive — validate pins only the total, never per solid
status: closed
branch: atrest/1-per-solid
opened: 2026-09-16
refs: [2767]
priority: P0
cost: D
parent: ATREST-1
closed: 2026-09-22
pr: 2977
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

## Closed 2026-09-22 by ATREST-1 (PR #2977)

Check 7's subject is the SOLID. `validate_geometric_certified` runs
one sign-certified walk per solid over `Body::faces_of_solid`, and
`NegativeVolume { solid }` names the offender. On a one-solid body the
face set is the arena entire and in arena order, so the walk is
bit-identical to the old whole-body one — an equivalence, not a
branch, with the tier-1 checks that establish it cited at
`check7_subjects`.

The reverted part beside a larger ordinary solid is refused, pinned by
a row in `crates/topo/src/tier3_tests.rs` that reds without the
change. A second instance turned up on the way:
`shell5_r2_probes::r2_the_new_door_mints_a_solid_with_no_outer_shell`
had PINNED this same admit-hole as measured-and-wrong (a solid whose
only shell is a cavity, passing because the body total was positive)
and is now a refusal.

Residues disclosed by the unit, each with its own file rather than a
sentence here: `work/atrest/the-multi-solid-reporting-quadrature-is-unscheduled`
and `work/atrest/expect-one-solid-on-solids-next-has-twenty-homes`.
