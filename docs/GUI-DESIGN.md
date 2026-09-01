# GUI / Editor Architecture — Design Document

**Status: RATIFIED architecture (G1–G5 agreed; GQ1–GQ5 resolved,
with the `editor-core` substrate shipped — the freshness note below
is the verified shipped-vs-absent inventory; GQ6's toolkit row
ratified 2026-08-16 off the mandated re-survey in
`docs/GQ6-RESURVEY.md` — **egui**, with iced as the named fallback —
and its remaining rows settled inside the v1 GUI units; GQ7's
general-purpose clauses re-homed to `docs/SELECT-DESIGN.md` §4, the
slimmed remainder still deferred to sketcher/tree design). The v1
GUI layer is built: units GUI-0…GUI-4 merged (PRs #1094, #1093,
#1101, #1106, #1113) against `docs/GUI-PLAN.md`, and the program
CLOSED 2026-08-28 on the ratified `docs/GUI-EXIT-WALK.md`.**
Companion to
`DESIGN.md` (read that first; this doc never overrides D1–D9). GUI
work is sequenced **after** "usable as a library" (DESIGN.md, Beyond
the kernel), but the decisions here were banked early because they
are cheap at design time and expensive to retrofit — several
constrained M4's recipe/naming design, not just the GUI itself.
Same conventions as DESIGN.md: decisions marked *agreed* are
settled.

*Freshness note (verified against the code 2026-08-28):* the middle
layer this doc banks on is REAL —
`editor-core` ships the recipe substrate (`Doc`/`DocEdit`/pure
`apply`, #81), the GQ2 per-node result DAG with descendants-only
poisoning plus memoized incremental evaluation and cooperative
cancelation (#83), one stable-name type with resolution/diagnosis/
`Rebind` (#87/#96/#102), GQ3 persist-all-edits (schema v1 at #112,
carried forward through a series of pre-release clean breaks; the
live number is `persist::SCHEMA_VERSION`) — StableName-
keyed appearance with the N3/N5 loss semantics (#92), the
dimension-checked total expression AST (GQ5's restrictive
dimension answer) with the text door and display-unit round-trip
the library program added, and arena-key→stable-name hit
inversion. Layer 3 is real too: the `viewer` crate ships the
camera and document-session operation vocabularies, the `Bvh::ray`
and GPU-id pick paths, the feature tree, the property panel, and
tree-shaped undo behind linear chrome. Still ABSENT, so nobody
reads more than shipped: progress reporting and in-op yield points
(v1 rules a busy indicator over the shipped `CancelToken`); the
sketcher and everything solver-shaped (M10); and the history
sidecar with its branch-picker UI, banked as GUI-6. References in
the body to M4 as future work are historical.

## G1 (agreed 2026-07-19): Three layers, and the boundary rules

The separation is not GUI-vs-library (two layers) but **three**: the
abstract "moves" live in a headless middle layer that is itself part
of the library product. D8 did the philosophical work already — *the
recipe is data*; the extension is: **changes to the recipe are data
too.**

1. **Kernel**: `build(params) → solid`. Unchanged.
2. **`editor-core`** (headless, no rendering dependency; its own row
   in DESIGN.md's crate table, under `viewer`):
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
- **Every operation the GUI performs is itself API** (Evan,
  2026-08-27): "select this object", "hide this part", "free-move
  this instance", a camera move — each is a typed operation on
  `editor-core` or layer-3 state values, callable with no renderer
  present; rendering is a pure view of the state those operations
  produce. Nothing is expressible only as a widget interaction.
- **Layer 3 is headless-testable** (the consequence of the rule
  above): replay synthetic event streams in CI, assert on emitted
  edit sequences. Only pixel-painting escapes unit tests.

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

## G3 (Evan, 2026-08-03): The v1 GUI minimum EXCLUDES live editing

Sourced from a conversation with a practicing mechanical engineer:
the minimum useful GUI needs exactly —
- click-to-select parts/features for editing (selection feeds the
  existing edit doors; no in-viewport manipulation),
- pan / rotate / zoom,
- moving completely-UNCONSTRAINED parts of an assembly relative to
  each other (visual fit-probing BEFORE defining a mate — no solver
  involvement, purely a display transform on unmated parts),
- hiding parts in an assembly (see behind them).

One addition was ruled onto these four at v1 planning (Evan,
2026-08-27, `docs/GUI-PLAN.md`): **defining a mate between
previously-unmated parts**, ruled in because free-move fit-probing
exists precisely to precede it.

**Live dragging/editing of partly-constrained geometry is NOT on
the scheduled path.** Everything drag-shaped in this document (the
UI-ideas sketchpad below: wall-mode drag, solved-assignment drag
previews, in-flight drag bands in GQ1/G2) stays recorded as ideas —
none of it is v1 work, and a future GUI milestone that wants it
proposes it explicitly. This supersedes any earlier reading under
which drag interactions looked main-path; GQ1's witness/ReWitness
semantics remain ratified for whenever dragging DOES arrive (the
solver contract is not drag-specific — selection-driven edits use
the same doors).

Kernel-relevant consequences, so pre-GUI milestones leave room:
hiding and free-move are DISPLAY-layer state (never persisted into
the recipe; layer-3 per G1), and fit-probing transforms must be
visually distinguishable from mated placement (an honesty
requirement, not a solver one).

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

### Colour: the theme is a user preference, the document overrides it (Evan, 2026-08-30)

Two things set colour, and the precedence between them is one rule.

- **A theme is a USER preference.** It supplies every semantic mark —
  selection, hover, free-move probe, focus, unresolved — the *default*
  body colour, and the ambient term. It is **never written into a
  document**: the same file has to be legible to a colourblind reader
  and to somebody running the palette they find prettiest, on their
  own screens. It is therefore not persisted by `editor-core` and
  takes no part in any content key.
- **A document overrides the body colour.** `Attr::Color` on a stable
  name (M4 PR 7) is authored, persisted, and travels with the file.
  Where a document states a colour, the theme's default body colour
  gives way; the theme never overrides it back.

Both sides are `editor_core::appearance::Rgba8` — exact 8-bit sRGB —
so the override is a substitution within one colour space rather than
a conversion between two. Linear light is entered once, at each
renderer's own door.

**Colourblind legibility is a claim a theme makes, not a constraint on
every theme.** A palette may state that its marks stay mutually
distinguishable under dichromatic vision, and one that does is held to
it by simulation in `crates/viewer/tests/theme.rs`; a palette that
makes no such claim is not lesser for it, and is not checked. Shipping
both is the point — a palette chosen to be beautiful and a palette
chosen to be discriminable are different jobs, and the failure worth
preventing is a palette claiming the second job and not doing it.
Because the marks are *mixed over* the body colour rather than
replacing it, what any such check must measure is the composited
colour, never the raw tint.

**A palette also states its GROUND** — what fills the viewport where
no geometry is drawn (`Theme::ground`; added 2026-09-01 at Evan's
report that a light theme showed its parts on a black field). It has
to be the palette's rather than the toolkit's for the reason every
other colour here is: the pane is a custom pass that paints only what
the model covers, so a ground left unstated is a ground the window's
clear colour decides behind the palette's back — and it is the surface
every swatch is finally seen against, so it is held to the same
separation bar the marks are.

That bar is what moved `colorblind-safe` onto a LIGHT ground. Its mark
ladder puts two of four marks below the body — a deep blue hover and a
near-black probe — and on a dark ground those two are the swatches
that disappear; measured, no dark ground clears the bar at all, while
a near-white one clears it twice over. A palette whose marks run
downward needs a ground above all of them.

**Preferences are remembered in a file people can open**, at
`$XDG_CONFIG_HOME/pncad/viewer.toml` — hand-editable TOML, chosen over
eframe's `persistence` blob for exactly that reason. The document is a
value and the storage is one thin edge over a `String`, so the browser
build's `localStorage` store is a second impl rather than a retrofit;
until it exists the web arm reports `Absent` and disables saving, the
same posture `frame::chooser_backend` takes where no portal exists.

The failure posture is deliberately **softer than the document
path's**, because a preferences file holds no work: malformed TOML
refuses, an unknown KEY reports and the rest of the file still
applies, and an unknown VALUE reports and falls back to the default. A
name typed on the command line is refused instead of falling back —
same word, different provenance, different answer: a file is a memory
of an older session and may name a theme since renamed, where a
typed name is a typo worth showing.

## GQ items (GQ1–GQ5 RATIFIED and shipped — kept as the rationale record; GQ6's toolkit row RATIFIED 2026-08-16 and its remaining rows settled in the v1 GUI units; GQ7 slimmed by the SELECT-DESIGN re-homing, its remainder deferred to sketcher/tree design)

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
predicate's exact form) are M4/M10 design work under this committed
direction. Concrete audit item: ezpz (Q3) must satisfy bit-identity
(libm-only math, no hash-order effects) if its f64 path runs inside
`build`. Mechanism note carried up from M2 PR 3's adversarial
review (the "S2" lesson, via the PR #32 orchestrator, 2026-07-20):
the witness contract must pin **which** point is the witness — a
loose contract admits wrong-but-certified selections (M2 sharpened
edge witnesses to the mid-parameter point for exactly this reason);
the sketch-level witness has the same aliasing question and must
answer it explicitly.

*Mechanism since ratified in full: `docs/SOLVER-DESIGN.md` (#79,
W1–W9 — including the witness-aliasing answer this note demanded).
`editor-core` carries the contract types (`WitnessDatum`,
`BranchCertification`, `WitnessBifurcation`) and the
`ReWitness`/`ReWitnessBulk` edits; the solver itself remains
unbuilt as sequenced (M10).*

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

*Shipped (verified 2026-08-06): `editor-core::eval` —
`NodeResult::{Ok, Failed, Poisoned{through}}` per node, failures
poison descendants only while independent subgraphs complete, plus
memoized incremental re-evaluation (content/naming keys, epochs)
and `CancelToken` returning the completed prefix as a typed
outcome. Progress reporting and in-op yield points remain absent.*

### GQ3 (RATIFIED 2026-07-19 round 3): All edits persisted in v1

**Ratified**: `DocEdit`s are persisted from the first version —
removing/disabling persistence later is far easier than adding it
(and session-spanning undo, macros, and collaboration all want it).
Banked consequences: the edit schema enters Band 4's versioning
discipline from the first persisted file; storage shape is
**snapshot + edit log** (details at editor-core design time).

*Shipped: snapshot + edit log is the on-disk format (`schema: n`
header + JSON body); save verifies the log replays through `apply`
before writing, and load replays it after. Schema v1 landed at M4
PR 6, and the versioning discipline has since carried a series of
ratified pre-release clean breaks (LQ7a — no migration machinery
before release).*

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

*Status: the restrictive dimension answer shipped — `Dimension =
Length | Angle | Count | Scalar`, every constructor
dimension-checked, dimension-changing products refused, the AST
total and finite by charter. The units/display layer landed with
the library program (LIB U8a/U8b, as banked): the `quantity`
newtypes and unit table at the D6 API boundary, the expression TEXT
door (`editor-core::parse`), and stored display units that
round-trip `25 mm`. The v1 GUI panels sat on canonical
meters/radians by ruling; that ruling was superseded post-close
(2026-08-29) and the panels now render and author in the stored
display unit — see `docs/GUI-PLAN.md`'s units row and
`docs/GUI-LOG.md`'s tail. The unit table also gained a `pi rad` row
(half-turns), which is a NOTATION rather than a physical unit and
says so in `quantity::units`' module docs; it is the default written
unit for an angle that remembers none.*

### GQ6: Toolkit and platform (toolkit RATIFIED 2026-08-16; the remaining rows settled inside the v1 GUI units)

**The mandated re-survey was performed 2026-08-16 →
`docs/GQ6-RESURVEY.md`**, which supersedes the snapshot below as the
factual record.

**Toolkit (RATIFIED, Evan, 2026-08-16): egui — and if egui does not
work out, iced.** A default with a named fallback, not a tie to be
broken later by a bake-off. Deciding factors: egui tracks current
wgpu (30) where iced 0.14 pins 27 and has not released since
2025-12; the docking chrome a feature-tree + viewport + property
panel needs on day one exists in egui's ecosystem and is thin in
iced's; and rerun is a production existence proof of exactly our
shape (egui panels + wgpu viewport). iced's MVU fit with G1 is real
but is an argument about where the architecture *lives*, and G1's
architecture already lives in `editor-core`, below any toolkit —
which is also why the fallback is cheap: switching costs the
interaction layer and nothing beneath it. The conditions that would
trigger the fallback are written down in advance in the re-survey's
§5.

**Settled inside the v1 units** (`docs/GUI-PLAN.md`, evidenced in
`docs/GUI-EXIT-WALK.md`): the viewport is a thin custom wgpu pass
under eframe's wgpu renderer; picking is a GPU ID-buffer pass over
our own deterministic `Bvh::ray` rather than parry3d; the docking
chrome is `egui_tiles` (MIT OR Apache-2.0, and a `Tree<Pane>` value
the app owns); and the immediate-mode seam measured GO with no §5
fallback condition met, so the iced fallback is not in play for v1.
The browser lane is deferred post-v1; the wasm guard below keeps
the compile-level option green.

Two candidates left the slate for good: **Slint** (its only
OSI-approved branch is GPL-3.0-only, which cannot ship in an
MIT-OR-Apache-2.0 product; the royalty-free branch is a proprietary
grant — and its renderers give a wgpu viewport no first-class seat)
and **GPUI** (upstream states it lacks the resources to maintain it
standalone). bevy remains listed but demoted.

The other rows stay as the snapshot left them, with two corrections:
Fornjot was archived 2026-06 and CADmium 2025-09, so "no CAD-ness to
borrow" is now *stronger*; and the wasm row below is **wrong in our
favor** — the whole kernel plus `editor-core` compiles to
`wasm32-unknown-unknown` today, `--features interval` included
(measured, not surveyed). **Guarded** since #807, by **one** step on
every code-tier pull-request run — `cargo check --workspace --exclude
pncad --exclude pncad-py --features interval --target
wasm32-unknown-unknown`. That is the `--features interval` half
directly; the default-features half rides on it, because enabling a
cargo feature is additive for the dependency graph and
`scripts/check-interval-cfg-additive.py` forbids any
`cfg(not(feature = "interval"))` under `crates/*/src`, so the interval
build compiles a superset of the library sources. Under `crates/*/tests`
the gate holds a different rule and the negation is legitimate; that does
not reach this guard, whose step builds no test targets. **Evan's ruling, 2026-08-21:** *"do add wasm
cross compiling for the interval build only. the lint for having
interval be purely additive suffices."*

**Two limits, both inherited rather than introduced.** That lint is
**syntactic** — its header states it cannot see through a gated `mod`
whose contents are non-additive — so its residual is now this guard's.
And the guard is `cargo check`, so it establishes that the crates
*compile*, not that they link or run. `docs/GQ6-RESURVEY.md` §4 carries
the full row-by-row split, including the one row this leaves unguarded
(`pncad` under the `wasm_js` backend cfg) and the dated dependency-graph
measurement the subsumption rests on.

(Named by step and not by job on purpose: the job this first landed in
was deleted by the CI-minutes audit that landed the same day, which is
the drift this whole sweep is about. `local-scripts/ci-local.sh` carries
the `HOSTED MIRROR:` marker that `scripts/check-ci-mirror-parity.py`
checks, and that is the citation which cannot go stale silently.)

Ecosystem snapshot 2026-07 (SUPERSEDED by the re-survey — kept as
the record of what was believed when GQ6 was banked): the
**substrate** is reusable, the **CAD-ness** is not
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
lane wasm-friendly; the `interval` feature is not, per issue #4
— that last clause is FALSE as of the re-survey: #4 was met by
removal at M5 PR 1 and the interval lane builds on wasm) —
G1 is deliberately agnostic to it. `rerun` served as the
zero-effort debug viewer through M5 (now closed); as of 2026-08 the
demo/montage pipeline renders through FreeCAD offscreen, which is a
corpus oracle, not a viewer or editor-substrate candidate — no
viewer commitment has been made.

*(Snapshot explicitly dated: the toolkit, ezpz, and wasm rows are
as of 2026-07 and nothing in this snapshot binds — the
re-survey-first instruction was the decision. It was carried out
on 2026-08-16 and the toolkit row is now settled on top of it; see
`docs/GQ6-RESURVEY.md` for the current facts, the measured wasm
result, and the conditions that would trigger the iced fallback.)*

### GQ7: Selection mechanics (slimmed — the general-purpose clauses re-homed to `docs/SELECT-DESIGN.md` §4)

What stays a GUI question: multi-select UX — click/drag/modifier
mechanics, hover, which filters are offered where, and pick-priority
when a click hits several entities — plus the convention that
selection does **not** participate in document history (undo never
changes what is selected). v1 ruled single-select
(`docs/GUI-PLAN.md`); the rest waits on sketcher/tree design.
Selection filters, heterogeneous sets as values, and
survive-the-vanishing-entity semantics are library surface, owned by
`docs/SELECT-DESIGN.md` and the naming doc's resolution-failure
semantics.

**Pick-priority — the first concrete instance (recorded at GAUTH-2).**
Edge picking made the clause real: a face fills the pixel an edge only
borders, so an edge is unreachable without a rule that lets it win near
its own boundary. The rule taken is *proximity in the picture*, scoped
to the body the cursor is over: the cursor's ray picks a face first,
and an edge **of that face's own body** within
`viewer::pick::EDGE_PICK_RADIUS_PX`, not hidden by the solid, beats it.
Everywhere else the face wins, and off the body nothing wins — the rule
is not a global "nearest entity in the picture", and stating it that
way would promise a search this mechanism does not do. The constant is
named and lives in that one place, so a later instance of the same
question cites it rather than minting a second radius; the mechanism
(seeded by the face pick, occlusion-checked, deterministic) is the
implementation's business and is documented at its own door.

The clause's other half arrived with it: **a tool may narrow which
kinds it accepts**, because a rule that is right for a bare cursor is
wrong for a tool that can only use one kind — with edges always
winning, faces narrower than the radius became unpickable while the
mate tool was open (`viewer::pick::PickKinds`, issue #1379). What is
NOT settled here is the filter vocabulary: which filters are offered
where, and how a tool states what it wants, still wait on sketcher/tree
design with GAUTH-5's edges-only blend tool as the second data point.
Nothing here widens GQ7 — which entity wins is still the GUI's
question.

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

## Undo as a history tree, not a stack (concept — Evan, 2026-07-25)

Motivation (Evan's pet peeve, common to most editors): undo N steps,
make one edit, and the redo branch is silently destroyed — real work
lost. Better design: edit history is a TREE (a DAG of document
states); undo moves a pointer toward the root, a new edit after undo
mints a sibling branch, and the abandoned branch remains reachable
(branch picker / history-graph UI).

Why this is nearly free here, unlike in most editors: `Doc` is an
immutable value and every `DocEdit` is a recorded value (PR 1 —
"undo/redo falls out of values"), so the tree is just parent
pointers over states we already materialize. No git/gitoxide layer:
wrong granularity (file blobs + text diffs vs recipe edits), and we
already own the better primitives — content keys for addressing,
the PR 1 structural node-granular diff for showing what a branch
changed (a semantic diff between branches, something git can never
give us). CRDTs (automerge/yrs) tabled: they solve concurrent
merging, not single-user branching.

Persistence: the on-disk schema is a linear snapshot + edit log; the
history tree is the additive evolution (log entries gain a parent
pointer) via the F3 migration chain when the GUI needs it. v1 ships
the tree-shaped *state* under linear chrome (`viewer::history`: an
edit after undo mints a sibling, nothing is destroyed); the branch
picker and the sidecar are GUI-6.

Visualization sketch (Evan, 2026-08-27; non-binding like the rest
of this section): render the history as a graph with the linear
history running top to bottom; an edit made after an undo mints a
new child node placed to the right of the child that redo would
have reached. Sized in the same conversation at one-to-two work
units including the separable sidecar file, sequenced after GUI
v1 (`docs/GUI-PLAN.md` banks it as GUI-6).

### State/history separation (Evan, 2026-07-27 — the sharpened
### form of the git-like instinct)

What the undo-tree concept above still owed a name: Evan wants it
POSSIBLE to share a document's state without bringing its entire
history — the state and the edit DAG should be separable artifacts,
not one inseparable file. Design facts already in place: F3's
schema is snapshot + edit log, so "state without history" is
structurally just a save with a compacted (empty) log — an
export/compact operation, not a format change; the future history
TREE (parent-pointer log evolution) can live as a separable sidecar
rather than inside the shared document. What this notes for the
GUI milestone: sharing/compaction is a first-class operation
(explicit, loud about what it drops), and the history sidecar's
format is designed so the main document never depends on it.
Non-binding until a GUI milestone picks it up.
