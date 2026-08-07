# LIBRARY-DESIGN: the usable-as-a-library program

Status: **DRAFT — design conversation OPEN** (started 2026-08-06 at
Evan's request; nothing here binds until ratified; the PR carrying
this doc waits for Evan's sign-off per the standing rule for design
conversations). This doc turns DESIGN.md's "Beyond the kernel"
sequencing item — *"usable as a library" ships before any GUI work
begins* — from a scoping paragraph into a designed program.
Proposals are stated firm per house style; every one of them is up
for pushback.

Evidence base: a code survey of the demo corpus (`demos/tour/`,
~5.5k lines, 18 scene modules), the step-export/editor-core test
corpora, and `editor-core`'s verified public surface, executed
2026-08-06. The pain table in §L2 cites file:line; the claims are
from reading the code, not the docs.

## L1. What "usable as a library" means (proposed firm)

Deliverable: a person who is not us can `cargo add` / `pip install`
the kernel and author, evaluate, measure, and export parametric
models from documentation alone, without reading kernel source.
Four legs:

1. **A Rust authoring façade** — one crate to depend on, a prelude,
   f64-first entry points (name pending Q9; placeholder acceptable).
2. **Python bindings** — the CadQuery/build123d audience named in
   DESIGN.md's sequencing stance.
3. **The authoring-ergonomics units the evidence demands** — PATHS
   implementation first among them (its status line now records
   this program as its natural slot).
4. **Docs, tutorials, and the corpus-as-examples.**

Non-goals of this program: the GUI (sequenced after, per DESIGN.md);
assemblies (Band 3, own design era); new feature breadth (Band 3's
list — this program makes what exists usable; the two interact only
where a missing affordance makes existing features unusable, e.g.
the mirror gap in §L2/P6).

## L2. The evidence: what authoring costs today

Structural facts first:

- **No façade exists.** `demos/tour` depends by path on ELEVEN
  kernel crates; its Cargo.toml documents a leak (`SurfaceKind` is
  a `topo` error payload that `topo` does not re-export, so
  matching on a refusal means depending on `geom-brep` directly).
  The de-facto authoring surface is `profile` + `sweep` + `topo` +
  `geom-core`/`geom-curves` raw.
- **Two incompatible 2-D vocabularies.** Extrude/revolve/fillet
  take `ProfileLoop<T>` (vertices + bulge + declared tangent
  joints, with `LoopBuilder` sugar — and, positively, NO demo ever
  sets a tangency flag by hand; all tangency arrives constructively
  through `fillet`/`fillet_corner`). Loft/sweep instead take
  `SectionSegments` = `Vec<Vec<SketchSegment<f64>>>`: every segment
  carries BOTH endpoints, so each interior vertex is typed twice
  and closure is checked by exact float `==` (`skin.rs:832`) —
  re-typed coordinates that must value-match, the exact disease the
  authored-once principle exists to prevent. It is also hardcoded
  `f64`, and has no builder, so the tour and the step-export corpus
  hand-roll byte-identical `quad()` helpers.
- **The document layer is bypassed.** One of 18 tour scenes authors
  through `editor-core` (heatsink); every other scene goes straight
  at the kernel. Three executed reasons: (i) profiles are OPAQUE to
  the document (`ProfileDesc(pub Profile<f64>)`; the
  expression/dimension layer reaches distances, angles, counts —
  never a sketch coordinate); (ii) vocabulary gaps (a Boolean node
  cannot consume a Pattern node's `Instances` payload — heatsink's
  union honestly lives outside the document); (iii) per-node insert
  verbosity (apply → doc → record.minted, hand-threaded).
- **Bodies exist in triplicate.** Several models live three times —
  tour scene, step-export fixture, editor-core corpus doc — with
  the constants re-typed each time, and the fixtures say so
  ("constant for constant", "VERBATIM").

The pain table (file:line into the surveyed tree):

| # | Pain | Site (representative) | Remedy (§L5) |
|---|---|---|---|
| P1 | Path-start frame (Gram–Schmidt + degenerate-axis dodge) hand-rolled at every `sweep_body` call; wrong cross order = silently skewed sweep | `skinned.rs:476-488`, `:162-171`, corpus twin | U4 |
| P2 | No exact 3-D arc path primitive: an S of two exact quarter-circles authored as 17 hand-indexed interpolation samples; exactness degraded to "approached" | `skinned.rs:297-308` | U4 |
| P3 | Hand-computed constants with offline derivations living in comments; nothing can ask the model what the parameterization chose | `skinned.rs:376-388`, `az.rs:47-51`, `letterforms.rs:174-187` | U5 |
| P4 | 16-digit coordinates transcribed as literals that must value-match (lily joint pins re-derived outside the codebase) | `lily.rs:758-768`, `:810-811` | U2/U5 |
| P5 | The 1/16 "decoupling" rule forces near-duplicate coordinate tables per letterform; nothing checks they differ only where intended | `letterforms.rs:121-171`, `az.rs:85-130`, `projectbox.rs` | U6 |
| P6 | Manual axis-angle placement with antiparallel special-casing; no point-at affordance; NO MIRROR anywhere (lily's three leaves placed by hand) | `diefillet.rs:98-122`, `lily.rs:248-276`, `:617-636` | U4 |
| P7 | World placement expressible only as a sketch-frame choice; sign/handedness errors yield a valid solid in the WRONG PLACE (lily finding 11; the 60-line user-space `Turtle` exists to compensate) | `lily.rs:147-195`, `:68-124` | U4 |
| P8 | Per-scene boilerplate: six near-identical `p2` helpers, five `validated` wrappers; `S::from_f64` tax on every literal | `bodies.rs:20-35` et al. | U1 |
| P9 | `flush_declarations` — 50 lines of demo code enumerating plane pairs to declare flush contact, duplicated from the kernel's own test support; every user gluing flush parts must reproduce it | `booleans.rs:67-117` | U6 |
| P10 | Fillet target selection = writing a topology query (filter edges by carrier kind × adjacent surface kinds); order-dependent, re-run against post-boolean topology | `diefillet.rs:198-233` | U7 |

The corpus also shows what already WORKS and must not regress: the
constructive fillet/tangency discipline (nobody hand-writes flags),
the tiered validation ladder as the user journey, and the
mass-properties/mesh/STEP cross-check ribbon (§L6).

## L3. The layer decision (proposed firm): bindings wrap the document layer; Python is a generator language

G1 (GUI-DESIGN, ratified) names the edit vocabulary as **the single
API surface** shared by the GUI, language bindings, macro
recording, and headless tests. This program takes that commitment
literally:

- **Python speaks `Doc`/`DocEdit`/`evaluate`/persist.** Never an
  arena key — the same boundary rule as G1's layer 3. Evaluation
  returns the GQ2 per-node result DAG as typed values; failures are
  typed payloads (Python exceptions carrying the structured error,
  never strings); documents persist as the same schema-v3 files the
  future GUI will read, so undo, macros, and session-spanning
  history are free for Python users the day the bindings exist.
- **Python authoring sugar emits recipe data.** D8's stance is that
  the host language generates recipes; Python becomes a host
  language. The builder vocabulary (the PATHS algebra above all)
  lives as a thin Python surface over the same Rust lowering — one
  semantics, two host languages.
- **No parallel direct-at-kernel binding surface.** The survey
  shows demos bypass documents for reasons that are GAPS, not
  preferences; binding the bypass would freeze the gaps into public
  API and fork the surface G1 unified. The one-shot user ("build a
  bracket, export STEP") is served by a small document — sugar can
  make that a few lines — not by a second API.

**The load-bearing consequence**: for parametric value to reach
Python, the document layer must stop being opaque to sketch
geometry. The ladder:

1. **PATHS v1 (unit U2)**: the algebra implemented as the ratified
   generator surface, lowering to `Profile` values. Authored-once
   ergonomics land immediately in both host languages; profiles
   still enter documents as opaque payloads (F4 unchanged).
2. **Profiles-as-programs v2 (the #104 recorded commitment)**: the
   program becomes the profile's definition; sketch coordinates
   join the expression/dimension layer; the F3 migration lifts v1
   documents (declared junctions → `.tangent()` calls, fillet arcs
   → `.fillet(r)` — the flags pin the lift, as PATHS records).
   This is its own ratification gate mid-program (LQ4), not a
   prerequisite for shipping useful bindings.

Prerequisite completions the survey surfaced (small, kernel/editor
side): the Boolean×Pattern payload gap; insert ergonomics (a
builder over `apply` that threads minted node ids). Both are
already-known shapes, not research.

## L4. The Python type story (proposed firm)

Two-layer checking, per Evan's directive (in-chat, 2026-08-06):
**static checking via `ty`, runtime checking at the user-input
boundary only.**

- **The PATHS lattice renders as distinct stub classes** —
  `PathOpen`, `PathPoint` (plain vs directed as distinct types),
  `PathAngle`, `PathDirected` — each exposing only its legal
  continuations, so a double director, `.tangent()` on a plain
  point, or a leading `.fillet` is a static type error under `ty`,
  not a runtime surprise. This mirrors PATHS §5 exactly: off-lattice
  states unreachable through the surface — there via typestate,
  here via stubs a type checker enforces.
- **Runtime checks live at the Rust boundary, once.** Binders and
  constructors validate their arguments where data crosses into
  Rust (the same typed geometry refusals as the Rust surface:
  junction checks, `NoCornerForFillet`, …); the Python layer never
  re-verifies internally and never grows `isinstance` ladders.
  Python users who ignore `ty` get the same fail-loud typed errors,
  just at runtime — degraded ergonomics, identical semantics.
- **Typed quantities at the boundary** (GQ5's second half, whose
  first consumer is this program, not the GUI): `25 * mm`
  constructs a Length; dimensions are distinct Python types
  (Length/Angle/Count — the small closed set, per D6's stance), so
  dimension errors are both `ty`-visible and construction-time.
  Canonical meters/radians underneath, exactly as GQ5 ratified.
- **Stubs are CI-checked against the PyO3 surface** — drift between
  the `.pyi` lattice and the compiled module is a red build, not a
  documentation bug.

## L5. The unit ladder (proposed; ordering constraints only where real)

- **U1 — façade crate + prelude.** One dependency, re-exports, the
  error-payload types reachable (closes the `SurfaceKind` leak),
  f64-first authoring signatures (the `S::from_f64` tax is paid
  once inside the seam; generic instantiation remains the kernel's
  interior). Kills P8.
- **U2 — PATHS v1 implementation.** The ratified algebra as spec'd,
  lowering to the v1 form; `LoopBuilder` remains the raw layer it
  verifies against. Kills the profile-level re-typing class (P4's
  profile half); makes corner/anchor work structural.
- **U3 — profile-vocabulary unification.** Loft/sweep sections move
  to the `ProfileLoop` form (or a shared successor); the
  double-typed-endpoint `SectionSegments` form and its `==` closure
  check retire. One profile vocabulary means U2's algebra serves
  all four body ops. Kernel-side unit; needs its own measured spec.
- **U4 — path & placement vocabulary.** Exact 3-D path legs (line,
  arc; the long-turn arc is #222's banked frontier and lands
  there), so P2's sampled quarter-circles become exact values; a
  kernel door for the path-start frame (P1's Gram–Schmidt recipe,
  written once, with the degenerate-axis policy stated); pose
  values with point-at and MIRROR placement doors (P6/P7 — the
  lily findings are the acceptance evidence); loft placements as
  checked (section, place) pairs, not parallel arrays.
- **U5 — read-back/interrogation doors.** The model answers what
  the parameterization chose: skin parameters, joint frames,
  named-entity poses via stable names. Kills the
  transcribed-decimal pin class (P3/P4) — a pin becomes a query
  plus an assertion, inside the system.
- **U6 — declared-relation ergonomics.** The flush-declaration
  helper promoted from its demo/test-common twins (P9) into the
  library surface, per the coincidence ladder (Banked principles)
  and CONTACT-DESIGN's vocabulary; a declared-offset relation
  ("these tables differ by exactly 1/16 where stated") to replace
  P5's hand-maintained duplicate tables.
- **U7 — selection ergonomics.** Name/selector-based feature
  targeting over editor-core's StableName + enumerate-all machinery
  (the M6-5 vocabulary), replacing P10's raw topology queries in
  user code.
- **U8 — GQ5 completion.** Units/display layer + the expression
  text parser (none exists — the AST is construction-only today);
  round-tripping `25 mm` lands here because bindings need it before
  any GUI does.
- **U9 — bindings proper.** PyO3/maturin, abi3 wheels, f64 lane
  first (interval/dual lanes are the M8 payoff and join later
  behind an extra; the evaluation service is already generic over
  `Real` by banked principle, so the door is structural). D9's
  pure-libm determinism means wheels replay BIT-IDENTICALLY across
  platforms — a headline library property; pin it with a doctest.
- **U10 — docs/tutorials/examples.** The corpus is the example set;
  the tour's `run_body` ladder (validate → measure → tessellate →
  cross-check → export) documented as the canonical user journey;
  the Band 4 docs-and-onboarding item lands here.

Real ordering constraints: U1 before U9 (the façade is what gets
bound); U3 before U2 covers loft/sweep (U2 can land for
extrude/revolve first — the algebra targets `ProfileLoop`, which
they already speak); U8 before U9's quantity surface. Everything
else is schedulable freely, including in parallel with M8 kernel
work — footprints are disjoint (survey: no demo touches Euler ops;
the units here touch authoring crates, editor-core, and a new
bindings crate).

## L6. What this program must not regress

- **Constructive tangency**: no demo hand-writes a tangency flag
  today; every new surface keeps declaration-by-construction.
- **Fail-loud**: bindings translate typed errors, never soften
  them; no silent defaults at the Python boundary.
- **The validation ladder as the journey**: tiers 1→2→3/3′,
  mass properties with certified pads, mesh cross-check, STEP
  export — the library documents this ladder rather than hiding it.
- **D9 replay**: nothing in the bindings may introduce
  hash-order/platform effects inside `build`-equivalent paths (the
  GQ1 audit note generalizes: anything that runs inside evaluation
  satisfies bit-identity).

## L7. Open questions

- **LQ1 — the façade's name**: couples to Q9 (project name).
  Placeholder crate name acceptable to start, per the standing Q9
  posture?
- **LQ2 — U3's shape**: retire `SectionSegments` outright, or keep
  it as the internal lowered form under a shared authoring type?
  Needs a measured kernel spec (consumer census exists in the
  survey).
- **LQ3 — U4's landing site**: exact path legs as `geom-curves`
  constructors, `sweep` vocabulary, or both? And does the pose/
  point-at/mirror family live in `topo::transform` or the façade?
- **LQ4 — v2 profiles-as-programs timing**: mid-program ratification
  gate as proposed, or pulled to the program's front so Python
  never ships the opaque-profile intermediate state?
- **LQ5 — program vs milestone sequencing**: DESIGN.md places the
  usability program post-M8. Proposal: this DESIGN CONVERSATION
  concludes now; implementation units interleave after the M7 exit
  walk where footprints are disjoint from M8, at Evan's per-unit
  discretion — the alternative (strict post-M8) stays available.
- **LQ6 — Python surface breadth at v1**: documents-from-day-one
  (proposed, per L3) vs an authoring-only first release. The
  proposal costs the prerequisite completions in L3; the
  alternative ships sooner and forks the surface temporarily.
- **LQ7 — distribution/versioning**: wheel cadence; pre-1.0 semver
  posture; whether persisted-schema versions couple to package
  versions or stay independent (they are user-visible compatibility
  promises either way).
