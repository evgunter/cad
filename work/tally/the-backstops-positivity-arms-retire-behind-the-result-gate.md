---
id: the-backstops-positivity-arms-retire-behind-the-result-gate
kind: issue
title: The volume backstop's vol ≥ 0 arms read the result's sign after the tier-3 result gate has: retire them once check 7 reads it the same way
status: closed
opened: 2026-10-03
priority: P3
cost: E
refs: [3987]
pr: 5842
closed: 2026-10-27
---


Left by `boolean-door-adopts-the-finished-body-type` (PR 3987, the dual
review's F7).

`ops::volume_backstop` (`crates/topo/src/boolean/ops.rs`) checks
`vol(A ∩ B) ≥ 0` and `vol(A ∖ B) ≥ 0` on the result
(`Posture::PlusV`, tier 3's own reading `validate::plus_v_read`). Every
result reaches the backstop through `ops::gate`, whose tier 3 runs check
7 on the same body first, so at a certifying scalar the arms are dead:
the review's mutant M3 (both arms removed) reddens only the two backstop
unit rows that call `volume_backstop` directly
(`volume_backstop_joint_and_sign_arms`,
`volume_backstop_reads_the_sign_against_the_band`). At a dual neither
runs (`gate_volume_backstop` answers `NotRunAtThisScalar`).

The unit item retires the arm "once check 7 reads the same predicate the
same way". Check 7's sign read and the backstop's differ until check 7
takes the backstop's interval re-derivation (PR 3977,
`reach/check7-interval`). Once that lands: delete the two arms and their
`Posture::PlusV`, and re-point the two unit rows at the result gate.

## Closed (2026-10-27)

The volume backstop left the boolean's operation path (`[ev]` PR 5780: the volume relations are a test oracle the suites run on their own results, `topo::test_support::volume_relations`), so nothing refuses or passes on it. The two arms, `Posture::PlusV` and `validate::plus_v_read` are deleted; tier 3's check 7 at the result gate is the one reading of the result's sign. The two unit rows went with the backstop; the oracle's `a_planted_result_breaks_its_relation` flags an inside-out ∩ and ∖ result.
