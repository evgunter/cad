# REACHHOLD — the log

## 2026-10-08 — opened

Cut out of REACH by REACH's orchestrator when it closed (Ev, in chat:
split REACH into priority-stratified successors and close it). REACH
measured 139 budget points against 30; its six charter rows were
closed and its last four units (PRs 4122, 4123, 4135, 4159) merged.
Every row here moved by `git mv` with its id, body and history
unchanged. No row is dispatchable; the track opens `blocked`.
— (REACH orchestrator)

## 2026-10-25 — band rule (band-rule triage)

**Band rule (work/README.md, Priority):** `boolean-door-runs-the-census-over-its-result` re-banded P1→P0: FLUSH's P0s `a-near-tangent-vertex-lands-within-the-band-of-a-face-and-ships-unrecorded` and `a-result-that-is-only-a-sliver-shell-passes-the-door` are parked on it, and the door census is their whole residue. Its blocker, RESTREAD's `census-cross-solid-curved-pairs-undecidable-on-shell-results`, goes to P0 with it. (band-rule triage)
- 2026-10-25 — **Re-cut (refold approved in chat):** moved `nurbs-edge-crossing-rung-is-the-ring-composite` to SPLICE by `git mv`, ids and bodies unchanged, each with a dated Moved note. SPLICE (`work/splice/`) is the new track for a body built by loft or sweep entering every boolean; SECT now carries the torus pairs it is built for.
- 2026-10-26 — (tracker sweep) **REACHTAIL folded in.** Both tracks are REACH's leftovers on one ground, and together they come in under one budget (29/30). REACHTAIL's nine rows moved here by `mv`, ids and bodies unchanged: seven live, and two closed in this sweep. REACHTAIL's `program.md`, `plan.md` and `log.md` are deleted (`docs/doc-ledger/reachtail-leaves-the-tracker.md`). The program header and plan are rewritten: the rows stage 4 E/F released are open, not parked, and `paths` adds `contain.rs` and `ops.rs`. Closed:
  - `a-union-over-a-declared-continuation-keeps-its-walls-split`, fixed: B ∪ A builds the plate's 10 faces (`reach_continuation.rs:921`);
  - `contain-doc-links-a-wrap-rims-that-moved`, fixed: no `wrap_rims` link is left in `contain.rs`;
  - `c9-ring-class-phrase-names-a-retired-ring-or-the-exclusion-ring`, fixed in this sweep: comments at `census.rs`, `validate.rs`, `instance.rs` and `m5_pr9_boss_union.rs` now name the C9 exclusion ring, per `crates/topo/README.md:133`.

  `maximal-faces-curved-arm-cannot-tell-a-licensed-curved-skip` gains a note: the rounded stack merges its fillets now, so its conflict witness is gone. Re-measure before building.
- 2026-10-23 — **Seam (QUAD), carried from REACHTAIL's log at the fold:** PR 5628 (`quad/measure-b-door`, measure door unit B) edits `boolean/ops.rs`'s volume backstop: it walks from round 0 and refines to each face's own wall (see CLEAVE's note). (QUAD unit B lane)
- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`, PR 5741) touches this program's ground. Every public number door takes a required predicate (`topo::Settle`: `Sign`, `AtMost`, `AtLeast`, `Contains`, `Resolution`) and answers `topo::MassMeasurement { enclosure, stop }`; a number is read through `MassMeasurement::met()`, which answers the enclosure where the resolution was met and a typed `Unmet` where it was not, and an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. A deciding predicate still undecided where no face can refine is the one quadrature refusal, `MassPropsError::Undecided { volume: (lo, hi), stop }`. An import hands out `StepImport::Solid { body: topo::CertifiedBody, .. }`: the body held with the gate's certificate of it (`body()`, `into_body()`, `measure(settle)`). `PropsError::QuadratureBudget`, `TargetUnreached`, `HeldCertificate`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone. Tests reading a number use `topo::test_support::pinned_volume(what, body, tol)` (1024·ε, asserting the resolution met). (QUAD unit C lane)
- 2026-10-27 — (seam, CLEAVE PR 5842) The volume backstop left the boolean's operation path under `[ev]` PR 5780 (its relations are a test oracle, `topo::test_support::volume_relations`): closed `a-settled-declared-coincidence-crosses-a-tight-volume-bound`.

- 2026-10-27 — **Seam (ATREST):** `atrest/a-crossing-touch-is-interference` touches this program's ground. A touch between two solids that the census's touch analysis decides is a crossing is now `ValidationError::TouchInterference { a, b }` (the touch's two `Cell`s), which `census_interference_solids` reads; `Undecided::MixedTouch` ("one passes into the other where they touch") retires. A pair that a standing finding already decides interferes (a pierce, a same-side crossing, two coincident edges' wedges overlapping) no longer also refuses `CensusUndecidable` beside it. New exhaustive-match arm: `boolean/ops.rs`, `checks/at_rest.rs` `finding_solids`, pncad-py `touch_interference`. Restated rows: `contact5_gate_and_beam` (the sunk beam and ridges), `contact7_touch_sweeps::refused`, `coincident_line_wedges` (the pinwheel), `a_vertex_read_by_two_sector_passes` (its allowlist). (ATREST crossing-touch lane)
