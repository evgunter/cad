---
id: contact-gate-readers-drop-the-arm-verdict-or-mint-invalid
kind: issue
title: topo: census's dihedral read drops the arm's rung and verdict, and contact_verify mints its gate refusals as Invalid
status: open
opened: 2026-09-29
priority: P3
cost: M
---

## What

Found by ENCL's sweep of the collapse gates (PR 3431), re-read against
main on 2026-10-08:

- `topo::census` (near line 2180) reads only
  `LeverEscalation::diag()` from `classify_dihedral`, so an arm
  the gate decided collapsed, and an undecided one, both reach the
  census as an undecided candidate, with no rung to end them by.
- `topo::boolean::contact_verify` hand-mints `MarginDiag::INVALID`
  escalations at seven sites. Each wants reading to tell a gate's
  definite verdict, which the funnel now carries tagged, from a real
  poison.

## Shape

Where the verdict reaches a person, keep the rung and the arm's
verdict, and end each as its own decision.

## The door that exists

A gate's rejection carries the margin the classifier decided, tagged
with the sign it refused (`geom_core::MarginDiag::rejected_sign`;
`geom_brep::recourse::Refused::rejected` reads it as a verdict). A
dihedral or `enters_material` escalation is a `geom_brep::LeverEscalation`:
its `rung` names the arm or the reading, and
`LeverEscalation::collapsed_arm` hands back the arm's verdict where the
gate decided it (ENCL, `work/encl/certify-collapsed-arm-gates-route-as-the-decision-they-guard.md`,
PR 3431). Certification (`CertCheck::TransversalityArm`,
`CertifyError::ArmCollapsed`, `CertCheck::ParamSpanMeter`) and the
validator (`WedgeCheck::Arm`, `ValidationError::NoDihedralArm`) route
through them.

## SEAL's claim on the contact_verify half (2026-11-09, PR 6634)

The seven `contact_verify` sites are SEAL's I1–I7 (`work/seal/glue-disagreements-and-held-contradictions-after-stage-4.md`, Re-census). Under the fork `glue-disagreements-carry`, each step's lane claims what it closes. The file is not moved, because the census-dihedral half above stays SECTOR's and stays open.

- **I7 `contact_tangent_second_order` is closed by PR 6634.** The decided arm is now the funnel's gate rejection (`k_stats::gate_rejection` on the decided reading, under `SizedPass::Positive`), not `INVALID`.
- **I1–I6 belong to `work/seal/a-tangent-reading-returns-a-verdict-and-only-a-record-is-contradicted`** (step 3), and close with it.
- **I1–I6 are closed by PR 6673** (`a-tangent-reading-returns-a-verdict-and-only-a-record-is-contradicted`). The tangent table's rows return `TangentVerdict::Ruled { row, by: Decided }` on the reading that decided them, and the `Rest` door's `RestVerdict::Ruled { row, by: Option<Decided> }`; none mints `MarginDiag::INVALID`. The contact_verify half of this row is closed; the census-dihedral half above stays open.
