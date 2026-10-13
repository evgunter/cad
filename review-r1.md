# SYM-17 Phase 2 — review R1 (PR #4197, frozen head `a89f7a54eb`)

**Verdict: APPROVE-WITH-FIXES.** The design is built as Ev ruled, and the type is sound. Every measurement re-takes to the PR's numbers. One gate is red and undisclosed (MAJOR-1), and it must be fixed before merge. The rest are doc and test-gap fixes.

## Findings
- **MAJOR-1 — `doc-tests` is red, and the PR body says CI is green but for one ε row.** `docs/GUIDE.md:551,600,656,685,758,798,848` and `docs/guide/fail-loud.md:149` still spell `OrthoFrame::axes_xy`. They are doctests (`crates/pncad/src/guide.rs:25,37`), and the prelude no longer exports the name (`crates/pncad/src/prelude.rs:153`). Hosted run 38323491246, job `test`, step `doc-tests`: 8 failed, E0433. `gate ok` is red for two reasons, not one. The "Local checks" list never ran `cargo test --doc`. *sure; demonstrated* (hosted log, and locally `cargo test --doc -p pncad`: the same 8 fail).
- **MINOR-1 — at f64 and Interval, what the witness means is false for door-admitted motions, and `compose`/`inverse` compound the gap without bound.** `rigid_motion.rs:21-24` says "at `f64`, rigid up to rounding" and "at `Interval` … ONE rigid motion per point". `check_rigid` admits a map within the band: a dimensionless 1e-9 at the default ε, and 1e-6 on the ε=1e-6 row. Products are not decided again (Ev's item 6), so the defect grows with the number of factors.
  - Probe `r1_an_admitted_defect_compounds_under_compose`: twenty squarings of an admitted 4e-10 stretch give a 4.2e-4 stretch. The door refuses it (`NotRigid col0_unit`), but `transform_rigid` moves a unit cube to an x extent of 1.000420, Ok.
  - Probe `r1_m_then_its_inverse_…`: `m ∘ m⁻¹` of the admitted stretch is `Undecided` at the door that admitted `m`.
  - A document reaches this through a chain of literal steps: `placement.rs:877` `chain_motion` → `Frame::rigid_motion` → `Motion::compose`. On `main` the chain's composite was decided once, at `transform_rigid`; now only each step is. The same holds for `transform.rs:12-16`'s "the type is what keeps [a uniform scale] out".
  - Ev's ruling covers not re-deciding. The claim, or a bound on a chain's defect, is what is off. *sure (doc false); demonstrated.*
- **MINOR-2 — at `Interval` the mints do not decide finiteness, and an overflow now refuses as a kernel defect.** `is_finite_length` waves an enclosure through (`rigid_motion.rs:28-31` discloses this), so `RigidMotion::<Interval>` `MAX∘MAX` mints with origin `[MAX, inf]`. `transform_rigid` then refuses `Certify{Escalated carrier_endpoint_start [0, inf]}`. On `main` the body door refused typed `NonFiniteMap` naming the component. The type's "every number finite" (`:3`) and the brief's "each decides finiteness" hold only at the point scalars. *sure; demonstrated* (`r1_interval_translation_finiteness`; `translation(+inf)` itself still refuses).
- **MINOR-3 — test gap (Q3).** I planted three edits in `geom-core` (`rigid_motion.rs`), and the module's own rows stayed green under each:
  - A2: `inverse` stops deciding finiteness (`:648`).
  - A3: `rotation_then_translation` stops deciding the translation (`:580`).
  - E: `inverse`'s entries widened 21× by dependency (`c + 10(c−c)`). The Interval inverse row (`:1263`) only asks that the result enclose the identity, so it gets *easier* as the result degrades.

  A plant on the mints as a whole (A), on the door admitting undecided (B), on folding undecided back into NotRigid (D), and on skipping `col01_orth` (F) each reds a row. I did not run the other crates under A2/A3. *sure; demonstrated.*
- **MINOR-4 — the K-REPORT addendum is contradicted by the PR's own counts.** `docs/K-REPORT.md:3042-3043` says "the population shrinks … no rule can fire on a population that only shrinks". But an instance's literal offset is now decided at the door on each of the group frame's two reads (`asm2a_instantiate` 466→476 net; `kstats` +10 per instance), and each mate side's literal is newly decided (`OWN_FRAMES` 2→22). The `transform_rigid_*` population grows on assemblies. *sure; by inspection plus the PR's counts.*
- **NOTE-1 — stale "transform_rigid checks rigidity" prose (Q4, one class).**
  - `crates/topo/src/instance.rs:33` ("re-checks rigidity with decided predicates");
  - `crates/geom-core/src/linalg/frame.rs:764` (test doc: "residuals `transform_rigid` decides on");
  - `docs/DRAFT-DESIGN.md:73` ("rigid by checked contract").

  *sure.*
- **NOTE-2 — two deviations are unscheduled (Q6).**
  - The revolve's decided axis (`sweep/src/revolve/axis.rs:141`, `revolve_placed_axis`) cites PATHS, but `work/paths/the-sketch-plane-is-its-frame.md` does not name it, and its `turned_between` is for `sweep_places` only.
  - The boolean re-cut's "by-construction route is the unbuilt `turned_between`" (`topo/src/boolean/ops.rs:5895`) has no item.

  *likely.*
- **NOTE-3 — the `Undecided` folds are each right.**
  - `Frame::admission_fault` (`placement.rs:342`) folds `Undecided` into `FrameFault::NotRigid`, whose sentence already says "not definitely rigid … it may …". That is honest, and the PR discloses it.
  - The boolean re-cut (`ops.rs:5897`) folds every refusal into `corrupt`, as `main` did through `transform_rigid`.
  - `wire.rs`, `placement.rs` (`Motion::rigid`, literals) and `step-import` (`PlacementMap`) keep `Undecided`.

  *sure; by inspection.*
- **NOTE-4 — the dispatch's premise is half wrong.** "Green except one ε row" is false: see MAJOR-1. The ε=1e-12 row itself does fail on `main` too (on `main` `0fdbf00755` at `CAD_TOLERANCE_EPS=1e-12`: FAIL, "evaluated: None"). *sure; demonstrated.*

## Claims
1. **Soundness of the mints.** At every scalar, each mint encloses the true rigid motion. Nothing public builds one otherwise: the fields are private; `by_construction`, `cross_of_orthonormal` and `from_aim` are `pub(in linalg)`; there is no serde, no `From`, and no trust door. I swept every `from_linear`/`by_construction` site. Rodrigues over an un-renormalized witness axis is a rotation at every point of the box. `compose` and `inverse` (the transpose) enclose the products and inverses of rigid maps. The caveats are MINOR-1 (door-admitted motions) and MINOR-2 (Interval finiteness). I did not probe Dual by execution: tangent transport holds by inspection, because `UnitVec3::new` and `turn` keep `n·n′ = 0`. *likely.*
2. **The door.** It is `main`'s predicate, unchanged: seven margins, order, names, F10 flag, the three translation probes. The only change is that `Err` maps to `Undecided` (plants B and D both red). *sure.*
3. **Nothing `OrthoFrame` promised is lost.** The accessors, the three decided mints, the world frames and `to_affine` carry over. `from_aim` is still sealed. The ported tests differ only in names (`git diff -M`). The leftover name `ortho_frame_error_tag` is under Style. *sure.*
4. **Re-baselines.** The counts are consistent: 10 per door × 2 reads − 1 composite = +10, and `OWN_FRAMES` is +2×10. The golden's mover is the `Transform` node. The rows moved to the door, and the Python tags were reviewed. All are said, but chaintol's header is red-free only because the doctest gate wasn't read (MAJOR-1). *likely.*
5. **Measurements.** These match, below. *sure.*
6. **Phase 1.** On `main` the first refusal at σ=0.01 is `col0_unit` ("may scale an axis"), and the bisected wall is `col01_orth` at 1.666e-10, at one link and at four, against the prediction ε/6 = 1.667e-10. Phase 1 holds. *sure.*
7. **Deviations.** Each is an improvement or neutral, except the two in NOTE-2. The mate-side stop rule holds: `intent/s3-b-…` rewrites `mate/solve.rs` (+885/−1355). It is filed on INTENT, P2. *sure.*

## Style
- **Q1.** `crates/viewer/src/display.rs:603` `is_rigid` is a second spelling of the seven margins against a raw slack. It predates this PR and is filed as `work/offer/viewer-free-move-decides-rigidity-by-its-own-predicate.md`, but it contradicts the "one home of that rule" sentence (`rigid_motion.rs:111`). *likely.*
- **Q1.** The retired name lives on in `pncad-py/src/tags.rs:892` `ortho_frame_error_tag`, `py/place.rs:184` `ortho_frame_err`, and `work/paths/the-sketch-plane-is-its-frame.md` (title, `OrthoFrame::turned_between`). *sure.*
- **Q1.** `revolve/axis.rs:141-146` maps the placed-axis decision onto the sketch axis's own variants (`DegenerateAxis`, `AxisEscalated`). Two different decisions read as one refusal. *likely.*
- **Q2/Q5.** The module doc lists `turned_between`, `translated`, `embed` and `pin` among the mints (`rigid_motion.rs:81-104`), but none of them exists. It is ratified-but-unbuilt text, read as a contract. *sure.*
- **Q7.** One literal frame is decided twice per instance, once for the content key and once for the op (`asm2a_instantiate.rs:410`). The log records the same decision twice. *likely.*
- **Q7.** `swept.rs:81-86` holds one rotation as two motions (`pivot`, `anchored`), so the point-turn spelling and the placement spelling stay distinct arithmetic. *unsure* whether that is wanted.
- **Q3.** chaintol's symbolic arm drops its "isometry wall is GONE" assertion on the grounds that no transform node can raise it. A `Transform` node with a literal step still can. On this chain that is true. *unsure.*
- **Q8.** I read `rigid_motion.rs` whole (1441 lines). Two test modules (`tests`, `motion_rows`) re-declare `band()` and `bits12` side by side (`:964`, `:1198`). *sure.*

## Code quality report
- **Counts.** MAJOR 1, MINOR 4, NOTE 4. Deviations: 7 reported, 1 silent (the red doc-tests and the CI statement).
- **Idiom/structure: 4.** The witness is cleanly sealed, and the mints are thin over `Mat3`'s Rodrigues.
- **Test quality: 3.** Plants A/B/D/F red, but A2/A3/E survive, and the compounding gap has no row.
- **Doc/comment honesty: 3.** The CI statement, "rigid up to rounding", "population only shrinks", and stale "re-checks".

## Questions exercised
Q1 (prose grep `verbatim|mirror of|…` plus a constants grep for `dot(…)-1`, `determinant()-`, `is_rigid`), Q2, Q3 (seven plants), Q4 (prose sweep for `NonFiniteMap|re-checks rigidity|checked at the door|transform_rigid … rigid`), Q5, Q6 (issue files read), Q7, Q8.

## Gate verification
Hosted run 38323491246 has two red steps, not one:
- ε=1e-12: `coincidence_door::a_ring_serving_two_pierces_records_their_fusion_unproven`. It reds at head locally ("evaluated: None") and on `main` `0fdbf00755` (release, the same text). It is not this PR's; it is filed four times on FLUSH.
- `doc-tests`: MAJOR-1. It is this PR's.

## Run locally (4 vCPU)
- `cargo nextest run --workspace --profile ci` at head plus probes: **13599 passed**.
- `cargo test --doc -p pncad`: **8 failed** (MAJOR-1).
- The ε=1e-12 row: head FAIL, main FAIL, the same "evaluated: None".
- Chain, one leaf, plain Interval, release:
  - PR "before" 1.666e-10 / 1.677e-10 (`col01_orth`), mine 1.6660e-10 / 1.6688e-10, at one and four links;
  - PR "after", 1 link: 6.49e-9 / 6.54e-9, mine **6.508e-9 / 6.519e-9** (`pcurve_envelope` `FidelityU`);
  - PR "after", 4 links: 6.73e-10 / 6.78e-10, mine **6.758e-10 / 6.770e-10**;
  - at σ=0.01 the first refusal is `carrier_endpoint_start` `[0, 8.9988e-5]`, with 8/15/24/35 refused, as the PR says.
- Symbolic, `symbolic_zero` / `frozen`, before → after: 2008→1978 / 2804→2626, 3014→2934 / 6870→5654, 4069→3929 / 10684→8516, 5124→4924 / 14498→11378. Exactly the PR's. Verdicts unchanged. Costs 0.24/0.41/0.58/0.82 s on this box.

## Probe commit
`sym/17-review-r1` `4debacac6`:
- `crates/topo/tests/review_sym17_r1_probes.rs` (3 rows);
- `demos/tour/src/chaintol.rs`'s ignored `r1_reach_probe` (bisection) and `r1_chain_rows_probe`.

The plants were run in a scratch worktree and are not committed; each is described in MINOR-3.
