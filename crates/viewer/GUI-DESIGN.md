# GUI design — the ratified plan

The viewer's design plan: clauses `G1`–`G5` and the GUI questions
`GQ1`–`GQ7`, ratified with GUI v1 and listed in `docs/DESIGN.md`'s
companion table. **Changing what a clause here decides is a design
decision and waits for Ev** (`CLAUDE.md`, *Git workflow*); re-wording
one because an approved change renamed a symbol or moved a count is not
a second decision and lands with that change.

How the details actually shook out — the module boundaries, the
vocabularies, the Holds rows, and everything the CI gates parse — is
`crates/viewer/README.md`, which the program maintains itself as the
code changes.

## The three layers (G1)

The split is three layers, not GUI-versus-library. The recipe is data
(D8) and so are changes to it.

1. **Kernel**: `build(params) → solid`.
2. **`editor-core`**, headless, no rendering dependency. The document
   is a value: the recipe DAG plus metadata. The edit vocabulary is a
   sum type `DocEdit` with one pure `apply : Doc × DocEdit → Result<Doc>`;
   undo is keeping the old value. Selections are values of the same
   stable-name type recipe nodes use to reference entities
   (`crates/editor-core/src/names/README.md`), so selection stability
   and reference stability are one problem. Hit-testing is an
   editor-core service, `ray → stable name`. Evaluation is memoized,
   incremental, epoch-stamped and cooperatively cancelable.
3. **Interaction**, this crate: tools fold input events into edits,
   `handle(event, ui_state) → (ui_state′, Vec<DocEdit>, overlay)`, and
   rendering is a function of the evaluated body, the selection and
   the overlays.

Boundary rules, each a type-level discipline:

- **The GUI never sees an arena key.** Only stable names cross the
  layer 2/3 boundary; the hit-test service inverts keys to names.
- **Transient gesture state never enters the document.** Rubber
  bands, in-flight drags and half-placed dimensions live in layer 3.
- **Preview versus commit is structural.** A gesture emits preview
  edits against scratch state and exactly one committed `DocEdit` on
  release: one undo step, one document transition.
- **Every operation the GUI performs is itself API.** Select, hide,
  free-move, a camera move: each is a typed operation on a state value
  (`CameraOp`, `SessionOp`), callable with no renderer present, and
  rendering is a pure view of what those operations produce. Nothing
  is expressible only as a widget interaction.
- **Layer 3 is headless-testable.** `tests/` replays event streams and
  asserts on the emitted edits; only pixel painting escapes. Pipeline
  CREATION is not pixel painting and does not escape: `src/gpu.rs`'s
  smoke row builds a device on a software adapter and constructs every
  render pipeline in the viewport, asserting nothing about a pixel.

  **What still escapes is more than pixels, and naming only the one
  exception would overstate the seat.** Buffer and texture allocation,
  render-pass encoding and the id pass's readback are all
  device-validated and none of them is pixel painting either; the
  smoke row reaches none of them, because each needs a frame rather
  than a constructor. So the honest line is that CONSTRUCTION is
  gated and everything downstream of a frame is not — which is what
  `work/chrome/viewer-first-light-on-real-hardware` still holds open.

The edit vocabulary is the one API surface shared by the GUI, the
Python bindings, macro recording and headless tests; each is a
consumer of `apply` and none knows about the others.

## Sketch editing (G2)

The sketcher is an editor-core instance one level down: its own
document (entities, constraints, solved state), its own edit
vocabulary and preview loop, and committing the sketch is one recipe
edit. A per-frame solve's payload is the entire solved assignment, and
which of constraints or assignment is authoritative is the witness
question, answered in `crates/editor-core/README.md` (W1–W9). The
sketcher is not implemented.

## What v1 is (G3)

The v1 GUI is click-to-select for editing (selection feeds the
existing edit doors), pan/rotate/zoom, free-moving completely
unconstrained instances of an assembly relative to each other
(fit-probing before a mate exists: a display transform, no solver),
hiding instances, and defining a mate between previously unmated
parts. Live dragging of partly constrained geometry is not on the
path; the witness contract stays ratified for whenever it arrives.
Hiding and free-move are display state, never persisted into the
recipe, and a free-moved placement is drawn distinguishably from a
mated one.

## Micro-decisions (G4)

- Dragging an expression-driven dimension refuses, with an affordance
  offering to edit the expression.
- Failures are typed values the GUI renders (the offending entity
  highlighted, the feature to act on marked in the tree); never
  exceptions or strings. Presentation is decided case by case. Which
  feature that is, is the payload's own answer: a row a failure merely
  reached draws POISONED and quiet, pointing at the row that carries
  the cause (`crate::tree`).
- Preview fidelity may degrade the chordal display tolerance, never ε,
  so preview cannot disagree with commit.

## Colour (G5)

A **theme** is a user preference: it supplies every semantic mark
(selection, hover, free-move probe, focus, unresolved), the default
body colour, the ambient term and the viewport **ground**
(`Theme::ground`, what fills the viewport where no geometry is drawn).
It is never written into a document, not persisted by `editor-core`,
and takes no part in any content key. A **document** overrides the
body colour: `Attr::Color` on a stable name is authored and travels
with the file, and the theme never overrides it back. Both are
`editor_core::appearance::Rgba8`, so the override is a substitution
within one colour space; linear light is entered once, at each
renderer's door. Colourblind legibility is a claim a theme makes, not
a constraint on every theme: a palette that claims its marks stay
distinguishable under dichromatic vision is held to it by simulation
in `tests/theme.rs`, measured on the composited colour, since marks
are mixed over the body colour. That bar puts `colorblind-safe` on a
light ground. Preferences live in hand-editable TOML at
`$XDG_CONFIG_HOME/pncad/viewer.toml` (`src/prefs.rs`); malformed TOML
refuses, an unknown key reports and the rest applies, an unknown
value reports and falls back, while a theme name typed on the command
line is refused rather than defaulted. Where the environment names no
config directory there is no file and the store keeps nothing, which
the toolbar says beside the picker rather than discovering silently at
the end of the session (the badge paragraph above).

## The GUI questions

- **GQ1, the solver/replay boundary.** Solver output is demoted to a
  stored witness that selects the branch; the kernel certifies. The
  mechanism is W1–W9 in `crates/editor-core/README.md`.
- **GQ2, partial builds.** Evaluation returns a per-node result DAG; a
  failure poisons only its descendants and independent subgraphs
  complete (`editor_core::eval`, `NodeResult::{Ok, Failed, Poisoned}`).
  The tree's POISONED badge is wider than that DAG relation: a mate
  solve refuses across the placement graph, which the result DAG has
  no edges for, and those rows draw as downstream of the mate the
  fault names (`crate::tree`).
  Progress reporting and in-op yield points are absent; v1 shows a busy
  indicator over the shipped `CancelToken`.
- **GQ3, persistence.** Every `DocEdit` is persisted: the on-disk form
  is a snapshot plus an edit log, verified to replay through `apply`
  on save and replayed on load. The format carries no schema version
  before release; a file a build cannot read refuses typed with the
  regenerate recourse.
- **GQ4, document scope.** One document is one part's recipe, which
  may evaluate to several bodies; references are document-local; an
  assembly is a recipe DAG of the same formalism whose cross-document
  references are a wrapper over the local name with a content pin
  (`crates/editor-core/ASSEMBLY.md`).
- **GQ5, typed quantities.** The expression sublanguage carries typed
  quantities: `Dimension = Length | Angle | Count | Scalar`, every
  constructor dimension-checked, dimension-changing products refused,
  canonical values in metres and radians underneath, display units
  stored as presentation metadata and rendered by the panels.
- **GQ7, selection mechanics.** v1 is single-select; selection does
  not participate in document history. Pick priority is proximity in
  the picture, scoped to the body under the cursor: the ray picks a
  face first, and an edge of that face's own body within
  `EDGE_PICK_RADIUS_PX`, not hidden by the solid, beats it; elsewhere
  the face wins and off the body nothing wins. A tool narrows the
  kinds it accepts through `PickKinds`. Multi-select UX and the filter
  vocabulary wait on sketcher and tree design; filters, heterogeneous
  sets and vanishing-entity semantics are `docs/SELECT-DESIGN.md`'s.

## Toolkit and CI posture (GQ6)

**Toolkit: egui/eframe, with iced as the named fallback.** egui tracks
current wgpu, has the docking chrome a tree + viewport + property
panel needs, and has a production existence proof of this exact shape
(rerun). G1's architecture lives in `editor-core`, below any toolkit,
so the fallback costs only the interaction layer. Slint (GPL-only
OSI branch) and GPUI (unmaintained standalone) are out; bevy is
demoted. The conditions that would send v1 to iced, recorded so the
switch is a judgement and not a mood: the immediate-mode loop needing
ad-hoc frame-to-frame state to keep `Doc` authoritative; an egui MSRV
bump forcing a compiler move the bit-identity gate is not ready for;
chronic wgpu or paint-callback migration cost. None is met.

**Viewport, picking, docking.** The viewport is a thin custom wgpu
pass under eframe's wgpu renderer (`src/gpu.rs`). Picking is our own
deterministic `Bvh::ray` query, authoritative, with the GPU id-buffer
pass advisory. Docking is `egui_tiles`, a `Tree<Pane>` value the app
owns. All of it sits behind the non-default `app` feature; without it
the crate is renderer-free and headless-tested.

**The pick index is built off the UI thread, and it adds no frame
state.** Tessellating a document's roots and building their triangle
BVHs is the expensive step behind every picture here — seconds on a
dense document, and the window did not repaint while it ran, because
`sync_scene` called `PickIndex::build` inline. It runs on its own
worker now, across the same submit/poll vocabulary the evaluation
crosses (`src/evalseam.rs`, three seams and three workers), keyed by
the `(generation, δ)` pair it was built for.

**And so is the display budget's fit, which is the step BEFORE it.**
`scene::fit_delta` prices a document by tessellating it at a ladder of
coarser δ — about an eighth of a full tessellation, measured on this
tree's own corpus at 0.10–0.13 of one on every document dense enough to
want a budget — and it ran inside `sync_scene`, before the index was
submitted, so an `Open` stopped repainting for it. It runs on the third
worker now, over the same submit/poll vocabulary and keyed by
generation alone, because the δ is its ANSWER rather than an input to
it. The index build WAITS for that answer: `PickCache::sync` takes an
`Option<δ>` and an unsettled one takes the nothing-to-index way out, so
the un-budgeted build the budget exists to avoid cannot be submitted by
a caller forgetting to write an `if`. The cache drops the index it
holds in that window exactly as it does on a submit, which is *current
or absent, never behind* unchanged.

**What that window looks like, exactly.** `PickCache` drops the index
it holds at the moment it submits, not when the replacement lands, so
between the two there is no index at all — the state is **current or
absent, never behind**. Replacing one picture's key with another's is
what keeps it that way for every ordinary transition, and the one
transition with no next key — a document opened or authored while a
build is still with the seam — is where the invariant has to be
enforced by hand: `PickCache::sync`'s nothing-landed arm FORGETS the
outstanding attempt, so the build that finishes afterwards has no key
to match and is discarded. Leaving it a key was a state in which the
index of a replaced document installed over the scene of the one
before it, with nothing running and nothing said.

The viewport goes on drawing the mesh it last
received, which is the previous document's, and three things say so
rather than letting it pass for the current one: the toolbar shows one
progress state and it reads `indexing…` (`frame::progress` — one
value, so an evaluation and an index build cannot light two spinners
for one wait), a click is refused typed as `pickcache::NotIndexed`, which
is a different answer from *nothing under the cursor*, and a hover is
left alone because it is an observation pushed on every frame and not
an act.

**What that does to the frame-state inventory** — the per-field
justifications on `ViewerApp`'s own non-document fields, in
`src/app.rs`, which is the live form of what GUI-3's §5 ratification
rested on. It gains **no entry**, and the claim is exactly that
narrow. The index itself is current or absent, so it is not derived
data that can be WRONG about the document — which is the shape GQ6's
first condition is about, and `Doc` stays authoritative exactly as
before. What the seam does add is state *about the seam*:
`PickCache`'s record of what it has asked for and not yet been
answered, read every frame by the indicator and by the refusal's
wording. That is a fact about work in flight, not a second opinion
about the document, and it is the same shape `DocSession::running`
already has — including the same failure, recorded rather than
claimed away: a worker that dies leaves either of them describing
work nobody is doing. `ViewerApp::fit` is the third instance and
stands on the same ground: what it holds is a request, and the δ that
comes back is checked against the generation and the δ in force before
anything is taken from it.

**The index seam's promise is weaker than the evaluation seam's, and
the asymmetry is deliberate.** It has no cancel — not a cancel that
does nothing, but no door at all. The shipped `CancelToken` is checked
BETWEEN NODES and the step behind this seam has no nodes to be checked
between: neither `mesh::tessellate` nor the BVH build takes a token,
and giving them one is other crates' territory. So the policy is
**restart without cancel**: a δ change mid-build lets that build run to
completion and discards its answer, which costs a second full build —
on a document whose index takes 13 s, about 27 s before the picture is
right. An edit made during an index build is not delayed by it, which
is why each seam is its own worker: one queue would have put an
uninterruptible build in front of the next evaluation and quietly
weakened the cancel-and-restart promise made above it. The fit seam is
the same argument one step further along — it and the index build are
sequential for ONE document and not across documents, so a fit for a
document that has just arrived must not sit behind an index build for
the δ someone typed a moment ago.

**Where the `app` feature gates.** The workspace nextest archive builds
this crate at DEFAULT features, so nothing behind the feature is in it.
The seat is a hosted row that runs
`cargo nextest run -p viewer --features app` in the `fmt` job, beside
the app-feature clippy row that already compiles the toolkit graph, on
the same seed-keyed `run_viewer_toolkit` axis and with a lavapipe
adapter installed for the pipeline smoke row. Archiving with
`--features app` was the alternative and was refused: the archive is
built once and downloaded by every leg of the `test` matrix, so the
toolkit graph's extra weight is paid per leg for rows that already
gate. The measured figures, and why they carry no guard, are stated at
that step in `.github/workflows/ci.yml` and only there. What the
default-feature lane is therefore NOT checking is printed there by
name, by the `app_lane_skipped_*` rows in `src/lib.rs`,
`tests/chrome_labels.rs`, `tests/error_display.rs` and
`tests/panel_display.rs`.

**wasm.** The whole kernel plus `editor-core` compiles to
`wasm32-unknown-unknown`, `--features interval` included, and CI
re-takes that reading on every code-tier pull request with one
`cargo check` step for the interval build only; the default-features
half rides on it because `scripts/check-interval-cfg-additive.py`
keeps the interval build a syntactic superset of the library sources.
The guard establishes that the crates compile, not that they link or
run. `pncad` and this crate additionally need `getrandom`'s wasm
backend named in both halves: the `wasm_js` feature (the stanza in
`Cargo.toml`) and `RUSTFLAGS='--cfg getrandom_backend="wasm_js"'`;
setting only the flag fails the build, which is why the feature is
declared here so the flag is all a builder has to remember
(`local-scripts/serve-wasm.sh`). The browser lane itself is deferred.

## Banked post-v1

GUI-5, the threaded web lane, and GUI-6, the history graph: a
branch-picker UI and a separable history sidecar over the tree-shaped
undo `src/history.rs` already keeps (an edit after undo mints a
sibling; nothing is destroyed). Both are in `docs/LONGTERM-IDEAS.md`'s
GUI section until dispatched.
