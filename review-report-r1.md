# Review R1 — PR #1101 (GUI-3: the document panels), frozen head 956ef3cf

**Verdict: APPROVE-WITH-FIXES** (conditional on hosted green at the merge head).

Probes: `crates/viewer/tests/review_gui3_r1.rs` (9 rows, green),
`crates/viewer/examples/r1_e2e.rs` (full consumer walk, green),
`review-probes/gui3-r1-mutants.md` (4 mutants + measurements). All on
branch `gui/gui-3-review-r1`.

## MAJOR

- **M-1. Dragging a document parameter commits one edit per frame** —
  `crates/viewer/src/app.rs` `properties_ui`, `Selection::Param` arm: the
  `DragValue` maps bare `changed()` to `SessionOp::SetParam`, so a continuous
  drag emits a committed `SetDocParam` (one undo step, one re-evaluation) per
  changed frame. This violates G1's ratified preview-vs-commit rule ("exactly
  one committed DocEdit on release — one undo step"), which `slot_ui` in the
  same file implements correctly with the drag_started/dragged/drag_stopped
  triple ten lines away. The affordance's "edit the parameter" navigation lands
  the user exactly on this widget, so it is a primary path, not a corner. The
  session vocabulary has no gesture arm for document parameters — the fix needs
  one (or the triple mapped onto SetParam). Class, not instance: any future
  `DragValue` in app.rs needs the triple and nothing enforces it.

## MINOR

- **m-1. A user Cancel lands the canceled prefix, contradicting the seam's own
  docs.** `evalseam.rs` module doc: "This crate discards that prefix … the
  session shows the last landed result." In fact `DocSession::land` checks only
  the generation, and a `CancelEvaluation` run answers with the CURRENT
  generation — so the canceled partial LANDS, replacing the previously landed
  complete evaluation (pinned in `r1_a_user_cancel_lands_the_canceled_prefix…`;
  e2e: diefillet cancel leaves 27/50 rows Unevaluated, and `sync_scene` then
  fails `NoProduct` into the status line while the stale picture stays up). The
  behavior may even be the right one; the docs and PR body describe a different
  one. Also `Generation`'s "a cancel and its restart share a generation"
  describes a restart no code path performs.
- **m-2. "Replaces any queued one" is inline-only.** Measured (r1_e2e §6): two
  back-to-back ThreadEvaluator submits answer TWO results —
  `[(gen0, Canceled), (gen1, Completed)]`. Nothing is replaced; the superseded
  job runs trivially canceled and its result is generation-discarded upstream.
  Session-level behavior agrees between lanes; the `EvalService` trait doc and
  the PR body state the inline shape as the seam contract, and the single CI
  thread-lane row (one submit) cannot see the difference.
- **m-3. Dragging a driven slot ends the frame with a Debug dump in the status
  line.** BeginGesture is refused with the affordance, but the same frame's
  PreviewGesture/CommitGesture are refused `NoGesture`, and each `perform`
  overwrites `status` — so the ratified affordance wording is displaced by
  `NoGesture`. (The inline "driven by …" row text remains visible, hence MINOR.)
- **m-4. The ε re-stamp row's "measured, not assumed" is satisfied by
  construction.** `doc_io.rs` `gallery_ring_at`'s partition/zip assertions check
  the row's own line-mapping (they cannot fail for the interesting reason); no
  row regenerates at another ε and diffs. I re-took the measurement out-of-band:
  regeneration at witness ε is byte-identical to the fixture; at
  `CAD_TOLERANCE_EPS=1e-12` exactly the `"epsilon"` line differs. Claim true
  today, unguarded tomorrow if the format grows a second ε-dependent byte that
  `load` tolerates.

## NOTE

- n-1. `tree::rows` iterates the LANDED evaluation's order, so a node added
  while busy would be absent (not Unevaluated) until its run lands. Unreachable
  through today's `SessionOp` vocabulary (no insert/delete op); flag for GUI-4+.
- n-2. Frame-state inventory omissions: `status: Option<String>` (retained,
  written by panel ops) and GUI-0's `pending_fit`/camera/input. None shadows a
  document field; the inventory's conclusion stands, its "complete" is slightly
  overstated.
- n-3. `ThreadEvaluator::spawn` `.ok()`s a thread-spawn failure: the app would
  silently never evaluate (busy never lights). Fail-loud would surface it.
- n-4. `describe()` renders non-affordance refusals via `{:?}` — user-facing
  Debug dumps of `Io`/`Parse`/`NoGesture` payloads.
- n-5. PR body: "new transitive crates … the wayland-* trio" — the wayland
  crates were already in the lock via winit; actual additions are rfd,
  pollster 0.4.0, block2/libc edges. Conservative direction, cosmetic.
- n-6. Mutant M2 (undo forgets `active_child`) is GREEN across all suites:
  `commit` already stamps the active chain along every path the v1 chrome can
  walk, so "undo records which child it left" is mechanism, observable only
  once GUI-6 jumps the cursor. Not a suite defect; a wording caution.

## Style (per the style-lane brief; questions exercised: Q1–Q8, Q2's blame check N/A on all-new code)

- S1 `props.rs:50` (likely): `SlotValue{Continuous,Count}` is a third spelling
  of the value dichotomy beside `DocParam{Continuous,Count}` and
  `Expr::literal/count`, with converters both ways (`param_edit`, `param_rows`).
  A new value class needs three touches. Constants grep found no duplicated
  literals; prose sweep (verbatim/mirror/kept-in-sync) clean over the touched area.
- S2 `lib.rs:1-37` (sure): the crate doc still reads as if navigation were the
  whole subject — `Camera`/`CameraOp` named, `DocSession`/`SessionOp`/undo/seam
  unmentioned. True but stale in emphasis; written at GUI-0, not re-read here (Q5).
- S3 `app.rs` (likely): the two DragValue mappings diverged in one file (the
  M-1 class). Nothing enforces the triple on the next widget.
- S4 `history.rs:177` (sure): `is_empty` == "holds only its root" (len ≤ 1) —
  documented, but the name fights the arithmetic; clippy's len/is_empty pairing
  forced an odd meaning.
- S5 `demos/tour/src/gallery.rs:11` (sure): doc typo "(`memos`: …)".
- S6 `props.rs:144-149` (likely): `slot_row` silently drops a slot `slots()`
  lists but `expr()` denies (`filter_map`). If `slots()` is authoritative the
  arm is dead; if not, a quiet tolerance in a fail-loud codebase.
- S7 `session.rs:529-544` (unsure): a no-move CommitGesture / preview-free
  CancelGesture still `request_eval`s an unchanged document — busy flickers, a
  generation is spent, nothing changed.

## Claims disposition (one line each)

1. UNDO TREE — **verified**; mutants M1/M3/M4 go red on the shipped rows, M2 equivalent (n-6); subtree retention, mid-path save, byte-stable open→save, `replayed` step-undo all re-derived in my suite.
2. EVALUATION SEAM — **verified with wording defects** (m-1, m-2): per-job token construction is sound (submit cancels only already-superseded jobs, so cancel-during-drain is honored — confirmed by construction and by a live diefillet mid-flight cancel, honored in 10 ms); memo lives in the worker/evaluator, nothing above the seam hands a prior back; `land` is public and rejects past AND future generations; thread lane in CI is one single-submit row — my e2e drove edits/cancel/gesture/double-submit on it.
3. EDIT EMISSION — **verified for slot gestures and single edits; falsified for param drags (M-1)**; replay rows do assert on emitted edits.
4. EXPRESSION AFFORDANCE — **verified**: `ExprKind` is `pub(crate)`, no operator identity crosses the API (sure — no unparser is writable today); both text-door directions pinned, plus my literal→expression→refuses-numbers walk.
5. TREE BADGES — **verified**: payload-byte pins (M4 red); real assembly workspace opened — 7 typed `InstantiatePart` refusals render as Failed with the resolver-absence message, 2 part docs clean, no crash.
6. GALLERY — **verified**: exactly {assembly, checks, diefillet, heatsink, ring} author documents, `scalar.rs` is the trait module; sizes/counts match (4+6); ring fixture byte-identical to regeneration; ε re-measurement confirms the one-line claim (m-4 for the row's self-check).
7. DEPENDENCY — **verified**: rfd 0.17.2, 2026-01-12, MIT (crates.io); default-features off + wayland/xdg-portal; no gtk/glib/gobject-sys in the lock; no copyleft in the added set; wasm guard excludes viewer already (n-5 on the PR's transitive list).
8. CI — **verified at step level**: run 33109314338 (lane=default, eps=default): `clippy` job ran the verdict step AND `clippy (viewer app feature)` (81 s, success); red run 33107827538 = exactly the two reported causes (viewer doc_io `ToleranceConflict{1e-12, 1e-9}` — own, fixed at head; `census_g2_carrier::the_band_edge…` — #1102, main's); demos lockfile diffs are the bvh+mesh editor-core edges from GUI-1, cargo-shaped, no hand edits.
9. SEAM-FRICTION GO — **inventory substantively confirmed** (no per-widget shadows, no id→value maps, no editing/dirty flags, no retained tree, no interior mutability — swept; `tree_rows`/`slot_rows` pure), two benign omissions (n-2). **GO on egui stands**; M-1 is a widget-mapping defect, not seam friction — if anything it evidences the PR's own "DragValue conflates dragging and typing" friction note.
10. VIEWER-CI GAP — **confirmed against ci.yml**: the `clippy` job (verdict + app-feature step) carries `lane != 'interval'`, so an interval draw skips the seed gate wholesale and silently; the red run 33107827538 WAS an interval draw — live demonstration. The doc-gate half is already every-lane (the `fmt` job). **Fix shape recommendation**: host the toolkit verdict + `clippy -p viewer --features app` in an every-lane job — the `fmt` job is the natural seat (already every-lane, already compiles the toolkit graph for rustdoc when seeded, already prints a verdict) — or a small dedicated job keyed only on `run_viewer_toolkit`; do NOT lift the lane condition off the whole default-clippy job (that spends the lane draw's savings).

## E2E scope/ergonomics

Full walk in `r1_e2e.rs` against the real `demo-tour gallery` output, on the
THREADED seam: open ring → tree (Profile/Datum/Revolve▸root, clean) → slot
inspection → two edits → undo → sibling edit → undo/redo → save (2 path edits)
→ reopen; diefillet mid-flight cancel honored in 10 ms, seam recovers, full
run lands 967 ms; heatsink 5-preview gesture → 1 commit → 1 undo step; checks
open→save byte-identical to the gallery original; assembly workspace: typed
refusals throughout, no crash. NOT exercised: the rfd dialog itself, pixel
painting, the `app`-feature widget mapping (which is where M-1 lives — found by
reading, not by replay; the excused surface is exactly where the one MAJOR
hides, which is worth the program remembering). Ergonomic notes: finding an
editable slot requires probing `slot_rows` per node (no "editable slots of the
document" query — fine for a GUI, mildly awkward for consumers); after a
cancel, resubmission needs any edit — there is no "re-evaluate" op (the e2e
nudged a parameter to its own value, which works but reads odd).

## Code quality report

Deviations: reported 3 (no expression pre-fill — substrate-limited, real;
plan's scene list corrected — verified true; viewer-wasm compile unguarded —
pre-existing, GUI-5's) vs silent 1 (M-1's param-drag mapping; the PR's
preview/commit prose implies all drags are gestures). Counts: 1 MAJOR,
4 MINOR, 6 NOTE, 7 style.

- Idiom/structure: **5** — one op door (`perform`), one commit door, one
  submit, ops-queue drain pattern; boundary discipline is structural, not
  conventional (session.rs:589-614).
- Test quality: **4** — tree-shaped assertions that mutants confirm
  discriminating (M1/M3/M4 red); docked one point for the re-stamp row's
  by-construction self-check (m-4) and the thread lane's single thin row.
- Doc/comment honesty: **3** — module docs are unusually good, but two seam
  sentences describe behavior the code does not have (m-1, m-2), and the
  "measured, not assumed" claim points at a self-satisfying assertion (m-4).

## Isolation / process

No look at `gui/gui-3-review-r2` or any other review lane, scratchpad, or
CI artifact; no MODEL-AB-LOG or A/B material read; no glimpses to disclose.
Nothing posted to the PR. Local runs used the build-slot wrapper, worktree
target only; no detached/background waits were started, none to cancel.
Local runs at 1e-12 were `-p viewer` scoped and never touched #1102's row.

Tokens: ~260K. Wall-clock: ~1 h 20 m (single session, no interruptions).
