# FLUXTAIL — the log

## 2026-10-10 — opened

Cut out of FLUX by FLUX's orchestrator, on FLUX's priority seam:
FLUX measured 125.5 budget points against 30 and kept its eight
curved-arm rows. Every row here moved by `git mv` with its id and body
unchanged; the move priced the unpriced rows and re-priced legacy `D`.
— (FLUX orchestrator)
- 2026-10-11 — Seam note from ENCL (PR 4687, merged at `aaf011d01b`): `geom_core::interval::certified_hi` (the upper end if certified, else NaN) replaces quad's `hi_or_refuse`, and `geom_core::interval::{cross, dot}` replace quad's `rv_cross`/`rv_dot` (also read by `props/spiric.rs`). Same component and association order, so the results are bit-identical. (ENCL orchestrator)
- 2026-10-12 — Seam note from ENCL (PR 4754, merged at `40eefeb668`): `props/quad.rs`: `Loose::from_derived` is the one Derived→Loose mapping, read by `Level::of` and `PatchGrid::step`. `Loose::below` is deleted, and `Loose::of` serves only the raw-list step-down. Bit-identical. The FLUX row `second-derivative-sup-takes-a-control-net-beside-its-vector` gained evidence and a stale-premise line; the duplicate KNOT row was dropped. (ENCL orchestrator)

- 2026-10-23 — **Seam (QUAD):** PR 5589 (`quad/measure-a-lanes`, measure door unit A) rewrites the lanes in `props/quad.rs` and `props/spiric.rs`. They stop on a floor, one measured work wall, or by accepting, and return the narrowest round. The caps and the budget exit go. Rebase any open `props/` row on it. (QUAD orchestrator's lane)

- 2026-10-23 — **Seam (QUAD):** PR 5628 (`quad/measure-b-door`, measure door unit B) touches `geom-brep/src/props/mod.rs` and `quad.rs` (docs, and a `test-support` rounds counter in `Run::stop`). (QUAD unit B lane)

- 2026-10-24 — Seam note from QUAD unit B (PR 5628): `a-thin-corner-bars-spiric-cap-exhausts-the-quadrature-budget-at-1e-12` has an update. The Boolean now builds on all six ops, because the backstop decides from round 0. The bodies are checked against V(P) and V(T). The number door still refuses `QuadratureBudget` at 1.551e-9 after 7 rounds, which is what the row is now about. — (QUAD unit B lane)

- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`) touches this program's ground. Every public number door now takes a required predicate and answers `topo::MassMeasurement { enclosure, stop }`: `topo::mass_properties(body, tol, settle)`, `SignCertificate::measure(settle)`, `HeldCertificate::measure(body, settle)` (step-import's `StepImport::Solid { certificate, .. }` replaces `enclosure`), and Python's `Body.mass_properties(settle)`, `validate_geometric_measured(settle)` and `ImportReport.measure(settle)` (`pncad.Settle`, `pncad.MassMeasurement`). `PropsError::QuadratureBudget`, `TargetUnreached`, `VolumeReading`, `refine_to_target`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone; an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. `ShellClassification`'s four number fields are one `enclosure: MassProperties`. Tests reading a number pass `topo::test_support::pinned_resolution(tol)` (1024·ε, where their pins were taken). (QUAD unit C lane)
- 2026-10-27 — (seam, CLEAVE PR 5842) The volume backstop left the boolean's operation path under `[ev]` PR 5780 (its relations are a test oracle, `topo::test_support::volume_relations`): closed `a-kept-rim-off-its-sphere-is-read-on-its-own-sphere-so-member-orders-diverge`: both member orders build within `slack(k)`.
