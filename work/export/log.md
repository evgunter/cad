# EXPORT — the log

## 2026-09-20 — opened

Cut out of EXCH, which was carrying 44.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed EXCH's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

10 rows arrived by `git mv` with their ids, bodies and history
unchanged. EXCH keeps its band 2100-2199; band 9200-9299 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.
- 2026-10-23 — Closed `step-import-source-debug-in-prose-panics-the-binding`. It was already fixed by D343 (PR 1854, `4f1f257c6b`), on the day the row was split out.
- 2026-10-25 — Seam: PRED S18 (PR 5684) edits `crates/step-export/src/{lib,writer}.rs`, deletes `volume.rs`, and closes `step-export-reclassifies-shell-roles-with-its-own-planar-flux`. The writer reads multi-shell roles off `topo::classify_shells_of`, so hollow curved bodies now refuse `VoidShellUnsupported`. (PRED orchestrator)
- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`, PR 5741) touches this program's ground. Every public number door takes a required predicate (`topo::Settle`: `Sign`, `AtMost`, `AtLeast`, `Contains`, `Resolution`) and answers `topo::MassMeasurement { enclosure, stop }`; a number is read through `MassMeasurement::met()`, which answers the enclosure where the resolution was met and a typed `Unmet` where it was not, and an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. A deciding predicate still undecided where no face can refine is the one quadrature refusal, `MassPropsError::Undecided { volume: (lo, hi), stop }`. An import hands out `StepImport::Solid { body: topo::CertifiedBody, .. }`: the body held with the gate's certificate of it (`body()`, `into_body()`, `measure(settle)`). `PropsError::QuadratureBudget`, `TargetUnreached`, `HeldCertificate`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone. Tests reading a number use `topo::test_support::pinned_volume(what, body, tol)` (1024·ε, asserting the resolution met). (QUAD unit C lane)
