# REACHTAIL — the log

## 2026-10-08 — opened

Cut out of REACH by REACH's orchestrator when it closed (Ev, in chat:
split REACH into priority-stratified successors and close it). REACH
measured 139 budget points against 30; its six charter rows were
closed and its last four units (PRs 4122, 4123, 4135, 4159) merged.
Every row here moved by `git mv` with its id, body and history
unchanged. Rows dispatchable: 5, for 8 points.
— (REACH orchestrator)
- 2026-10-08 — Seam note from ENCL (PR 3431, `encl/collapsed-arm-gates`, merged): a definitely collapsed dihedral lever arm and a collapsed NURBS span meter now refuse as their own decisions instead of folding into a poisoned margin. `geom_brep::enters::LeverEscalation` carries a private gate verdict (`refused`, minted only by `LeverEscalation::arm(gate)`; re-quote with `with_diag`, read with `collapsed_arm()`), and struct literals of it no longer compile outside `enters`. New: `CertifyError::ArmCollapsed`, `ValidationError::NoDihedralArm` (pncad tag `no_dihedral_arm`), `CertCheck::ParamSpanMeter`/`SpanMeterCollapsed`, `recourse::Refused::rejected`; `DIHEDRAL_ARM` has an `at_zero` note (cone apex); the arm texts now read "long enough … to measure the angle between them". `LeverEscalation::of_rung` is gone; the boolean seam routes by rung through `BooleanDecision::of_lever`. (ENCL orchestrator)

## 2026-10-09 — 4 rows re-homed from CONTACT

CONTACT closed (Ev, in chat 2026-09-29: close CONTACT and move its
rows to successors) and re-homed here, by `git mv` with ids and bodies
unchanged, the rows this track's charter fits, for 5.5 points:
`c9-ring-class-phrase-names-a-retired-ring-or-the-exclusion-ring`,
`sphere-chart-trim-folds-any-number-of-rim-levels`,
`full-turn-question-has-three-spellings`,
`edge-face-crossing-cut-and-pass-five-decide-one-crossing-twice`.
The rest went to RESTREAD, INSIDE, SECTOR and CONTACTHOLD
(`docs/doc-ledger/contact-leaves-the-tracker.md`).
— (CONTACT orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4366, merged): `geom_brep::enters::LeverEscalation`'s `rung` and `diag` are private; read them with `rung()`/`diag()`, re-quote only through `with_diag`, which keeps the gate's verdict. `BooleanError::of_lever_rung(gate, read, rung, diag)` is the one boolean spelling. The dihedral lever-arm decision is told in one shape ("long enough, for how its faces curve, to measure their angle"), with the lever "clearly longer and no face curves tightly there"; pin `validate::tests::the_dihedral_arm_is_told_in_one_shape` (it reads source literals: a natural "long enough … angle … face" wording elsewhere trips it). (ENCL orchestrator)

- 2026-10-23 — **Seam (QUAD):** PR 5589 (`quad/measure-a-lanes`, measure door unit A) re-words one doc in `topo/src/boolean/ops.rs`, and re-pins `reach_volume_backstop`'s open-range model (≈ 5e-13·s; it was 2.9e-12·s while the lanes had round caps). (QUAD orchestrator's lane)

- 2026-10-23 — **Seam (QUAD):** PR 5628 (`quad/measure-b-door`, measure door unit B) edits `boolean/ops.rs`'s volume backstop: it walks from round 0 and refines to each face's own wall (see CLEAVE's note). (QUAD unit B lane)

- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`) touches this program's ground. Every public number door now takes a required predicate and answers `topo::MassMeasurement { enclosure, stop }`: `topo::mass_properties(body, tol, settle)`, `SignCertificate::measure(settle)`, `HeldCertificate::measure(body, settle)` (step-import's `StepImport::Solid { certificate, .. }` replaces `enclosure`), and Python's `Body.mass_properties(settle)`, `validate_geometric_measured(settle)` and `ImportReport.measure(settle)` (`pncad.Settle`, `pncad.MassMeasurement`). `PropsError::QuadratureBudget`, `TargetUnreached`, `VolumeReading`, `refine_to_target`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone; an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. `ShellClassification`'s four number fields are one `enclosure: MassProperties`. Tests reading a number pass `topo::test_support::pinned_resolution(tol)` (1024·ε, where their pins were taken). (QUAD unit C lane)
