---
id: ef-bound-backed-migrates-to-region-confinement
kind: issue
title: ef_bound_backed's declared face-pair arms are grandfathered (region-unconfined); their migration is CONTACT-12's dropped step 2
status: closed
opened: 2026-10-08
priority: P3
cost: M
pr: 4559
closed: 2026-10-10
---


Split off `overlap-lane-boundary-crossing-cuts` when CONTACT-12 was re-scoped under the D10 hold (2026-10-08). CONTACT-12 lands the boundary-crossing cuts this migration was blocked on.

The migration itself is still to do: re-attempt MATE-9's region-confined `ef_bound_backed` under the measured-migration protocol (`crates/topo/README.md`, Grandfathered rungs). The anomaly pin `review_mate4a_r2_probes::r2_an_unrelated_declared_pair_backs_the_ef_bound` is its guard.

CONTACT-12's dual review (R1, MINOR-5) found that the parking existed only in prose, with no row of its own.

## The premise that parked it was false (2026-10-10, CONTACTHOLD)

The row was parked on `declared-pairs-retire` as "likely moot there".
It is not: `ef_bound_backed`'s face-pair arms read `Declared.faces`,
which `Declared::index` builds from the op's curve and patch
`ContactRecords`, not from user declarations. F deletes `declare`,
`DeclaredPair` and the declared seats; it does not touch
`ContactRecords`, so the rung stays, and since E an undeclared flush
seat makes the same records a declared one did. The guard pin
`review_mate4a_r2_probes::r2_an_unrelated_declared_pair_backs_the_ef_bound`
was still on main after E.

## Done

The face-pair arms read `Declared::vf_face_backed_confined`: a pair
incident to the bound backs it only where it passes Door 1 and Door 2
reaches `PositiveArea` (`pair_verified`, either frame order).
Incidence already puts the bound in the closed intersection of the two
regions, so no point test is run: CONTACT-12's version (commit
`0bd1071f45`, reverted in `ba2a473811` for the D10 hold, not on its
merits) also ran `pair_holds_point`, which the review of PR 4559 showed
is implied at both call sites and only pushed in-band escalations. The anomaly pin is re-baselined as
`r2_an_unrelated_declared_pair_backs_no_ef_bound`.

## Closed (2026-10-10, PR 4559)

Merged after a full single review and a verified fix pass.
