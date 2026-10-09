# SSIARITH — the log

## 2026-10-08 — opened

Opened by SSI's orchestrator as SSI closed (Ev, in chat: "if you could
wind down and open successor programs for the rest"). SSI finished its
P0 slate and all but one P1 row; its thirty open rows were split along
the priority seam and by theme into SSIEDGE (P2), SSIARITH (P3) and
SSIMARCH (P3). This program took 11 of them, 24.5 budget points.
Rows moved with `git mv` and keep their ids.
- 2026-10-08 — Seam note from ENCL (PR 3431, `encl/collapsed-arm-gates`, merged): a definitely collapsed dihedral lever arm and a collapsed NURBS span meter now refuse as their own decisions instead of folding into a poisoned margin. `geom_brep::enters::LeverEscalation` carries a private gate verdict (`refused`, minted only by `LeverEscalation::arm(gate)`; re-quote with `with_diag`, read with `collapsed_arm()`), and struct literals of it no longer compile outside `enters`. New: `CertifyError::ArmCollapsed`, `ValidationError::NoDihedralArm` (pncad tag `no_dihedral_arm`), `CertCheck::ParamSpanMeter`/`SpanMeterCollapsed`, `recourse::Refused::rejected`; `DIHEDRAL_ARM` has an `at_zero` note (cone apex); the arm texts now read "long enough … to measure the angle between them". `LeverEscalation::of_rung` is gone; the boolean seam routes by rung through `BooleanDecision::of_lever`. (ENCL orchestrator)
- 2026-10-08 — Seam note from ENCL (PR 4331, `encl/adoption-at-rest-eps-in`, merged): `Reading::Adopt` is gone, and adoption certification reads as at rest. At the import door, the file's ε_in picks the words, for error text only (`geom_core::FileCoincidence`; `MarginDiag::sized_recourse_in_file`; `FileCoincidence::miss_recourse_in_file`; `geom_brep::certify::recourse_in_file`; `SizedDecision::recourse_in_file`).
  - A size at or below ε_in reads "This {size} is below the file's declared coincidence distance ε_in = X m, so the file does not state it", or "may lie below … may not state it" where only the nearer end is. It ends in the lever plus "re-export … declared below {near} m and tighten below {near/K} m" where a value exists.
  - A miss within ε_in names re-exporting, plus the set-ε-to-ε_in stopgap.
  - `CertifyError::ResidualExceeded` now carries `margin: MarginDiag`.
  - The `reporting-margin-door.sh` gate pins the two new sentence functions; recourse.rs is at 5 sites.
  - DESIGN.md D4 commitment 3 is reworded (the magnitude carrier moved). (ENCL orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4422, merged): `geom_brep::recourse::RefusedArm::SignCertain` now takes `Option<MarginDiag>`; construct with `SignCertain(None)` unless the decision is a residual miss, and match with `SignCertain(_)`. `certify::definite_miss_in_file` / `Unsized::definite_residual_in_file` are gone; `Unsized::residual_in_file` is the one door. (ENCL orchestrator)
