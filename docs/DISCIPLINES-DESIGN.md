# Disciplines and checks — the registry design conversation

**Status: WIP — provisionally accepted (Ev, 2026-08-25, in the
origin conversation: merge as idea-WIP, provisionally accepted).**
Not yet RATIFIED: the classification and pattern are the accepted
working frame; full ratification (and a DESIGN.md companion-table
row) waits for the first implementing unit's conversation.
(Born in the 2026-08-24 configurable-checks conversation; proposals
DS1–DS8, open questions DS-Q1–DS-Q6. Round 2 revised DS2 and DS7 per
Ev's pushback: the mandatory criterion is identification, not
"an op forks"; the permissive mode is a recording dial with a viable
middle position, not a rejected monolith. Round 3 corrected DS2's
collinear example against the code (undeclared same-carrier joints
pass `validate` today; #433 is the open authoring-gate/data-gate
divergence), replaced DS6's severity cap with the waiver rule, and
sharpened DS7's snap/fuse statement into its three layers. Round 4
is the first implementing unit's contact with reality: the registry
and the connectedness resident shipped, I1(0b)'s exactness claim
narrowed to the count, the staleness direction proved non-optional
within one review cycle, DS-Q3 is answered for grade 4, and DS-Q4
carries its first-PR position. Round 5 opens the registry to
out-of-tree checks — DS9, three rulings from Ev plus one
recommendation awaiting reaction. Round 6 ruled that arm: the funnel
is exposed to plugins, with verdict-producer provenance replacing
default exclusion; and the machinery unification is planned, #981.) This doc schedules **no
implementation** and changes **no behavior** — every mandatory check
named below keeps its current force whether or not this is ratified.
What it ratifies, if accepted, is a *classification* and a *pattern*:
which rules of the kernel's check family are configurable at all, by
what criterion, and what shape a new one plugs in as.

Vocabulary, fixed for this doc. A **discipline** is a rule of the
declared-coincidence family: membership in a positive-codimension
stratum of configuration space (tangency, contact, carrier identity,
perpendicularity, parameter equality) must be *declared* when
definite, is *verified never trusted* when declared, and *escalates*
when in-band. A **check** is a pure analysis over a finished
evaluation producing findings (moldability, sliver scale,
connectedness) — no declaration vocabulary, nothing to verify. Both
live in one registry because they share the finding/menu/severity
machinery; they differ in everything upstream of it.

## Grounding (ratified text this doc builds on, not re-litigates)

- **The coincidence ladder** (DESIGN.md round 8; the #42 invariant):
  coincidence is structural or declared, never value-inferred;
  discovery is never declaration (F1 — no scan-to-bless); in-band
  escalates, never guesses.
- **The three shipped disciplines**, each hand-built: profile
  tangency (#101 — `profile::validate::judge_joints`, PATHS declares
  by construction), carrier equality (the four-rung ladder,
  `topo/src/boolean/carrier_eq.rs` module docs), declared contact
  (CONTACT-DESIGN C4 — per-class verify tables, four typed failures).
- **The verify-table shape** (CONTACT-DESIGN C4): must-verify-DEFINITE
  / contradiction triggers / bridged residue — a declaration is
  trusted exactly on its bridged residue and nowhere else.
- **The detect/declare protocol** (SELECT-DESIGN §3): detector = the
  verifier run in candidate-generation mode (the anti-twin rule);
  findings pass through user-visible hands as values; a fused
  detect-and-declare door is forbidden permanently (GS-Q3).
- **The advisory lane charter** (LONGTERM-IDEAS I1): per-part checks,
  certified where the geometry supports it, honest heuristics where
  it doesn't, labeled as such, never silently mixed.
- **The run-global configuration posture** (D4): one value per run,
  never per-entity; K as the one strictness knob (`Tolerance.k`,
  ratified default K = 10, `docs/K-REPORT.md`); and D9 — bodies are
  never persisted, they re-derive from recipes.
- **No value-inferred merging** (F7): booleans precondition maximal
  faces and refuse typed; `merge_coplanar_faces` glues on the
  structural and declared rungs only.
- **Equivariance** (engineering convention 4): kernel constructions
  and selection rules privilege no absolute frame.
- **The reach goal** (M10): a model is evaluated over a parameter
  distribution, not at a point; recipes must carry the intent that
  makes topology well-defined over a parameter *neighborhood*.

## DS1 — One pattern, currently hand-instantiated three times

Every shipped discipline is the same five-part shape:

1. a **stratum predicate** at some jet order k, with its margin one
   order up (order 1 normal-independence for `Intersection`, order 2
   κ_rel for `TangentIntersection`, order ∞/structural for
   conformality — the C1 pattern);
2. a **declaration vocabulary** (recipe data by stable name:
   `tangent_joints`, `Node::Declare { pairs, class }`), each record
   carrying a **provenance** (DS7's ladder: constructor-authored /
   user-stated / auto-recorded);
3. a **verify table** (must-verify-definite / contradiction triggers
   / bridged residue);
4. a **detector** that is the verifier in candidate-generation mode
   (anti-twin: detect-then-declare can never disagree with
   verify-at-use);
5. a **refusal menu** whose arms all record intent or change
   geometry (declare the named class / move the geometry — never
   absorb).

Adding a new discipline today means re-implementing all five by hand.
**Recommendation**: the registry is this pattern made structural — a
new discipline supplies (1)–(3) and inherits (4)–(5) plus the shared
finding type, menu rendering, and severity plumbing from the
framework. The registry unifies the *machinery around* the
predicates, not the predicates: each stratum's mathematics stays
where its geometry lives (the C4 tables in `topo`, the joint
classifier in `profile`), exactly as `ContactClass` is defined lowest
and re-exported upward today.

## DS2 — The identification criterion: which disciplines admit no knob

**A discipline is irreducibly mandatory iff its declaration licenses
an identification** — a quotient that merges independently-authored
entities into one (a single carrier from two descriptions, the
declared-REST zip, a blessed conformal patch), so that the *built
solid itself* depends on whether the bless happens. Value evidence
cannot substitute for the declaration there even when definite, as a
matter of principle rather than posture: bit-equality at nominal is
a fact about a **point** in parameter space; identity is a claim
about the **family** (equal under every assignment the M10
distribution explores). Only intent can assert the family-level
claim, and gluing on the point-fact makes topology a function of a
coincidence nobody stated. Carrier equality and declared contact are
identification-grade; they carry no switch today and none under this
design.

**Classification-grade disciplines are everything else**: the
stratum verdict tunes edge descriptions, legality, and messages, but
whenever a body builds it is the same body — the fork downstream
consumes the *geometric verdict*, which exists whether or not anyone
declared, and the declaration only gates passage. Profile tangency
is classification-grade (reclassified in round 2 per Ev's
collinear-segments observation; the example corrected in round 3
against the code). The collinear case, precisely: two consecutive
collinear segments classify as `JointClass::SameCarrier` — not
tangency — and an **undeclared** same-carrier joint *passes*
`validate` today (a straight run subdivided at an interior vertex is
well-formed loop data, the shape STEP import and raw authored loops
routinely produce). What refuses is (a) the **declared-tangent**
same-carrier joint (`TangencyContradicted { same_carrier: true }` —
identity misdeclared as tangency, a category error the verify table
catches: tangency is a relation between distinct carriers), and
(b) the PATHS **authoring lattice's** zero-turn junction check
(§4 invariant 1 — an authoring-act gate, "what did you mean by this
corner?", with the post-fillet continuation exempt because it
extends the leg rather than minting a collinear neighbor). Nothing
auto-joins at any layer, consistent with the §2c axiom (verbs cannot
consult carrier identity; the junction check is the chain's own
emission-layer bookkeeping). So the undeclared-collinear body
demonstrably builds, with the same topology as the slightly-off
case; the discipline's verdicts move *acceptance* (and F7's
`NonMaximalFaces` door downstream), never the solid. The
lattice/`validate` divergence on exact collinearity is RULED
(**#433**, 2026-09-01): they agree — the authoring door was missing
its spelling, which is now the straight continuation and its declared
point-target form; both lattice halves and the SEAM have landed (the
seam's own declaration rides the closing verb's target — PATHS §6's
revised PQ4), and the issue's remaining half is the raw-door
demotion. It remains the clearest in-repo instance of this grade's
authoring-gate vs data-gate structure. Likewise the
smooth-vs-transverse fork (MappedCurve vs Intersection descriptions,
wedge legality, prefer-intrinsic) resolves from sampled geometry,
not from the declaration.
Classification-grade disciplines therefore satisfy DS3's severity
invariant and are dial-eligible (DS7) **in principle**; profile
tangency stays at the strictest dial position as the ratified thesis
default, and its declarations have non-check consumers regardless of
dial (`ValidatedLoop::blend_arcs` — the structural
fillet-identification API — and loft joint carriage), so PATHS
constructors keep authoring them at every position.

The criterion is checkable, not a judgment call: name the merge the
declaration licenses, or the discipline is classification-grade.

## DS3 — The severity invariant: what makes any knob legal

A registry entry may carry a configuration knob iff it can hold this
invariant:

> **No knob position ever changes the evaluated solid.** Whenever a
> body builds, it is the same body at every position; positions
> differ only in what is *accepted* — a refusal at the
> recipe-validation door, a finding, or silence.

Consequences, each load-bearing:

- **Monotone strictness.** Tightening only shrinks the accepted set.
  No configuration change can silently alter geometry — the failure
  mode of cross-config document exchange is exactly one:
  refuse-on-load with the finding and its menu.
- **Cheap reconfiguration.** Unlike ε, a knob change requires
  re-*validation* only, never re-evaluation — no geometry re-derives.
- **No per-body state.** Knob configuration is run-global (the D4
  posture). Bodies are never persisted (D9), so there is no at-rest
  bitmask to stamp; a document carries its declarations, which are
  meaningful at every position.
  *As built (round 4), the v1 realization is stronger than
  run-global*: `ChecksConfig` is a value passed to the `run_checks`
  door — no global state, no initialization, no persistence.
  "Run-global" survives as the semantic posture (one config governs
  a run's report, nothing per-body), not as storage. Persisting
  expectations (an AppearanceMap-shaped, StableName-keyed store) is
  a named later unit (a format change — the corpus regenerates);
  DS-Q1's document-demanded strictness rides with it.

This invariant is the sharp boundary of the design: anything that
can hold it is knob-eligible; the identification disciplines (DS2)
cannot, and are not. The knob mechanics themselves are trivial (a
level enum consulted at the finding sink — one macro's worth); the
design content is the invariant, not the mechanism.

Two knob shapes, by kind. **Checks** (no declarations) carry
**off / warn / error**. **Disciplines** carry the **recording dial**
of DS7 — **ignore / auto-record / require** — because for a
discipline the meaningful middle position is not a bare warning but
a *recorded* one: recording is the acknowledgment mechanism, the
diff basis that separates a known coincidence from a new one. A
warn-without-record position would re-flag every known coincidence
on every validation forever, or never distinguish new from known;
it is deliberately absent.

## DS4 — Grade 2: classification geometric disciplines

Full discipline shape (declarations in the recipe, verified never
trusted, in-band escalates at `require`) — knob-eligible by DS2/DS3.
Residents:

- **Profile tangency (built; dial at `require` as the ratified
  default).** Reclassified here from the mandatory grade by DS2's
  criterion; nothing about its current behavior changes, and moving
  its default is a design conversation this doc does not open.
- **Declared right angles (reserved, not built** — the GS-Q2
  convexity posture**).** Perpendicularity is intent-only by
  construction: convention 4 makes the kernel equivariant, so no
  boolean, sector table, or classification can ever consume "these
  faces are exactly perpendicular" — an undeclared right angle is
  just a transverse crossing. What the declaration buys is the
  stratum: under M10 a perpendicularity holding by accident of
  nominal values is a point property, destroyed by perturbation; a
  declared one is recorded design intent the stackup can hold the
  model to. Verify table: definite = normal orthogonality at the
  derived angular threshold (lever arm named per site, D4);
  contradicted by definite non-orthogonality; bridged = the in-band
  residue between independently-authored faces. No corpus site
  demands it yet; the vocabulary slot is named so the first demand
  lands as a registry entry, not a milestone.

## DS5 — Grade 3: recipe-layer disciplines (parameter-space strata)

The same pattern one level up the pipeline: the stratum lives in
parameter space, the structural rung is the Expr DAG, and the kernel
never sees the discipline at all — it is `editor-core` machinery over
recipes.

**First named resident: the parameter-coincidence lint.** Stratum:
the diagonal {pᵢ = pⱼ}. Rungs:

- **structural** — the two values share the Expr subterm that makes
  them equal (both read `width/2`): intended, nothing to declare;
- **declared-same** — unify into one variable (the repair that makes
  the coincidence structural);
- **declared-distinct** — an explicit disavowal: "equal by
  coincidence, keep independent." New vocabulary — the geometric
  disciplines have no anti-declaration, but here both menu arms
  record intent, which is what makes `require` livable;
- **refusing** — DAG-disjoint expressions, definitely-equal values,
  no declaration either way: the finding.

For literal operands the equality test is exact — no ε anywhere; for
derived values it is the ordinary Q1 definite-equality trilean.
Scope: **named parameters only**, not raw coordinate literals —
base-rate control is what keeps the finding a signal. The consumer
that makes the declaration meaningful is M10: same variable = the
pair comoves under the distribution; declared-distinct = independent
marginals. The lint is the completeness condition for a tolerance
stackup being well-posed over the definitely-equal pairs — and it
catches the one classic parametric defect no geometric check can
see: the copy-pasted dimension that was meant to be shared, where
both models are bit-identical at nominal and differ only in
parameter space. At the dial's `auto-record` position the recorded
object is the acknowledged observation (the pair, not a chosen arm —
the machine cannot pick unify-vs-distinct; upgrading to either arm
is the user's review).

The frame-coincidence variant (a vertical line authored as two
points rather than direction + origin + length) is recorded as a
possible later resident, explicitly subordinate to authoring
vocabulary: the constructive spelling (direction/length constructors,
shared datums) is the primary artifact, the lint only points at
places to use it — the PATHS `.fillet(r)` precedent, where the
tangent spelling was made easy rather than the tangent value policed.

## DS6 — Grade 4: advisory checks, and the certified/heuristic split

The I1 lane's members (moldability/draft — DRAFT-DESIGN DR6's checker
twin; the sliver lint; the connectedness lint) enter the same
registry as **checks**: no declaration vocabulary, no verify table —
a pure analysis over an evaluation producing findings with margins,
rendered through the same sink as discipline findings. They graduate
by the LONGTERM-IDEAS process note (milestone plan + sign-off), not
by this doc; what this doc fixes is the shape they graduate *into*.

**First resident SHIPPED (round 4): the connectedness check.**
`editor_core::checks::run_checks` / `enforce_checks` (exported via
`pncad::document`, mixed_pins' reports-never-gates posture), resident
`CheckId::Connectedness`, kind Certified, default Warn; components =
count of `Outer` shells per (root, output) subject via
`topo::classify_shells`; the expected-components input is the
resident's acknowledgment mechanism (no generic waiver machinery
shipped — the waiver vocabulary above stays paper until a resident
needs it). Two corrections earned on contact:

- **I1(0b)'s "exact/combinatorial" narrowed to the COUNT.** The void
  exclusion (a cavity is not a component) is a decided orientation
  read — per-shell signed volume at one named funnel site
  (`chk_shell_volume_sign`, comparand V_shell/A_shell, an honest
  length), sharing tier-3 check 7's flux walk (one implementation).
  The check stays fully certified — in-band or zero-volume shells
  refuse typed into the report, and a count over a partial
  classification is a guess, so one unclassifiable shell poisons the
  whole subject's count — but it is not predicate-free. The
  structural alternative (marking void shells at birth in
  `insert_void`, the one ratified cavity door) was considered and
  deferred: a stored mark must be maintained across
  revert/regroup/graft; the derived read needs no maintenance and
  inherits check 7's refusal posture wholesale.
- **The staleness direction is not optional polish.** The first
  review found the two halves of one hole: an annihilated boolean
  (∅ — F8's typed success) yields zero subjects and would have
  reported clean under an explicitly stated expectation, and an
  expectation keyed at a dead root was silently ignored — a stale
  acknowledgment reading as "checked and fine". The fix is the
  census's two-directional pattern verbatim: every stated
  expectation must be consumed by a subject; an unconsumed entry is
  a typed `StaleExpectation` finding; default expectations bind only
  existing subjects, so a legitimate ∅ with nothing stated stays
  clean. The waiver rule's staleness clause above is thereby
  confirmed by its first resident within one review cycle.

**Second resident SHIPPED (2026-08-29): the product-separation
check.** Resident `CheckId::Separation`, kind Certified, over every
pair of gathered solids the product took from DIFFERENT root subjects,
held to `topo::SolidSeparation`'s box-level certificate. It discharges
the obligation `topo::graft_disjoint_all_keyed` leaves to its callers
(that door asserts nothing about its operands, so someone must
establish disjointness) for the caller that had none — the product
gather. It REPORTS: a document whose roots interpenetrate still draws,
because the finding is the only thing that says the almost-right
picture is wrong.

**Its severity knob is `Advisory` (Off/Warn), not `Severity`, and that
is this doc's waiver rule biting for the first time.** The rule above
is an *iff*, and this resident ships no acknowledgment record — so
`error` is not merely unset, it is unrepresentable, enforced by the
type rather than by a comment asking callers not to reach for it. The
declared-contact suppression it does have is NOT a waiver and must not
be counted as one: it is derived from mates rather than authored about
a finding, keyed by kernel arena entities rather than stable names,
carries no provenance, and has no staleness direction.

**This is the resident that makes the waiver vocabulary stop being
paper.** The note above records it as deferred "until a resident needs
it"; one now does. `demos/tour`'s `heatsink.pncad` produces five
findings that are all CORRECT — its fins are unioned into its base
only in demo-side code, so its product genuinely is overlapping solids
— and there is no way to acknowledge them short of turning the
resident off wholesale, which turns it off for the diefillet defect
too. The shape it wants is `expected_components`' one subject up:
keyed by the `(root, output)` PAIR a finding names, with the same
two-directional staleness. Until it exists, `Advisory` is the honest
knob and the noise is the standing demonstration of the gap.

**Recommendation, refining I1's warn-never-refuse at the severity
knob (round 3, per Ev: blocking is fine if exceptions are
declarable).** Any check — certified or labeled-heuristic — may
offer `error` **iff it ships a waiver vocabulary**: a per-finding,
stable-name-keyed acknowledgment record ("the 2 µm step is
deliberate" — the sliver lint's own confirmation sentence from I1,
made data), carried with provenance like any declaration and with a
staleness direction (a waiver whose finding no longer exists is
flagged for cleanup — the `StaleContactDeclaration` shape, at warn).
A waiver is the check-side analogue of the parameter lint's
declared-distinct arm: it records intent *about a finding* rather
than making a geometric claim, so there is nothing to verify — only
to match, and to stale. The certified/heuristic label (certified:
the finding is a theorem — connectedness, moldability's hull bounds;
heuristic: a judgment that can be wrong in both directions — the
sliver threshold, machinability rules) is then honesty of language
and a default level, not a force cap: waiving a certified finding
accepts a theorem ("yes, this part is not moldable — it is not
molded"); waiving a heuristic finding overrides a judgment; the
label keeps a message from dressing the second as the first. What
stays banned is narrower than before: a check refusing **without a
waiver door** — what cannot be answered except by changing geometry
must not block on a verdict that might be wrong.

## DS7 — The permissive spectrum: provenance and the recording dial

**The provenance ladder.** Every declaration record carries how it
came to exist, and the framework treats the rungs differently only
in review affordances, never in verification (C4 verifies geometry,
not sincerity):

- **constructor-authored** — intent structural in the verb (PATHS
  `.fillet(r)` declaring the tangency it constructs; shipped
  precedent). Present at every dial position.
- **user-stated** — `Node::Declare`, `tangent_joints`, the
  detect/declare sugar; findings passed through user-visible hands
  as values (GS-Q3).
- **auto-recorded** — machine-written at a definite finding's first
  appearance, tagged as such, upgradeable to user-stated on review.

The user-stated/auto-recorded distinction is held loosely (round 3):
verification treats every provenance identically — C4 verifies
geometry, not sincerity — so the tag buys only review affordances
and honest blame in messages. Collapsing those two rungs is a cheap
simplification if the review door proves unwanted; the
constructor-authored rung is structural and stays either way.

**The recording dial**, per classification-grade discipline
(identification-grade disciplines have no dial at any position, DS2):

- **require** — undeclared definite coincidence refuses; the
  strictest position and the ratified default posture.
- **auto-record** — a definite coincidence arising where the author
  is currently working is recorded with `auto` provenance;
  subsequent evaluations diff the definite findings against the
  recorded set, and **changes complain**: a recorded coincidence now
  contradicted or stale fires the existing C4 alarms
  (`ContactContradicted`, `StaleContactDeclaration`) exactly as for
  user declarations, and a new finding outside the author's current
  edit surfaces as a finding rather than auto-recording. The batch
  bless-current-findings affordance is this position's one-shot
  form, and the review door for accumulated auto-records.
- **ignore** — nothing recorded, nothing diffed.

**The in-band family is already dialed elsewhere.** The escalation
band is governed by K (`Tolerance.k`, D4) — collapsing it toward the
precision floor is an existing per-run mechanism, not new machinery,
and its ratified default (K = 10, K-REPORT) is untouched here. Noted
because a "permissive profile" composes the two knobs, and because
the composition's semantics deserve stating in three separate layers
(round 3):

- **the verdict** — below ε a separation claim is not even
  representable, so a sub-ε pair honestly classifies coincident.
  That is forced by precision, not a snap, and a verdict alone glues
  nothing;
- **identification** — merging entities takes a structural or
  declared rung (or the dial's auto-record) at every K; no verdict
  reaches topology without one;
- **the data** — nothing is ever rewritten: no vertex moves, no
  per-entity tolerance grows; a declaration bridges the sub-ε
  residue semantically while both stored descriptions keep their
  exact values.

K governs none of these three. It governs only the guard band over
*distinguishable* values (ε to Kε — representable differences too
small to trust as design intent), so K = 1 means "any representable
difference is a trusted difference" and the kernel honestly builds
the sliver. Industry snapping is value proximity at a fat tolerance
driving identification and data rewrite at once; here the third
layer never happens and the second never happens by value.

**What each position honestly costs.** `require`: the declaration
interaction, made cheap by detectors and constructive verbs.
`auto-record`: **blind at birth** — an accident present from the
first evaluation (a radius typed equal to a wall thickness at
authoring time) is recorded as intent with no alarm; `require`
surfaces exactly that case. Recipes also accrete unreviewed intent —
mitigated by provenance tags and the review door, but the recipe is
no longer purely a record of stated intent. `ignore`: no diff basis,
so *edits* bless silently too — the accidental-coincidence alarm
never rings — and no recorded stratum intent exists for M10 to
bridge: interval/dual replay of a coincidence that was definite at
nominal f64 is honestly indeterminate over the parameter box and
escalates un-bridged. `ignore`-position models are point-defined;
`auto-record` keeps them germ-defined mechanically (the machine
declaration bridges), losing only human confirmation; `require`
keeps intent human-stated. The default posture stays `require`; the
dial exists so the trade is a stated choice, not a fork of the
kernel.

## DS8 — Sequencing, sizing, and the no-speculative-registry rule

This doc is design-only. The registry machinery lands with its
**first configurable resident**, not before — building reviewed
framework with no caller is the dead-code pattern the M5 reviews
punished, and today every implemented member of the family sits at
its strictest position. The shipped disciplines are **not
refactored onto the registry** for organization's sake; they adopt
shared machinery only where a second consumer makes the sharing real
(the finding/menu/severity sink at the document layer is the
first such seam — `refusal_menu` in
`editor-core/src/eval/wire.rs` already renders discipline refusals
through one door). The unification is PLANNED as of round 6 — #981:
the sink first, then the DS1 discipline scaffolding materializing
with the parameter-coincidence unit; the predicates themselves stay
in their geometry homes per DS1. The first resident (connectedness) SHIPPED at
round 4 with the registry's finding/severity/report plumbing; next
in the named order: the parameter-coincidence lint (Expr-layer only,
no kernel contact — the first discipline-shaped resident), then the
moldability checker (rides with the draft verb per DR6). The
recording dial ships, if ever, with its first discipline whose
default is not `require` — a dial with one used position is dead
code with a settings page.

## DS9 — Out-of-tree checks (round 5)

The registry opens to third-party checks. Three rulings (Ev,
2026-08-25, the origin conversation's continuation):

- **Extensibility wins over gatekeeping.** No earned-certified rule:
  the `CheckKind` label is the author's claim, and a user who
  installs a check trusts its author — the kernel does not police
  the label.
- **First-class over protective caps.** Registered third-party
  checks get the full machinery — the severity range, `error`,
  waivers — rather than being saved from their own defects. What
  keeps this safe is DS3, not vetting: no check, in-tree or out, can
  change an evaluated solid, so a bad check's blast radius is its
  own findings. The same ground scopes D9: reports are never
  persisted and enter no content keys, so a nondeterministic
  third-party check degrades only its own report; determinism
  remains an obligation on in-tree checks, a quality bar for
  external ones.
- **The kernel never learns Python exists.** Language bindings enter
  through one adapter, dependencies pointing upward only:
  editor-core defines the check trait; `pncad-py` ships a single
  Rust adapter check whose implementation invokes a registered
  Python callable and translates its output into the shared finding
  vocabulary. One adapter serves every Python check.

Consequences, binding on the implementing unit when it comes:

- Registration becomes a document-layer API over a dyn check trait;
  check identity becomes a namespaced id with the in-tree residents
  in a reserved namespace (the v1 closed enum was the right first
  shape and is superseded by this ruling when registration lands —
  the D3 closed-enum argument never bound here: checks are unary
  over an evaluation, no pairwise dispatch table exists).
- DS6's waiver rule moves from per-check obligation to **framework
  provision**: the report machinery supplies finding-keyed
  acknowledgment records generically, so "error requires a waiver
  door" holds by construction for every registered check.
- **Funnel participation (RULED, round 6 — Ev: expose it, and
  record provenance rather than excluding).** The plugin API exposes
  `decide`-with-`Margin` as the sanctioned comparison path, so a
  plugin inherits the escalation band, multi-ε behavior, and
  verdict-log testability instead of hand-rolled float compares.
  Verdict recording gains **producer provenance**: every recorded
  decision carries which check/discipline (kernel subsystem, check
  id, or plugin id) produced it, namespaced plugin sites included —
  one census stream, filtered by its consumers. A K re-derivation
  filters to kernel-origin sites, so third-party verdicts flow
  through the same funnel with no contamination of the K evidence
  base and no second-class channel. The provenance field is a small
  `k_stats` unit and a prerequisite of opening registration. The
  k-lint roster and the #214 dimensional audit remain repo-scoped
  regardless — they scan source, a physical limit rather than a
  policy.

## The grade table

| Grade | Kind | Placement criterion | Knob | Residents (built / named) |
|---|---|---|---|---|
| 1 | identification discipline | declaration licenses a quotient (DS2) | none, ever | carrier equality, declared contact (built) |
| 2 | classification discipline | same solid whenever it builds (DS3) | ignore / auto-record / require | profile tangency (built, `require`); right angles (reserved) |
| 3 | recipe-layer discipline | parameter-space stratum; kernel never sees it | ignore / auto-record / require | parameter coincidence (named) |
| 4 | advisory check | pure analysis, findings only | off/warn/error; error requires a waiver door (DS6) | connectedness (built, Warn); sliver, moldability, machinability (I1, parked) |

One pattern (DS1), two derivable placement criteria (DS2, DS3), one
shared sink, one provenance ladder (DS7). A proposed new rule is
placed by answering two questions — *does its declaration license an
identification? can it hold the severity invariant?* — neither of
which is a matter of taste.

## Open questions

- **DS-Q1 — Document-demanded strictness.** Knobs are run-global;
  should a document be able to *demand* a minimum position for
  itself ("this recipe requires parameter-coincidence at require"),
  so a strict author's model refuses under a lax reader's config?
  Cheap either way; DS3's monotonicity means the pragma can only
  refuse, never change geometry. Lean: yes, later, with the first
  grade-3 resident.
- **DS-Q2 — The symbolic middle rung.** `width/2` vs `0.5*width`:
  DAG-disjoint yet formally equal. Does the structural rung extend
  to a normalized-Expr equality (exact and deterministic
  normalization only), or do these land in the refusing rung with
  "unify" as the menu arm? Lean: refusing rung in v1 — normalization
  is a rabbit hole and the menu arm is the honest repair — but the
  question is recorded so v1's choice is a choice.
- **DS-Q3 — Funnel participation. ANSWERED for grade 4 (round 4);
  grades 2/3 stay open.** The first resident's one decided predicate
  went through `k_stats::decide` at a prefix-named site (`chk_*`),
  exact rungs did not, the comparand was an honest length (no
  `decide_flagged`, no ledger row). One wrinkle absorbed: "one
  decided predicate" means one SITE — a padded bracket is read at
  both ends (Outer needs the low end definitely positive, Void the
  high end definitely negative), so quadrature bodies may log up to
  two verdicts per shell at the site; closed-form shells log exactly
  one.
- **DS-Q4 — Naming.** "Discipline" vs "lint" vs "check" as the public
  vocabulary; this doc uses discipline/check with the knob shapes as
  the cross-cutting axis. *First-PR position (round 4)*: shipped
  vocabulary is "check" (`CheckId`, `CheckKind`, `Severity`,
  `run_checks`, `enforce_checks`), funnel prefix `chk_`; the doc
  keeps "discipline" for the declaration-bearing family. Reversible.
- **DS-Q5 — The auto-record boundary.** `auto-record` distinguishes
  "arising where the author is currently working" (record) from
  "appearing elsewhere under an edit" (complain). That boundary is
  an editor/document-layer notion (the node being authored vs.
  everything downstream), not a kernel one — evaluation is pure —
  and drawing it wrong in either direction re-opens a cost: too wide
  auto-blesses the classic in-node accident (radius = thickness
  typed into the edited node); too narrow nags on every authoring
  stroke. Needs its own small design pass with the first dialed
  discipline; findings must be keyed by stable name for the diff to
  survive edits at all.
- **DS-Q6 — Profile tangency's dial.** DS2 reclassifies it as
  dial-eligible; whether to actually expose its dial (vs. leaving it
  pinned at `require` as thesis) is deliberately not proposed here.
  Its declarations feed non-check consumers (`blend_arcs`, loft
  carriage) and constructors author them at every position, so the
  dial degrades those queries' coverage rather than their
  correctness — recorded so the exposure decision is made with that
  cost named.
