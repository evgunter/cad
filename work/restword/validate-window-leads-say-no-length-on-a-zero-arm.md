---
id: validate-window-leads-say-no-length-on-a-zero-arm
kind: issue
title: topo: the at-rest window's leads and NoDihedralArm say 'not long enough' or 'no measurable length' on a Zero verdict, and print no margin
status: open
opened: 2026-10-13
priority: P3
cost: E
---

(Filed by the ENCL implementer of `zero-arm-payloads-overclaim-and-print-no-margin`, from its sweep for Zero/collapse payload Displays.)

Since that unit, geom-brep's Zero-arm payloads print their margin and claim no size: whether a reading shows none is the ending's to say, from `geom_core::MarginDiag`'s one classification (D4 ¶1 (i): the margin rides the payload as data). The Displays below are on this program's ground and still read the arm instead.

## What

- `crates/topo/src/validate.rs` `classify_certify`, the at-rest window's lead for a certify refusal (its `let why = match e` over `CertifyError`). These leads state unhedged claims on every Zero verdict, a straddling enclosure or a positive sub-floor size included, which do not establish them:
  - `ArmCollapsed`: "it is not long enough, for how its faces curve, to measure their angle";
  - `SpanMeterCollapsed`: "its spline's certified speed floor gives it no measurable length";
  - `NotTransverse` (and `PlaneNurbs(NotTransverse)`): "its faces are tangent where its description says they cross";
  - `NotSecondOrderSeparated`: "its faces agree to second order, so they do not fix where it runs, which its description says they do".

  `IntervalNotForward { Zero }`'s "its length is within the tolerance of zero" is true of a band-decided zero, though not literally of a symbolic-tier proved zero whose rounded number lies past the zero band.
- Same file, `impl Display for ValidationError`, `Self::NoDihedralArm`: "an edge is not {DIHEDRAL_ARM_CLAUSE}", with no margin. The certify payload it mirrors now reads "the edge is not shown {DIHEDRAL_ARM_CLAUSE} (margin … m)" (`CertifyError::ArmCollapsed`).
- Same file, the offset-meter lead (`M::NormalFloor { verdict: R::Negative { .. } }`): "the face's surface normal degenerates, so it has no offset". The thinness is never negative (`MeterError::NormalFloor`'s field doc), so this arm is unreachable or says more than its verdict. The meter's own payload now reads "the face's normal is not shown non-zero (margin … m, from normal length …)" and leaves the offset's refusal to the fit's lead.

## Repair shape

Say "not shown …" of the reading and print `(margin … m)`, as `CertifyError`'s and `MeterError`'s Displays do; leave any no-size claim to the ending. Pins: the `validate.rs` tests asserting `text.starts_with("an edge is not long enough")`, and the row at "an edge is not long enough, for how its faces curve".

## Evidence added 2026-10-19 (ENCL, PR 5157)

PR 5157 merged the geom-brep side: zero-arm payloads now read "not shown …" with `(margin 0 m)`. TANG's U-lever also reworded `DIHEDRAL_ARM_CLAUSE` to "between faces wide enough, for how they curve, to measure their angle". So `classify_certify`'s `ArmCollapsed` lead, "it is not long enough, for how its faces curve", now names the edge's length, which is no longer the decision's noun. It disagrees with the payload it leads as well as hedging less than it should. Compose the clause rather than re-spelling it. (ENCL orchestrator)

## Evidence added 2026-10-20 (ENCL, PR 5437)

- `classify_certify` (`crates/topo/src/validate.rs` ~2908-2930) still reads "its faces are tangent where its description says they cross" for `CertifyError::NotTransverse` and `PlaneNurbs(P::NotTransverse)` on a Zero verdict. Its recourse then offers to tighten the tolerance below the margin, so the lead says the faces are tangent while the ending says a smaller tolerance may show them cross. `validate::tests` pins the row ~16810 (`says(CertifyError::NotTransverse { lever: None, sample: 4, verdict: zero(5.0e-10) })`). Since PR 5437, `PlaneNurbsRefusal::NotTransverse` renders certification's own sentence ("the faces are not shown to cross (margin … m …)"), so the two lanes' payloads agree and this lead disagrees with both.
- This is a class, not one instance. Each of `classify_certify`'s at-rest reasons is a second hand-written telling of a decision whose payload certify already writes: every span decision (`IntervalNotForward`, `WindingExceeded`, `StatedInterval`, `SpanMeterCollapsed`) and every transversality decision (`NotTransverse`, `ArmCollapsed`, `NotSecondOrderSeparated`, `TubeNotSeparated`). Sweep the whole `let why = match e`. Compose each reason from the decision's own sentence or clause, as `ArmCollapsed` already composes `dihedral_arm_clause!`, and do not re-word them one at a time.
- The hedge word differs between the doors too. On a Zero verdict, certify says "not shown {clause}" (`CertifyError::ArmCollapsed`). Topo's check 4 (`ValidationError::NoDihedralArm`) and `classify_certify`'s `ArmCollapsed` lead say "is not {clause}".

## Noted 2026-10-20 (ENCL, `encl/check-word-one-noun`)

The undecided arm has the same second telling: `certify_undecided` (`crates/topo/src/validate.rs`) words each check's escalation by hand, and only the two span reasons say their check word's quantity ("its length", "its image's length"). The rest name the decision another way than certify's check words (`CertCheck`'s `Display`, `crates/geom-brep/src/certify.rs`):

- `ParamWinding`: "whether its arc stays short of a full turn". It shares the size "arc", but the word is "the stored arc's headroom to one full turn of its carrier".
- `ParamSpanMeter`: "whether its spline has a measurable length". The word is "the lower bound on the spline carrier's length from its metered knot domain".
- `Transversality` and `TransversalityTube`: "its faces meet too nearly tangentially". The words are "the crossing angle" and "the uniqueness tube's lower bound on the crossing angle".
- `TangentSecondOrder` and `TangentTube`: "its faces curve apart too little". The words are "the curvature difference" and "the second-order tube's lower bound on the curvature difference".

Fold them into the same sweep.
