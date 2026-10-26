# GAUGE — the log

## 2026-10-08 — opened

Cut out of REACH by REACH's orchestrator when it closed (Ev, in chat:
split REACH into priority-stratified successors and close it). REACH
measured 139 budget points against 30; its six charter rows were
closed and its last four units (PRs 4122, 4123, 4135, 4159) merged.
Every row here moved by `git mv` with its id, body and history
unchanged. Rows dispatchable: 15, for 25.5 points.
— (REACH orchestrator)

## 2026-10-09 — 2 rows re-homed from CONTACT

CONTACT closed (Ev, in chat 2026-09-29: close CONTACT and move its
rows to successors) and re-homed here, by `git mv` with ids and bodies
unchanged, the rows this track's charter fits, for 3.5 points:
`gate-maximal-faces-may-miss-coincident-curved-faces-on-different-keys`,
`census-touch-guard-needles-miss-a-point-free-call`.
The rest went to RESTREAD, INSIDE, SECTOR and CONTACTHOLD
(`docs/doc-ledger/contact-leaves-the-tracker.md`).
— (CONTACT orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4422, merged): `geom_brep::recourse::RefusedArm::SignCertain` now takes `Option<MarginDiag>`; construct with `SignCertain(None)` unless the decision is a residual miss, and match with `SignCertain(_)`. `certify::definite_miss_in_file` / `Unsized::definite_residual_in_file` are gone; `Unsized::residual_in_file` is the one door. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4474, merged): the material pairing is a decision. `geom_brep::MATERIAL_PAIRING` and `MATERIAL_PAIRING_CLAUSE` end it at tier 3 (`WedgeCheck::MaterialPairing`) and split finish (`SplitFinishError::DescribeSideEscalated`), with a true tolerance offer. Where the wedge would re-decide below m/K, `pairing_at_wedge` quotes the wedge and offers w/K, as `at_wedge` does. `MaterialStations`' `Break` is `MaterialStop { check, cause }`. The cusp-side Zero reads via `decide_nonzero`. New k_stats predicates appear on the refusal path only: "material_pairing_offer_wedge" and "material_pairing_wedge". (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4745, merged at `512ef4b6c7`): topo's too-close-to-call sentences now take the undecided shape. Split endings: finish.rs ×3 and `SliverVertex` share `splitting::split_lever_ending` (lever alone; the unreadable-margin note only on poison); `SectionError` refusals share `SECTION_PLANE_RECOURSE`; `SectionWindingUndecided` uses one wording. Join endings: `SplitJoinError::Escalated` ends `diag.ending(JOIN_RECOURSE)` (the same ending geom-brep gives the decision); `OrderEscalated` picks by predicate (`order::FRAME_AXIS`/`FRAME_ARM` take the lever alone, order_u/v are sized); `RingHoming` leads 're-homing a hole loop: ' and ends `ContainDecision::Loop(..).lever_ending` (no tolerance; follows refusal_routes' Placement ruling). `UnderBoolean` is removed and `BooleanError::Join` prints the join's own message. Left open on `split-escalations-end-a-poisoned-margin-in-the-plane-lever`: `CrossingEscalated`, contradiction-minted poison, the `pc_axis_plane_parallel_straddle` 'subdivide' vs lever-alone straddle clash, and `chord_arc_leave_germ`/`_section` (unsized, but offer the tolerance). (ENCL orchestrator)
- 2026-10-13 — Seam note from ENCL: PR 4766 has merged. It covers the sweep slice of the undecided-wording sweep (sweep crate endings by decision kind), and geom-core now has `Indeterminate::lever_ending` as the one lever-only-at-build ending. Lever-only decisions (angles, cos/sin, winding, form selection, gauges, walks) end through it. Do not compose `lever_recourse` + the unreadable note by hand. (ENCL orchestrator)
- 2026-10-18 — Seam note from FUSE, which closes (`docs/doc-ledger/fuse-leaves-the-tracker.md`): five coverage rows, `coincident-shell-has-no-fixture-for-its-unpaired-and-mixed-arms`, `shared-vertex-crossings-that-tie-or-interleave-are-unprobed`, `a-carried-vertex-on-face-row-at-a-pinch-is-unprobed`, `door-arms-only-a-hand-split-operand-reached` and `planar-door-consumers-survive-the-sense-dropping-mutant-at-topo`, moved here by `git mv`, ids kept and bodies unchanged but for re-pointed `work/fuse/` citations. (FUSE close lane)

- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`) touches this program's ground. Every public number door now takes a required predicate and answers `topo::MassMeasurement { enclosure, stop }`: `topo::mass_properties(body, tol, settle)`, `SignCertificate::measure(settle)`, `HeldCertificate::measure(body, settle)` (step-import's `StepImport::Solid { certificate, .. }` replaces `enclosure`), and Python's `Body.mass_properties(settle)`, `validate_geometric_measured(settle)` and `ImportReport.measure(settle)` (`pncad.Settle`, `pncad.MassMeasurement`). `PropsError::QuadratureBudget`, `TargetUnreached`, `VolumeReading`, `refine_to_target`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone; an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. `ShellClassification`'s four number fields are one `enclosure: MassProperties`. Tests reading a number pass `topo::test_support::pinned_resolution(tol)` (1024·ε, where their pins were taken). (QUAD unit C lane)
- 2026-10-25 — **Band rule (work/README.md, Priority):** `shared-vertex-crossings-that-tie-or-interleave-are-unprobed` re-banded P3→P1. Its one open arm, the fan interleave, is what cleave's `three-corners-alternating-round-a-corner-refuse-at-the-join` (P1) is parked on: three of that row's ops refuse `SharedVertexCrossings` at the reconciliation. (band-rule triage)

- 2026-10-25 — **Re-cut (refold approved in chat):** moved `a-nurbs-edges-sector-departure-is-its-chord` to SPLICE, `section-area-skips-a-spiric-or-nurbs-section-edge` to SPLICE, `split-section-boundary-curved-arm-untested-past-the-edge-gate` to SPLICE by `git mv`, ids and bodies unchanged, each with a dated Moved note. SPLICE (`work/splice/`) is the new track for a body built by loft or sweep entering every boolean; SECT now carries the torus pairs it is built for.
