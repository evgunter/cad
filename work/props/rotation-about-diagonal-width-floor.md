---
id: rotation-about-diagonal-width-floor
kind: unit
title: Mat3::rotation_about's diagonal carries a width floor at exact angles (1 − cos plus cos's own enclosure), and Affine3 composition through MappedCurve::restrict grows it per split
status: closed
parent: certified-lane-non-real-contract-audit
opened: 2026-09-05
closed: 2026-09-05
pr: 1980
branch: props/rotation-floor
refs: [certified-lane-non-real-contract-audit, 1277, mapped-curve-restrict-composes-placements-per-split]
---

The audit's member 5 and its rider, split out of PROPS-1 because the
fix respells every rotation in the kernel for a measured sixth of the
residue — a decision of its own, not a member to take in passing.

## Measured (CERT-3, PR 1277, and the S-CERT correction at its fix pass)

`Mat3::rotation_about` (`crates/geom-core/src/linalg/mat.rs`) builds
`t = 1 − cos θ`. The backend's `cos` at the exact point `θ = 0`
encloses `[0.9999999999999996, 1]`, so `t` encloses `[0, 4.44e-16]`
where its true value is exactly zero — a floor independent of the
angle. `2·sin²(θ/2)` encloses `[0, 2.5e-323]` there, and
`Mat3::identity_minus_rotation_about` already uses the half-angle
forms for exactly this reason.

The payoff is smaller than the floor suggests, and that is the reason
this is its own item. The diagonal entry `t·nᵢ² + c` is ~8.88e-16 wide
because the `1 − cos` floor and `cos`'s OWN enclosure at an exact
angle ADD: retiring `1 − cos` alone recovers 0% (the near-unit sum
still rounds outward by an ulp); respelling both `t` and `c` from the
half angle recovers ~17% at a `RevolvedPoint` start sample and 0% at
its full-period sample. The irreducible part is the backend's `cos`
enclosure at exact angles, not a spelling. Instrument:
`crates/geom-core/tests/cert3_evidence.rs` (`#[ignore]`d rows).

Rider, same diagonal: repeated `MappedCurve::restrict` composition
through `Affine3::Mul` re-applies the diagonal enclosure per split —
stored width grows linearly, +3.55e-15 per split on an exact-axis
fixture (a law row on PR 1277). The fix belongs to whichever unit
takes the respell; treat the two as one entry.

## What a decision owes

Whether a ~17%-at-best narrowing of every rotation's diagonal is worth
moving `f64` bits under every rotation in the kernel (goldens, k-lint
baselines, content keys through stored frames). If yes, the unit is
the half-angle respell of `t` and `c` inside `rotation_about` plus the
composition-growth fix, with PROPS-1's re-baseline machinery reused;
if no, the doc at `rotation_about` states the floor and its
decomposition so nobody re-measures it, and the rider is re-homed to a
composition-side fix (compose in the parameter, not the placement).

## Home

`crates/geom-core/src/linalg/mat.rs` — S-CERT's ground until the
inheritance, and the audit member's caseload is this program's.

## Closed

**Ruling: no respell.** The item's own measurement decides it, re-taken
at this head by `crates/geom-core/tests/cert3_evidence.rs`'s
`start_sample_residue_decomposition` (interval lane; the row is
`cfg(feature = "interval")`): at the `RevolvedPoint` start sample
`θ = 0`, `t` alone from the half angle leaves `width(R·p)` at 100 % of
the shipped `2.6645352591003757e-15`, and `t` and `c` both from the
half angle at 83 % (`2.220446049250313e-15`); at the full-period sample
`θ = 2π`, `t` alone is 133 % and both is 100 % — the ~17 % / 0 % pair
the item recorded, unmoved. A sixth of the residue at best is not worth
moving `f64` bits under every rotation in the kernel; the irreducible
part is the backend's `cos` enclosure at exact angles.

**What landed instead** (PR 1980, a doc unit — no arithmetic moved):
the paragraph at `Mat3::rotation_about` that states the floor as the
SUM of the two enclosures, what each respell recovers with the
instrument named, why `identity_minus_rotation_about` takes the
half-angle forms for a different reason, and that the floor is the
backend's; and the composition rider re-homed to
`work/issues/mapped-curve-restrict-composes-placements-per-split.md`,
whose fix is composition-side — compose in the parameter, keep one
placement — and belongs to whoever owns `MappedCurve`, not to a
respell that is not happening.
