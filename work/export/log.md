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
- 2026-11-02 — **REIMPORT merged into EXPORT.** Under the budget rule of 2026-11-02 (only dispatchable P0, P1 and unbanded rows count) REIMPORT had fallen far under budget, and its charter is one story with EXPORT's. Its 9 item files moved here by `git mv`, ids and bodies unchanged, each with a dated note; its plan is folded into `plan.md` and its charter, `paths` and `keep_out` into `program.md`, and its directory left the tracker (`docs/doc-ledger/reimport-leaves-the-tracker.md`, which names the SHA its log is recoverable at).
- 2026-11-17 — `main-red-swept-elbows-step-fixture-drifted-in-five-cap-directions` closed: `swept_elbow.step`'s five end-cap direction lines moved 3 ulps of a 6.1e-17 component when PR 6988 put the sweep's station step on the least turn's W1 (Rodrigues) spelling; judged rounding by A/B (the quaternion spelling restores every fixture), fixture regenerated. (EXPORT fixture-drift lane)
- 2026-11-17 — **Sitting opened (EXPORT orchestrator); track `active`.** Slate: the P0 and the five P1 rows. Dispatch plan, each unit's letter and review tier decided before the model draws (the draws were taken the same minute; this entry was committed just after them, which is the order slip to know of):
  - `export/fixture-drift` — `main-red-swept-elbows-step-fixture-drifted-in-five-cap-directions` (P0, E). **Orchestrator's read**: trace the move to its commit, judge it rounding or not, regenerate the fixtures deliberately; nothing in it is code. Outside the model comparison.
  - `export/void-shells` — `step-export-refuses-every-hollow-body` (P1, M). Letter M, **single FULL review**: a writer arm plus the import side reading `BREP_WITH_VOIDS` back, with a round-trip contract to falsify.
  - `export/smooth-join` — `step-import-adopts-a-smooth-join-without-the-must-carry-rule` (P1, M). Letter M, **dual review**: a new certification rung in the adoption ladder, where a wrong candidate that certifies is silent wrong geometry.
  - `export/walks-and-zeros` — `step-lanes-spell-the-half-edge-to-face-walk`, `three-crates-fold-signed-zero-with-one-body-and-three-unlinked-arguments` (re-priced `E` from legacy `D`: its filer's reading, disclosure not one home, is the fix and no fork stands open on it), with `signed-zero-module-hand-counts-the-bitwise-readers-it-exists-for` (P4, E) riding along on the same module. Letter S, **single STYLE review**.
  - `step-import-discards-the-entity-ids-that-are-its-identity-channel` (P1, legacy D, re-priced `M` with `design: true`): **a design fork** (how an import's source partitions from a recipe's, and what the per-evaluation scope claim means for an imported body). Designers first, per `docs/DESIGN-FORK-PROTOCOL.md`.
- 2026-11-17 — **`walks-and-zeros` merged (PR 7067)**: single STYLE review (Opus 5.5, no draw under rule 12, so no model-compare row) and a fix pass that took one home over disclosure. `geom_core::fold_signed_zero` now replaces six production copies and six test copies; `adopt.rs`'s walks go through `first_he`/`body_loop`/`loop_face`/the topo door; `face_of_half_edge`'s doc states the rule callers follow. Closed: `step-lanes-spell-the-half-edge-to-face-walk`, `three-crates-fold-signed-zero-…`, `signed-zero-module-hand-counts-…`, and bind's `pncad-py-spells-the-hash-signed-zero-fold-twice`.
- 2026-11-17 — **`smooth-join` merged (PR 7081).** It had a dual tier with letter M, and one review, since the first raised no MAJOR. That review was by Opus 5.5, with no draw under rule 12, so no model-compare row is recorded. A fix pass followed:
  - the M7-2 coincidence rung was removed, since the rule rung subsumes it;
  - `geom_brep::must_carry_reads` is now one predicate;
  - the band is built once;
  - TAIL_TURBINE is pinned once, with its probe kept as a slow row;
  - the S7 label row was filed (P4).

  Closed: `step-import-adopts-a-smooth-join-without-the-must-carry-rule`.
- 2026-11-18 — **`void-shells` merged (PR 7077).** Single FULL review by Opus 5.5, no draw under rule 12, so no model-compare row. The fix pass covered:
  - the census goldens re-blessed at every ε;
  - the bound law restated for voids;
  - OCCT-written hollow fixtures;
  - unreversed-void mutant rows;
  - stale narration and docs, the exhaustive solid keyword, and the S5 helper row filed.

  The class it exposed (a shared-fixture-list diff skips the slow rows that iterate the list) is filed on GREEN as `a-diff-to-a-shared-fixture-list-skips-the-slow-rows-that-iterate-it`. Closed: `step-export-refuses-every-hollow-body`.
