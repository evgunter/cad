# Review R2 — PR #1094 (GUI-0, `viewer` scaffold), frozen head f511a00

Verdict: **APPROVE-WITH-FIXES**, conditional on hosted green at f511a00
(which run 33094649160 already shows). The fixes are the MINOR rows below;
none breaks the unit's own scope.

Reviewed: PR body + full diff; GUI-0-SPEC; GUI-PLAN; GUI-DESIGN G1/G3/GQ6;
GQ6-RESURVEY §§1–2, 5; reviewer-style-lane brief; review-and-dependency-policy;
test-suite-cost. Probes: `review-probes/gui-0-r2-mutants.md`. Consumer suite:
`crates/viewer/tests/review_gui0_r2.rs` (7 rows, fuzz-harness seeds/effort,
registered in `tests/all.rs`; `test-utils` added as dev-dependency).

## Findings

**MAJOR** — none.

**MINOR-1 (gate surface).** The new façade door `pncad::tolerance::witness()`
is the first spelling of the ambient-tolerance read that
`scripts/gates/witness-not-ambient.sh` cannot see: the gate scans
`grep -F 'Tol::witness'`, and the new door's call sites do not contain that
literal. Any crate above `pncad` (today: `viewer`'s own library modules) can
now mint a witness ambiently without the gate firing — before this PR every
spelling carried the scanned literal. The tree today is clean (only
`src/bin/viewer.rs` calls it; `tol: Tol` is a parameter everywhere else in
`viewer`), so this is a latent enforcement hole, not a live defect. Fix: teach
the gate the façade spelling (e.g. also grep `tolerance::witness`), or take
the PR's own named alternative (exclude `crates/*/src/bin/`).

**MINOR-2 (claim attribution / test shape).** Mutant 3 (drop `/ aspect` in
`projection_matrix`) leaves **all 13 shipped camera_ops rows green**,
including the framing row the PR body credits with "catches a broken
projection". The containment assertion is structurally one-sided: the fit
backs off by the *smaller* half-angle, so scaling the wider NDC axis by
`aspect` can never push a fitted scene outside [-1, 1] at any aspect. The
break IS caught — by `a_pan_keeps_the_point_under_the_cursor` — so the suite
pins the projection, but not where the PR body says. Fix: none required
beyond knowing it; my `framing_fits_random_boxes_through_the_projection` row
does not close this either (same one-sidedness), the pan rows are the guard.

**MINOR-3 (silent misfit).** `Camera::framing` / `CameraOp::Frame` accept any
positive finite aspect, but below aspect ≈ 0.023 the required distance
exceeds `MAX_DISTANCE_FACTOR` and `Camera::new` clamps it silently, so the
"fit" no longer contains the scene and nothing refuses (measured: worst
|ndc| = 4.79 at aspect 0.005 — table in the mutants file). Against this
crate's own refuse-rather-than-degrade posture. Related: the app's Fit button
and startup framing pass hardcoded `aspect = 1.0` rather than the pane's real
aspect (crates/viewer/src/app.rs:122,196) — exact for panes wider than
square, over-full for narrower ones.

**MINOR-4 (doc claims a type cannot deliver).** `input::map_stream`'s
`# Errors` says "the camera at that point is the last good one", but the
`Err` arm carries no camera — the caller cannot recover it
(crates/viewer/src/input.rs:195). Reachable: a NaN drag delta maps to a
refused op.

**NOTES.** (a) `Camera::new` doc: "every other argument is taken as given and
refused if it is not usable" — distance is silently clamped into the band
(camera.rs:242). (b) `far()` doc says "the back of the bounding sphere"; code
is `distance + 2·radius`, one radius beyond the back (camera.rs:390). (c)
`CameraError::UnusableBounds` is reused for non-positive *aspect* in
`fitted`/`projection_matrix` — the arm's own doc speaks only of boxes.
(d) `POLE_MARGIN = 1e-3` is restated as a literal in
tests/camera_ops.rs:159 — a hand-synced copy of a private constant
(defensible as a pin; undisclosed as one).

## Claims to falsify — disposition

1. PARTIAL: winding mutant killed by the volume row; lost-fov mutant killed by
   the pan row; dropped `/aspect` mutant SURVIVES the framing row (MINOR-2)
   and is killed by the pan row.
2. VERIFIED: Doc → evaluate → product → tessellate, no hand-built geometry
   anywhere; my independent suite re-pins the authored dims axis-by-axis and
   bounds the enclosed volume by a first-principles chordal bound — green.
3. VERIFIED: no egui/wgpu type in layer-3 signatures (grep + read); no arena
   keys (grep for key types clean; `RecipeNodeId` is document vocabulary);
   typed refusals with correct arms under random planted refusals;
   fold-stops-at-first-refusal holds. Note: the app's inline event loop
   skips-and-continues on refusal rather than stopping — different semantics
   from the tested `map_stream` (see Style).
4. VERIFIED: `ci-filter.py`'s `_closure` is a reverse-dependency walk, so
   `viewer` (downstream of `pncad` → everything) lands in scope of nearly
   every kernel PR; step-level jobs API shows step 7 "clippy (viewer app
   feature - eframe + wgpu)" conclusion=success, 16:45:31→16:46:46 (75 s) on
   the PR's run — not job-name green.
5. VERIFIED EXACTLY against runs 33094649160 / 33092409948: whole run
   833→709 s, build+archive(default) 638→580 s, clippy 143→218 s,
   fmt+rustdoc 250→316 s; each of the two jobs crosses one minute boundary
   (2.4→3.6, 4.2→5.3 min) = +2 billed; test/archive chains untouched.
6. VERIFIED with the MINOR-1 caveat: `witness()` is one kernel call, no
   behavior; the gate header genuinely lists `crates/pncad/src` as unscanned
   "the place a program starts" — an honest reading — but the new spelling is
   invisible to the gate's grep (the finding).
7. VERIFIED against crates.io on 2026-08-27: egui_tiles 0.17.0 = 22 d
   (0.17.1 = 9 d, correctly refused), egui/eframe 0.36.1 = 20 d, wgpu 30.0.0
   = 57 d, bytemuck 1.25.2 = 39 d; egui_dock is MIT-only; self_cell 1.3.0
   `Apache-2.0 OR GPL-2.0-only`; epaint_default_fonts as stated. Independent
   census over the app graph (182 packages): the self_cell GPL alternative is
   the only copyleft-family token; nothing copyleft over our code.
8. VERIFIED: hosted wasm step green with the exclusion (step-level); locally
   `cargo check -p viewer --target wasm32-unknown-unknown` fails inside
   `getrandom` via `pncad` — the stated dependency-graph reason is true.
9. VERIFIED: `ViewerApp` holds only Doc/tol/δ/derived scene/revision/camera/
   input map/tiles tree/status; nothing mirrors Doc or Camera; zero interior
   mutability in the crate (grep for RefCell/Mutex/Cell/unsafe: clean). The
   GUI-3 qualifier is honest framing.
10. VERIFIED: `cull_mode: None` documented at the site with the reason; the
    volume-sign row catches reversed winding with no GPU involved (mutant 1).

## Style (per the style-lane brief; questions exercised: Q1–Q8)

- **Q1 (sure):** `camera::fold` and `input::map_stream` are consumed by no
  shipped code — only tests; `viewport_ui` re-implements the same loop inline
  with *different* refusal semantics (record status, keep going vs stop at
  first refusal). Three spellings of "fold events through the camera"; the
  tested one is not the one the app runs. Where else to look: any future
  event-replay consumer will pick one of the three. Prose sweep
  (verbatim/mirror-of) over `viewer`: clean. Constants sweep: the `1e-3`
  restatement (NOTE d).
- **Q2 (sure):** the two longest justifications (cull-mode-off; app-feature
  CI comment) both check out factually — verified rather than trusted.
- **Q3 (sure):** the framing containment rows cannot go red for a lost
  `/aspect` (one-sided; mutant-verified). Also
  `a_finer_delta_never_coarsens_the_mesh` survives reversed winding (abs
  error) — fine only because the winding row owns that claim.
- **Q4 (sure):** the bvh aggregator-header count fix is exactly its own
  file's doctrine applied to itself; no other citations of the "twelve"
  found.
- **Q5 (likely):** module docs match contents except the NOTES above.
- **Q6 (sure):** app-non-default carries its own mechanical guard (the
  unconditional clippy step) — deviation-as-improvement, complete.
  `cull_mode: None` ("one-line change for whoever first runs this on
  hardware") and the epaint Ubuntu-font question ("flagged, not decided
  here") are disclosed but **unscheduled** — no unit or issue owns either.
- **Q7 (likely):** `status: Option<String>` stores `format!("{error:?}")` —
  the typed refusal is flattened to a Debug string at the earliest moment and
  *that* is the retained UI state; the ratified micro-decision wants failures
  as typed values the GUI renders. Presentation-only today; worth re-taking
  at GUI-3. (unsure) `Camera` speaking `bvh::Aabb` couples layer 3's public
  vocabulary to the spatial-index crate's box type; sanctioned by the pncad
  direct-edge ruling, but a layer-3 ergonomics choice someone may revisit.
- **Q8 (sure):** read camera.rs (628 lines, the largest) end to end; new
  crate, no accumulation.

## E2E exercise — scope/ergonomics

Headless driving of the public surface was smooth: `framing → map → apply →
project` composes without adapters; `ViewportSize::aspect() -> Option` is
honest; `DisplayTolerance::scaled` is a pleasant chrome seam. Rough edges
found by driving: the extreme-aspect silent misfit (MINOR-3); a consumer
stitching `plate_with_hole` + `scene_of` handles two disjoint error types
(`SceneDocError`, `SceneError`) for one pipeline. **The eframe/wgpu half
cannot execute here — no GPU or display in this container** — so for that
surface the toolchain/CI commands are the e2e equivalent: local
`cargo clippy -p viewer --features app --all-targets -- -D warnings` clean
(mirrors the hosted step, which ran green at the step level). Nothing
app-side was ever executed, matching the PR's own disclosure; the
maintainer-side `cargo run -p viewer --features app` remains part of the
review.

## CODE QUALITY REPORT

Counts: MAJOR 0, MINOR 4, NOTE 4. Spec deviations: 3 reported (app
non-default feature; cull_mode off; pncad witness door) + 1 spec-sanctioned
and reported (wasm-guard exclusion); silent deviations found: 0.

- Idiom/structure: **4** — private-invariant Camera, typed closed enums,
  pure apply, zero interior mutability, clean feature gating; docked for the
  unconsumed `fold`/`map_stream` twins beside a divergent inline loop and the
  `UnusableBounds` arm reuse.
- Test quality: **4** — the volume-by-divergence-theorem and
  pan-as-property rows pin real contracts (2 of 3 mutants killed at the
  credited row); docked because the framing rows are structurally one-sided
  (mutant 3) and one private constant is hand-restated.
- Doc/comment honesty: **4** — every measured number I re-derived (CI
  seconds, crate ages, dep counts 63/255, licence rows) was exactly right,
  and the not-exercised list is unusually honest; docked for the three
  false-in-detail sentences in NOTES and MINOR-4.

## Process

Lane isolation: no fetch/read of the other review lane's branches or
artifacts (including after being told `gui/gui-0-review-r1` now exists on
origin); MODEL-AB-LOG and the A/B memory not read; no glimpse to disclose.
Local runs were unique-signal (mutants, random-state sweeps, wasm negative
probe, license census); the pinned suites ride the PR gate and the verdict is
conditional on hosted green. No detached waiters left armed (one self-matching
poll loop was killed and is recorded here). Approx cost: ~290K tokens,
~75 min of active review wall clock.

**Interruption disclosure (fair-pair rule):** the review container restarted
mid-review, after the review commit 9e17446 was created locally but before it
was pushed. Wall-clock gap: last pre-restart action ~17:20 UTC, resumed
17:38 UTC 2026-08-27 (~18 min gap, no review work performed in it). All
findings and measurements above were completed before the restart; on resume
the worktree was verified intact at 9e17446, the branch pushed, and the
results re-confirmed post-restart (full 33-row suite green, consumer suite
green at CAD_FUZZ_EFFORT=8 on a fresh seed, `cargo fmt --check` clean). No
in-flight measurement was lost.

