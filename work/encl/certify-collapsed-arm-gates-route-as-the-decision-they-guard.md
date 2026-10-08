---
id: certify-collapsed-arm-gates-route-as-the-decision-they-guard
kind: issue
title: geom-brep: dihedral_arm and nurbs_span_meter refuse under the decision they guard, with the poisoned-margin note, because the funnel folds the gate's verdict into MarginDiag::Invalid
status: dispatched
branch: encl/collapsed-arm-gates
pr: 3431
opened: 2026-09-28
priority: P3
cost: M
---



(ENCL implementer, residue of
`certify-span-and-zero-arms-cannot-carry-their-decisions-full-ending`.)

## What

`dihedral_arm` (`crates/geom-brep/src/dihedral.rs`, `classify_dihedral`)
and `nurbs_span_meter` (`certify.rs`, `run_checks`'s NURBS span arm)
run `decide_positive` on a lever arm or a metered extent before the
decision proper. A definite non-positive verdict comes back as
`geom_core::k_stats::decide_positive`'s `Indeterminate` carrying
`MarginDiag::Invalid`, and certification reports it under the decision
it guards (`Transversality`, `ParamSpan`), ending in that lever plus
"an unreadable or collapsed margin may indicate a kernel bug worth
reporting".

## Why it stays

At the call site the gate's own verdict is gone: the funnel folds a
definite non-positive sign and a poisoned margin into the same
`MarginDiag::Invalid`, and D4 ¶1 (i) forbids telling them apart by
predicate name. Routing the gate as its own decision needs the gate to
hand its verdict back (a typed collapse from `decide_positive`, which
is `geom_core`'s), and for `nurbs_span_meter` the question is already
split on NURBS's slate: a reversed knot domain reads as a collapse
there (`work/nurbs/nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one.md`).
The note the gates end in is true of both halves they fold (a
collapsed arm on sound geometry and a poisoned margin are both a
kernel or data defect), so the ending is not wrong, only unsplit.

## Shape

Once `decide_positive` returns the gate's verdict (or the NURBS row
separates the reversed domain), give each gate a `CertCheck` of its own
with its own lever, and route it through `certify::recourse`.

## Validate reads the same gate (ENCL, PR 3398)

`topo::validate`'s check 4 reports `classify_dihedral`'s escalations as
`SliverDihedral { check: WedgeCheck::Dihedral, .. }`, which ends through
its `WEDGE` sized decision. An in-band `dihedral_arm` (a `Value` margin,
not the collapsed `Invalid` this row names) therefore reads "if this
angle is intended, tighten the tolerance below m/K" with `m` the arm, a
length: the size noun is the wedge's. Once the gate hands its verdict
back, give the arm its own `WedgeCheck` (or route it with this row's
`CertCheck`) and its own size noun.

## The span meter's domain question is NURBS's

This row routes `nurbs_span_meter` as a decision of its own
(`CertCheck::ParamSpanMeter`, `CertifyError::SpanMeterCollapsed`) and
leaves what a negative verdict MEANS to
`work/nurbs/nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one.md`:
a floor that turns negative (a spline turning back on itself) and a
knot domain stored reversed both read `Refused::Negative` here, and
both end in the meter's lever. Checking the domain's orientation
before the meter, with its own arm, is that row's; once it lands the
meter sees only the floor.

## The SSI march reads the same gate (PR 3707)

`ssi/march.rs` `march` runs `decide_positive("ssi_transversality_arm", …)`
on the folded lever arm before `ssi_transversality`. Its escalation is
`SsiError::Escalated { decision: TraceDecision::TransversalityArm, .. }`,
and `TraceDecision::ending` (`crates/geom-brep/src/ssi.rs`) routes it
through `CertCheck::Transversality`, the decision it guards. An in-band
arm therefore reads "if this angle is intended, tighten the tolerance
below m/K" with `m` the arm, a length. When this row's gate gets its own
`CertCheck` and size noun, route `TraceDecision::TransversalityArm` the
same way.
That routing is done on this row's branch: `TraceDecision::ending`
ends `TransversalityArm` by `CertCheck::TransversalityArm`.

## What main already carried, and what this row adds

Two units landed the gate's verdict while this row's PR waited.
CLEAVE's gate rejections keep the decided margin, tagged with the sign
the gate refused (`geom_core::MarginDiag::rejected_sign`), so a
collapse no longer reads as a poisoned margin. PR 3513 routed the
dihedral arm as its own decision at every reader
(`CertCheck::TransversalityArm`, `WedgeCheck::Arm`,
`geom_brep::DIHEDRAL_ARM`). What stays on this row is the definite arm:
a collapsed arm is a verdict, not "too close to call", so
`LeverEscalation::collapsed_arm` hands it back (read before the arm's
margin is re-quoted at the wedge it meters), and certification and the
validator report it as `CertifyError::ArmCollapsed` and
`ValidationError::NoDihedralArm`; and the spline meter's own decision,
`CertCheck::ParamSpanMeter` / `CertifyError::SpanMeterCollapsed`.

