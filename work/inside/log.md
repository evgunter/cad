# INSIDE — the log

## 2026-10-09 — opened

Cut out of CONTACT by CONTACT's orchestrator when it closed (Ev, in
chat 2026-09-29: close CONTACT and move all remaining rows to 2-3
successor programs). CONTACT measured 99.5 budget points against 30;
its units CONTACT-10, -11 and -12 merged (PRs 4363, 4368, 4372) and
CONTACT-13 parked on the D10 hold. Every row here moved by `git mv`
with its id, body and history unchanged. Rows dispatchable: 11, for 30 points.
— (CONTACT orchestrator)
- 2026-10-09 — Seam note from ENCL (PR 4461, merged at `a71723601f`): `topo::ShellRead::of` now returns `Result<Self, ShellClassifyError>`, with no `Option`: the unreadable selection was unreachable. `SolidFaces::of_shell` returns `Self`, and `ChartGroups::of_live` groups faces already read off the live arena (`of_body` uses it). `PieceSortError::RoleUnread` carries the role read's `ShellClassifyError` and renders it. (ENCL orchestrator)
- 2026-10-10 — Seam note from ENCL (PR 4497, merged at `4c7fb9b0fc`). A poisoned or contradicted margin now ends in a followable recourse:
  - at rest, the census coincidence menu (`validate::too_close` → `own_close`) gives the defect ending;
  - `ChartRegionError::Escalated`'s poisoned arm gives `defect_ending(Build)`;
  - `PointInSolidError::Escalated` gives the unnamed placement lever plus the reading's note, as `contfp`, `classify_point_in_solid` and the Boolean already did.

  `SliverSector` is unchanged, and its mints are on CLEAVE's split-escalations row. (ENCL orchestrator)
- 2026-10-11 — ZIP closed (`docs/doc-ledger/zip-leaves-the-tracker.md`); `tangent-lever-row-escalates-containment-at-eps-1e-6` moved here because its escalation is `contain.rs`'s containment ladder.
- 2026-10-12 — Seam note from ENCL (PR 4745, merged at `512ef4b6c7`): `RingHoming` in `chord_join` now routes its walk escalation through `ContainDecision::lever_ending`, the same repair `census-containment-escalation-drops-its-decision` asks for in census; that row's bullet is updated. (ENCL orchestrator)
- 2026-10-17 — Seam note from RESTREAD: claimed `point-in-solid-refuses-an-l-shaped-torus-face` (moved by git mv); it rides `restread/torus-face-bounded-by-an-oblique-circle-refuses-point-classification`, now decided as one torus region reading replacing the chart-window class. (RESTREAD orchestrator)
- 2026-10-18 — Seam note from RESTREAD: claimed `a-touch-at-a-saddle-corner-refuses-unanalysed` and `a-solid-touching-itself-at-a-vertex-reads-its-star-from-the-vertex-alone` (moved by git mv); they ride `restread/a-pinch-point-the-census-reads-as-mixed-touch-where-the-body-holds-no-overlap`, decided as one whole-link touch reading at a point with one complete cone test. (RESTREAD orchestrator)
- 2026-10-19 — Seam note from RESTREAD (PR 5331, `restread/link-touch-reading`): `a-touch-at-a-saddle-corner-refuses-unanalysed` and `a-solid-touching-itself-at-a-vertex-reads-its-star-from-the-vertex-alone` close with the pinch row: a touch at a point reads each solid's whole link there, and a saddle corner or a self-touching solid reads like any other link. `TouchUnanalysed`'s text now names a face meeting the point twice over or an outline that does not close. (RESTREAD implementer)

- 2026-10-23 — **Seam (QUAD):** PR 5628 (`quad/measure-b-door`, measure door unit B) edits `boolean/solid_contain.rs`'s `at_infinity_side`: the names ride in `SignNames`, and the walk refines past the reporting target while the sign straddles. (QUAD unit B lane)

- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`) touches this program's ground. Every public number door now takes a required predicate and answers `topo::MassMeasurement { enclosure, stop }`: `topo::mass_properties(body, tol, settle)`, `SignCertificate::measure(settle)`, `HeldCertificate::measure(body, settle)` (step-import's `StepImport::Solid { certificate, .. }` replaces `enclosure`), and Python's `Body.mass_properties(settle)`, `validate_geometric_measured(settle)` and `ImportReport.measure(settle)` (`pncad.Settle`, `pncad.MassMeasurement`). `PropsError::QuadratureBudget`, `TargetUnreached`, `VolumeReading`, `refine_to_target`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone; an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. `ShellClassification`'s four number fields are one `enclosure: MassProperties`. Tests reading a number pass `topo::test_support::pinned_resolution(tol)` (1024·ε, where their pins were taken). (QUAD unit C lane)
- 2026-10-31 — (seam, FLUSH) Claimed and closed `aimed-rays-start-at-a-conic-carriers-reach-where-the-ray-quadrics-lose-their-roots` (P1) as a duplicate of FLUSH's `an-aimed-rays-start-past-a-sections-whole-ellipse-loses-its-wall-roots-and-reads-the-wrong-side`, which PR 5781's dual review measured more fully. FLUSH dispatches it next; its witness pose is carried on FLUSH's row. The fix will touch `solid_contain.rs`'s ray × quadric roots and the side reader's reach, which is INSIDE's ground. — (FLUSH orchestrator)
