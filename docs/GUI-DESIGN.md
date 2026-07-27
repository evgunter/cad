# GUI / Editor Architecture — Design Document

**Status: v0, skeleton.** Companion to `DESIGN.md` (read that first;
this doc never overrides D1–D9). GUI work is sequenced **after**
"usable as a library" (DESIGN.md, Beyond the kernel), but the
decisions here are banked early because they are cheap at design time
and expensive to retrofit — several constrain M4's recipe/naming
design, not just the eventual GUI. Same conventions as DESIGN.md:
decisions marked *agreed* are settled; GQ items are open.

*Freshness note (M4 8c exit sweep, 2026-07-27):* the middle layer this
doc banks on is now REAL — `editor-core` exists with the recipe
substrate (#81), the GQ2 per-node result DAG + evaluation service
(#83), one stable-name type with resolution/Rebind (#87/#96/#102),
GQ3 persist-all-edits as schema v1 (#112), and StableName-keyed
appearance with the N3/N5 loss semantics (#92). References in the
body to M4 as future work are historical; the GUI layer itself
remains unbuilt as sequenced.

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

### GQ1 (RATIFIED 2026-07-19 round 4): The solver/replay boundary — witness as authoritative branch selection

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

**Ratified (Evan, round 4: "the clear correct choice for us"):**
solver output demoted to witness; kernel certifies; interval replay
runs interval-Newton **contraction seeded from the f64 witness**
(existence/uniqueness in a box) instead of interval-solving from
scratch. Mechanism details (contraction specifics, the margin
predicate's exact form) are M4/M6 design work under this committed
direction. Concrete audit item: ezpz (Q3) must satisfy bit-identity
(libm-only math, no hash-order effects) if its f64 path runs inside
`build`. Mechanism note carried up from M2 PR 3's adversarial
review (the "S2" lesson, via the PR #32 orchestrator, 2026-07-20):
the witness contract must pin **which** point is the witness — a
loose contract admits wrong-but-certified selections (M2 sharpened
edge witnesses to the mid-parameter point for exactly this reason);
the sketch-level witness has the same aliasing question and must
answer it explicitly.

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

### GQ4 (RATIFIED 2026-07-19 round 5): Document scope — local refs + wrapper, assemblies in the same formalism

Decided-now rather than deferred because the naming doc's central
artifact — the stable-reference type — depends on it, and the
extension shape is composition, not modification. Ratified:

- **One document = one part's recipe** — one parameter space + one
  feature DAG. A recipe may evaluate to **multiple bodies** (the
  kernel already permits it; booleans/multi-body workflows want it):
  "part" ≠ "one solid."
- **References are document-local**; the stable-ref type carries no
  document component.
- **Cross-document references arrive with assemblies (Band 3) as a
  wrapper type** — (document identity × local ref) — composing the
  existing ref type, never modifying it; nothing built pre-assembly
  is touched.
- The naming design doc proceeds assuming locality, with the wrapper
  named as the sanctioned extension point.
- **The uniformity principle (Evan's synthesis, the ratifying
  addition): the document boundary is a namespace/versioning seam,
  NOT a change of formalism.** An assembly document is a recipe DAG
  of the *same shape* as a part document — its nodes are features
  like any other (instantiate-part via wrapped ref, mates,
  patterns) — so everything already built applies to assemblies
  with zero new machinery: **GQ1 verbatim** (mates are a constraint
  system with the same finite-discrete-solutions structure —
  flipped-bracket instead of elbow-up — so witness-as-branch-
  selection, certification, and bifurcation-as-typed-error transfer
  unchanged), **GQ2** (a failed part suppresses its assembly
  subtree; siblings complete), **GQ3** (assembly edits are persisted
  `DocEdit`s), plus naming and undo.

**Alternatives considered (recorded round 5).** The decision space
is three axes: (1) ref type shape, (2) document granularity,
(3) binding semantics — live vs. pinned-with-explicit-update.
**B — globally-qualified refs from day one**: discarded — every
local ref (the overwhelming majority) pays a doc field that is
always "self" (the wrapper reappears *inside* the type, but
everywhere); forces designing document identity blind, before the
assembly design that gives it meaning; and muddies purity — under
the ratified shape, `build`'s input stays (params, this document),
with doc-identity resolution strictly above the kernel.
**C — one workspace, no boundaries**: compelling shape (D8's
code-generates-recipes already blurs file boundaries), absorbed as
the uniformity principle above; rejected as literal architecture
because it has no modularity seam — no unit of exchange, versioning,
or parts-library attachment.
**D — multi-part documents (part-studio)**: already contained in
"one recipe, multiple bodies"; export/BOM part-labeling is metadata,
not reference architecture.
**E — assemblies embed by value**: the fully-pinned extreme of
axis 3; absorbed as a pin option the wrapper can hold.
**Axis 3 — binding semantics — RATIFIED in direction (2026-07-19
round 6): pinned-with-explicit-update, the Cargo.lock model.** Part
as package; the wrapper holds the version pin; "update" is a
recorded document edit (free under GQ3); an assembly is therefore a
self-contained reproducible value — never silently retargeted by
edits to referenced documents. Axis 3 is the historical bleeding
ground (external-reference hell, in-context fragility, circular
updates); the pin model excises the whole class. Detailed semantics
(pin representation — content hash vs. version id; update
granularity; conflict surfacing) are assembly-design work under
this committed direction. The strongest future consumer,
in-context modeling (a part referencing an assembly neighbor's
face), lands on this same extension point: the industry lesson is
that in-context refs must be mediated by an explicit
captured-context object with pin/update semantics — the
wrapper-plus-pin shape again, held by the part document.

### GQ5 (RATIFIED 2026-07-19 round 4, superseding round 3's raw-meters reading): Typed quantities in the expression sublanguage

Round 3 read D6 as "expressions are raw meters, unit strings are
parse-time sugar." Evan's round-4 revision, ratified: **the
expression sublanguage carries typed quantities** — once display
units are stored anywhere (round-tripping `25 mm` requires it), raw
storage means the type system knows less than the data does;
conversion errors must be impossible at the type level. This is
D6 *applied* to the recipe layer, not a revision of it: D6's raw
meters/radians still governs kernel-internal code; the expression
language is user-facing recipe data — the API boundary — so typed
quantities there are the newtype principle one layer up. Canonical
values remain meters/radians underneath (units erase before kernel
`T`; scalar genericity untouched); display unit is presentation
metadata. **Banked M4 decision this creates**: the expression
language's *dimension algebra* — same-kind add/sub and scalar
multiply are obvious; products/ratios force choosing between a small
dimension lattice and forbidding dimension-changing operations in
v1 (D6's "~five quantities, not the SI lattice" stance suggests the
restrictive answer). Fold into D8 at M4 planning.

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

## UI ideas (non-binding sketchpad)

Ideas captured during design conversations — NOT ratified decisions;
they inform sketcher/editor design when it happens. Each cites the
contract it builds on.

- **Wall-mode drag (Evan, PR #79 conversation, 2026-07-23)** — on
  SOLVER-DESIGN W2/W4: the default click-and-drag mode refuses to
  cross the discriminant locus. As the drag approaches it, the
  preview solver's `solver_branch_margin` shrinks; at the wall the
  dragged point visually sticks, with an indication of the
  parameter-space wall it has hit. Consequence: every default-mode
  drag is a fold-free homotopy, so the drag-end ReWitness is
  uniquely branch-selected and needs NO disambiguation dialog. An
  explicit modifier key crosses the wall — the keypress is the
  recorded intent to flip branches, so even flips are chosen, never
  silent. The drag path is legitimate *UI* input (it authors the
  ReWitness proposal); purity is untouched because only the recorded
  endpoint enters `solution()`.
- **Bulk ReWitness on clean certificates** — on SOLVER-DESIGN W4:
  certified-same-branch rewitnessing is semantically invisible, so
  the editor should do it in bulk (piggybacked on commit edits)
  rather than nag; dialogs are reserved for certificate refusals,
  which concentrate at genuinely degenerate geometry.
- **Margin as an ambient affordance** — on W3/T6: the
  `solver_branch_margin` value is a live scalar during editing;
  surfacing it (e.g. subtle proximity shading near walls) turns
  "why did it ask?" into something the user saw coming.
- **Scale-relative sliver lint** — from #89 (Evan, 2026-07-24): any
  feature whose margin is so small it renders indistinguishably from
  exact coincidence at GUI scale is *probably* a mistake — but the
  kernel must not refuse it (K guards certification honesty, not
  intent; GUI-scale K ≈ 1e5–1e6 would refuse legitimate
  micro-features). Instead: a document-layer lint reusing the normal
  K machinery verbatim — evaluate the margined predicates, compare
  margins against a *display-relative* threshold (viewport scale ×
  pixel size, not ε), and badge offending features in the tree/
  viewport ("this edge is 3e-9 from exactly touching — intended?").
  Pure UI concern: no kernel change, no new predicate family, no
  effect on evaluation or certification. The kernel-side "should K
  itself be larger" question stays separate, gated on the M5 exit
  K-snapshot (#89 remains the tracking handle).
- **Painted operands through booleans** — from #92 (Evan, 2026-07-25):
  joining painted bodies never errors (resolves-anywhere semantics;
  paint keeps resolving on the operand node). The GUI renders the
  displayed node's appearance, so paint-what-you-see always works;
  when a boolean is appended above appearance-carrying names, the
  PR 4 suggestion ladder surfaces one-click Rebind offers for the
  wrapping derivations (recorded intent). The industry default —
  silently following faces via topological-naming heuristics — is
  exactly the N5-banned shape; if one-click proves too manual, a
  "carry appearance through this boolean" policy can enter the N5
  menu as its own ratification.
