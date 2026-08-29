# LIBRARY-DESIGN: the usable-as-a-library program

Status: **RATIFIED** (design conversation opened 2026-08-06 at
Evan's request; first-round rulings in-chat the same day, recorded
at §L7; Evan's sign-off 2026-08-06 in-chat, merged as PR #229).
Recorded open residue: LQ7's tail (wheel cadence, post-release
schema/package version coupling — deferred to implementation
time). LQ3 was ratified 2026-08-10 (#362) — see its entry in §L7. This doc turns DESIGN.md's
"Beyond the kernel" sequencing item — *"usable as a library" ships
before any GUI work begins* — from a scoping paragraph into a
designed program.

Evidence base: a code survey of the demo corpus, the
step-export/editor-core test corpora, and `editor-core`'s verified
public surface, executed 2026-08-06 (§L2) — the claims are from
reading the code, not the docs.

**The §L5 unit ladder is SHIPPED except U4's tail**: every other
unit merged (U6 dispatched inside SELECT-DESIGN rather than as its
own unit, per LB11, which also records its declared-offset half as
out of scope), and of U4, LQ3(b)'s composition door and
LQ3(c)'s frame family landed while LQ3(a)'s open-chain path
vocabulary and the `wire_sweep`/`Node::Tube` discharge stay the
named design conversation. The live residuals are the register at
the tail of `docs/LIB-LOG.md`, whose largest standing item is the
assembly surface: authorable through the façade, still unbound in
`pncad-py` for AUTHORING — the seam it is reached through is bound
(LIB-G18a: `evaluate(doc, resolver=store)`, so an assembly document
loaded from a workspace evaluates), and no door that WRITES one is.
What follows is the program's design, not its status.

## L1. What "usable as a library" means

Deliverable: a person who is not us can `cargo add` / `pip install`
the kernel and author, evaluate, measure, and export parametric
models from documentation alone, without reading kernel source.
Four legs:

1. **A Rust authoring façade** — one crate to depend on, a prelude,
   f64-first entry points (name pending Q9; placeholder acceptable).
2. **Python bindings** — the CadQuery/build123d audience named in
   DESIGN.md's sequencing stance.
3. **The authoring-ergonomics units the evidence demands** — PATHS
   implementation first among them.
4. **Docs, tutorials, and the corpus-as-examples.**

Non-goals of this program: the GUI (sequenced after, per DESIGN.md);
assemblies (Band 3, own design era); new feature breadth (Band 3's
list — this program makes what exists usable; the two interact only
where a missing affordance makes existing features unusable, e.g.
the missing mirror affordance).

## L2. The evidence

The program was scoped from a code survey of the demo corpus, the
step-export/editor-core test corpora, and `editor-core`'s public
surface (2026-08-06), which measured four structural costs: no
façade (the tour depended by path on eleven kernel crates, with a
leaked error payload forcing a twelfth); two incompatible 2-D
vocabularies (`ProfileLoop` for extrude/revolve/fillet against
`SectionSegments` for loft/sweep, the latter double-typing every
interior vertex and checking closure by float `==`); a document
layer bypassed by all but one scene, because profiles were opaque to
it; and bodies authored in triplicate across tour, fixture and
corpus. Ten specific authoring pains were tabulated with file:line
sites, each mapped to the unit below that would kill it.

*The table itself is retired.* It tracked nothing live once its
units shipped; the surveyed line numbers no longer resolve, and what
is genuinely still open is the residual register at the tail of
`docs/LIB-LOG.md` — the one place that tracks it. The survey is in
this file's git history if the original sites are ever wanted.

What the corpus showed already WORKS, and must not regress, is a
standing constraint rather than a finding: the constructive
fillet/tangency discipline (nobody hand-writes flags), the tiered
validation ladder as the user journey, and the
mass-properties/mesh/STEP cross-check ribbon (§L6).

## L3. The layer decision: bindings wrap the document layer; Python is a generator language

G1 (GUI-DESIGN, ratified) names the edit vocabulary as **the single
API surface** shared by the GUI, language bindings, macro
recording, and headless tests. This program takes that commitment
literally:

- **Python speaks `Doc`/`DocEdit`/`evaluate`/persist.** Never an
  arena key — the same boundary rule as G1's layer 3. Evaluation
  returns the GQ2 per-node result DAG as typed values; failures are
  typed payloads (Python exceptions carrying the structured error,
  never strings); documents persist as the same files the GUI
  reads, so undo, macros, and session-spanning history are free
  for Python users the day the bindings exist.
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
geometry. **Ruled (Evan, LQ4): the v2 switch is pulled to the FRONT
of the program** — Python never ships the opaque-profile
intermediate state. The front-loaded arc, still two sequenced
steps:

1. **PATHS implementation (unit U2)**: the algebra implemented as
   the ratified generator surface, lowering to `Profile` values —
   authored-once ergonomics land immediately in both host
   languages.
2. **Profiles-as-programs v2 (the #104 recorded commitment),
   immediately after**: the program becomes the profile's
   definition; sketch coordinates join the expression/dimension
   layer. Both steps precede U9 — bindings ship against the v2
   representation from day one. Note the LQ7 ruling's consequence:
   pre-release, the switch may land as a CLEAN BREAK (the in-repo
   corpora regenerate); the mechanical v1→program lift PATHS
   describes (flags pin the constructors) remains available as a
   tool, not owed as a compatibility promise.

Prerequisite completions the survey surfaced (small, kernel/editor
side): the Boolean×Pattern payload gap; insert ergonomics (a
builder over `apply` that threads minted node ids). Both are
already-known shapes, not research.

## L4. The Python type story

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

## L5. The unit ladder (ordering constraints noted only where real)

- **U1 — façade crate + prelude.** One dependency, re-exports, the
  error-payload types reachable (closes the `SurfaceKind` leak),
  f64-first authoring signatures (the `S::from_f64` tax is paid
  once inside the seam; generic instantiation remains the kernel's
  interior). Kills the per-scene boilerplate class — the repeated
  `p2`/`validated` helpers and the `S::from_f64` tax on every
  literal.
- **U2 — PATHS implementation, v2-fronted per LQ4.** The ratified
  algebra as spec'd, lowering to the v1 form, with the
  profiles-as-programs representation switch following immediately
  (§L3's front-loaded arc); `LoopBuilder` remains the raw layer the
  lowering verifies against. Kills the profile-level re-typing
  class at the profile level — transcribed 16-digit coordinates
  that must value-match; makes corner/anchor work structural.
- **U3 — profile-vocabulary unification.** RULED (Evan, LQ2):
  retiring `SectionSegments` as an authoring surface IS the goal —
  one profile vocabulary, so U2's algebra serves all four body ops;
  loft/sweep sections move to the `ProfileLoop` form (or the v2
  program form directly, given LQ4). Whether a vestigial internal
  lowered form survives inside `sweep` is for this unit's measured
  spec, not a design commitment. The double-typed-endpoint form and
  its `==` closure check leave the public surface either way.
- **U4 — path & placement vocabulary.** Exact 3-D path legs (line,
  arc; the long-turn arc is #222's banked frontier and lands
  there), so sampled quarter-circles become exact values; a kernel
  door for the path-start frame (the Gram–Schmidt recipe written
  once, with the degenerate-axis policy stated); pose values with
  point-at and MIRROR placement doors, since manual axis-angle
  placement with antiparallel special-casing puts a valid solid in
  the wrong place (the lily findings are the acceptance evidence);
  loft placements as checked (section, place) pairs, not parallel
  arrays.
- **U5 — read-back/interrogation doors.** The model answers what
  the parameterization chose: skin parameters, joint frames,
  named-entity poses via stable names. Kills the
  transcribed-decimal pin class — a pin becomes a query plus an
  assertion, inside the system, instead of a hand-computed constant
  whose derivation lives in a comment.
- **U6 — declared-relation ergonomics.** The flush-declaration
  helper promoted from its demo/test-common twins into the library
  surface, per the coincidence ladder (Banked principles) and
  CONTACT-DESIGN's vocabulary — every user gluing flush parts
  otherwise reproduces 50 lines of plane-pair enumeration; and a
  declared-offset relation ("these tables differ by exactly 1/16
  where stated") to replace hand-maintained duplicate coordinate
  tables.
- **U7 — selection ergonomics.** Name/selector-based feature
  targeting over editor-core's StableName + enumerate-all machinery
  (the M6-5 vocabulary), replacing raw topology queries in user
  code — filtering edges by carrier kind against adjacent surface
  kinds, order-dependent and re-run against post-boolean topology.
- **U8 — GQ5 completion.** Units/display layer + the expression
  text parser (none exists — the AST is construction-only today);
  round-tripping `25 mm` lands here because bindings need it before
  any GUI does.
- **U9 — bindings proper.** PyO3/maturin, abi3 wheels, f64 lane
  first (interval/dual lanes are the M10 payoff and join later
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
else is schedulable freely, including in parallel with the kernel
milestones — footprints are disjoint (survey: no demo touches Euler ops;
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

## L7. Questions — rulings (Evan, in-chat, 2026-08-06) and what stays open

- **LQ1 — the façade's name: RULED.** The façade crate carries the
  eventual project name — one name for the project and its entry
  crate (Q9 decides the name itself; placeholder until then, per
  the standing Q9 posture).
- **LQ2 — U3's shape: RULED in direction.** Retiring
  `SectionSegments` as an authoring surface is the goal — Evan's
  framing: that retirement is what the PATHS program is FOR at the
  loft/sweep seam. The internal-form residue question goes to U3's
  measured spec (see U3).
- **LQ3 — U4's landing site: RATIFIED.** The question it settled:
  exact path legs as `geom-curves` constructors vs `sweep`
  vocabulary vs both, and where the pose/point-at/mirror family
  lives.

  **RATIFIED (Evan 👍 on #362's sign-off comment,
  2026-08-10, with the resonance amendment below folded; M8
  orchestrator's kernel-side concurrence on (b) recorded on the
  thread).** The proposal as ratified: The 2026-08-10 substrate survey pins the walls:
  `sweep_body` consumes ONE `NurbsCurve3<f64>`; every scene
  hand-samples its path (17-point S-curve, interpolate degree 3)
  and hand-rolls the start frame (Gram–Schmidt with a
  `n.z.abs()<0.9` dodge); `Node::Sweep` already spells the
  path as a PROFILE NODE whose first loop's chain is the
  trajectory; and `wire_sweep` refuses everything via
  `SWEEP_FRONTIER` because a profile chain is many segments and
  §10.4 wants one curve — the banked "joined-path composition
  lane". So LQ3 is really three sub-questions:

  (a) **Where do exact 3-D path legs live?** RATIFIED: in the
  `profile`/PATHS layer as an OPEN-CHAIN vocabulary (the 2-D
  algebra's junction discipline, minus closure), because the
  document layer has already committed to "path = profile node" —
  a second path type would fork the surface G1 unified. NURBS
  legs stay VQ7-banked.

  (b) **Where does chain→curve composition live?** RATIFIED: a
  `geom-curves` door (exact C¹ join of line/arc/nurbs legs into
  one curve — the §10.4 consumer's own vocabulary), which is
  ALSO the discharge site for the banked SWEEP_FRONTIER: with
  the door in place, `wire_sweep`'s refusal narrows from
  everything to genuinely-unjoinable chains. This un-banking is
  kernel-side work and needs the kernel program's concurrence
  (coordinate with the M8/ASM side; it is not a LIB unit to
  self-authorize).

  (c) **Where does the pose/point-at/mirror family live?**
  RATIFIED: frame CONSTRUCTORS in `geom-core` (point-at, mirror,
  path-start frame with the degenerate-axis policy stated, written
  once), consumed by `SketchPlane::from_frame`
  and loft/sweep placements as plain `Affine3` values;
  document-level Expr-ized placement stays deferred (VQ8's pose
  conversation), so no schema change rides this unit.
  **Amendment (Evan, #362, 2026-08-10): resonance with the PATHS
  placement vocabulary is REQUIRED.** The 2-D algebra already
  has rigid placement and mirroring (`nurbs(curve)` places a
  curve value rigidly; `nurbs_reversed`/`nurbs_mirrored` are the
  structural variants — reflection across the departure line,
  curvature signs flip). The 3-D frame family must use the SAME
  TERMS for the same concepts (mirror means reflection with the
  stated orientation consequence; placement means rigid, no
  scale/deform) so the two surfaces read as one vocabulary;
  outright unification only if it falls out naturally — not
  worth forcing (Evan's stated guess), and U4's spec must SAY
  which of the two it did.

  Consequence: U4 becomes two dispatchable units (path legs +
  composition door; frame-constructor family), the sweep audit
  rows (15–18) get a real path to YES, and `Node::Tube`'s schema
  bump remains a separate coordination item.
  Alternatives considered and why not: path legs as raw
  `geom-curves` constructors only (no junction discipline — the
  exact re-typing class PATHS exists to kill); path legs as
  `sweep`-local vocabulary (invisible to the document layer,
  contradicting Node::Sweep's existing spelling).
- **LQ4 — v2 profiles-as-programs timing: RULED — pulled to the
  front.** See §L3's front-loaded arc and U2. Python never ships
  the opaque-profile intermediate state.
- **LQ5 — sequencing: RULED.** Implementation units run IN PARALLEL
  with the kernel milestones where footprints are independent, at
  Evan's per-unit discretion — this program is not sequenced behind
  them. (DESIGN.md's roadmap carries the ruling.)
- **LQ6 — Python surface breadth at v1: RULED —
  documents-from-day-one.** The L3 prerequisite completions are
  accepted as program scope.
- **LQ7 — distribution/versioning: RULED in part.** (a) NO
  backwards-compatibility machinery of any kind before release —
  no migration chains, no deprecation shims; pre-release breaks are
  clean breaks (consistent with the persisted-schema clean-break
  precedent, and with the §L3 note on the v2 switch). (b) Version
  numbers RESET immediately before release; internal version
  numbers before that are free to burn. Remaining open, no strong
  opinion recorded, defer to implementation time: wheel cadence,
  and whether schema versions couple to package versions
  post-release.

## L8. Second-round rulings (Evan, in-chat, 2026-08-06, at program start)

Recorded by the program orchestrator; operational detail in
`docs/LIB-LOG.md`.

- **LQ5 execution — first authorized batch**: U1 and U2 start now,
  in parallel with the M6/M7 close-out (footprints verified
  disjoint from that runway). Units past U2 are delegated to
  orchestrator judgment where footprints are independent; genuine
  design forks still escalate per the standing model.
- **Façade placeholder name**: `pncad` until Q9 ratifies the real
  name. At rename time, grep for `pncad` AND audit `cad`
  occurrences broadly — many of the latter (workspace paths, repo
  name, doc prose) should become the real name too.
- **v2 profiles-as-programs spec timing (§L3 arc, sharpened)**:
  the representation-switch design conversation is drafted AFTER
  U2's algebra is implemented and the demo corpus is reworked
  onto it — the rework is the evidence for what the
  representation should be (which constructor arguments want
  expression-layer binding, what the programs actually look like
  at corpus scale). The §L3 commitment is unchanged: the switch
  still precedes U9, and Python never ships the opaque-profile
  intermediate state.
- **A/B protocol**: library-program implementation dispatches
  draw from their own LIB-labeled block series in
  `docs/MODEL-AB-LOG.md`, so the two concurrent orchestrators
  never consume the same draw.
