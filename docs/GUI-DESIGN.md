# GUI / Editor Architecture — Design Document

**Status: v0, skeleton.** Companion to `DESIGN.md` (read that first;
this doc never overrides D1–D9). GUI work is sequenced **after**
"usable as a library" (DESIGN.md, Beyond the kernel), but the
decisions here are banked early because they are cheap at design time
and expensive to retrofit — several constrain M4's recipe/naming
design, not just the eventual GUI. Same conventions as DESIGN.md:
decisions marked *agreed* are settled; GQ items are open.

## G1 (agreed 2026-07-19): Three layers, and the boundary rules

The separation is not GUI-vs-library (two layers) but **three**: the
abstract "moves" live in a headless middle layer that is itself part
of the library product. D8 did the philosophical work already — *the
recipe is data*; the extension is: **changes to the recipe are data
too.**

1. **Kernel** (M0–M7): `build(params) → solid`. Unchanged.
2. **`editor-core`** (headless, no rendering dependency; slotted into
   DESIGN.md's crate table between `model` and `viewer`):
   - **The document is a value**: recipe DAG (D8) + document metadata
     (appearance, named views, …).
   - **The edit vocabulary**: a sum type of abstract moves (`DocEdit`:
     add/delete/reorder/suppress feature, set parameter, set
     expression, rebind reference, …) with one pure function
     `apply : Doc × DocEdit → Result<Doc, EditError>`. Undo/redo is
     free — documents are values; keep the old one.
   - **The reference/selection model**: selections are values of the
     *stable-name type* — the same type recipe nodes use to reference
     entities. Selection stability and recipe-reference stability are
     **one problem, solved once** by the persistent-naming design
     (its own pre-M4 doc, per DESIGN.md Band 1). Hit-testing is an
     editor-core service: `ray → stable ref`, on the M2 PR 6 mesh
     back-references.
   - **The evaluation service**: memoized incremental evaluation,
     evaluation epochs (stale results discarded by epoch — D9 makes
     memo keys well-defined and staleness detection trivial),
     cooperative cancelation, progress.
3. **Interaction** (the GUI proper): **tools** are modal controllers
   folding input events into edits —
   `handle(event, ui_state) → (ui_state′, Vec<DocEdit>, overlay)` —
   and rendering is a function
   `frame = render(evaluated body, selection, overlays)`.

**Boundary rules (each is a type-level discipline, not a
convention):**

- **The GUI never sees an arena key.** Keys are body-lineage-scoped
  (Q1 genericity boundary) and die on recompute; only stable refs
  cross the layer 2/3 boundary.
- **Transient gesture state never enters the document.** Rubber
  bands, in-flight drags, half-placed dimensions live in layer 3
  only.
- **Preview vs. commit is structural.** A continuous gesture emits a
  stream of *preview* edits evaluated against scratch state, and
  exactly one *committed* `DocEdit` on release — one undo step, one
  document transition. Same shape either way; commit is the one that
  enters the history (ratified in-conversation 2026-07-19).
- **Layer 3 is headless-testable**: replay synthetic event streams in
  CI, assert on emitted edit sequences. Only pixel-painting escapes
  unit tests.

The edit vocabulary is the **single API surface** shared by the GUI,
language bindings (Python), macro recording, headless tests, and —
eventually — collaborative editing. None of these know about the
others; all of them are consumers of `apply`.

## G2 (agreed 2026-07-19): Sketch editing is a nested editor

The sketcher is an editor-core instance one level down: the sketch
has its own document (entities + constraints + solved state), its own
edit vocabulary, its own preview loop; **committing the sketch is one
recipe edit**. Two facts recorded from the ratifying conversation:

- A per-frame solve's payload is the **entire solved coordinate
  assignment** (dragging one point moves others through constraints),
  not the dragged value. Preview and commit both apply "a solved
  state"; commit records the last one.
- What is *authoritative* in the committed record — constraints or
  solved assignment — is **not** a GUI question; it is GQ1, and it
  constrains M4's recipe format.

## Ratified micro-decisions (2026-07-19)

- **Dragging an expression-driven dimension → refuse, with an
  affordance** ("driven by `width/2 − margin` — edit the
  expression?"). Chosen as easiest and most principled; explicitly
  cheap to replace if a solve-for-free-variable or
  override-with-diagnostic mode is later wanted.
- **Error presentation is decided case by case** within the standing
  constraint that failures are typed *values* the GUI can render
  (highlight the offending entity, mark the failing feature in the
  tree) — never exceptions or strings. The case-by-case part is the
  presentation, not the plumbing.
- **Preview fidelity may degrade the chordal (display) tolerance —
  never ε.** The M2 chordal-vs-ε separation is the lever; stating it
  here makes "preview disagreed with commit" conceptually impossible
  rather than merely tested-against.

## Open questions

### GQ1: The solver/replay boundary (pre-M4/M6 — constrains the recipe)

If a sketch dimension is driven by a model parameter, replay at a new
parameter value re-solves *inside* `build` — putting a Newton solver
inside D9's bit-identity boundary and Q1's trilean discipline (solver
internals branch on convergence tests that are not predicates;
interval-instantiating a constraint solver is research).

**Why a stored witness at all (clarified with Evan, 2026-07-19 round
3): the constraints alone do not define the sketch.** A constraint
system generically has a *finite set* of discrete solutions
(reflections, elbow-up/elbow-down — 2^k-ish configurations all
satisfying the constraints exactly); the user's drag selected one,
and the constraint set does not record which. "Solve from scratch"
delegates that choice to initial-guess heuristics — hidden state
deciding topology, the disease D8/Q1 ban (and the root of every
parametric system's "my sketch flipped" bug: a specification gap,
not a solver bug). Decomposition: **authoritative geometry** = the
constraints + D4 ¶2 certification (residuals ≤ ε — no drift is
possible; any certified solution satisfies the constraints
regardless of seed); **authoritative branch selection** = the
witness. This is `Intersection { s1, s2, witness }` with a
0-dimensional solution variety. Purity is preserved because the
witness is recipe data (D8): replay computes
`solution(constraints, params, witness)` — deterministic in all
three; *continuation along the parameter path* (history-dependent)
is excluded. As params vary at fixed witness, the implicit function
theorem defines "the branch containing the witness's basin" until
the constraint Jacobian degenerates — a genuine bifurcation,
surfaced as a typed error with distance-to-singularity as the
margin (the sliver-band pattern), never a silent flip; a
large-parameter-jump witness landing near a basin boundary escalates
on the same margin. The witness refreshes at every committed sketch
edit, so it is always the user's most recent explicit choice.

**Proposed mechanism (direction agreed in principle; details are the
open part):** solver output demoted to witness; kernel certifies;
interval replay runs interval-Newton **contraction seeded from the
f64 witness** (existence/uniqueness in a box) instead of
interval-solving from scratch. Concrete audit item: ezpz (Q3) must
satisfy bit-identity (libm-only math, no hash-order effects) if its
f64 path runs inside `build`.

### GQ2 (RATIFIED 2026-07-19 round 3): Partial-build semantics — per-node result DAG

`Result<Solid>` is all-or-nothing; a usable tool shows "feature 7
failed, here is the body through feature 6, downstream suppressed."
**Ratified**: evaluation returns a **per-node result DAG** (a
value), the solid being the final node's success — fail-loud
preserved (failures typed and mandatory to confront), last-good
prefix free for the GUI. **A failure poisons only its descendants**
(typed "upstream failed" status); independent subgraphs — other
bodies, sketches, datum geometry — complete normally (Evan's
addition). Exact API shape is M4 design work; the codomain
commitment is what is banked here.

### GQ3 (RATIFIED 2026-07-19 round 3): All edits persisted in v1

**Ratified**: `DocEdit`s are persisted from the first version —
removing/disabling persistence later is far easier than adding it
(and session-spanning undo, macros, and collaboration all want it).
Banked consequences: the edit schema enters Band 4's versioning
discipline from the first persisted file; storage shape is
**snapshot + edit log** (details at editor-core design time).

### GQ4: Document scope (open — Evan unsure)

Working assumption, *not* ratified: one part per document,
references document-local; cross-document references arrive as a
typed extension with assemblies (Band 3). The naming design doc must
**flag every place it assumes reference locality** so the assumption
stays cheap to revisit.

### GQ5 (RESOLVED 2026-07-19 round 3 — already answered by D6): Units

D6 decides it: expressions are raw meters/radians; unit strings
(`25mm + t/2`) are parse-time sugar at the input boundary, converted
on entry. One-line residue for GUI time: the user's display unit is
presentation metadata (so a field round-trips as `25 mm`, not
`0.025`), trivially decided then.

### GQ6: Toolkit and platform (decide at GUI time; re-survey first)

Ecosystem snapshot 2026-07 (knowledge dated — re-survey before
committing): the **substrate** is reusable, the **CAD-ness** is not
(no sketcher, feature tree, or error UX exists to borrow, in Rust or
license-compatible elsewhere). Candidates: **egui** (immediate-mode;
`rerun` proves egui + wgpu viewport + panels at production scale) vs.
**iced** (Elm-architecture — G1 *is* MVU, so the philosophical fit;
thinner ecosystem); Slint (license model needs checking against
MIT/Apache), GPUI (Zed-shaped), bevy (brings free picking/camera at
the cost of a game engine's ECS worldview). Viewport: thin custom
wgpu regardless (Fornjot/truck confirm nothing worth coupling to).
Picking: GPU ID-buffer pass for hover/click exactness + CPU ray-cast
(`parry3d`, Apache-2.0) for snapping — both sit on the M2 PR 6
back-references. Web/wasm is a live strategic option (Zoo and CADmium
both chose it; pure-Rust `libm` means D9 accidentally made the f64
lane wasm-friendly; the `interval` feature is not, per issue #4) —
G1 is deliberately agnostic to it. `rerun` stays the zero-effort
debug viewer through M5; it is a viewer, not an editor substrate.

### GQ7: Selection mechanics

Multi-select and heterogeneous sets, selection filters, and the
convention that selection does **not** participate in document
history (undo never changes what is selected, but tools must survive
the referenced entity vanishing under them — a consumer of the GQ4/
naming-doc resolution-failure semantics). Details at sketcher/tree
design time.
