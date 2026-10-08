---
id: a-plate-touching-a-cut-torus-at-its-cap-rim-builds-an-assembly
kind: issue
title: A plate touching a 270° torus on its cut-cap rim (δ = 0) builds an Assembly, and ∩ comes back Empty
status: open
opened: 2026-10-06
priority: P2
cost: M
---


Found by the review of PR 4122 (`analysis/reach-review/4122`,
`review.md` NOTE-2, probe (b) in `probes/review4122_probes.rs`).
Pre-existing: the review ran main's `topo` source on the same cases
and got the same results.

## What

A torus about y (tube centre `(0.6, 0.3)·s`, tube radius `0.2·s`) is
revolved through 270°. A thin plate is set on the plane of its support
along an oblique direction `d` whose unconstrained best azimuth lies
outside the kept window. The support then falls on the cut cap's rim,
and the plate touches the solid there at `δ = 0`. ∪ in both orders
builds a `BooleanResultKind::Assembly`, and ∩ returns
`BooleanResult::Empty`.

Measured at ε 1e-9 on PR 4122's head, at six of the probe's 48
(scale, direction) cases: `s = 10⁻³` dir 7 and dir 15, `s = 1` dir 11,
and `s = 10³` dir 1, dir 4 and dir 14. All six put the support at
azimuth `3π/2`, the window's far cut.

**The contact is recorded.** In each of the six, the ∪ `Assembly`'s
`ContactRecords` hold exactly one `VfContact`: a vertex of the torus on
a face of the plate (`a_on_b`). `vv`, `b_on_a`, `ve`, `ee`, `curves` and
`patches` are empty. The `Assembly` therefore carries the touch, and an
empty ∩ is what a point touch measures. Whether this is the intended
answer for a curved touch on a cut cap, or should refuse as the other
touches here do, is the question.

Other directions onto the same rim behave differently. The ones tried
by hand at `s = 1`, pose identity (`d = (0.8, 0.3, 0.6)`,
`(0.6, −0.2, 0.8)` and `(0.7, 0.5, 0.4)`, normalized) refuse at `δ = 0`
with `Escalated { Coincidence(SectorSide, Moot) }`. At `δ = −10⁻⁹` they
refuse with `Coincidence(EdgeOnPlane, Moot)` or
`Join(UnpairedLooseEnds { count: 2 })`. At `δ = 10⁻⁶` they build an
`Assembly` with no contact, which is correct.

## At `δ = −ε` since PR 4179

PR 4179 (CLEAVE) places a conic's in-band graze root at the sinusoid's
extremum (`conic_plane_meet`, `crates/topo/src/splitting/classify.rs:750-777`)
instead of at the residue's crossing `φ + acos(−D/R)`. The cap rim
grazes the plate's plane at the solid's support, so `δ = −ε` now reads
as `δ = 0` does. Measured on PR 4122's head after it merged main, at
ε 1e-9, `s = 10³` dir 4: at both gaps ∪ builds an `Assembly` holding one
`VfContact` (the minted rim vertex on the plate face, `5e-13` from the
closed-form support point), and ∩ is `Empty`. With
`classify.rs` taken from before PR 4179 (`70b6e9a6f`), `δ = −ε` refused
every op with `Join(UnpairedLooseEnds { count: 2 })`: the one root kept,
`φ + δ`, sat `√(2εR)` ≈ 6e-4 m along the rim from the touch, and its
twin `φ − δ` was dropped, so the split was one-sided. The operand gate
does not clear the pair at either gap: the contact is found by the
planar lane's conic roots behind it (`crates/topo/src/boolean/reduce.rs:1246`,
the `plane_crossing_lane` arm).

## Where it is pinned

`crates/sweep/tests/operand_gate_support_plates.rs` excludes this case
from its disjoint-at-touch check (the 270° torus, a cut direction,
`δ ∈ {0, −ε}`), and only where ∪ records the contact. It still samples
those results' membership. The row draws the first
six of the probe's directions, so two of the six cases above are in it.
Dropping the exclusion turns it red at ε 1e-9 at exactly those two,
`s = 10³` dir 1 and dir 4, in ∪ (both orders) and ∩.

## Hold

The contact machinery is under the D10 hold (`docs/DESIGN.md` D10).
This item records the behaviour. It does not propose extending declared
contacts, the placement registry or intent spellings.
