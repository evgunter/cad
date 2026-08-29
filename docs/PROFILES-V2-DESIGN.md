# PROFILES-V2-DESIGN: profiles as programs — the representation switch

Status: **RATIFIED** (design conversation PR #242, three rounds with
Evan 2026-08-08: drift-proofing construction accepted round 1;
round-2 probe retracted the bowtie claim; round 3 delegated VQ1 to
orchestrator judgment — **RULED (b)-direct**, see §V7 VQ1 for the
ruling and rationale). VQ2–VQ9 reviewed in detail and
AGREED by Evan (PR #242 round 4, 2026-08-08) — full sign-off, not
retroactive. Sequencing consequence: the
vocabulary-growth units precede the switch; U9 queues behind it
(Evan: no hurry). Drafted per LIBRARY-DESIGN §L8 ruling 3 on the
merged evidence of #233/#238. House register: proposals firm where
the evidence decides them; the V4 option analysis is retained as
the record of the central decision.

## Grounding (committed; this doc does not re-litigate)

- **The program is the profile's definition; derived segments are
  caches/provenance** — #104's recorded v2 commitment ("profiles-as-
  programs is the DECLARED END STATE, not an option"), affirmed in
  PATHS-DESIGN's harmonization paragraph ("the algebra IS the intended
  core representation of paths … exactly as Q8 definitional surfaces
  work — the constructing function is the surface").
- **Sketch coordinates join the expression/dimension layer** so
  parametric value reaches Python (LIBRARY-DESIGN §L3: "the document
  layer must stop being opaque to sketch geometry"; LQ4 ruled the
  switch to the front — Python never ships the opaque-profile state).
- **Clean break, pre-release** (LQ7a): no migration machinery, no
  shims; the in-repo corpora regenerate. The mechanical v1→program
  lift is a tool, not a compatibility promise.
- **Sequencing**: the switch precedes U9 bindings.

Evidence base, cited throughout: the U2 PR-1 implementer report
(~/.local/share/cad-work/lib-u2-pr1-report.md — findings F1–F11) and
the U2 PR-2 corpus-rework report (lib-u2-pr2-report.md — §6 evidence,
walls W1–W6, the per-loop disposition census); the code as merged
(`crates/profile/src/path.rs`, `crates/editor-core/src/{expr,node,
profile_desc,persist/wire}.rs`); PATHS-DESIGN.md; DESIGN.md D8/D9/Q8;
LIB-LOG's v2 evidence accumulator.

## V1. What a stored profile-program IS

**A profile-program is the constructor-call sequence as data**: an
ordered list of program steps, one per algebra verb, each step a tag
plus its authored arguments — one tag per row of PATHS-DESIGN §3's
vocabulary table (`At`, `Angle`, `Toward`, `Line`, `ArcTo{spec}`,
`Fillet{r}`, `CloseTo`, `Circle`, …), exactly the Tier-0 core plus
the Tier-1 sugar's *expansions* (V7/VQ5 for what sugar stores). A
profile remains `plane + loops`; each loop's payload becomes one
program.

**Typestate flattens by erasure, and re-arms by replay.** The lattice
markers (`Tip<P, A>` — path.rs §5 representation) are a compile-time
property of the *authoring surface*, not of the data: the runtime state
is already one struct (`pos: Option<PosData>, ang: Option<T>`), and
PATHS §5 says outright that off-lattice states are "representable at
runtime but unreachable through the surface". The stored form is the
erased step list. On load and on every evaluation, the step list is
**replayed through a runtime-checked driver that mirrors the lattice**:
every transition the markers forbid statically is a typed refusal
dynamically (double director, leg from a half-bound tip, `.tangent()`
on a plain point, leading fillet, mid-replay `To(Start)`…). This is
the repo's own precedent applied verbatim: `WireExpr` persists the
plain AST and is REBUILT through the dimension-checking smart
constructors on load, so "a corrupt or hand-edited file can never
smuggle an ill-dimensioned tree past the construction door"
(persist/wire.rs module docs). The profile-program gets the same door.

**Where can a PathError-refused program exist at rest?** Two classes,
deliberately different:

1. **Lattice violations** (the statically-ill-typed class) can exist
   only in a hand-edited or corrupt file — no authoring surface can
   produce them. The replay driver refuses them typed at load
   (strict-door philosophy: a non-canonical file is a corrupt file).
   They are never a document state.
2. **Geometry refusals** (`JunctionTangent`, `NoCornerForFillet`,
   `AnchorOutsideTrimmedExtent`, `NonpositiveLeg`, …) CAN exist at
   rest, necessarily: once arguments carry expressions (V2), whether
   the program elaborates cleanly depends on the parameter binding.
   A program that runs at `r = 0.5` may refuse at `r = 5.0`. The
   stored form therefore admits programs that fail elaboration under
   the current binding; the failure surfaces as a typed evaluation
   error on the node (`NodeErrorKind`), exactly like an extrude whose
   distance expression goes nonpositive. This is fail-loud working as
   designed, not a representation defect — and it is the honest cost
   of the commitment: v1's stored segments could not fail to exist;
   v2's stored programs can. (Precedent: Fillet nodes already refuse
   at evaluation when an upstream edit removes a selected edge.)

**Implementation shape** (firm in direction, details to the unit
spec): the `profile` crate stays serde-free and editor-independent
(G1 layering; wire.rs writes foreign types structurally today). So:

- `profile::path` grows a **replay driver** — `replay(steps:
  &[ResolvedStep<T>]) -> Result<ProfileLoop<T>, PathError<T>>` — the
  dynamic mirror of the typed surface, sharing the same `Core`
  emission machinery (path.rs already accumulates
  verts/joints/entry-pose in `Core`; the driver drives the same code
  the binders call). A differential pin is mandatory: every typed-
  surface chain and its recorded step list must lower bit-identically
  (the PR-1 differential-census style, re-aimed).
- `editor-core` owns the **Expr-bearing step type** (as it owns
  `Node`/`Expr`/`Datum`): steps whose continuous arguments are `Expr`
  trees; evaluation resolves them in the parameter environment and
  hands `ResolvedStep<f64>` to the replay driver.
- The **typed authoring surface records as it lowers**: `Core` also
  accumulates the step list, so `Open.at(..)…to(Start)` yields both
  the lowered `ProfileLoop` (immediate use, kernel-direct authoring)
  and the program (document authoring). One authoring surface, two
  consumers — no second spelling of any verb.

**Drift-proofing the two surfaces (added for Evan's round-1
question).** The typed surface and the replay driver can be defined
so the dangerous drift direction is *unrepresentable*, not merely
tested away. The driver holds the in-flight tip as an enum over the
four lattice states, each variant containing the TYPED value
(schematically `enum DynTip { Open(PartialPath<Open>),
PlainPoint(..), Directed(..), Angle(..) }`), and applying one step is
a match on (variant, verb tag) whose arm bodies can only call **the
one typed binder that is well-typed at that state** — binder bodies
are never duplicated and the lattice is never re-stated as data. An
illegal (state, verb) pair has no well-typed body available, so the
only writable mistake is a MISSING arm (over-strict refusal), never
an over-permissive one: "driver accepts what the surface refuses"
cannot be written down. The remaining safe-direction drift (driver
refuses a legal chain) is exactly what the mandated differential pin
catches — every typed chain's recorded program must replay to the
same loop. **Serde's role is transport, not the door**: the
Expr-bearing wire step type derives serde for ENCODING (editor-core,
the wire.rs pattern), but deserialization cannot mint a
`ProfileLoop` — the only path from steps to geometry runs through
the driver, hence through the typed binders and every check they
carry (lattice, sign gates, junction classification). "Just serde"
with no driver would make `Deserialize` a second constructor door
that skips all of it — the precise thing wire.rs's strict-door rule
exists to prevent ("a corrupt or hand-edited file can never smuggle
an ill-dimensioned tree past the construction door"). So serde is
used, exactly once, at the layer where it is safe.

Alternative considered and rejected: storing the program as opaque
host-language source (a closure, a Python function). Rejected by D8
verbatim — user models as functions were rejected because a
value-dependent branch in user code silently breaks interval replay;
the recipe is data. The step list IS the D8-shaped residue of the
call sequence: structure explicit, every value a typed slot.

## V2. Expression binding

**Which arguments become Expr-bearing: every continuous scalar; no
structural datum.** Concretely, with dimensions:

| Step argument | Dimension | Evidence it wants binding |
|---|---|---|
| point coordinates (`at`, `line_to`, `arc_to`, fillet anchors, targets) | Length ×2 | PR-2 §6: slab's extents tuples "are already dimension expressions the chain re-flattens"; plate's parametric pressure sits in hole centers; letterforms' 1/16 offset re-typed into ~24 literals |
| `line(len)`, NURBS `len1`/`len_end` | Length | direct |
| `fillet(r)` radius | Length | diefillet::ball "wants radius (one dimension)" (PR-2 §6) |
| `angle(θ)`, `turn(δ)` | Angle | slant/taper parametrization (letterform slant, PR-2 §6) |
| `arc_to` bulge | Scalar | authored data (M2 convention); stays a slot even though most bulges are constants |

Structural data stays literal in the step: the verb tag itself, the
`Start` target, the plain-vs-directed distinction, joint declarations
(these are consequences of verb choice, never values). This is
exactly D8's structural/continuous divide, applied inside the
profile: changing a radius is a continuous edit; changing *which
verbs* make up the program is re-authoring (the analogue of the
frozen fillet selection — structure is a commitment).

**How program arguments join the document's dimension layer**: the
same way every other node slot does. Steps hold `Expr` (dimension-
checked at construction, `DimensionError` typed); expressions
reference document `ParamName`s; persist stores the AST and rebuilds
through the smart constructors (wire.rs, unchanged pattern). The
content key and naming key hash the program structure (step tags)
plus each Expr exactly as nodes hash Exprs today — the derived
segment floats LEAVE the key (V3).

**Slot addressing for edits** (firm in shape): today `DocEdit::
SetExpression` addresses `(node, SlotId, path-into-Expr)`; `SlotId` is
a small closed enum (Distance, Radius, …). A program has arbitrarily
many expression slots, so the address grows one coordinate: **(step
index, argument role)** — e.g. `Profile { step: u32, arg: StepArg }`
with `StepArg` the small closed per-verb role enum (TargetX, TargetY,
Length, Radius, Angle, Bulge…). Step indices are stable because
program structure only changes by re-authoring (above), the same
stability argument the frozen fillet selection makes. The Expr-tree
sub-path mechanism (`Expr::descend`'s u8 path) is reused unchanged.

**What re-evaluation means**: evaluation resolves the
program's Exprs in the parameter environment **at f64**, replays the
step list through the driver, and obtains fresh
`ProfileLoop<f64>` segments + declared flags; the profile then embeds
to the evaluation scalar and validates exactly as today
(`ProfileDesc::embed` + `Profile::validate` → `ValidatedProfile<T>`).
Three load-bearing consequences:

1. **Elaboration stays a C6 f64 structure-selection step** (PATHS §5:
   "pure f64 structure selection — it decides leg parameters, never
   topology"). The interval/dual lanes consume the f64-elaborated
   segments via `embed`, exactly as they consume stored f64 bits
   today. No new lane-divergence surface; D9 bit-replay holds because
   libm-pure Expr eval + libm-pure elaboration is deterministic.
2. **The authoring-time junction checks re-run at every evaluation.**
   A parameter edit can drive a junction into the ε_input tangent
   band or push a fillet corner behind its ray — and the node then
   refuses typed (`JunctionTangent`, `NoCornerForFillet`…). This is
   the algebra's §4 guarantee made *parametric*: no binding of the
   parameters can produce silent accidental tangency, because the
   check runs under every binding that is ever evaluated.
3. **Verified-never-trusted survives untouched.** The program's
   `.tangent()`/fillet steps EMIT declared flags at replay (the
   lowering unchanged); `validate` re-verifies every flag in the
   evaluation scalar (#101 layer, not touched by this switch —
   profile_desc.rs already documents "the declaration is part of the
   description and is re-verified at validation"). The program is
   upstream insurance under every binding; the flags remain the
   contract of record at the segment layer.

Authoring-time checking still happens too (fail-loud early): the
document authoring surface evaluates the program under the *current*
parameter environment as it is built, so the author sees refusals at
the verb, not at first evaluation. Literal-only programs (the whole
reworked corpus today) behave bit-identically to the typed surface —
pinned by the V1 differential requirement.

**Orchestrator verification of the f64-resolution claim (executed
2026-08-08).** `Doc::param_env<T>` (doc.rs:221-236) stores parameter
values as f64 and EMBEDS them into `T` via `from_f64`; node-slot
Exprs then evaluate at `T` per lane (`expr::eval<T: Decide>`,
eval_node's `ParamEnv<T>`). So today's machinery has a designed
asymmetry: node MAGNITUDE args (an extrude distance) are lane-live —
the interval lane sees a genuinely interval-evaluated expression —
while profile GEOMETRY is f64-pinned (ProfileDesc stores f64 bits;
lanes consume them via `embed`). The claim above keeps profile
programs on the f64-pinned side, which is (i) exactly the current
substrate's behavior, so no lane sees anything it doesn't see today,
and (ii) required by C6: program arguments feed structure decisions
(fillet fits, junction classes), and structure must be selected
once, identically for every lane — per-lane replay at `T` could
classify a trilean differently across lanes. The asymmetry (profile
Exprs resolve at f64, magnitude Exprs stay lane-live) is therefore
inherited, not invented — but it deserves Evan's explicit eyes,
because under v2 the SAME document parameter can feed both kinds of
slot.

## V3. Caches and provenance

**Derived segments become a cache: keyed, rebuildable, and NOT
persisted.**

- **What the cache is**: the `ProfileLoop<f64>` (vertices + bulges +
  tangent_joints) the replay produced under the current binding —
  i.e. exactly the v1 representation, demoted from definition to
  derived value. It lives where derived values live: the evaluation
  memo layer (`eval/memo.rs`), keyed by content key, which under v2
  hashes program + Exprs + the resolved parameter values that feed
  them — so any edit that could change the segments changes the key.
  Invalidation is therefore the existing memo story, no new
  machinery.
- **Caches do NOT persist; they rebuild on load.** Rationale, two
  independent legs: (1) the repo's stated precedent — WireExpr
  "deliberately does not persist" the cached dimension because it
  re-derives; the strict door refuses rather than repairs, so a
  persisted cache would have to be VERIFIED against a replay at load
  anyway (a mismatched cache is the corrupt-file class), making
  persistence pure liability; (2) **D9 makes rebuild exact**: same
  build + same inputs → bit-identical outputs, so the reloaded cache
  is guaranteed bit-equal to the saved-time cache. D9 is what makes
  "don't persist" FREE; conversely, if the cache did persist, D9
  is what would make the mandatory load-time verification always
  pass — persisting buys nothing under determinism except file bloat
  and a new corruption surface. (Cross-build replay is already
  outside D9's promise for every derived value in the system; the
  profile cache is not special.)
- **Provenance runs program-ward, and stable naming survives.**
  Derived vertex/edge indices are *program-structural*: elaboration
  never decides topology (C6), so the segment count and order are a
  function of the step list alone, not of parameter values. A
  parameter edit moves vertices; it never renumbers them. Stable
  names that reach into profiles (`ProfileEdgeRef`/`ProfileVertexRef`
  in the naming layer) therefore stay valid across continuous edits
  — same freeze semantics as today, now with a proof shape (the C6
  boundary) instead of an accident of storage. Re-authoring the
  program (structural edit) may renumber — which is the existing
  rule for structural edits everywhere.
  **REVISED (2026-08-08, the required measurement contradicted
  the claim):** loop canonicalization (`canonicalize_loop`:
  rotation to the lex-min vertex + shoelace orientation) is
  GEOMETRY-DEPENDENT, so a parameter edit whose vertices cross a
  lex-order band RENUMBERS the canonical indices and repoints
  StableNames — the "moves vertices, never renumbers" sentence
  above is FALSE at those crossings (it holds vacuously today
  only because profiles are slot-free). The v2 posture is
  therefore the existing freeze doctrine, not a stability proof:
  a renumbering edit makes stale selections refuse VANISHED,
  fail-loud, exactly the M6-5 contract; the renumbering class
  (lex-band crossings) is documented and the acceptance scene
  pins a stable case and a Vanished case. Canonicalization
  itself is out of the switch's fence.
  **REVISED AGAIN (round 2, Evan's seamlessness question): the
  renumbering class can be ELIMINATED for program loops, by
  construction.** A chain program has an intrinsic start (the
  entry vertex) and an authored direction — so profile-entity
  NAMING for program-defined loops anchors to PROGRAM-STRUCTURAL
  positions (step indices), not to the geometry-derived lex-min
  rotation. Structure decides, geometry never (the C6 shape,
  now real): parameter edits cannot renumber, because nothing
  geometric enters the index. Orientation is likewise stable
  within valid parameter ranges (winding sign is continuous and
  a zero-area crossing refuses as degenerate before any flip).
  The lex-min canonicalization survives wherever a
  geometry-canonical form is genuinely needed internally, but
  stops being the naming substrate for program loops. Scope: the
  SWITCH unit's editor-core PR; the freeze doctrine remains the
  backstop for STRUCTURAL edits, as everywhere.

Alternative (persist the cache alongside the program, "segments as
provenance in the file") — rejected above on the strict-door + D9
argument; state it in review if the load-time replay cost at scale
worries anyone (it should not: replay is a linear pass over a few
dozen steps per loop; the corpus's largest program is the A-outline
at ~20 steps).

## V4. The schema break and the raw-loop question (VQ1, the central decision)

The break itself is committed shape: `SCHEMA_VERSION` bumps by one
with an EMPTY migration table (LQ7a: clean break; the chain machinery
stays, carrying nothing — the table at the time was already `&[]`).
`Node::Profile(P)` remains the payload seat; what changes is what `P`
is. What replaces `ProfileDesc(opaque Profile<f64>)` is a
**ProfileProgram**: plane placement + per-loop programs, wire-shaped
in editor-core structurally (kernel crates stay serde-free).

**The wall evidence forces the question this section exists for.**
The PR-2 census: of 26 tour loop sites, 12 moved to the algebra; 14
CANNOT be algebra-authored today, in four hard classes (PR-2 walls
W2–W4, PR-1 finding F7):

- **Closed carriers** — plain circles, the corpus's most common raw
  shape (bodies::circle ×2, bossplate::boss, curvedcut::disc,
  lily::circle_loop): PQ4 (ruled: no mid-carrier seams) + the
  same-carrier rule refuse every spelling; F7's stadium/slot gap is
  the same class (both-sides-tangent closer + parallel carriers).
- **Via-point arcs** (4 loops: vase, sheave, lily leaves) and
  **centre-first arcs** (2 loops: lantern — documented carrier
  intent — and ball): no PATHS binding mode; hand-deriving bulges
  would re-type computed values, the exact forbidden thing (W2).
- **Arc-carrier fillets** (rocker outline + eye: 5 fillets, arc×line
  and arc×arc): v1's fillet door is line×line only (PATHS §7 banked
  arc-arrival fillets "additive, with a use case" — rocker IS the
  use case).
- **The bowtie** (finale_fail_loud): deliberately raw forever — it
  exists to demo the raw layer's fail-loud validate.

Plus the measured near-miss: the bracket (W1) — algebra-spellable but
ulp-shifted because `.angle(PI)` directors are sin_cos-dirty while
chord-derived directions are exact; and W5, no far-end-anchor
spelling for a post-fillet side ending at a sharp vertex.

So: v2 commits "the program is the definition" while the algebra can
define only 12 of 26 corpus loops. The schema must answer what the
other 14 ARE. This is **VQ1**, the central question, **RULED
(b)-DIRECT** — §V7's VQ1 entry carries the ruling and its ground.
The three options are kept below as the record of the decision:

**(a) Raw loops remain a first-class sibling representation.** A
loop's payload is `Chain(program)` | `Raw{vertices, bulges,
tangent_joints}` — per-loop, never mixed within a loop (PATHS §6's
representation-uniqueness ruling already forbids intra-loop mixing;
per-loop choice respects it). Pro: ships now; the bowtie needs Raw
regardless; honest about the authorable surface. Con as usually
framed: raw loops stay parametrically dead — and the census says the
dead loops are where the parametric pressure LIVES (PR-2 §6:
"bodies::plate's parametric pressure is entirely in the RAW loops —
hole centers ±1.5, r=0.7; the movable rectangle was the boring
part"). A v2 that Expr-binds only algebra loops delivers parametric
Python for the boring parts of the corpus.

**(b) Grow the algebra's missing binding modes first, then switch.**
The gaps are enumerable and mostly cheap: a closed-carrier primitive
(`circle(center, r)` — a one-step program form, not a chain, so PQ4
is not touched: PQ4 ruled where a *chain's seam* may sit, and a
circle program authors no seam; the conventional two-arc split
becomes the primitive's private lowering, exactly M2's precedent);
via-point and centre-first arc modes (`arc_via(via, end)`,
`arc_center(c, end)` — the closed forms already exist as
`sugar::bulge_from_via`/`bulge_from_center`, today's raw-layer
helpers; the algebra step stores the authored points and derives the
bulge in elaboration, so nothing computed is re-typed — killing W2
for 6 loops); the far-end-anchor spelling (W5). That covers 11 of
14. The remainder — arc-carrier fillets (rocker's 5, W4) — is real
solver-free closed-form work (line×arc, arc×arc corner
construction), the one genuinely expensive item. Pro: the definition
story is uniform. Con: v2 (and therefore U9) queues behind
vocabulary growth, including the expensive item; and the bowtie
still needs a raw door, so (b) alone cannot be complete.

**(c) Two constructor vocabularies under ONE program umbrella**
(RECOMMENDED). Reframe what "program" means by taking Q8 at its
word: *the constructing function is the surface* — and the raw layer
HAS a constructing function, `LoopBuilder`. A loop-program is a step
sequence in exactly one of two vocabularies: the **chain vocabulary**
(the PATHS algebra, V1) or the **raw vocabulary** (`vertex(p, bulge)`
steps + declared-joint indices + the existing `fillet`/
`fillet_corner`/`arc_via`-style raw verbs — LoopBuilder's call
sequence as data). Both vocabularies get Expr-bearing arguments
(V2's table applies to raw vertex coordinates, bulges, and raw-fillet
radii identically). Mixing within a loop stays forbidden (§6).
Consequences:

- Every loop in the corpus becomes a definition-by-program on day
  one, and every loop gets parametric reach — plate's hole centers
  and radius become Exprs WITHOUT waiting for algebra growth.
- The chain/raw distinction stops being "new vs legacy format" and
  becomes what it truly is: two authoring layers with different
  guarantees (chain: accidental tangency unrepresentable under every
  binding; raw: declarations are claims, verified at validate —
  `TangencyContradicted` remains reachable, and a parameter edit can
  break a claimed tangency → typed refusal at evaluation. Fail-loud,
  and honestly weaker — the doc must say so).
- The algebra still grows the cheap modes (circle, arc_via,
  arc_center) — as authoring improvements on their own schedule,
  DECOUPLED from the schema switch. Arc-carrier fillets stay banked
  additive (PATHS §7) with rocker as the recorded use case.
- The bowtie is simply a raw program that validate refuses — the
  demo survives unchanged in meaning.

This section's own recommendation was **(c)**, on the census
inversion — the parametric pressure sits in exactly the loops (a)
would freeze, and (b) appeared to gate the entire bindings program
on the rocker fillet closed forms, the one item with real geometric
risk. **The ruling went to (b)-direct instead** (§V7, VQ1), on an
argument this section had not found: schema evolution under LQ7 is
asymmetric, so a second vocabulary creates removal debt on a
pre-release deadline while chain-only forecloses nothing. Both
options make "the program is the definition" true of every profile
at the switch; (b) gets there with one wire vocabulary instead of
two. Sequencing consequence carried either way: U3 (SectionSegments
retirement) targets the program form directly, per LQ2's "or the v2
program form directly, given LQ4".

**Round-1/2 sharpening — (b) vs (c), for Evan's lean-(b). REVISED:
round 2's probe landed** — the round-1 claim "the bowtie forces a
raw seat in every option" was WRONG and is retracted. The bowtie's
junctions are all locally sharp and legal, so the algebra authors it
happily and only global `validate` refuses it (PR-2's own
observation: "algebra-authored ≠ validated"); moreover the demo is a
KERNEL-layer tour scene (LoopBuilder-direct) that no document ever
persists, and the raw kernel layer stays public under every option
regardless (§V6). A chain-only v4 schema is genuinely reachable
under full (b) growth. The corrected decision surface:

- **(b)'s virtue — the cleaner end state**: one vocabulary,
  accidental-tangency-unrepresentable EVERYWHERE, no
  weaker-guarantee sibling in the schema. This matters most at
  RELEASE, because post-release there are no clean breaks (LQ7):
  whatever vocabulary the schema ships with is permanent surface.
- **(b)'s cost — the gate**: the switch, and with it Expr-bearing
  profiles, the parametric payoff, and U9 (bindings ship against
  v2, LQ4), queue behind vocabulary growth INCLUDING the one item
  with real geometric risk (rocker's five arc-carrier fillets:
  line×arc / arc×arc corner construction). The cheap growth
  (circle, arc_via, arc_center, far-end anchor — closed forms
  already exist as `sugar::bulge_from_via`/`bulge_from_center`) is
  not the gate; that tail is.
- **(c)'s virtue**: schema complete and parametric value delivered
  AT the switch (plate's hole centers/radius become Exprs
  immediately — the census's pressure points); risky geometry off
  the critical path; the lift tool measures chain-coverage
  convergence (§V5).
- **(c)'s cost, properly stated**: a second vocabulary with weaker
  guarantees (declarations are claims, `TangencyContradicted`
  reachable) — which, if it survives to release, is permanent.

**The reversibility rider that reconciles them (recommended
package, revised):** pre-release breaks are free (LQ7a). Rule (c)
now, authorize the cheap growth immediately, and add a
**pre-release review point**: if chain coverage reaches the whole
corpus before release (growth wins the race), the raw vocabulary is
REMOVED in a clean break and (b)'s end state ships; if not, raw
ships Expr-bearing rather than gating release on rocker's closed
forms. This dominates (b)-first in every branch except one: if the
raw vocabulary should never exist even TRANSIENTLY (so corpus and
binding code never grow raw-program habits whose later removal is
churn). Whether that transient existence is acceptable is the
actual VQ1 ruling.

## V5. The v1→program lift — tool status post-clean-break

What survives of PATHS-DESIGN's "mechanical lift" language after
LQ7a's no-migration ruling: **a development-side authoring tool, not
a load path and not a schema feature.** Its one job: take a v1-form
loop (vertices + bulges + declared flags) and mint the equivalent
chain-vocabulary program. Only half of PATHS-DESIGN's harmonization
paragraph is a flag READ: `tangent_joints` is the v1 form's one
declared datum, so declared junctions → `.tangent()` and everything
else → sharp `line_to`/`arc_to` steps, seam last; recovering
`.fillet(r)` would mean un-trimming the corner — inference, not a
flag read, and anchor-sensitive exactly as F10 describes — so the
fillet spelling stays banked.

What it provably CANNOT lift — the same walls as V4, plus two
subtler classes measured by PR-2:

- The 14-loop class: closed carriers, via/centre-authored arcs
  (unless the modes land first — the lift should then use them),
  arc-carrier fillets. Refusal, typed, naming the wall.
- **Anchor-inconsistent filleted loops** (PR-1 finding F10): the
  canonical trim inputs are anchor-based, so the lift is
  bit-identical only when the v1 authoring is anchor-consistent;
  otherwise the lifted program changes SAID, not shape — segments
  differ in bits. The tool must report which (bit-identical vs
  value-equal) it achieved; PR-2's scene-by-scene care is the
  precedent.
- **Angle-director ulp drift** (PR-2 W1, the bracket): a lift that
  spells a direction as `.angle(θ)` inherits sin_cos quantization.
  The lift must prefer chord-derived spellings (`line_to`), and VQ4
  (exact directors) closes the residue.

Firm: keep the tool in-repo with a differential harness (lift →
replay → compare against the source segments), because it is also
the acceptance instrument for the chain vocabulary's growth — each
new binding mode turns some refusals into lifts, measurably. Not
owed to users as a promise; not run at load, ever (clean break).

## V6. What does NOT change — the fence

- **The #101 verify layer**: flags verified-never-trusted,
  `UndeclaredTangency`/`TangencyContradicted`, fit gating,
  same-carrier-is-identity — runs unchanged on replayed output under
  every binding (V2 item 3). The algebra remains upstream insurance;
  the flags remain the segment-layer contract of record.
- **`LoopBuilder` as the raw layer**: stays public, stays the
  fail-loud demo surface; under V4(c) it additionally becomes the
  raw vocabulary's recording surface (same record-as-you-lower shape
  as V1's third bullet).
  **AMENDMENT RATIFIED (2026-08-11, issue #377, Evan 👍 on #386;
  driven by his in-chat ruling "LoopBuilder should go away";
  strengthened per his two follow-ups before sign-off):** this whole sentence ENDS
  at the migration unit. LoopBuilder leaves the `profile` crate's
  public surface ENTIRELY and moves to test-support (the
  banished-to-the-test disposition): its one load-bearing role is
  the differential twin — an independent second implementation
  the PATHS lowering is verified against — and that value
  survives intact living beside the tests that use it (no source
  code outside sugar.rs calls it; every in-src mention is a doc
  comment). The V4(c) recording-surface clause is STRUCK as
  never-implemented-and-no-consumer: recording landed only on the
  lattice (SWITCH-P's record-as-you-lower); raw loops are
  kernel-direct and never persisted, so nothing ever needed raw
  authoring to record (the corpus clean-break confirmed this —
  its raw census went to circle_split/declared-subdivision or
  stayed kernel-layer). **EXECUTED (LIB-LBRET):** `LoopBuilder` and
  its `close_*` family now live in `crates/profile/src/test_support.rs`
  behind a `test-support` feature that only dev-dependencies enable —
  no `src/` in the workspace names it, and the differential twins go on
  verifying the lattice against it unchanged. Raw `ProfileLoop` DATA
  (`polygon`/`new` — the bowtie) remains kernel vocabulary; it
  never was LoopBuilder. The handful of cross-crate TEST
  consumers (step-export fixtures, mesh, k-lint litmus) migrate
  to lattice or raw-data spellings at the unit.
  **AMENDED (LIB-RETTAIL, Evan's ruling on #413, 2026-08-12 —
  "yes we should demote ProfileLoop"; his framing: kernel
  vocabulary should be private, and the broken-on-purpose bowtie
  cannot justify a public authoring tier):** the clause "raw
  `ProfileLoop` DATA (`polygon`/`new` — the bowtie) remains
  kernel vocabulary" survives as to KERNEL vocabulary and ENDS as
  to the presented surface. `new`/`polygon` moved off
  `ProfileLoop`'s inherent impl onto the `profile::RawLoop`
  trait, so the minting doors travel with the TRAIT rather than
  with the (still nameable) type; `pncad` replaced `pub use
  profile;` with a curated `pub mod profile` that omits `RawLoop`
  (the LB13 precedent), and `pncad::authoring::polygon` was
  deleted. The bowtie is no longer the justification for any of
  it: it left the tour for `profile`'s rejection suite. Residue
  stated rather than glossed: `ProfileLoop` is plain data with
  public fields, so a struct literal still constructs one
  wherever the type is nameable — sealing that means private
  fields plus accessors, a change to the plain-data convention
  and not a housekeeping edit. And the twins named above no
  longer verify against `LoopBuilder`: their target is a blessed
  recorded fixture (LIB-RETTAIL), which pins bit-identity exactly
  as hard and gives up only the twin's INDEPENDENCE — a
  property the shim had already lost in substance, since its
  `fillet_corner` and the lattice's arc fillet both run the one
  ratified `sugar::arc_fillet_trims`.
- **Junction predicates and the k_stats funnel**: `path_junction_
  turn`, `path_corner_*` etc. classify at replay exactly as at typed
  authoring; no new predicate semantics, only a new call site.
- **The validation ladder**: tiers unchanged; `ValidatedProfile` is
  still minted only by `validate` on segments; downstream ops
  (extrude/revolve/fillet/loft/sweep) consume `ValidatedProfile`
  and never see programs.
- **D9**: replay is deterministic (libm-pure, no ordering effects);
  the switch adds no platform surface. The U9 doctest pinning
  cross-platform bit-replay (LIBRARY-DESIGN U9) should include a
  parametric profile.
- **PQ4 and the §6 rulings**: mid-carrier seams stay refused for
  chains; no intra-loop mixing; no concatenation operator. The
  closed-carrier forms (`circle`, `circle_split`) are program FORMS,
  not a PQ4 relaxation — they author no seam, so the chain rule they
  read close to is untouched.

## V7. Question ledger

VQ1 was delegated to orchestrator judgment and ruled; VQ2–VQ9 were
reviewed in detail and agreed by Evan (PR #242 round 4). Each entry
is the ruling plus the argument that decided it.

- **VQ1 — raw loops' status: RULED — (b) DIRECT** (Evan delegated
  the call, PR #242 round 3, 2026-08-08: "use your judgment …
  no particular hurry … seems like less work overall to just do
  (b)"; orchestrator ruling recorded here). The deciding argument,
  sharper than rounds 1–2 found: **schema evolution under LQ7 is
  asymmetric.** If a genuine raw-at-rest need ever emerges, adding
  a raw vocabulary is an ADDITIVE extension — legal even
  post-release; removing one (the (c)-then-review path) is
  SUBTRACTIVE and must happen pre-release or never. Chain-only
  forecloses nothing; two-vocabulary creates removal debt on a
  deadline. With Evan's no-hurry weighting, (c)'s one remaining
  virtue (earlier parametric delivery) loses to less total work
  (one wire vocabulary, one Expr surface, one driver, one slot
  scheme) and the cleaner end state. Consequences: (1) the
  vocabulary-growth units PRECEDE the switch — the cheap set
  (circle primitive, arc_via, arc_center, far-end anchor, VQ4
  exact directors) first, then the arc-carrier fillet modes
  (measure sugar's existing arc-leg fillet forms, M5 S2/#137,
  before sizing — the closed forms may largely exist); (2) the
  switch lands chain-only when the persisted corpus authors fully
  (kernel-layer validate-refusal demos like the bowtie never
  persist and stay LoopBuilder-direct, §V6 — **amended at
  LIB-RETTAIL: the bowtie is neither LoopBuilder-direct nor a demo
  any more. It authors through the LATTICE, since its four corners
  are sharp and the local junction checks pass it, and it lives in
  `profile`'s rejection suite. Nothing about the chain-only switch
  changes: it still never persists**); (3) U9 queues behind
  the switch per LQ4, accepted under no-hurry; (4) the V4 §
  option analysis is retained as the record of why.
- **VQ2 — derived-segment caches do NOT persist.** V3's
  strict-door + D9 argument. Reopen only if load-time replay cost
  surprises at corpus scale (it should not; programs are tens of
  steps).
- **VQ3 — edit addressing is `(step index, per-verb argument
  role)`**, extending the SlotId scheme with Expr sub-paths
  unchanged (V2). A flat slot enumeration per program was rejected:
  it makes step identity implicit and breaks the structural-edit
  story. Detail belongs to the implementing unit's spec.
- **VQ4 — exact directors are added as new vocabulary.** PR-2 W1:
  angle directors are sin_cos-dirty, and the corpus's one line×line
  fillet could not move because `.angle(PI)` carries 1.22e-16 into
  the ray. A direction-valued director spelling (`.toward(dx, dy)`
  or axis tokens) whose components are exact data closes it; Angle
  stays for genuinely angular authoring (slant). This is the
  difference between "the showcase fillet lifts bit-identically"
  and "it cannot move" — small, high-leverage.
- **VQ5 — sugar stores CORE STEPS ONLY.** PATHS Tier-1 sugar "adds
  no semantics", so sugar (and builder functions like
  `path_polygon`, and the wanted `rect`/`polygon` verbs — PR-2: 12
  of 26 sites are polygons) expands AT AUTHORING to core steps.
  Crucially this loses NO parametric value: expansion
  happens at the Expr level, so `rect(x0,x1,y0,y1)` corners are
  Expr pairs SHARING the extent subtrees — the slab evidence's
  "re-flattening" disease was about literals, and Exprs don't
  flatten. Alternative (first-class sugar steps, richer provenance
  for a future GUI's "this is a rect") is additive later; do not
  pay its schema cost now.
- **VQ6 — junction checks take the EVALUATION's tolerance.** PR-1
  F2 found that nothing said where path-authoring tolerance comes
  from (the implementation used run-global `Tolerance::get()`).
  Under v2 the checks run inside document evaluation, so the
  tolerance is whatever the run context pins. F2 closes here.
- **VQ7 — NURBS legs are out of this switch's scope**, unblocked
  by it, and get their own unit later. PR-1 F1: there was no v1
  representation to lower to, and PATHS banked them for v2 "exactly
  as anticipated". They are a SEGMENT-vocabulary extension
  (ProfileLoop has no NURBS segment), not a program-shape question
  — V1's program form carries them the day the segment layer does.
- **VQ8 — plane placement stays stored f64**, not Expr-ized:
  that is the U4 pose-vocabulary conversation (P6/P7), not this
  switch. The wire shape must leave the seam visible — placement as
  its own struct, never inlined into the program.
- **VQ9 — where authoring-time checks bind (restated, firm).** The
  document authoring surface checks under the CURRENT param env;
  re-evaluation re-checks under every binding (V2). No "check only
  at authoring" option survives verified-never-trusted; no "defer
  all checks to eval" option survives fail-loud-early ergonomics.

## V8. Acceptance shape (for the eventual unit specs, not binding)

- The reworked corpus round-trips: author → program → persist v4 →
  load → replay → validate → byte-identical exports vs the pre-
  switch baselines (PR-2's diff harness re-aimed at the schema).
- One scene gains a REAL parameter (plate's hole radius is the
  natural pick — PR-2 §6) and demonstrates: edit param → re-evaluate
  → new geometry; drive it into a refusal → typed error names the
  step.
- The differential pin of V1 (typed surface vs recorded program,
  bit-identical) and the U3-rider zero-geometry-diff row LIB-LOG
  already banked (NOTE-3).
