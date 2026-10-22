# FIT — the log

## 2026-09-20 — opened

Cut out of VGEOM, which was carrying 43 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed VGEOM's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

6 rows arrived by `git mv` with their ids, bodies and history
unchanged. VGEOM keeps its band 5300-5399; band 8500-8599 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## A note from CHROME (2026-09-22) — 2 rows re-homed here

CHROME measured 88 budget points against its 30-point ceiling and was
cut along its priority seam (`work/chrome/log.md`, same date), opening
FORMS and OFFER. Per `work/README.md`'s re-homing rule, rows whose
charter fit a live program went to it instead of into a new directory.
Moved here by `git mv`, ids, bodies and history unchanged:

- `a-flat-rung-row-uses-an-absolute-epsilon-on-a-scaled-value`
- `mispaired-ids-exempts-the-empty-window`

The first is a display-fit rung row, beside `a-flat-rung-pair-is-read-as-flat-below-it`. The second is the id/patch pairing guard on the pick seam: VGEOM declined it on its charter test (`work/vgeom/log.md`, 2026-09-21 — *what fails to arrive is an identity, not a figure*), which is this program's pick-index ground and sits beside `id-readback-failure-reads-as-nothing-under-the-cursor`.

If a row does not belong here, say so on this log and CHROME's
successor will take it back.

Signed (CHROME orchestrator).

## 2026-09-27 — seam note from S-DUP (#3304)

#3304 rewrote the rustdoc of `crates/viewer/src/camera.rs`'s `CameraError::UnusableBounds` arm. It dropped a sentence that justified keeping the arm unsplit by "a promoted review suite that pins this arm", which is the reading Ev withdrew.

- **The arm doc.** It is now a per-door list of what a caller can infer from the arm. It covers `projection_matrix`, `ray_through`/`datum_view`, `fitted`/`apply(Frame)` and `framing`, and says which input each one checks first.
- **The first line and `Display`.** They no longer say "positive finite": a non-finite input is `NotFinite` first.
- **Unchanged.** The API is the same, and the arm stays unsplit. Whether to split it is still this ground's call.

Signed (S-DUP orchestrator).

- 2026-09-30 — Seam note from AUTH-14 (`author/edge-name-fault`). The index's loud edge-name arms now reach the author. `pickindex.rs`: `PickIndex::edge_names_in(node, body)` walks a body's drawn edges and answers `EdgeNames { named, refused }`, where `EdgeNamesRefused { first: EdgeNameFault, refused, drawn }` renders through the fault's own `Display`; every refusal from that walk is loud (the ids come from the index's own window), and a body the index does not draw answers nothing and refuses nothing. `EdgeNameFault` gains `Eq`. Test doors: `PickIndex::unname_edge` (cfg(test)) plants the naming layer's refusal; `PickCache::index_mut` (cfg(test, app)). `blend.rs`: `load_all_edges` refuses the whole load with `BlendEvent::EdgesUnnamed { target, refused }` when any drawn edge refuses (was `NoEdgesOnTarget` when all did, a partial set when some did). `marks.rs`: `HeldEdges::segments` is `HeldEdges::mark`, answering the segments and the refusal; `EdgeOverlay::held_refused` carries it. `pane/viewport.rs` writes it per frame into `ViewerBehavior::held_edges_refused`; `app.rs` zeroes/assigns `ViewerApp::held_edges_refused` like `profiles_undrawn` and draws `frame::held_edges_badge` (Advisory, the pick-index seam's subject; `impl SeamSubject for EdgeNamesRefused`). `frame.rs`: `tool_notice` answers `EdgesUnnamed` `Retold::Again`. `gpu.rs`: one test literal gains the field. `test_support.rs`: `plate_indexed(tol)` (the fixture `marks.rs`'s tests held privately). README: the badge population is eleven. Tests: `frame_policy.rs` (badge count 11, the new retold row), `error_display.rs` (`edge_names_refused_forwards_its_first_refusal`), `blend_authoring.rs` (the held-set row reads `mark`). (AUTH-14 implementer)

- 2026-09-30 — Seam note from AUTH-14 (`author/edge-name-fault`), review fixes; supersedes the shapes in the note above. `pickindex.rs`: `EdgeNamesRefused` is `{ node, body, first: UnnamedEntity, named, refused }`, rendered as "the index names N of the M edges it draws on body B of node K; the first it cannot: <EdgeNameFault::Unnamed>"; `edge_names_in` reads the window through `PartWindows::named_in`, so only the unnamed arm can refuse; `EdgeNameFault` does not gain `Eq` after all. `blend.rs`: `BlendEvent::EdgesUnnamed { refused }` (no separate `target`). `frame.rs`: `SeamSubject for EdgeNamesRefused` is `Subject::Document` (the line's `tool_notice` subject too), not the pick-index seam. `test_support.rs`: `unnamed_edge(node, body)`. Filed `work/chrome/the-per-frame-badge-reads-are-three-hand-copied-fields`. (AUTH-14 implementer)

- 2026-09-30 — Seam note from AUTH-14 (`author/edge-name-fault`), third pass. `pickindex.rs`: `PartWindows::in_target` and `named_in` both cut their run through the new `PartWindows::laid_out`, which panics (`unreachable!`, naming the window and both list lengths) when a window runs past the entities or names laid out with it, instead of answering an empty run that reads as a body with no edges; `a_window_past_its_names_is_not_an_empty_body` is its `#[should_panic]` row. (AUTH-14 implementer)

## 2026-10-22 — the budget is a target (PR 5600)

Closes `the-budgets-predicted-count-is-not-always-an-over-count` and
`a-flat-rung-pair-is-read-as-flat-below-it`.

- `TRIANGLE_BUDGET` is documented as a target with a stated
  precision, and `FittedDelta::predicted` is deleted.
- `answer`'s Flat early return is gone, because the law already
  commits the request there.
- `display_budget.rs` holds the drawn count to 0.9–1.05× on every
  bound corpus row. Measured: 0.927–1.0025.

Review was a style lane. All five claims held, including the ~8.1×
Flat bound, re-derived. Its two contradiction findings were fixed in
place: S1, "a factor of at least two", which was false; and S2, the
direction of the probe's error. A second error direction, the
sagitta-sized excess of up to ~1.1%, is now stated beside the
other.

Filed: `a-dense-floor-is-priced-off-the-law-from-the-scale-probe`
(threshold 62 500, the halving break) and
`flat-harmlessness-is-unproven-on-the-nurbs-refinement-lane`.
## 2026-10-22 — taken up; the slate against the D10 hold

An orchestrator (session 01QgNokkLTQtNJ3sdoLiqtf6) takes the track; status `active`.

**The hold.** Other programs hold off starting units that meaningfully
touch dependency, placement or intent (`docs/DESIGN.md`, State of the
build). No FIT row changes how any of those is said, but stage 3 D′
(`work/intent/a-placement-pins-by-written-values-without-a-mate.md`,
`[ev]` PR 5494, open) retires loose spaces and decides what the viewer
shows: a set of spaces drawn through one composing door shared with
export. Two rows stand on that ground and are parked on the D′ row:
`one-failed-root-refuses-the-whole-pick-index` (its walk is
`PickIndex::assemble`'s world-plus-loose-spaces filter, and its question
is the shown set), and the pickindex row's §1, split into
`a-moved-copys-pick-span-crosses-its-frame-unconverted` (it reads
`DisplayView::moved_placements`, free-move over loose spaces). Every
other row was checked live on main at `85136071`.

**Re-priced.** Legacy `D` read and priced `M`. `ctrl-wheel-reaches-no-zoom`
`E` → `M` with `design`: its body names three open questions. The
pickindex row is `M` with `design` (§3 alone). The aspect-sentence row
had no band: `P3`, `E`, riding with zoom-to-fit. Load 30.5 → 25.5.

**Overlap.** No open PR touches FIT's files, and every sibling on the
viewer ground (VGEOM, VSEAM, VNEWS, CHROME, POLISH) is `ready` with no
orchestrator, so no lane is live beside this one. The neighbouring rows
on `pick_for` are named in the plan.

**Next.** The two P0s, each weighed by a designer pair first.
(FIT orchestrator)

## 2026-10-22 — the two P0s weighed and dispatched

Both P0 rows were weighed by a designer pair (blinding on
`analysis/design-fork/fit-p0`). Both pairs converged in their first
reports, and no ratified text moves, so neither is a fork for Ev. The
orchestrator adopted each:

- **`ctrl-wheel-reaches-no-zoom`.** egui's `zoom_modifier` is set to
  `NONE` once, so a ctrl/⌘+wheel is an ordinary `Scroll` at the one
  wheel rate. A pinch (`zoom_delta()`, i.e. every pinch on every
  backend, which the row undercounted) is `ViewportEvent::Pinch
  { factor }` → `Dolly(1/factor)`, with no rate. The pair named the
  variant `Magnify` and `Pinch`; `Pinch` was taken, naming the gesture
  as `Scroll` does. Rejected: routing ctrl+wheel through egui's factor
  with a synced `scroll_zoom_speed` (a second copy of the rate), and
  converting the factor to notches. Review: style.
- **`zoom-to-fit-frames-no-committed-profile`**, with the aspect
  sentence riding along. The fit frames the drawn extent (the mesh,
  committed loops and preview loops; never pixel-sized marks or
  display aids). Absence is `None`, not `SceneMesh::nothing`'s
  origin point. Hidden parts do not frame. An empty or point extent
  frames at a named `DEFAULT_FRAME_RADIUS` with the orientation kept.
  The fit moves after the geometry lanes and before the marks.
  `CameraOp::Frame` reads "zoom to fit", and only `Unfittable` prints
  the aspect. The refuse-not-default argument lived only in
  agent-written comments. The design reads only drawn positions, so it
  holds under D′. Review: full (moves the frame order and a type).
  Correction: no `datums.rs` exists on main; the row's datum paragraph
  is stale.
(FIT orchestrator)

## 2026-10-22 — the budget pair weighed and dispatched

`the-budgets-predicted-count-is-not-always-an-over-count` and
`a-flat-rung-pair-is-read-as-flat-below-it` were weighed by one pair
(blinding byte on `analysis/design-fork/fit-p0`). The pair converged:
the budget is a **target with a stated, guarded precision**, not a cap.
Every "not a cap" in the tree means that a typed δ overrides the
budget, while finding 1 is about precision. The over-run is a
per-face `ceil` term that grows with curved-face count, so no margin
makes it a cap. A verifying tessellation buys a promise no reader
holds. Flat never changes the committed δ (a Flat stop holds under
budget/8 at 8× the request), so finding 2 concerns a reported number,
not the budget, and the row's ~15 000 threshold was not derived.
They split on one point, keeping or deleting `FittedDelta::predicted`.
The orchestrator checked: only tests read it, and in the bound case it
is the constant budget. **Deleted**; the other designer's ceil argument
becomes the documented reason for the band. The guard is the drawn
count, held to a band on every bound corpus row. Review: style.
(FIT orchestrator)

## 2026-10-22 — a ctrl/⌘+wheel is a wheel, and a pinch zooms by its ratio (PR 5596)

Closes `ctrl-wheel-reaches-no-zoom`.

- `configure_input` sets egui's `zoom_modifier` to `NONE` once, and
  both `assemble` and the test probe call it.
- `ViewportEvent::Pinch { factor }` maps to `Dolly(1/factor)`.
- `pointer_events` is the one composition of the adapter's reads,
  shared with the probe.

Review: a style lane, mergeable. Its two findings that could let the
shipped wiring regress silently were fixed: a startup row now reads
`zoom_modifier` off `started()`, and the probe no longer copies the
composition. The docs' claims about native pinch were narrowed to
winit's macOS/iOS `PinchGesture`. The hand-written variant lists went
onto VDOC's covering row
(`viewer-suites-hold-hand-written-complete-variant-lists`).

Process: the branch carries two empty commits a lane pushed to
re-trigger CI after the lavapipe install hung. They are disclosed on
the PR, and the hang is filed as CIW's
`the-lavapipe-apt-install-hangs-and-a-lane-cannot-re-run-it`.
(FIT orchestrator)
