# M8-4 — the `nurbs_iso_derive` Intersection arm (boundary-iso mint)

Orchestrator work order for the M8 slate's unit 4 (M8-PLAN item 4,
scope tightened per this spec — the plan's "integral walls" wording
predates #353's collapse of the two wall classes). Enabling context:
M7-8/#288 (declare-and-check plane×NURBS certification), M8-3
(#309 flux enclosure + #353 IsoArc chart mint), #327 (imported-chart
domains, carrier-keyed dispatch).

## The gap (measured, current)

An edge whose certified description is `EdgeGeometry::Intersection
{ s1, s2, witness }` with a described (non-placeholder) NURBS surface
on one side has no chart-image derivation on the NURBS side:
`nurbs_iso_derive` (crates/topo/src/pcurves.rs:412) has no
`Intersection` arm, so the edge falls to the catch-all
(pcurves.rs:592–596) and refuses
`PcurveCertifyError::IsoUnsupported`, killing the whole body at
`mint_pcurves`. Since #353 the refusal is wall-kind-independent:
both the integral mixed body and the rational arc prism now mint
their charts and stop at exactly this arm. Producer today: STEP
adoption's declare-and-check rung only (no native op mints these
edges — the NURBS SSI rung is out of scope and unimplemented).

The fixture geometry in both flip rows is a wall–wall seam that IS
the NURBS chart's own u-boundary column, restated foreign (same
knots/degree/weights as the chart's `boundary_iso_u` row, one
control point 1 ULP off). The honest scope is therefore:

**In scope**: an `Intersection` whose carrier lies on a chart
BOUNDARY iso. **Out of scope, keeps refusing typed**: interior or
diagonal intersection carriers (the trimmed-NURBS/cut-loft lane;
`side_of`'s interior-iso refusal at pcurve_cache.rs:2762 is the
posture precedent). The excluded class gets a NAMED follow-up issue
at PR time (standing rule).

## Required shape (binding)

- Add the arm inside today's taxonomy: mint `Pcurve::IsoLine` for a
  boundary-resident `Intersection` carrier, certified by the
  EXISTING SEAM class (`run_iso_checks`,
  geom-brep/src/pcurve_cache.rs:2856–2947). The expectation from
  substrate: no new certification class is needed — if one turns
  out to be, STOP and report before building it.
- **Key on the carrier + boundary residency, not the description
  form or operand order** (the #353 carrier-keyed dispatch
  principle, pcurves.rs:495–509): native and round-tripped forms
  must behave identically, and which of s1/s2 is the plane must not
  matter.
- Boundary values come from the chart payload's own knot domains
  (the #327 `cu0..cv1` read, pcurves.rs:426–441) — NEVER from
  `[0,1]` literals. Side and direction selection reuse `side_pick`
  (pcurves.rs:444–465) and the u-direction probe idiom
  (pcurves.rs:545–572): selection-then-certification, definite or
  escalate, no silent guessing.
- D2: read the description, not the topology. D3: the new arm is a
  compiler-guided edit at every dispatch site — no `_ =>` widening;
  the catch-all's message text updates to name the newly supported
  kind. D4: typed refusals; no UV-space quantity ever compared to
  ε — every margin metered to metres through a lever arm
  (pcurves.rs:610–630). D9: fixed schedules and candidate order.
  C5: `IsoUnsupported` stays typed-and-permanent for the excluded
  class — never a runtime fallback.
- **Do not pre-empt #427** (pcurve unification is an M9 design item:
  no general curve-in-UV representation, no re-keying `MappedCurve`,
  no retiring `IsoCurve`/`IsoArc`). **Do not fold in #388**
  (`ExtrudedPoint` rung — dm1's blocker, a different unit by
  ruling). #389's exclusions stay theirs; the TAIL_TURBINE negative
  control stays green.

## Riders (small, in scope; STOP and report if either ripples)

- R1: `adopt.rs:664–670` writes literal `u: 0.0 / 1.0` into
  `IsoCurve` descriptions; on an imported chart with a non-[0,1]
  domain that names an interior column (unreached today, dm1 dies
  earlier). Fix to payload-domain values with its own test row.
- R2: `PcurveCertifyError::IsoUnsupported`'s doc
  (pcurve_cache.rs:636–642) still lists "a rational chart" and
  "an arc-parameterized cap rim" as refused classes; both certify
  since M8-3. Correct to the current refused set (invariant text,
  no history).

## Acceptance

1. **The two flip rows execute their own retirement text**
   (step-import/tests/recognize_pins.rs:241 integral twin, :354
   rational arc prism): first-class import, seam edge carrying a
   certified pcurve, asserting what each row's FLIP panic message
   names. No stale "no iso derivation" prose survives anywhere
   (S9 duty).
2. **ε-row three-outcome honesty** (the recurring lesson): the
   flip is honest at default and 1e-6 ONLY. At ε=1e-12 the arc
   prism refuses at ADOPTION (certified sup ~6.3156e-12) and the
   integral twin at `PlaneNurbsCertificate` — those cells stay
   pinned as refusals with their posture asserted, not deleted.
   The #288 walk-row retirement text states these cells explicitly.
3. **Negative controls**: a constructed INTERIOR-column
   `Intersection` refuses typed (the excluded class's pin, red if
   the arm over-reaches); TAIL_TURBINE unchanged; the tamper/at-rest
   rows in sweep/tests/m5_pr6_pcurves.rs stay green.
4. **Imported-chart row**: a non-[0,1]-domain chart mints the
   boundary Intersection correctly (natural home:
   geom-brep/tests/imported_chart_arc_rim.rs's suite).
5. Loop closure: `walk_loop`/`loop_closes` succeed on the flipped
   bodies; hosted CI fully green (the only gate); render lanes
   re-blessed only if a scene actually changes, through the hosted
   producer.

## Process

Standing unit protocol: one implementer (arm already drawn: block
M8-14 slot 2), one blinded adversarial reviewer + fix pass; A/B row
at merge with per-phase tokens/wall-clock; difficulty M and
task-class NUMERIC logged pre-dispatch (this spec); review ordinal
claimed from the ledger ON MAIN at review-dispatch time and pushed
to main immediately. Merge-main before opening the PR and BUILD THE
UNION; no Co-Authored-By in lane commits; comments state invariants,
not history; if the k-lint gate fires, do not change geometry to
silence it — re-derive per the K-REPORT runbook or escalate.
