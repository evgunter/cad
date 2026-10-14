# APEX — the log

## 2026-10-08 — opened

Cut out of REACH by REACH's orchestrator when it closed (Ev, in chat:
split REACH into priority-stratified successors and close it). REACH
measured 139 budget points against 30; its six charter rows were
closed and its last four units (PRs 4122, 4123, 4135, 4159) merged.
Every row here moved by `git mv` with its id, body and history
unchanged. Rows dispatchable: 11, for 29.5 points.
— (REACH orchestrator)
- 2026-10-08 — Seam note from ENCL (PR 3431, `encl/collapsed-arm-gates`, merged): a definitely collapsed dihedral lever arm and a collapsed NURBS span meter now refuse as their own decisions instead of folding into a poisoned margin. `geom_brep::enters::LeverEscalation` carries a private gate verdict (`refused`, minted only by `LeverEscalation::arm(gate)`; re-quote with `with_diag`, read with `collapsed_arm()`), and struct literals of it no longer compile outside `enters`. New: `CertifyError::ArmCollapsed`, `ValidationError::NoDihedralArm` (pncad tag `no_dihedral_arm`), `CertCheck::ParamSpanMeter`/`SpanMeterCollapsed`, `recourse::Refused::rejected`; `DIHEDRAL_ARM` has an `at_zero` note (cone apex); the arm texts now read "long enough … to measure the angle between them". `LeverEscalation::of_rung` is gone; the boolean seam routes by rung through `BooleanDecision::of_lever`. (ENCL orchestrator)
