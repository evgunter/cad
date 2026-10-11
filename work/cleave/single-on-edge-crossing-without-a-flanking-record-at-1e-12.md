---
id: single-on-edge-crossing-without-a-flanking-record-at-1e-12
kind: issue
title: A wedge's edge crossing a cube face near-tangent refuses ClassificationInvariant 'single-on-edge crossing without a flanking record' at eps 1e-12 (36 runs)
status: open
opened: 2026-10-10
priority: P1
cost: M
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

## Built (branch `cleave/single-on-edge-record`)

Re-measured on main at `6a304186a8` (after PR 4581): the census's only
`ClassificationInvariant` left, at any of ε = 1e-9, 1e-6 and 1e-12, is
this one, the same 36 runs at 1e-12.

**Cause: the record `pair_search` minted was the wrong reading.** At
`w345 nt e1 a12 d1e-8 pc`, both vertices are edge points: the prism's
vertical edge crosses the cube's edge, the prism face `a2` (`x = 2`)
holds the cube's edge to 8.8e-16 m (read On), and the cube face `b3`
lies 1e-8 rad off `a2`. `pair_search` reads the pair `(a2, b3)` off
`n̂_a × n̂_b`, which at that tilt swings 4.5e-8 rad off the edge (the
edge's offset over the planes' sine), and `within(b3, ±d)` reads it
definitely outside at the arm (`−6.6e-8`). So the only record carrying
the edge is `(a2, b0)`, Table II's named flanker, while the fold rule
puts the crossing on `b3`, where both flankers' keys say it is.

**Fix** (`boolean/recl.rs` `resolve_edge_sector`): a `Yes` cell whose
fold flanker has no record mints it, the pair's codes read by
`pair_codes`, through `place_germ`, as the edge-edge rule already
does. Both flankers hold the on-bound, and the reference met one of
them along it, so it meets the other there too.

Census after, every ε row against main: only the 36 runs move. 12
build sound at their closed forms (`a12 d1e-7`, `d1e-8`, every op and
order), 18 refuse `Escalated Coincidence(Carriers)` as their other
order already did on main (`a3`, `a11` at `d = −1e-7…−1e-9`, `cp`), and
6 refuse `Escalated Coincidence(SectorSide)` (`a12 d1e-9`: the wedge's
bisector reads 5.2e-11 m off the cube face, within the band).

Pinned in `crates/sweep/tests/near_tangent_pierce_poses.rs`: the nine
poses join `POSES` (every row: closed form, tier 3′, no kernel-defect
refusal), and `the_edge_sector_witnesses_build_every_run_at_1e_12`
requires `a12 d1e-7` and `d1e-8` to build all six runs.

Not taken: reading the common line off a bound read On inside
`pair_search` itself. Measured, it moved 240 runs at ε = 1e-6 that
built or answered empty to `Escalated Coincidence(Sectors)` (the On bound's membership is
read in band where `d`'s was definite) and 24 at 1e-12 to
`ClassificationInvariant "germ direction not uniquely within its
sector pair"` (`insert::germ_dir` re-reads the noisy `d` on the extra
records).
