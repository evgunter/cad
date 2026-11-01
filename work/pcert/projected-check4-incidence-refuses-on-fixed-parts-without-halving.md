---
id: projected-check4-incidence-refuses-on-fixed-parts-without-halving
kind: issue
title: geom-brep: a projected row's check 4 reads its incidence on the fixed INCIDENCE_CUTS parts and refuses without halving them
status: closed
opened: 2026-10-11
priority: P3
cost: M
closed: 2026-10-14
---


Filed by `encl/rung3-hull-refine` (row `analytic-rung3-hull-limb-refuses-on-fixed-parts-without-subdividing`), which made the analytic rung-3 limb halve the parts of the same projected hull.

## What

Check 4 of a net's projected row (`pcurve_cache::projected::projected_envelope`) reads its `Incidence` term once, through `pcurve_cache::projected_hull_lane` with no extra cuts: each knot span in `projected::INCIDENCE_CUTS` fixed parts, converted to metres by `projected::net_incidence`. A part whose Bernstein bound does not clear inflates the envelope, and the row refuses, with no halving. A tighter bound is one round away.

## Repair shape

The lane now takes `cuts` (`FittedLane::projected_hull`), and every `SpanHull` carries a certified metre bound (`meters`) and end floors (`floors`). So check 4 could run `ssi::certify::subdivide` over the incidence term as `projected::net_offset_sup` does, with a care that net does not have: the envelope sums several terms, so the incidence must be halved against what the band leaves after the other terms, not against the band alone. Unmeasured: how many rows refuse on this term today is not known; measure before building.

## Closed 2026-10-14: the fixed parts are already tight

Measured by instrumenting `projected_envelope`'s Net arm. For each f64
projected row it logged the incidence on the fixed `INCIDENCE_CUTS`
parts (`cuts = &[]`, today's read) next to the same `net_incidence` on
a hull cut 64 times across `[t0, t1]`, plus the check-4 total and its
verdict.

**Suites** (geom-brep, topo, sweep, step-import, mesh, editor-core;
release ci, 10,512 tests pass):
- 275 incidence reads; 97 envelopes computed, 51 refused before the
  total. Of the 51, 45 are "a fitted chart image needs the fitted door
  at this scalar" and 6 are sector refusals; none is about incidence.
- No envelope exceeds the band's zero. The largest envelope / zero is
  0.89.
- fixed / refined incidence over all 279 reads: median 1.00, p90 1.11,
  max 5.65. **Every read above 1.012 (71 of them) is an
  `envelope_lemma_fuzz::projected` row.**
  - 66 have carriers deliberately off the surface (incidence `1e-8` to
    `7e-3`), and refuse whatever the bound. 57 of those are
    `the_projected_envelope_dominates_the_sampled_displacement`.
  - 5 sit at rounding scale (`7e-16` to `1.5e-14`), where the ratio is
    noise.
  - The 43 reads from rows outside the fuzz, honest carriers, have
    fixed / refined at most **1.0026**. The p90 of 1.11 is the fuzz
    rows', not theirs.
- **No read lies in [0.1ε, 10ε]**, and none has fixed > ε ≥ refined.

**Constructed poses** (unit cylinder, ε = 1e-9):
- A rational arc with its middle weight nudged (a one-signed bump): the
  envelope equals the true off-distance to 4 digits at every size.
- Sign-changing carriers along the axis (degree 3, 5 and 7, control
  points alternating ±δ radially): fixed / refined is 1.012, 1.005 and
  1.011.

The 8-part Bernstein read is within about 0.3% of the 64-cut read on the
suites' honest rows. On the constructed sign-changing carriers it is
within 1.2%. Halving could flip a verdict only for a carrier whose true
miss sits within about 1% under the band, after the other terms. That
is a false refusal in the sliver, and a fitted carrier near ε can reach
it. It is filed as its own row:
`work/pcert/projected-check4-incidence-can-falsely-refuse-in-the-sliver-below-the-band.md`
(P4). The reason not to halve today sits at `projected::INCIDENCE_CUTS`.

**Re-taking the census.** The probe instruments
`projected_envelope`'s Net arm, so it cannot be an `#[ignore]`d row. It
is on the branch `pcert/census-probes` (f44ce42a). Run there:
`PCERT_P4_LOG=/tmp/p4.log cargo nextest run --release --profile ci -p geom-brep -p topo -p sweep -p step-import -p mesh -p editor-core`.
Each line reads `P4INC fixed refined <test>`.
