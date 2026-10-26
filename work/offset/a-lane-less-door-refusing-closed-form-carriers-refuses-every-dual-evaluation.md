---
id: a-lane-less-door-refusing-closed-form-carriers-refuses-every-dual-evaluation
kind: issue
title: Limb 2 for every analytic Intersection carrier makes every lane-less door refuse closed-form edges, and a Dual64 holds no lane: the dual corpus stops evaluating
status: closed
opened: 2026-10-25
priority: P0
cost: M
parent: closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them
closed: 2026-10-25
pr: 5689
---

Found by the unit's implementer lane (branch
`offset/limb2-for-every-carrier`, 2026-10-25). It blocks the unit: the
branch is pushed and no PR is open, pending a ruling.

**The conflict.** C2 (`crates/geom-brep/README.md`, ratified on PR 5652)
owes limbs 1 and 2 by every `Intersection` carrier of every kind, and says
of a carrier between two analytic surfaces: "A door that holds no lane
certifies no such edge: it refuses `CertifyError::NurbsLaneNotSupplied`".
Limb 2 is certification arithmetic, so it lives on the `NurbsLane`
(`crates/geom-brep/src/certify.rs` `NurbsLane::certified`, bounded on
`CertifiedBounds`). Run for every carrier kind, `run_checks` must take the
lane for every analytic pair's `Intersection` edge — every plane × plane
line of a box included — so a lane-less door refuses all of them.

`topo::AtRestPolicy::nurbs_lane` answers `None` at `Dual64` (DL1, "a dual
does not certify"; `crates/topo/src/props.rs`), so every Euler door at a
dual now refuses its first analytic edge as
`EulerOpError::NurbsLaneUnsupported`. Measured on the branch (release,
`cargo nextest run -p editor-core -p sweep -E 'test(/dual/) | test(/Dual/)'`):
**39 of 63 rows fail**, among them
`m10_di_dual_corpus::the_gather_opens_at_dual64_and_the_witness_set_is_pinned`,
`r1_dual_probes::*`, `r2_m10_di_probes::*`, `revolve_determinism::dual_value_channel_matches_f64_bitwise`
and `reach_volume_backstop::a_dual_builds_the_tilted_boss_on_the_f64_bits`;
every extrude in the Band 4 corpus refuses
(`Extrude(Op { source: NurbsLaneUnsupported { scalar: "dual" } })`). That
contradicts DUAL-DESIGN DL2/DL3 as `m10_di_dual_corpus.rs` states them:
"the whole Band 4 corpus evaluates AND gathers at `Dual64`".

**The forks**, for the designers and Ev:

1. Keep C2 literal: a lane-less door refuses every closed-form analytic
   edge, and dual evaluation of nearly every document refuses. Retires
   DL2/DL3's corpus claim.
2. A scalar that may not certify (DL1) checks a closed-form carrier on its
   samples alone at a lane-less door, and its edge is no C2 certificate
   (it never was one: a dual does not certify). Keeps DL2/DL3; C2's "a
   door that holds no lane certifies no such edge" gains a carve-out for
   the closed-form kinds, which is a change to a ratified sentence.
3. A dual holds a lane that runs limb 2 on its value channel, which is
   bit-identical to `f64`'s: the certificate is the `f64` body's. Keeps
   both texts' substance, but changes DL1 and the `compile_fail` doc row
   that keeps `NurbsLane::<Dual64>::certified()` from compiling.

The lane-less `f64` doors (`EdgeCurve::certify`, `EdgeCurve::recertify`)
are a smaller instance of the same question; the branch moves the
geom-brep tests that certify analytic `Intersection` edges onto
`certify_via` with `NurbsLane::certified()`, and `topo`'s `seqgen`
split-site assertions (`crates/topo/src/seqgen.rs` `split_site`, which
re-certifies through the lane-less `recertify` on purpose) are not yet
moved.

**Ruled (Ev, PR 5689, 2026-10-25).**
- **No dual certifies, by type.** The edge door splits into mint, generic over any scalar, and certify, bounded on the certification trait. A dual only mints. No optional lane exists anywhere.
- **Door and rest.** At `f64` the door certifies in full, and rest re-derives the same certificate.
- **The dual keeps its values.**
- **No anchor and no per-node walk.** The sensitivity report checks once that the dual passes and the `f64` evaluation had equal build identities: document pin, parameter bits, tolerance, and the resolved parts' pins. It catches drift in production by comparing the measured value bit for bit.
- **Naming the node.** Ev took the one-number comparison (B), "presuming we can straightforwardly add names later if we need them".

The build is conic unit 1, re-scoped. Two points stay unchecked until then:
- whether a dual can escalate an undecided sign as `f64` does;
- whether a pin of the resolved parts exists.
