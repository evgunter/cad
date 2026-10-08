# SSIEDGE — the log

## 2026-10-08 — opened

Opened by SSI's orchestrator as SSI closed (Ev, in chat: "if you could
wind down and open successor programs for the rest"). SSI finished its
P0 slate and all but one P1 row; its thirty open rows were split along
the priority seam and by theme into SSIEDGE (P2), SSIARITH (P3) and
SSIMARCH (P3). This program took 9 of them, 22.5 budget points.
Rows moved with `git mv` and keep their ids.
- 2026-10-08 — Seam note from ENCL (PR 3431, `encl/collapsed-arm-gates`, merged): a definitely collapsed dihedral lever arm and a collapsed NURBS span meter now refuse as their own decisions instead of folding into a poisoned margin. `geom_brep::enters::LeverEscalation` carries a private gate verdict (`refused`, minted only by `LeverEscalation::arm(gate)`; re-quote with `with_diag`, read with `collapsed_arm()`), and struct literals of it no longer compile outside `enters`. New: `CertifyError::ArmCollapsed`, `ValidationError::NoDihedralArm` (pncad tag `no_dihedral_arm`), `CertCheck::ParamSpanMeter`/`SpanMeterCollapsed`, `recourse::Refused::rejected`; `DIHEDRAL_ARM` has an `at_zero` note (cone apex); the arm texts now read "long enough … to measure the angle between them". `LeverEscalation::of_rung` is gone; the boolean seam routes by rung through `BooleanDecision::of_lever`. (ENCL orchestrator)
