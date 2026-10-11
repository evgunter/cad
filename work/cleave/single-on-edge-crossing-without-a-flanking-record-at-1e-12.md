---
id: single-on-edge-crossing-without-a-flanking-record-at-1e-12
kind: issue
title: A wedge's edge crossing a cube face near-tangent refuses ClassificationInvariant 'single-on-edge crossing without a flanking record' at eps 1e-12 (36 runs)
status: dispatched
opened: 2026-10-10
priority: P1
cost: M
branch: cleave/single-on-edge-record
---


## What

Found by `near-tangent-pierce-poses-reach-three-classification-invariants`
(branch `cleave/near-tangent-invariants`), re-taking JOIN's near-tangent
census (`crates/sweep/examples/near_tangent_census_probe.rs`). It is the
fourth `ClassificationInvariant` site that census reaches, and the only
one that branch leaves: at ε = 1e-12, 36 runs of the `w345` wedge refuse
`ClassificationInvariant { "single-on-edge crossing without a flanking
record" }` (`boolean/recl.rs` `resolve_edge_sector`), on main and on
the branch alike: every op and order of `w345 nt e1 a12` at
`d = 1e-7, 1e-8, 1e-9`, and every op of the `cp` order of `w345 nt e1
a3` and `a11` at `d = −1e-7, −1e-8, −1e-9`.
None at 1e-9 or 1e-6.

Repro: `CAD_TOLERANCE_EPS=1e-12 NT_ONLY=w345 NT_POSE="nt e1 a12 "
NT_D=1e-8 cargo run -p sweep --release --example near_tangent_census_probe`.

Measured at that pose (`pc U`), the B side (the cube) owning the
on-bound: `f_s = 3`, `f_e = 0`, `ref = 2`, keys `(Out, In)`, Table II
`(No, Yes)`, so `crossing_flank` puts the crossing on flanker 3. The
records that carry an On are `(a 0, b 1)` and `(a 2, b 0)`, the latter
with `sb = (In, On)`: the On record sits on flanker 0 (`f_e`), and no
record `(2, 3)` or `(1, 3)` exists. The fold rule and `pair_search`'s
records name different flankers of the one on-bound, the cube's sector
bound `(0, −0.995, −0.098)` that the near face holds to within 1e-8 rad.

## Owed

Find which of the two readings of that bound is wrong (the flank keys'
`side_code` at its reach, or the record `pair_search` minted), and make
them one; PR 4177 changed this file's flanker decisions on 2026-10-06.
