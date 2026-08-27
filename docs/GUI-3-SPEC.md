# GUI-3 spec — the document panels

Unit 3 of `docs/GUI-PLAN.md` (RATIFIED 2026-08-27). Builds on the
merged GUI-0 `viewer` crate (chrome, camera, scene path). Read the
plan's Rulings/Scope/Undo-note sections, `docs/GUI-DESIGN.md` G1 +
the ratified micro-decisions + the undo-tree section, and the
GUI-0 unit entry in `docs/GUI-LOG.md` before starting. Standing
lane obligations: `docs/prompts/implementer-discipline.md`.

GUI-1's hit-test service and GUI-2's selection are NOT
dependencies — the panels select via the feature tree; viewport
click-to-select arrives with GUI-2. Do not touch `crates/bvh` or
`editor-core::resolve::pick` (a concurrent lane owns them).

## Deliverables

1. **Feature tree over the GQ2 result DAG**: one pane listing the
   open document's recipe nodes with per-node status from the
   evaluation (`NodeResult::{Ok, Failed, Poisoned{through}}`) —
   Failed/Poisoned badges render the TYPED payloads' messages
   (failures are values the GUI renders; never strings invented in
   layer 3). Tree selection is a typed layer-3 selection value.
2. **Property panel**: shows the selected node's parameters;
   edits apply as `DocEdit`s through pure `apply`, re-evaluated
   with the shipped memoized incremental evaluation. The
   **expression-driven-dimension refusal affordance** (ratified
   micro-decision): editing a value driven by an expression
   refuses with the "driven by `<expr>` — edit the expression?"
   affordance (editing the expression text is in scope only as a
   plain text field over the existing typed expression API — no
   parser work; if no text round-trip exists in the substrate,
   scope the affordance to navigate-and-display and SAY SO).
   Values display as canonical meters/radians (ruling: U8's
   units/display layer is not a dependency).
3. **The evaluation seam** (the plan's standing constraint, made
   real here): evaluation runs behind a seam — background thread
   natively, never assumed by the interaction layer — with a
   **busy indicator and the shipped `CancelToken`** (ruling:
   progress reporting and in-op yield points stay absent). An edit
   during evaluation cancels-and-restarts or queues; pick one,
   document it, test it headlessly.
4. **Undo/redo, linear chrome over tree-shaped state** (the plan's
   undo note, verbatim intent): parent pointers over retained
   `Doc` values; an edit after undo MINTS A SIBLING rather than
   truncating; v1 chrome exposes only undo/redo along the current
   branch; nothing is destroyed. Save writes the current path's
   linear log — persistence untouched. Preview-vs-commit is
   structural (G1): a continuous gesture (slider drag) evaluates
   preview edits against scratch state and commits exactly one
   `DocEdit` on release — one undo step.
5. **Open/save**: typed layer-3 operations `open(path)`/`save(path)`
   over the shipped snapshot+log persistence, with a native file
   dialog as a THIN veneer over them (the dialog itself is the one
   thing that escapes headless testing; the operations do not).
   Dialog dependency chosen in-unit under the ~2-week age rule and
   MIT/Apache compatibility; name it and its licence in the PR.
6. **The demo-document gallery** (the plan's acceptance substrate):
   an exporter mode in the tour — each document-authored scene
   (assembly, checks, diefillet, heatsink, scalar today) saves its
   `.pncad` into a gallery directory. Openable in the app via the
   dialog. Scenes that drive the kernel API directly are NOT
   re-authored here (that is banked per-scene LIB work).

## The measurement this unit re-takes

GUI-0's seam-friction reading was provisional because the spike
edited nothing. GUI-3 is where the authoritative-document-under-
immediate-mode question actually gets tested: editing, undo, and
async evaluation against `Doc` as a value. The PR body must
re-take the reading against GQ6-RESURVEY §5's three fallback
conditions, with specifics (what frame-to-frame state the panels
needed, if any). This is the egui→iced go/no-go data.

## Constraints

- G1 boundary rules bind: tools are
  `handle(event, ui_state) → (ui_state′, edits, overlay)`; every
  operation is typed API callable with no renderer; no arena key
  in layer 3; transient gesture state never enters the document.
- CI replays synthetic event streams asserting on EMITTED EDITS
  (and on undo-tree shape); only pixel-painting escapes.
- The viewer-CI posture (GUI-LOG ruling) is live: your changes
  seed `viewer`, so the toolkit rows run on your PRs — confirm the
  filter's verdict step drew RUN, not by job-name green.
- No dependency that breaks the wasm guard; evaluation-seam code
  must not assume threads (wasm runs it inline/Worker later).
- Demos discipline (`memories/demo-purpose.md`): the exporter and
  any demo document changes go through public doors only.

## Testing

Headless CI at minimum: event-stream replays for edit emission
(property edit → one committed DocEdit; slider gesture → previews
+ exactly one commit); undo-tree rows (undo→edit mints sibling,
nothing destroyed, redo along current branch, save writes the
current path); evaluation-seam rows (busy state visible to the
chrome as a value, cancel token honored, results land by epoch —
stale results discarded); refusal affordance rows (expression-
driven edit refuses typed); open/save round-trip through the
typed operations on a gallery document; tree badge rows (a failing
document renders Failed/Poisoned from typed payloads).

## Out of scope

Viewport selection/highlight (GUI-2); the sketcher (G2, later
milestone); mates and assembly interaction (GUI-4); the history
graph UI and sidecar (GUI-6, banked); units display beyond
canonical; any solver work; error-UX breadth beyond the tree
badges and refusal affordance.

## Acceptance

Open a gallery `.pncad` through the dialog; see its feature tree
with live statuses; edit a parameter in the panel and watch the
re-evaluation land (busy + cancel work); undo/redo behave linearly
with the sibling-minting semantics under test; save round-trips.
Hosted CI green (the gate; the toolkit rows drawn RUN).

Branch `gui/gui-3-panels`; merge `origin/main` immediately before
opening the PR and re-merge whenever main moves. NO Co-Authored-By
trailer in lane commits (A/B blinding;
`memories/model-ab-experiment.md`).
