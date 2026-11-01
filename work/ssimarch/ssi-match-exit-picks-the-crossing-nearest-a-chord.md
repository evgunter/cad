---
id: ssi-match-exit-picks-the-crossing-nearest-a-chord
kind: issue
title: ssi/ends: match_exit pairs an uncapped march's exit with the unused crossing nearest where the leaving chord meets the side, a proximity guess; settle the exit onto the side and match by identity
status: open
opened: 2026-10-04
priority: P3
cost: M
---



(SSI implementer on `ssi/neighbour-cap`, PR 4034, from the design of
`ssi-a-step-capped-by-a-neighbouring-crossing-can-spend-the-step-budget`,
which put it out of that unit's scope.)

## What

`Ends::match_exit` (`ssi/ends.rs`) ends a march from a crossing at the
unused crossing on the side its last step left through, within the
step's reach of the last inside state, and where two qualify (branches
converging on a side) takes the one nearest where the step's chord
meets the side. The chord's meeting point is an estimate of the exit:
the locus between the two states is an arc, not the chord. With the
crossing cap retired (PR 4034), a straight branch's march leaves the
wall in one step of up to the domain's diagonal, so the chord can be
long and its meeting point can lie far from where the locus crosses
the side. The certificate refuses a wrong pairing (limb 3), so no wrong
answer follows, but a wrong pick costs the branch a refusal
(`CrossingUnmatched`, or a refused candidate) where the right pick
would certify. Not met by a fixture.

## Done when

The exit is settled onto the side it crossed (Newton on the locus with
the side's coordinate held, from the chord's meeting point), and the
crossing it matches is the one the boundary pass certified at that
point, by identity within the settling residual; the nearest-point
pick retires, and a row where two unused crossings sit on the side the
step leaves pins the pick.
