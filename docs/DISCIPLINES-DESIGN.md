# Disciplines and checks — the registry design conversation

**Status: DRAFT — design conversation, awaiting Evan's sign-off.**
(Born in the 2026-08-24 configurable-checks conversation; proposals
DS1–DS8, open questions DS-Q1–DS-Q4.) This doc schedules **no
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
  never per-entity; and D9 — bodies are never persisted, they
  re-derive from recipes.
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
   `tangent_joints`, `Node::Declare { pairs, class }`);
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

## DS2 — The fork criterion: which disciplines are mandatory

**A discipline is mandatory iff some kernel operation forks on its
stratum** — the op must answer "on or off" to build anything at all.
Sector classification at a touching edge forks on tangency; the
one-carrier-or-two decision forks on carrier identity; the
declared-REST zip forks on contact. For these, "off" has no
semantics: the op either has no answer at a fork it must take, or
takes the answer from value proximity — which is the EPS snapping
the ladder exists to kill. All three shipped disciplines are
mandatory by this criterion, which is why none of them carries a
switch today and none acquires one under this design.

The criterion is *checkable*, not a judgment call: name the op and
the fork, or the discipline is not mandatory. It also gives the
honest answer to "why is tangency mandatory but draft angle isn't" —
the asymmetry is not an accident of history; one stratum has
construction forks and the other has none.

## DS3 — The severity invariant: what makes a switch legal

A registry entry may carry a severity knob — **off / warn / error** —
iff it can hold this invariant:

> **No severity level ever changes the evaluated solid.** All levels
> build the same body or refuse; `error` is a refusal at the
> recipe-validation door, never a fork inside an operation.

Consequences, each load-bearing:

- **Monotone strictness.** Raising severity only shrinks the accepted
  set. No configuration change can silently alter geometry — the
  failure mode of cross-config document exchange is exactly one:
  refuse-on-load with the finding and its menu.
- **Cheap reconfiguration.** Unlike ε, a severity change requires
  re-*validation* only, never re-evaluation — no geometry re-derives.
- **No per-body state.** Severity configuration is run-global (the
  D4 posture). Bodies are never persisted (D9), so there is no
  at-rest bitmask to stamp; a document carries its declarations,
  which are meaningful at every severity level.

This invariant is the sharp boundary of the whole design: anything
that can hold it is *lint-grade* and gets the knob; anything that
cannot (because an op forks on the answer — DS2) is *kernel-grade*
and does not. The severity plumbing itself is trivial (a level enum
consulted at the finding sink); the design content is the invariant,
not the mechanism.

## DS4 — Grade 2: configurable geometric disciplines

Full discipline shape (declarations in the recipe, verified never
trusted, in-band escalates at `error`) — but no op forks on the
stratum, so the severity knob is legal. The declaration's value is
intent and robustness, not constructability.

**First named resident: declared right angles.** Perpendicularity is
intent-only by construction: convention 4 makes the kernel
equivariant, so no boolean, sector table, or classification can ever
consume "these faces are exactly perpendicular" — an undeclared right
angle is just a transverse crossing. What the declaration buys is the
stratum: under M10 a perpendicularity that holds by accident of
nominal values is a point property, destroyed by perturbation; a
declared one is a recorded design intent the stackup can hold the
model to. Verify table: definite = normal orthogonality at the
derived angular threshold (lever arm named per site, D4); contradicted
by definite non-orthogonality; bridged = the in-band residue between
independently-authored faces. **Reserved, not built** (the GS-Q2
convexity posture): no corpus site demands it yet; the vocabulary
slot is named so the first demand lands as a registry entry, not a
milestone.

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
  record intent, which is what makes `error` livable;
- **refusing** — DAG-disjoint expressions, definitely-equal values,
  no declaration either way: the finding.

For literal operands the equality test is exact — no ε anywhere; for
derived values it is the ordinary Q1 definite-equality trilean.
Scope: **named parameters only**, not raw coordinate literals —
base-rate control is what keeps the finding a signal (models live on
grids; parameters colliding at equal values is rare and
informative). The consumer that makes the declaration meaningful is
M10: same variable = the pair comoves under the distribution;
declared-distinct = independent marginals. The lint is the
completeness condition for a tolerance stackup being well-posed over
the definitely-equal pairs — and it catches the one classic
parametric defect no geometric check can see: the copy-pasted
dimension that was meant to be shared, where both models are
bit-identical at nominal and differ only in parameter space.

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

**Recommendation, refining I1's warn-never-refuse at the severity
knob**: a **certified** check (connectedness — exact; moldability —
certified normal enclosures) may offer `error`, holding DS3's
invariant like any lint; a **labeled-heuristic** check
(machinability, the sliver threshold) caps at `warn` permanently — a
heuristic must never refuse. This gives I1's honest-labeling rule
teeth: the certified/heuristic label is not just message text, it
bounds the check's maximum force.

## DS7 — Rejected: the auto-bless mode

Considered and rejected: a run-global permissive mode in which a
*definite* coincidence verdict at a discipline's op site is taken as
intended and recorded, in place of the undeclared refusal (in-band
escalating in every mode — bridging in-band without intent is value
gluing, never on the table). The mode is coherent and cheap — the
refusal sites already sit on computed verdicts, and the record
carriage works unchanged with auto-minted records — and it is
rejected on three grounds:

1. **It relieves only the cheap cases.** In-band still escalates, so
   the mode is stricter than industry kernels where they are
   permissive, while the exact-coincidence cases it does relieve are
   the ones `declare_all(find_*())` already reduces to an accept.
2. **"Assume the result was intended" is only valid at authoring
   time.** A declaration records intent about a configuration the
   author looked at, and is edit-robust in both directions
   (`ContactContradicted`, `StaleContactDeclaration`). A mode records
   a policy, and blesses coincidences that did not exist when the
   choice was made — a later parameter edit that lands two features
   exactly tangent sails through with the alarm off precisely in the
   accidental-tangency scenario the discipline exists for.
3. **It erases the stratum intent M10 consumes.** An auto-blessed
   contact definite at nominal f64 is indeterminate over the
   parameter box; interval replay escalates with no declared intent
   to bridge. Auto-blessed models are point-defined; declared models
   are germ-defined — and the reach goal shapes the architecture
   even while deferred.

Structurally, the mode is GS-Q3's forbidden fused detect-and-declare
door promoted to a global. Its honest cousin is in scope instead: a
batch **bless-current-findings** authoring affordance that *writes
the declarations* — the same zero-thought UX at authoring time, with
the findings passing through user-visible hands as values and the
artifact keeping the intent.

## DS8 — Sequencing, sizing, and the no-speculative-registry rule

This doc is design-only. The registry machinery lands with its
**first configurable resident**, not before — building reviewed
framework with no caller is the dead-code pattern the M5 reviews
punished, and today every implemented member of the family is
mandatory-grade and already built. The three shipped disciplines are
**not refactored onto the registry** for organization's sake; they
adopt shared machinery only where a second consumer makes the
sharing real (the finding/menu/severity sink at the document layer is
the plausible first such seam — `refusal_menu` in
`editor-core/src/eval/wire.rs` already renders discipline refusals
through one door). Likely first implementation pressure, in order:
the connectedness lint (exact, data already computed), the
parameter-coincidence lint (Expr-layer only, no kernel contact), the
moldability checker (rides with the draft verb per DR6).

## The grade table

| Grade | Kind | Fork? | Severity knob | Residents (built / named) |
|---|---|---|---|---|
| 1 | mandatory discipline | yes | none, ever | profile tangency, carrier equality, declared contact (all built) |
| 2 | geometric discipline | no | off/warn/error | right angles (reserved) |
| 3 | recipe-layer discipline | no (kernel never sees it) | off/warn/error | parameter coincidence (named) |
| 4 | advisory check | no | certified: off/warn/error; heuristic: off/warn | connectedness, sliver, moldability, machinability (I1, parked) |

One pattern (DS1), two derivable placement criteria (DS2, DS3), one
shared sink. A proposed new rule is placed by answering two
questions — *does an op fork on it? can it hold the severity
invariant?* — neither of which is a matter of taste.

## Open questions

- **DS-Q1 — Document-demanded strictness.** Severity is run-global;
  should a document be able to *demand* a minimum severity for
  itself ("this recipe requires parameter-coincidence at error"), so
  a strict author's model refuses under a lax reader's config? Cheap
  either way; the monotonicity of DS3 means the pragma can only
  refuse, never change geometry. Lean: yes, later, with the first
  grade-3 resident.
- **DS-Q2 — The symbolic middle rung.** `width/2` vs `0.5*width`:
  DAG-disjoint yet formally equal. Does the structural rung extend
  to a normalized-Expr equality (exact and deterministic
  normalization only), or do these land in the refusing rung with
  "unify" as the menu arm? Lean: refusing rung in v1 — normalization
  is a rabbit hole and the menu arm is the honest repair — but the
  question is recorded so v1's choice is a choice.
- **DS-Q3 — Funnel participation.** Grade-2/3 decided predicates go
  through `k_stats::decide` with their own site prefix (the `sel_*`
  precedent), exact rungs do not — presumed, needs confirming
  against the #214 ledger discipline when the first resident lands.
- **DS-Q4 — Naming.** "Discipline" vs "lint" vs "check" as the public
  vocabulary; this doc uses discipline/check with severity as the
  cross-cutting knob. Bikeshed deliberately deferred to the first
  implementing PR.
