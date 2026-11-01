---
id: closed-form-edge-carriers-pass-on-samples-with-no-stated-bound-between-them
kind: unit
title: A closed-form Intersection carrier passes its certificate on nine samples with no stated bound between them; the standing is provenance read off the variant
status: closed
opened: 2026-10-24
priority: P0
cost: H
refs: [offset-lane-has-no-conic-carrier]
parent: offset-lane-has-no-conic-carrier
branch: offset/limb2-for-every-carrier
closed: 2026-10-26
pr: 5728
---


Filed by OFFSET's conic designer pair (2026-10-24, `[ev]` PR 5652; both
designers agree on the finding, and split on the fix Ev is asked about).

`crates/geom-brep/src/certify.rs` `EdgeCurve::certify` gives an
`Intersection` edge whose carrier is a `Line`, `Circle`, `Ellipse` or
`Spiric` the nine-sample residuals, transversality and the witness
mid-pin only; a `Curve3::Nurbs` carrier additionally owes C2's hull limb
and tube. Nothing in that sampled check bounds the residual BETWEEN
samples (only the tangent lane states a sup bound). Its standing is C1's
"zero by construction because the kernel derived them from their pair"
(`crates/geom-brep/README.md`), which the code spells as "not the `Nurbs`
variant": provenance the door cannot check.

For the conic kinds a bound exists and is not written: a conic composed
with a quadric has a residual of fixed low degree (a degree-2
trigonometric polynomial for a circle or ellipse), so nine samples bound
it over the span up to a fixed constant; the pcurve cache already
argues this way for `Pcurve::Harmonic` ("a corruption hiding between
samples is unrepresentable"). For `Spiric`, and for `Circle` against a
torus, the residual is not of finite degree, so the samples alone bound
nothing between them (unsure whether the stored surfaces constrain it
another way).

What closes it depends on Ev's answer on PR 5652. If the obligation
follows the CLASS read from the carrier's data, the certificate door
states a between-samples constant per class and routes the classes
without one to the hull limb. If it follows the KIND, C1 states the
low-degree reason for the conic kinds and the `Spiric` question is
answered separately. Either way the sentence C1 rests on today should not
stand as it is. Filed by the OFFSET orchestrator.

## Claimed by OFFSET (2026-10-24) — conic build, unit 1 of 3

Ev ruled on PR 5652 (`offset-lane-has-no-conic-carrier`, "Ruling"):
the certificate is derivation-blind. This row is its first half, now
`crates/geom-brep/README.md` C2: **limbs 1 and 2 are owed by every
`Intersection` carrier of every kind, however it was minted.**

Build: `EdgeCurve::certify` (`crates/geom-brep/src/certify.rs`) runs limb
2 (the composite hull bound between samples, `analytic_rung3`'s limb-2
path, reading the carrier through `carrier_net`'s enclosing net) for
`Line`, `Circle`, `Ellipse` and `Spiric` carriers too, on every surface
including the cone and torus per-span lever (`net_offset_sup`). The tube
obligation is unchanged in this unit: closed-form kinds keep not owing it
for now, which is today's behaviour and what unit 3 replaces with the
pair rule. Delete what C1/C2's retired derivation wording justified in
code comments. Expect pins to move at rest (runtime and stored
certificates): measure the corpus cost of the new limb 2 on closed-form
edges and report it; re-baseline and say what moved. Name the spiric
net's tightness on a long oval (both designers unsure) with a row.

Banded P0 as part of the P0 build; it is also what closes the latent gap
(a spiric slightly off its torus owes no bound today).

**Re-scoped (Ev's ruling on PR 5689, 2026-10-25).** Limb 2 for every carrier is built on the new door shape:
- **The door split.** Mint is generic over any deciding scalar. Certify is bounded on the certification trait and takes its lane as a plain argument. A dual only mints. `Option<NurbsLane>` and every "not run here" arm go; the policy's other optional lanes move to the same shape.
- **Sensitivities in `editor-core`.** The pairing rework is its own unit (`the-sensitivity-report-binds-by-build-identity-and-checks-the-measured-value`).
- **Cost.** The limb-2 cost row (`closed-form-limb-2-costs-milliseconds-per-curved-edge`) rides the instrument.
- **Check first:**
  - whether a dual escalates an undecided sign as `f64` does (`geom-core/src/dual.rs` `sign_within`);
  - whether a pin of the resolved parts exists.

  File a row for each gap found.

The lane's earlier branch is `offset/limb2-for-every-carrier`, head `452e7d8757`.

## Closed

Built on branch `offset/limb2-for-every-carrier` on Ev's ruling on PR
5689.

- **The door split.** `geom_brep::EdgeCurve::mint` (generic over
  `Decide`) constructs the canonical description, carrier, interval and
  turn into a `MintedEdge`; `EdgeCurve::certify`, `certify_stated`,
  `recertify` and `recertify_stated` are bounded on
  `Decide + CertifiedBounds` and take `NurbsLane` as a plain argument
  (`crates/geom-brep/src/certify.rs`). What an edge carries is the
  scalar's: `geom_core::Real::Carries` is the `Certificate` at a
  certifying scalar and `geom_core::Uncertified` at a dual, and only a
  certifying scalar can hold a record (`CertifiedEnclosure::carried`)
  while only a tangent-transport scalar can hold none
  (`geom_core::TangentTransport`, `EdgeCurve::uncertified`). The policy's
  edge door is `topo::AtRestPolicy::attach_edge`/`reattach_edge`; a
  certifying door at a dual is a compile error (the `compile_fail` row on
  `EdgeCurve::certify`). `Option<NurbsLane>`, `CertifyError::NurbsLaneNotSupplied`,
  `topo::policy_lane::ByPolicy`, `EulerOpError::NurbsLaneUnsupported`
  and `TransformError::NurbsLaneUnsupported` are gone; the spline foot
  moved off the lane to `geom_brep::SplineFoot`.
- **Limb 2 for every closed-form carrier**, as the earlier branch built it
  (`edge_nurbs::analytic_edge_limbs`, `pcurve_cache::closed_form_offset_sup`);
  the tube obligation is unchanged (unit 3).
- **The dual corpus evaluates again**: the 63 dual rows of `editor-core`
  and `sweep` pass (39 refused on the earlier branch).
- **At `f64`** every at-rest door's check 2 runs the scalar's edge door,
  so the `_structural` doors now re-derive every edge in full too.

Filed: `a-dual-reads-the-closed-form-where-f64-escalates-to-the-quadrature`
(the escalation check), `the-policys-other-lanes-encode-certification-rights-as-a-runtime-none`
(the other optional lanes). The cost row
`closed-form-limb-2-costs-milliseconds-per-curved-edge` stays open with
this PR's measurements.
