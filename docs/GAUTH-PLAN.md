# GAUTH — part authoring in the GUI: the plan

**STATUS: OPEN. Scope ruled by Evan in-chat (2026-08-31): Phase A
and Phase B definitely; fillet/chamfer authoring and assembly
instance authoring wanted.** Program prefix `gauth/`; A/B ordinal
band **900–999** (claimed in `docs/MODEL-AB-LOG.md` in this plan's
opening commit); blocks named `GAUTH-B<n>`. The live status is the
tail of `docs/GAUTH-LOG.md`.

Every decision this plan leans on is ratified elsewhere and cited,
not re-litigated: the three-layer split and boundary rules
(`docs/GUI-DESIGN.md` G1 — operations are API, tools fold events
into edits, one committed `DocEdit` per action, headless-testable),
partial evaluation and persistence (GQ2/GQ3), the v1 GUI's shipped
chrome and tools (`docs/GUI-PLAN.md`, program closed on
`docs/GUI-EXIT-WALK.md`), and the sketcher's exclusion (G2 — the
nested sketch editor is M10-era work and nothing here starts it).

## The gap

The viewer edits, deletes, hides, free-moves and mates, but cannot
create: the only chrome door to `DocEdit::InsertNode` is the mate
tool, and there is no new-document operation. Everything below
layer 3 already supports creation — `InsertNode` mints and
validates, the root list maintains itself as the DAG's sink set
(`editor-core::roots`), undo/persistence/incremental evaluation
ride the existing machinery, and profiles are programs as data
with template constructors (`LoopProgram::{circle, circle_split,
polygon}`). The program is five units of layer-3 doors plus one
piece of genuinely new pick craft (edges).

## Standing constraints, binding on every unit

- **Every creation is a typed `SessionOp` committing exactly one
  `DocEdit::InsertNode`** (or, for GAUTH-1's new-document op, one
  session-state replacement) — G1's operations-are-API rule. Chrome
  renders forms and emits ops; nothing is expressible only as a
  widget interaction.
- **Creation forms are minimal; the property panel is the editor.**
  A tool inserts with its few required fields and sensible
  defaults; every slot is then edited through the shipped panel
  (expressions, units, range probes). Do not duplicate panel
  affordances into creation forms.
- **Refusals are typed values rendered in place** (the ratified
  error micro-decision); no strings minted in chrome.
- **Headless tests replay synthetic op/event streams in CI and
  assert on emitted edits**; only pixel-painting escapes. Each
  unit's acceptance includes at least one end-to-end row driving
  the real `DocSession` with no renderer.
- **Single-select stays ruled** (GUI-PLAN). Tools needing several
  picks hold them in tool state, the mate tool's shape.
- No unit takes a dependency that fails the wasm guard, and the
  interaction layer keeps assuming no threads (GUI-PLAN's carried
  constraint).

## The units

Difficulty is pre-logged here, before any block draw, per the A/B
protocol. Wave 1 = GAUTH-1, -2, -3 (independent, concurrent);
wave 2 = GAUTH-4 (after GAUTH-1 — same chrome/session files) and
GAUTH-5 (after GAUTH-2 — consumes edge picks).

### GAUTH-1 — from nothing to a solid (Phase A). Difficulty: L

The `ring` demo's authoring sequence
(`demos/tour/src/ring.rs::document`) behind ops and chrome.

1. **`SessionOp::NewDocument { name }`**: replaces the session's
   document with `Doc::empty(DocumentId::derive(&name), tol)`,
   clearing path, history, selection and display state; refused
   mid-gesture. Chrome: a `New…` control beside `Open…` with one
   name field. **Identity ruling (orchestrator, logged in
   GAUTH-LOG)**: the id is authored at creation from the typed
   name — `DocumentId::derive`, the deterministic spelling the
   demo corpus uses; no re-minting at save. A workspace with two
   parts derived from one name refuses at resolution (the
   workspace's existing duplicate-id refusal), which is the
   fail-loud recourse. Alternative recorded, not taken:
   `pncad::workspace::random_document_id` exists, but a New door
   should not mint what a user cannot re-derive.
2. **Add-datum tools**: three small forms (plane / axis / point)
   each committing one `InsertNode` of the corresponding
   `Datum` variant with literal `Expr` slots. Defaults: plane
   origin 0 normal +z; axis origin 0 direction +z; point at 0.
3. **Add-profile tool**: template vocabulary, not a sketcher —
   **circle** (`LoopProgram::circle`) and **rectangle**
   (`LoopProgram::polygon` from width/height, centered) with
   Length fields; one `InsertNode` of `Node::Profile`. Plane
   placement: world XY by default; when the current selection is
   a planar face, offer placing on that face's frame (the
   `select::face_frame` door the mate tool uses), frozen f64 in
   the program's placement struct — stated in the form as a
   snapshot, not a reference. **Spec'd fallback**: if the
   face-frame arm proves deep, land XY-only and file the face arm
   as an issue before merge (a scheduled follow-up per protocol
   v5, not a silent narrowing).
4. **Extrude tool**: requires a selection resolving to a
   `Profile` node (tree pick, or a face pick whose feature is a
   profile — refuse typed otherwise); distance field; one
   `InsertNode` of `Node::Extrude`.
5. **Revolve tool**: profile pick + axis pick (a `Datum::Axis`
   node), two sequential picks in tool state; angle defaults to a
   full turn; one `InsertNode` of `Node::Revolve`.

Acceptance: a headless op stream — NewDocument("hollow-ring"),
two circle profiles, an axis, a revolve — reproducing the ring
demo's document (same nodes, same roots; assert equality at
whatever strength the persistence layer's comparison honestly
supports and state which); plus chrome-level smoke via the
existing synthetic-event lanes; plus save → reload of an authored
document through the shipped snapshot+log door.

### GAUTH-2 — edges as picks. Difficulty: M

Substrate only — no document edits. The pick path and `Selection`
are face-shaped; fillet/chamfer selections are edge stable names.

1. **Selection vocabulary**: an edge selection value (stable
   name + owning node + body), a `Selection` arm for it, hover
   included, with the same resolution-failure survival semantics
   the face arm has.
2. **The hit-test service**: `screen point → edge pick` as a
   typed layer-boundary service beside the shipped `ray → face`
   one. Mechanism latitude, direction recommended: the
   tessellation carries edge back-references (the same M2 mesh
   back-reference family the face path resolves through); a
   screen-space distance test against the picked-region's edge
   polylines, seeded by the existing face pick / ID buffer, is
   acceptable and cheap; a second GPU ID pass over rendered edge
   primitives is acceptable if the first proves insufficient.
   Deterministic; no arena key crosses into layer 3.
3. **Pick priority**: an edge within a small pixel radius beats
   its face; the constant is named, documented, and lives in one
   place (GQ7's pick-priority residue — this is its first
   concrete instance; record it as such).
4. **Rendering**: hover/selection highlight for edges, visually
   distinct from face highlight, via theme marks (composited per
   the colour rules in GUI-DESIGN).

Acceptance: headless hit-test rows (synthetic meshes, known
camera, asserted picks, including the edge-beats-face radius and
its boundary); selection-survives-vanishing-referent rows for the
edge arm; no renderer in any assertion.

### GAUTH-3 — placing an instance (assembly authoring). Difficulty: M

1. **`SessionOp::AddInstance { id }`** committing one
   `InsertNode` of `Node::InstantiatePart` with an empty
   interface record. The `DocRef` is minted through the shipped
   workspace door: `Workspace::open` on **the open document's own
   directory** (the DirResolver rule in `viewer::docio`, restated
   nowhere), `documents()` for the listing, `current_pin` for the
   pin. A session with no backing file refuses typed with the
   recourse named ("save the document first — references resolve
   against the file's directory").
2. **Chrome**: an `Add part…` dialog listing the workspace's
   documents (id + filename); picking one inserts. Scan refusals
   (duplicate id, unreadable sibling) surface the workspace's own
   message, at the dialog rather than the tree, since no node
   exists yet.
3. **Placement**: none authored — an instance carries no frame
   (A11 puts placement on clusters); the inserted instance is
   immediately hideable, free-movable and matable through the
   shipped GUI-4 tools, which is the workflow this door exists to
   feed.
4. The instantiate node's resolution/evaluation refusals
   (PinMismatch / EpsilonSeam / Unresolved) already render on the
   tree badges; verify the authored-path variants appear there
   and read sensibly.

Acceptance: headless — in a directory holding the tour assembly's
part documents, author a new assembly document, AddInstance twice,
mate the instances with the existing op, save, reload, re-evaluate;
refusal rows for the unsaved-document and duplicate-id arms.

### GAUTH-4 — combining bodies (Phase B). Difficulty: M

After GAUTH-1 (shares the creation chrome and session files).

1. **Boolean tool**: two sequential body picks (a face pick
   resolves to its owning node; tree picks equally valid), an op
   choice (Union / Difference / Intersection), `declare: None`;
   one `InsertNode` of `Node::Boolean`. Operand order is data for
   Difference — the form says which pick is which.
2. **Split tool**: target pick + tool pick, same shape.
3. **Transform tool**: one body pick + translation/axis/angle
   fields (defaults zero/+z/0); one `InsertNode`.
4. **Pattern tool**: one body pick + rule choice — Linear (count,
   spacing, direction) and Circular (axis pick, count) only;
   `Explicit` is not a form's job. Structural count spelled per
   the structural-slot discipline.
5. **`PlacedUnion` is out of scope**, recorded here: the group
   boolean's authoring wants the same form as Pattern plus fusing
   semantics, and can ride a later unit once the Pattern form has
   settled the vocabulary; nothing in this unit forecloses it.

Acceptance: headless op streams for each tool including the
refusal arms (self-boolean, a pick whose referent vanished
mid-tool); an authored two-body boolean document evaluating,
saving, reloading; Difference asserted non-commutative in the
authored order.

### GAUTH-5 — fillet/chamfer authoring. Difficulty: M

After GAUTH-2 (consumes edge picks).

1. **The blend tool**: edge picks ACCUMULATE in tool state
   (single-select stands; the tool holds a set, the mate tool's
   pattern one size up), with per-pick add/remove and a live
   count; a kind choice (fillet/chamfer) and one Length field
   (radius / setback); commit = one `InsertNode` of
   `Node::Fillet` or `Node::Chamfer` via the canonicalizing
   constructors. The selection freezes at commit by the ratified
   #217 semantics — the tool's text says so.
2. **The all-edges affordance**: a "select all edges of this
   body" action calling the `all_edges` door and loading the
   result into tool state as an ordinary (frozen) set.
3. Blend refusals are already typed through the node's
   evaluation; verify the authored path renders them on the tree
   badge and that a stranded selected edge refuses per the
   resolution-failure semantics rather than shrinking silently.

Acceptance: headless — author a box (GAUTH-1 ops), accumulate
edges, commit a fillet, assert the inserted node's canonical
selection; a chamfer twin; an all-edges row; a
selection-vanished refusal row; save/reload of an authored
blended document.

## Reviews and merges

Each unit runs the standing machinery: blinded implementer lane
per the A/B block draw, cross-model dual review on a frozen head
with explicit claims to falsify plus the style lane by path,
orchestrator-adjudicated union fix pass, record-at-merge in
`docs/MODEL-AB-LOG.md`. Units self-merge on green per the standing
rules; nothing here ratifies an open design question, and the one
identity ruling in GAUTH-1 is logged as a unilateral decision for
Evan's retroactive review.

## Sizing

Five PR-sized units in two waves; the machine runs three
implementer lanes at once (Evan's sizing for this box). GAUTH-2 is
the one with genuinely new craft (pick-path depth); GAUTH-1 is the
largest by surface. Nothing here approaches the sketcher, the
solver, or live dragging — those stay where their docs put them.
