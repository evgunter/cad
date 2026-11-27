---
id: a-boolean-ships-a-body-its-own-census-escalates-on-beside-a-one-band-seam
kind: issue
title: A tube 1e-9 off a one-band ball's seam meridian builds every op, and tier 3 prime escalates on the body it ships
status: closed
opened: 2026-11-23
priority: P1
cost: M
refs: [a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote]
closed: 2026-11-26
pr: 7744
---

Found by the LATHE one-band lane (2026-11-23, PR 7414's review fix pass,
the slow set's base/head diff).

## What

`crates/sweep/tests/a_tube_ending_on_a_ball.rs`,
`the_probe_family_ships_no_wrong_body`, the probe `turn 0.000000001`:
the unit-radius tube ending on the ball of radius `√2` (`ball_poled_z`),
turned `1e-9` rad about `z` so its seam ruling sits `1e-9` off the
ball's seam meridian. `B ∪ A`, `B ∩ A` and `B ∖ A` build at their closed
forms, pass tier 2 and the certificate, and are legal operands, but
tier 3′ (`validate_pseudomanifold`) refuses each:
`CensusEscalated { predicate: "curve_pair_span_end_chord", margin ≈ 1.0e-9 }`
in the band `[1e-9, 1e-8]`. On main (two π-bands) every op of the probe
was sound or refused typed.

## Why it matters

The Boolean ships a body whose own at-rest census cannot decide it: a
result that should have refused typed at the gate, or been built so the
census reads it. The probe family's contract is "sound or refused".

## Wanted

The gate to run the census the at-rest check runs (so an escalation
refuses there, typed), or the join to keep the tube's ruling and the
ball's meridian apart, or together, decidedly. The row is pinned to its
stop while this is open.
