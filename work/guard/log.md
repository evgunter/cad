# GUARD log

Newest entries at the bottom; the tail is the program's live status.
Plan: `work/guard/plan.md`.

## Opened (2026-09-11)

Opened in the tracker cut of 2026-09-11 (Ev's direction, in-chat;
`docs/WORK-TRACKS-2026-09.md` addendum 3), which read every non-closed
item in `work/issues/` and `work/code-quality/` — 110 of them — and cut
eleven programs from the pile. This is the successor on
`scripts/gates/*`, the ground GATES held from 2026-09-06 to 2026-09-08
(`docs/DOC-LEDGER.md` sweep 7) and left unfinished.

Eleven rows moved in by `git mv`, each with a `## Re-homed` record: four
from `work/issues/`, seven from `work/code-quality/` (three of them Track
K rows, one Track V). `D212` and `G4` are the two GATES re-homed to
code-quality at its sweep and are back on the ground they name.

No branch exists yet. The first act is not a unit: `G4`'s park cites
#1647, which merged, and `work.py lint` has been naming it stale since
2026-09-06 — verify it, then open or re-park it, and `D212` with it.

## Announced seam from TOPO (2026-09-14): one REGISTER entry in `loop-boundary-discards.sh`, with the loop-re-parenting unit

TOPO's `loop-reparenting-euler-ops-leave-rows-certified-against-the-wrong-chart`
(PR 2549, branch `topo/loop-reparenting-rows`) adds one `&mut Body`
limb, `Body::drop_rows_on_chart_change` in
`crates/topo/src/euler_ring.rs`, which walks a moved loop's cycle to
drop the pcurve rows that changed chart with it. Reading the loop's
boundary is a `LoopBoundary` discard, so `loop-boundary-discards`
reds until it is registered — which it did, on the first hosted run
(`UNREG|crates/topo/src/euler_ring.rs|…|drop_rows_on_chart_change`),
and that is the gate doing exactly what its header says it is for.

The single edit in this program's `scripts/gates/loop-boundary-discards.sh`
is one REGISTER line and nothing else — no matcher, no window, no
disposition vocabulary, no selftest:

```
"crates/topo/src/euler_ring.rs|drop_rows_on_chart_change||1|audited: the Empty arm is the same answer as the Cycle arm's — an empty loop holds no half-edge, so it holds no pcurve row, and the door has nothing to drop"
```

It is filed `audited` rather than `unaudited` because the arm really
has been named: the discarded case is `LoopBoundary::Empty`, and an
empty loop carries no half-edge, so the map this door is about holds
nothing keyed on it. The gate reads neither disposition, as its header
says, so this is a claim for a reviewer and not for the script.

The register's own counters moved with it: 79 discards the matcher
sees, 75 entries, **2 audited** (was 1) and 73 unaudited. The gate and
its `--selftest` both pass locally on the head. Signed (TOPO
implementer lane, `topo/loop-reparenting-rows`).

## The register entry moved with its walk (2026-09-14, PR 2549's fix pass)

Both reviews of the head above asked for the loop walk under the drop
to have one home rather than a fresh copy, and it now does:
`pcurves::loop_rows` in `crates/topo/src/pcurves.rs`, which
`pcurves::stored_rows` and `Body::drop_rows_on_chart_change` both call.
The `LoopBoundary` discard moved with it, so this program's
`scripts/gates/loop-boundary-discards.sh` REGISTER loses two lines and
gains one — still one edit, still no matcher, window, vocabulary or
selftest change:

```
-  "crates/topo/src/euler_ring.rs|drop_rows_on_chart_change||1|audited: …"
-  "crates/topo/src/pcurves.rs|stored_rows||1|unaudited"
+  "crates/topo/src/pcurves.rs|loop_rows||1|audited: the discarded variant is named and answered — a loop whose boundary is not a cycle returns the NoCycle answer, distinct from Corrupt, and it holds no half-edge, so it holds no pcurve row"
```

The `stored_rows` entry goes because that function no longer discards a
boundary; the door's goes because it never did anything but call the
walk. The disposition is `audited` for a stronger reason than the
entry it replaces: the arm is not merely reasoned about, it is NAMED —
`LoopRows::NoCycle` is a distinct answer from `LoopRows::Corrupt`, so a
caller cannot collapse the two by accident — and
`an_empty_boundary_ring_moves_with_the_map_untouched`
(`crates/topo/tests/loop_reparenting_pcurve_rows.rs`) measures it
through a real `LoopBoundary::Empty` ring. The gate reads no
disposition, as its header says, so that is for a reviewer.

Counters after the move: **78** discards the matcher sees (was 79),
**74** entries (was 75), **2 audited**, 72 unaudited. The gate and its
`--selftest` both pass on this head. Signed (TOPO fix-pass lane,
`topo/loop-reparenting-rows`).


**(SYM orchestrator) Seam announced after the fact, 2026-09-14 — SYM-6**
(PR #2604): the fix pass re-took the site-contract paragraph in
`scripts/gates/register-equal-allowlist.sh`'s header ("HANDLED BY
ARM": `Disputed` handled and never asserted, `Contradicted` a proof
the site asserts on), gate logic untouched, gate green. The change
followed Ev's D2 on `[ev]` #2552; disclosed in the PR body as a
territory crossing.

## 2026-09-22 — announced seam from VGEOM: a new viewer gate, both halves

(VGEOM orchestrator. Announcement, not a request.)

`vgeom/field-product` (#3067) added
`scripts/gates/viewer-numeric-field-door.sh` and wired it as a named
step in `ci.yml`'s `discipline` job. Both halves, per
`scripts/gates/README.md`; `local-scripts/ci-local.sh` runs the whole
directory, so it needed no edit.

**The rule**: no bare `egui::DragValue` under `crates/viewer/src`
outside `crates/viewer/src/widgets.rs`. One home rather than two
exemptions — the door's own constructor IS a `DragValue::new`, and the
rows holding what the door adds live in that file's `#[cfg(test)]`
modules. The matcher takes `DragValue::new` and
`DragValue::from_get_set`, reads code only (`gate_rust_code`), and
deliberately does **not** skip `#[cfg(test)]`: a bare field in a row
elsewhere is exactly the arrival the rule is about.

**Why it exists rather than a comment**: the crate's census — every
numeric field goes through the door, the one bare site is the test
harness's own — was a measurement taken once. reviewer-style §Q6 says
a claim resting on a measurement owes a mechanical guard, and this is
it.

**Proved to fire three ways**: `--selftest` plants four shapes and
passes the negatives; a bare `DragValue::new` appended to a real
viewer file redded with file and line, tree restored; and in CI in
`discipline (evaluation-code)`, both modes, self-test first.
`gate-roster.sh` and `check-ci-mirror-parity.py` pass.

**Disclosed blind spots**, in the gate's own header: `egui::Slider`
(a bare field under another name — none in the crate, and banning the
name would be a rule about a widget nobody asked for), and a field
built in another crate.
- 2026-09-29 — Seam note from ENCL: PR 3418 (merged `3094222a13`) implements Ev's ruling on `[ev]` PR 3402. `Decide::sign_within` returns `Decided { sign, margin }` on every outcome. `k_stats::decide` still returns `Sign`, and its sibling `decide_reported` returns `Decided`; the two share one classify and one log write. `MarginDiag` is now opaque and for error reporting only: no variant to match, no field, no ordering, no f64 conversion. The recourse wording comes from `sized_recourse`, and the only numeric door is `diagnostic_f64_for_error_text()`. `scripts/gates/reporting-margin-door.sh` pins door calls, mints, `sized_recourse` callers and `terminal_sliver: true` per file. `Indeterminate` gains `terminal_sliver`, decided at classify time. If your code matched `MarginDiag::Value/Enclosure/Invalid` or read its numbers, it now uses `kind()`, `is_invalid()` or the error-text door; this PR touched those sites mechanically. (ENCL orchestrator)

## Filed from TOPO: `the-bit-identity-consumer-gate-cannot-see-a-to-bits-read-of-bounds` (2026-09-29)

Ev's ruling on PR 3156 makes `query.rs`'s `to_bits()` compare of
`Bounds::lo`/`hi` a production bit-identity consumer. None of
`bit-identity-consumer.sh`, `bit-identity-punning.sh` or
`bounds-allowlist.sh` can see that spelling. The row carries the
shape. It lands red until TQUERY's repair of the site, so it rides
with or after that.
- 2026-09-30 — Seam note from TOPO: PR 3532 (`topo/merge-helpers-announce`) edits one entry of `scripts/gates/loop-boundary-discards.sh`'s register. The winding's let-else moved with its walk from `planar_loop_winding` into `planar_loop_winding_decided` (key renamed, count 1). The fix pass then made that arm return `LoopWinding::Empty`, a named answer the merge's role pass reads as not positive, and marked the entry `audited` on that ground. No other entry and no matcher code changed; the gate passes at 79 sites and 76 entries. (TOPO fix pass)
- 2026-10-08 — Seam note from ENCL (PR 4331, `encl/adoption-at-rest-eps-in`, merged): `Reading::Adopt` is gone, and adoption certification reads as at rest. At the import door, the file's ε_in picks the words, for error text only (`geom_core::FileCoincidence`; `MarginDiag::sized_recourse_in_file`; `FileCoincidence::miss_recourse_in_file`; `geom_brep::certify::recourse_in_file`; `SizedDecision::recourse_in_file`).
  - A size at or below ε_in reads "This {size} is below the file's declared coincidence distance ε_in = X m, so the file does not state it", or "may lie below … may not state it" where only the nearer end is. It ends in the lever plus "re-export … declared below {near} m and tighten below {near/K} m" where a value exists.
  - A miss within ε_in names re-exporting, plus the set-ε-to-ε_in stopgap.
  - `CertifyError::ResidualExceeded` now carries `margin: MarginDiag`.
  - The `reporting-margin-door.sh` gate pins the two new sentence functions; recourse.rs is at 5 sites.
  - DESIGN.md D4 commitment 3 is reworded (the magnitude carrier moved). (ENCL orchestrator)

- 2026-10-13 — **Seam note from NURBS: PR 4984 merged** (`2639b944`), unit 2 of the quotient ruling.
  - **Second order on the quotient form.** `C″`, `S_uu`, `S_uv` and `S_vv` now read the one Bernstein quotient bound over `W³`. Entry points: `geom_core::spline::{accel_pieces, cell_second_quotients, polynomial_area_pieces}` and `NurbsCurve3::accel_upper_bound`.
  - **Moved onto it:**
    - `geom-brep` `patch_bound` `rational_cells` (first- and second-order boxes, `m_sup`). The refinement and `coordinate_hull` are gone.
    - `mesh` chords' rational `sup|C″|`.
    - `offset_meters` `cell_normal` / `patch_regularity`, which now reads the area floor `PatchCell::m_inf`.
  - **Integral arm:** the area floor reads `A_u×A_v` at `2p−1` and certifies to degree 28. Past that it uses a conservative floor of 0.0.
  - **Refusals:**
    - a rational degree above 18 gives `PatchBoundError::DegreeAboveCeiling`, a typed recourse;
    - chords past the ceiling give `UnsupportedCurve`;
    - `QuotientRefused` is new (kernel defect);
    - `RefinedWeightLostPositivity` is gone;
    - `RefinementFailed` now means the integral insertion chain only.
  - **Mesh consequence:** a rational face above degree 18 now refuses `UnsupportedNurbsFace` in nurbs_cert, and offset refuses through `MeterError`.
  - **Goldens re-baselined, with causes in the PR body:** `m_sup`, chord counts, offset goldens, the budget and endpoint censuses, and meter ceilings.
  - **Perf:** the per-face bound is about 2× base on small rows and up to 11× on wide rational faces. Filed at `work/nurbs/second-order-quotient-pieces-cost-the-face-bound-eleven-times-main.md`. — (NURBS orchestrator)
- 2026-10-14 — **Seam (NURBS):** PR 5114 (`nurbs/interval-scale-exact`, in review) edits `from-f64-sites.tsv`: the halving's scale site now reads through `Real::scale_pow2`. (NURBS orchestrator)
- 2026-10-14 — **Seam (ENCL):** PR 5101 merged. At rest, one definite refused reading ends one way whichever arm routes it: `StoredDefinite::Lever` gives the lever plus the note; `Contradiction` gives `KERNEL_OR_FILE_DEFECT_ENDING`, at rest and at the door past ε_in. `reporting-margin-door.sh` gains a list; also filed `loop-boundary-discards-selftest-fails-on-a-clean-tree` here (red on main; the most recent gate edit is TANG 8f0b0ffd25). (ENCL orchestrator)
- 2026-10-18 — Seam note from WIRE: PR 5273 (merged `7e636c2bca`) added one line to `scripts/gates/probe-suite-census.sh` `RUN_FLOOR`, `plain:editor-core:parts_entered_before_the_map:3`, so the probe sweep runs the part pre-pass's sample-sequence rows. Nothing else in the gate changed. (WIRE orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5499 merged (`16b6b72f`, NURBS build C, TESS ground, FULL). **Lofted `Circle` sections now mesh.** The mesher's only door, `geom_brep::patch_bound::patch_cells_across_creases`, admits degree-≥2 creases on the rational arm; the integral arm, degree-1 creases and every other `patch_cells` caller (offset_fit, offset_meters) stay C¹-gated. A crease becomes a CDT constraint in `mesh::trimmed`. `geom_brep::sub_arc_joint(k, m)` is the one spelling of a sub-arc joint (shared by `sweep::skin::segment_curve`, `IsoArc` breaks and chord schedules; bits identical). An `IsoArc` rim with m ≥ 2 sub-arcs rounds its chord count to a multiple of m, so the joint is a chord point. A boundary crossing a crease off a chord point refuses `UnsupportedCurve`; creases in both directions refuse `UnsupportedNurbsFace`. No golden moved; the falsifier stays ≤ 0.4999 over 11 lofts × 20 δ. Closes `lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`. Still open: `work/tess/lofted-rational-walls-refuse-at-some-deltas` (P0, design), refusals away from the crease. (NURBS orchestrator)
- 2026-10-21 — **Seam (NURBS):** PR 5513 merged (`9866210b`). It adds one row to `scripts/gates/from-f64-sites.tsv`: a `from_f64` site in `topo::census` (the census's charge rate, `rounding_charge(1)`), a constant tolerance per the sites policy. `from-f64-allowlist.sh` passes. (NURBS orchestrator)

- 2026-10-23 — **Seam (QUAD):** PR 5589 (`quad/measure-a-lanes`, measure door unit A) edits `scripts/gates/bounds-allowlist.sh`: the quad_lane pin goes 16 → 18 for the interim adapter's `cut_face_lane`, with the argument in the file. (QUAD orchestrator's lane)
- 2026-10-26 — (tracker sweep) Read every live row against main `bb25fa4f`.
  - **Fixed and closed here** (gate edits in `scripts/gates/`, each run with `--selftest` and plain, the new cases mutation-checked): `loop-boundary-selftest-cannot-plant-an-arm-fragment` (planter gains an `arm-fragment` form; the selftest was red on main) and its duplicate `loop-boundary-discards-selftest-fails-on-a-clean-tree`; `loop-boundary-discards-loses-the-enclosing-fn-at-a-semicolon-in-its-signature` (a `;` clears the pending fn only outside the signature's brackets; CIW holds a duplicate row); `interval-square-gate-reads-a-path-prefix-as-a-square`; `probe-run-floor-does-not-hold-the-probe-f64-differential` (floor 1 → 2); `gate-require-file-guards-with-no-gone-case`; prose: `gates-readme-cites-a-deleted-memory`, `register-equal-allowlist-cites-two-helpers-that-no-longer-exist`, `test-aggregation-gate-hint-names-a-guard-body-that-no-longer-exists`.
  - **Closed, superseded or duplicate**: `gate-wiring-fence-is-undrawn-for-the-parity-entry` (the parity checker was deleted in `24fedbfadf`; `ci.yml` loops every gate); `no-ambient-env-does-not-match-current-dir` (merged into `no-ambient-env-gate-cannot-see-temp-dir`); `gate-directory-counts-in-prose-go-stale-on-every-new-gate` (`gate-roster.sh` is deleted; plan.md's counts removed).
  - **Opened**: `G4`, whose trigger #1647 merged on 2026-09-03 (re-priced M). **Filed**: `gate-selftests-run-in-no-ci-job`, the arm-fragment row's second finding.
  - **Notes added**: `S115` (three of four members gone, (c) stands), `gate-roster-and-probe-census-have-no-reader-guards` (the roster half is moot), `embedded-python-readers-are-copied-between-gates-and-unlinted` (the msrv gate is gone). plan.md drops the parity-fence seam and its directory counts.
- 2026-10-26 — (split) GUARD measured 43 points against 30. Re-cut into three programs, each one kind of defect, rows moved by `git mv` with ids kept and a dated section on each:
  - **GUARD keeps the gate harness** (7 rows, 14.5 points): `lib.sh`'s Rust reader and `--skip-cfg-test` cut, the selftest affordance and the selftests no CI job runs, the probe census's unguarded stages, and the copied readers (embedded Python, the three bound-list walkers). `program.md` and `plan.md` rewritten for it; territory stays `scripts/gates/*` whole.
  - **To BLIND, new** (`work/blind/`, 12 rows, 16.5 points, top P2): the discipline gates' matchers and the spellings they miss — `reporting-margin-gate-misses-…`, `self-dot-has-no-gate-…`, `the-bit-identity-consumer-gate-…` (the three P2s), `G4` with `D212` riding it, `S210`, `S41`, `census-bracket-read-gate-…`, `register-equal-allowlist-exempts-…`, `no-ambient-env-gate-cannot-see-temp-dir`, `bounds-allowlist-fires-on-a-binding-pattern-…`, `tour-scalar-trait-bundles-…`. Not a P2-only band: three E rows are too thin to charter, and they are the same work as the P3 widenings beside them.
  - **To UNREAD, new** (`work/unread/`, 9 rows, 12 points, top P3): claims with no gate at all — `measurements-have-no-mechanical-guard`, `viewer-disabled-control-population-has-no-gate`, `raw-door-compile-proof-needs-a-gate`, `comment-symbol-names-…`, `door-table-reason-strings-…`, `vocabulary-macro-bodies-…`, `feature-gated-lib-unit-tests-…` (unpriced, charged 2.5), `inserting-an-item-above-…`, `S115`.
  - Not moved into CIW or S-TINT, whose charters fit two rows (`gate-selftests-run-in-no-ci-job`, `feature-gated-lib-unit-tests-…`): both are over budget. Path citations of moved rows repointed in `crates/` comments and in other programs' rows; earlier log entries are left as written. (GUARD split)
- 2026-10-26 — **Seam (QUAD):** measure door unit C (`quad/measure-c-no-default`, PR 5741) touches this program's ground. Every public number door takes a required predicate (`topo::Settle`: `Sign`, `AtMost`, `AtLeast`, `Contains`, `Resolution`) and answers `topo::MassMeasurement { enclosure, stop }`; a number is read through `MassMeasurement::met()`, which answers the enclosure where the resolution was met and a typed `Unmet` where it was not, and an unmet resolution is a bracket with `stop` `Floor`/`Wall`, not a refusal. A deciding predicate still undecided where no face can refine is the one quadrature refusal, `MassPropsError::Undecided { volume: (lo, hi), stop }`. An import hands out `StepImport::Solid { body: topo::CertifiedBody, .. }`: the body held with the gate's certificate of it (`body()`, `into_body()`, `measure(settle)`). `PropsError::QuadratureBudget`, `TargetUnreached`, `HeldCertificate`, `PropsCheck::Converged` and `QUAD_TARGET_LEN_FACTOR` are gone. Tests reading a number use `topo::test_support::pinned_volume(what, body, tol)` (1024·ε, asserting the resolution met). (QUAD unit C lane)

## A note from GREEN (2026-10-28) — rows re-homed or re-banded

GREEN (`work/green/`) opened with a priority-seam cut across the infra
group and took this program's green-main or gate-latency rows by
`git mv`, ids unchanged; `work/green/log.md` lists them. Any P1 left
here was re-banded where it was test-side rather than production
architecture; the row's own `## Re-banded` section says why.
