---
id: the-dual-attach-gate-refuses-lane-classes-its-f64-anchor-certified
kind: issue
title: a Dual Euler door refuses plane x NURBS Intersection and analytic rung-3 edges that the f64 anchor certified: not run at a dual, or delegated to the value scalar?
status: closed
opened: 2026-10-16
priority: P2
cost: M
design: true
refs: [stackup-pairing-reads-the-shells-dual-refusal-as-a-violation, onpair-has-no-incidence-lane-on-a-spline-adjacent-pair]
closed: 2026-10-26
pr: 5728
parent: closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them
---


Filed by the designer lanes of the plane × spline rim fork, off their
question. It is a question for Ev, to be raised as its own fork in an `[ev]` PR.

## Finding

Every Euler door that mints or re-describes an edge certifies its spec
through `topo::policy_lane::certify` (`crates/topo/src/policy_lane.rs`),
with the plane × NURBS lane `AtRestPolicy::nurbs_lane` returns.
- At `Dual<T>` that is `None` (`crates/topo/src/props.rs:4349`: "a dual
  does not certify (DL1)").
- So every lane class refuses there with
  `EulerOpError::NurbsLaneUnsupported` (`crates/topo/src/euler.rs:877`):
  a plane × NURBS `Intersection`, a NURBS × NURBS `Intersection`, and a
  spline carrier under an analytic `Intersection`. The list is on
  `CertifyError::NurbsLaneNotSupplied`
  (`crates/geom-brep/src/certify.rs:406`).
- Pinned by
  `topo::policy_lane::tests::the_attach_door_refuses_an_analytic_rung3_edge_at_the_dual`
  (`policy_lane.rs:145`).

Every other class is decided at the dual by value-part delegation (Q1),
so its verdict is the f64 anchor's. So at a dual the attach gate runs
one set of certificates and refuses another, by class. Meanwhile:
- the at-rest gates are absent at a dual (`AtRestPolicy for Dual`:
  `gate_at_rest` answers `NotRunAtThisScalar`);
- DL3 says certified validation is "structurally absent at Dual",
  because the dual's value channel is the f64 build already validated.

## Consequence (read off the code, not run)

`editor_core::stackup` pairs every `Dual64` pass with the f64 anchor
node by node (`crates/editor-core/src/stackup.rs:718` `pair_pass`,
`:763` `same_arm`). A node the anchor built and the pass refused is
`PairingViolation::ResultArm` (`stackup.rs:275`), which reports a stale
document. So a sensitivity run over any document with one of these
edges fails as a whole, blaming the build.

The pairing half is the same symptom
`stackup-pairing-reads-the-shells-dual-refusal-as-a-violation` files
for the shell door. The question here is the attach gate's own.

Since OFFSET's build-identity binding (2026-10-25) the report compares
no result arms: such a node leaves the measured value under it with no
value in the pass, and that entry reads `MeasureRefused`; the run no
longer fails whole.

## Measured witness

Under the fork's probe (loft door switched to `OnPair`), four sweep
lofts refuse at `Dual64` with
`Euler(NurbsLaneUnsupported { edge: Some(..), scalar: "dual" })`
(`loft_profiles_door::at_dual_a_seeded_section_value_rides_into_the_walls`,
`encl_curved_loft_shell::a_dual_scalar_refuses_the_spline_seam_re_anchor_by_name`,
`r1_p2_probes::r1_dual_scalar_wall_seam_arm_names_the_missing_lane`,
`a_wall_seam_takes_the_column_it_lies_on::at_a_dual_a_split_seam_takes_its_stated_column`).
The same loft builds at f64.

## The question (for Ev)

What does the attach gate do at a dual for a class whose certificate
is certification arithmetic?
1. **Not run at a dual.** DL3's shape: the dual's edges carry the value
   lane's certificate, and nothing is re-derived. This needs a "not run
   here" state where `EdgeCurve` today makes an uncertified value
   unrepresentable.
2. **Delegated to the value scalar.** The lane runs at `T` on the dual's
   value channel and its certificate is lifted. The gate is then total
   at a dual, and its result arm equals the anchor's by construction.
   Needs a ruling against DL1 ("a Dual never certifies") and DL5 (a read
   that mints a certificate is never lane-exempt).
3. **Refuse by name (today).** Keep the refusal and give `pair_pass` a
   valve for it, as the shell item proposes.

## Closed

Claimed by OFFSET and closed by conic unit 1
(`closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them`),
which builds Ev's answer to this question as ratified on PR 5689 (C2 in
`crates/geom-brep/README.md`, DL3 in `docs/DUAL-DESIGN.md`): neither of
the three options above, but the door split. The attach gate at a dual
is the dual's edge door, which only mints
(`topo::AtRestPolicy::attach_edge`, `crates/topo/src/props.rs`; the
dual's body `policy_lane::minting_attach`), and the edge carries no
certificate (`geom_core::Real::Carries` is `geom_core::Uncertified` at a
dual). `EulerOpError::NurbsLaneUnsupported` and
`CertifyError::NurbsLaneNotSupplied` are gone. The four lofts named in
the witness are in the dual corpus the unit ran, which passes in full;
the pairing half is the sibling unit
`the-sensitivity-report-binds-by-build-identity-and-checks-the-measured-value`.
