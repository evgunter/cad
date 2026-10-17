# CAD Kernel — Design Document

Present tense only. Everything in this document is ratified: changing
it is a design decision, discussed with Ev and recorded in the PR that
makes it, and the history behind a clause lives in that PR and in git,
not here. The document leads with the principles; every section after
them is an instantiation and opens by naming the principles it derives
from.

**Citations.** A clause is cited by its stable semantic id in
brackets: a section's id (`[tolerance]`), or a clause's within it
(`[tolerance.recourse]`). An id keeps its meaning for as long as it
exists, and a retired id is never reused. Citations across the tree
written before this revision use the old numbering (D1–D10, "the
central commitment", Q1–Q9, `D4 ¶1 (i)`, frontier (c)–(f)): those
resolve against this file at commit `95c77e152a`, and
[Old numbering](#old-numbering) at the end says where each now lives.
Update an old citation as a drive-by when touching the text that
holds it.

## State of the build

**The intent refactor is in progress, and much of what
[intent], [variables], [operations], [repetition], [placement],
[coincidence] and [assertions] describe is not built yet.** The INTENT
program builds it in stages (`work/intent/plan.md`), with COINCIDE,
REFS and INLANE as its successors; every other program holds off
starting units that meaningfully touch dependency, placement or
intent until it lands (`d10-one-way-to-say-intent-is-unbuilt`).

- **Built:** variables (stage 1); operations that read and define
  variables, selections as definitions, the product as a list
  (stage 2); booleans gluing on a Zero verdict and the boolean
  declare surface retired (stage 4 E and F).
- **Not built:** spaces and placement (stage 3 — `Transform`,
  `Datum`, gauges, offsets and the mates' spanning tree still exist in
  the tree); the rest of the coincidence door and its lint (stage 4);
  assertions and the at-rest lints (stage 5); tangency constructions
  (stage 6).
- **What retires as the build reaches it.** Where a companion clause
  disagrees with these sections, these govern, and the companion
  clause retires when the program reaches it: the declared-contact
  seats (CONTACT-DESIGN C4's `BooleanCoincidence` on booleans, a
  mate's `ContactClass`, C6's `Fit` as a class); the
  undeclared-coincidence, undeclared-contact and undeclared-tangency
  refusals, which become `unproven-coincidence` findings, with
  ASSEMBLY A5's hard error on an unattributed contact; DISCIPLINES
  DS2's identification grade; the axis declaration channel;
  `ParamSource`; NAMES N6's recipe-source stamps (`GeomSource`, with
  `AxisSource`); PARAM-LINT's declared-distinct record; the profiles'
  stored tangent-joint flags; ASSEMBLY A3, A10's sink rule, A11 (2)'s
  gauges and offsets, A11 (4)'s declaring mates and A12's reading
  edges; and A5's at-rest gate (`assemble` and its refusal), whose
  census becomes a check, with DISCIPLINES DS6's product-separation
  resident (`Separation`).

The program's exit walk checks that list against the code
(`work/intent/plan.md`, "Exit criteria").

## Vision

A greenfield B-rep solid-modeling kernel in Rust, built API-first: the
kernel and its programmatic modeling API are the product; any GUI is a
thin client. Code quality and functional style are explicit goals — a CAD
kernel's job is to define what a shape *is*, and the implementation
should read that way.

**Reach goal (shapes the architecture):** native error propagation —
distributions over model parameters propagated through the model to
detect self-intersection and to compute tolerance stackups.

## Principles

The root principle is **functional style**: a CAD kernel's job is to
define what a shape *is*, and the implementation reads that way —
values rather than mutable objects, pure functions rather than hidden
state, data rather than behaviour. The six principles below are that
stance applied to a solid modeler.

### The model is a pure function `[pure-model]`

> **A model is a pure, replayable function from the values of its free
> variables and a tolerance to its product.** Deterministic, no hidden
> state. The B-rep is a derived value, never a mutated-in-place
> object.

Determinism is over the pair: the same values at the same ε give the
same product. ε is one value per run, committed once
([tolerance.epsilon]). Whether the findings of the at-rest checks are
part of this function's output is open ([open.lint-output]).

Everything else is derived. A body is never a source of truth: it is
the evaluation of its construction, and where the two disagree the
body is wrong ([topology.body]). What follows from holding this from
day one:

- **Error propagation** is "evaluate the same function with a
  different scalar type" (intervals, duals) instead of a rewrite
  ([scalars]).
- **Undo, caching and diffing** are free — models are values.
- **Testing** is property-based testing over parameter space.

The data-shape consequence: the geometry *evaluation* layer is generic
over a scalar type `T` (default `f64`); topology stays concrete
([scalars]).

*Instantiated in:* [scalars], [topology], [recipe], [operations],
[determinism].

### Intent first `[intent]`

The document records what its author means, and ideally nothing else;
everything the kernel builds is computed from it; and what was written
is checked against what was meant.

- **The document stores intent.** `[intent.stored]` A slot that admits
  more than one value holds a variable, so a number the author wrote
  is a value they can name, share and vary ([variables]). What the
  kernel needs beyond that — a 3-D carrier, a pcurve, a profile's
  loop — is a derived cache of a description of what the thing *is*,
  certified against it ([edges], [recipe.forms]). An imported file has
  no variables, so adoption recovers descriptions and never intent
  ([import]).
- **Intent is stated one way per kind of statement.**
  `[intent.statements]` Dependency is reading a variable
  ([operations]); location is a placement ([placement]); that two
  things coincide is said either by constructing them as one, which
  proves it, or by asserting a bound, which claims it and is checked
  ([assertions]). Nothing else carries dependency or intent, and
  nothing restates what a construction already says.
- **Geometry follows the values.** `[intent.geometry]` What the kernel
  builds is decided by margined verdicts on the values
  ([intrinsic], [verdicts]), not by what anyone meant: a coincidence
  decided Zero glues whether or not it was intended. A body is the
  same whatever intent was stated and whichever checks are enabled.
- **What was written is checked against intent.** `[intent.checked]`
  Where the kernel decides something from values that the document
  does not establish, a lint says so. The `unproven-coincidence` lint
  checks every coincidence decided from values against what the
  document proves, and reports one that holds only at the current
  values together with the edit that would make it one construction;
  an assertion at its site quiets it ([coincidence], [assertions]). A
  lint reports and never changes a body.
- **What counts as established.** `[intent.structural]` The principle
  is that an identity the document establishes is one that very likely
  reflects intent. Its instantiation is one simple, consistent rule:
  two cells are structurally one when the kernel's own verdict on them
  is zero as a polynomial in the document's variables
  ([coincidence.structural]). The rule may change when a change cuts
  "confirm what I already said with numbers" noise without much more
  "I wasn't warned about a problem in my model"; because glue is
  decided by value, such a change moves findings and never a body.

*Instantiated in:* [variables], [operations], [repetition],
[placement], [coincidence], [assertions], [edges], [recipe],
[import].

### Decide on the thing itself `[intrinsic]`

A decision is taken only on unambiguous, intrinsic properties of the
thing it is about. Two ways of missing that are kept apart:

- **What is meant, not a proxy for it.** `[intrinsic.meaning]` A
  decision reads the property it is about, not one that usually goes
  with it. Properties that are correlated but not dependent are kept
  apart, so a thing with an unusual mix of them is handled, not
  misread: no rule lumps them into one category and then decides on
  whichever member it happens to read. An edge's selector asks whether
  its curve is known in advance, of the geometry and never of where the
  body came from ([edges]); the census reads the cells themselves,
  never the lines or planes they lie on ([topology.tiers]); coincidence
  is a margined verdict, never bit-equality ([verdicts]). Where it
  matters that two things are exactly the same, compare their content
  by hash, never their names: a reference pins its document's content
  hash, and a pairing is decided by document identity, never by node
  ids ([assemblies]).
- **What matters, not something contingent.** `[intrinsic.contingent]`
  Nothing is decided on what does not matter to the answer: how
  floating-point arithmetic happened to round, the order things were
  built, enumerated or listed in, the frame something was computed in,
  how a value was spelled, a hash map's iteration order.

- **Don't guess.** `[intrinsic.no-guess]` Where the geometry does not
  determine an answer at the tolerance in force, the kernel does not
  pick one. Today that is the sliver band: a margin within K·ε of zero
  refuses typed, with its decision's recourse ([verdicts]).
- **Correct in absolute terms.** `[intrinsic.absolute]` Margined
  comparison is not transitive: a near b and b near c does not make a
  near c. Two rules keep that from getting out of hand. Everything is
  correct in absolute terms — within ε of what it describes — never
  merely close to something else that is; and topology is decided
  once, by a verdict, after which cells are one or apart by topology
  and nothing re-asks the question from values
  ([verdicts.closeness]).
- **ε is a pointwise length.** `[intrinsic.epsilon]` "Correct to within
  ε" means every point lies within distance ε of where it should; a
  margin is a distance at which that does or does not hold
  ([tolerance.margins]).
  A quantity that is not such a distance — a mean width 2A/P, a mean
  thickness V/A — decides nothing against ε.
- **Determinism and equivariance** `[intrinsic.invariance]` are
  `[intrinsic.contingent]` applied to the run and to space: the same inputs give the
  same bits ([determinism]); a designed rule commutes with rigid
  motions and reflections, preferring intrinsic quantities to orders
  and tie-breaks ([determinism.equivariance]); and an operation's
  computing frame changes no verdict outside the sliver band
  ([placement.frame]).

*Instantiated in:* [verdicts], [tolerance], [determinism],
[placement], [edges].

### Few kinds of thing, one way to do each `[one-way]`

The system has as few kinds of thing as it can, and whenever possible
one way to do each thing, done in one place and stated in one place. A
new kind is introduced only where no existing one is it: an assembly
is a document, and the kernel has no assembly type ([assemblies]); a
copy, a pattern and a moved body are all placements ([placement]); a
pattern is an index, not an operation ([repetition]). Two ways of deciding one fact can decide it
differently inside the band; two places stating one fact can drift
apart. A family of authored forms is not a second way when every form
lowers through one reading ([recipe.forms]). The canonical places are
the next section.

### Bad states are unrepresentable `[unrepresentable]`

Before a bad state is handled at run time, ask whether a change of
type removes it, and prefer that whenever its cost stays local
([failure.taxonomy], row 0). Degenerate sites live in argument types,
not behind null checks ([topology.operators]); an empty loop is a
variant, not a sentinel ([topology]); encodings are tagged, never
in-band magic values ([engineering]); a margin is a `Margin<T>`
constructed only through doors that state its dimension
([tolerance.margins]); a span borrows the structure it indexes
(SPLINE-DESIGN S1); surface and curve kinds are closed enums
([surfaces]).

### Failure is loud, and its kind is plain `[loud-failure]`

Nothing absorbs a failure: no silent discard, no hidden healing, no
fallback that answers where a method declined, no loosening to make
something pass. A miss is surfaced, whoever caused it: absorbing it
hides it, and the recourse it names says whether the model or the
kernel is at fault ([tolerance.recourse]). A failure caused
by a kernel bug is a different kind of thing from a failure caused by
the input, and which kind a failure is is always plain: a bug panics,
as early as it is detectable; an input never panics and gets a typed
error that names the failing check, the entity and the recourse; valid
input the kernel has not built yet gets a typed `Unsupported*` refusal
naming its blocker. These are the project conventions of [failure] and
[tolerance.recourse].

## Canonical places `[one-way.places]`

The one place each of these is done or stated. A second place for any
of them is a defect.

| What | Its one place | Clause |
|---|---|---|
| The tolerance | one ε per run (`geom_core::Tolerance`); angles derived from it at a named lever arm | [tolerance.epsilon] |
| A topology-determining decision | a named predicate returning a trilean and a margin | [verdicts] |
| A question about carriers | one reading per question | [one-way.readings] |
| What a decision may do with its answer | lane choice or structural act | [verdicts.consumers] |
| A refusal's recourse | one per decision, written in the decision's levers | [tolerance.recourse] |
| Topology construction | the Euler operators | [topology.operators] |
| A cavity's birth | `topo::insert_void` | [topology.conventions] |
| A body's at-rest validity | one gate, for native and imported bodies alike | [topology.tiers], [import] |
| Contact records across an op | one lineage, one carry | [topology.tiers] |
| An edge's geometry | one intensional description; every representation a certified cache | [edges] |
| A locus one chart defines | `Chart`, one (surface, `Pcurve`) form | [edges] |
| Dependency | reading a variable | [operations] |
| Location | a placement | [placement] |
| A pose's symmetry and a mate's fold | one `Subgroup` type | [variables.poses] |
| A coincidence decided from values | recorded at the one coincidence door | [coincidence] |
| Whether a coincidence is structural | that door, and nowhere else | [coincidence.structural] |
| A value authored several ways | a family of forms lowering through one reading | [recipe.forms] |
| Which version of a document a reference means | its content pin | [assemblies.pins] |
| Which document a held value is of | the identity it carries, never its node ids | [assemblies.pairing] |
| A document's validity | `persist::check::validate_document`, at save and at load | [engineering] |
| The frontier | typed `Unsupported*` refusals, grep-able from the tree | [failure] |

### One reading per question `[one-way.readings]`

**One reading per question.** Every decision the kernel takes about
geometric carriers is a *reading*: of one carrier and a point (its
residual, distance, foot and chart coordinates); of a carrier pair's
*pose*, one classification per pair (apart, coincident ±, coaxial,
parallel axes, point touch, line tangent, generic, in band), whose
trileans are its parts and are decided once; or of a pair *outcome*
(the section's carrier, C5; the section's components and their
witnesses; two spans' meetings; a curve's crossings of a surface).
**Total**: a reading is one exhaustive match over its kind or kind
pair. An unlisted arm refuses typed, naming what it lacks; no arm
answers a default value; one method is fixed per arm at write time, and
a second method for the same question exists only as a test oracle. A
stage may precede the method only if it is total in kind, decides the
same question soundly at the same band, and cannot change the verdict
by which stage decided (a box cull before exact roots); a method
reached because another declined is a fallback, and is forbidden.
**One**: each question has one reading, which every consumer reads,
because two readings of one fact are two predicates that can decide it
differently inside the band ([tolerance]). An outcome takes the pose reading's
answer and decides no pose of its own. A consumer serving fewer pairs
matches on the reading's answer and refuses the rest typed; it never
matches on kinds to choose a method. Which pairs a reading serves is
derived from the reading, never kept beside it: where a consumer asks
at kind level before it holds the operands, the reading dispatches
through its own kind-level description (kinds to arm, then the arm's
method), so the description cannot disagree with what the reading
does. A total evaluator's poison for a
kind with no closed form is not a decision, and no reading reads it
without first refusing on kind. Readings over bare carriers live in the
lowest crate that can state them, `geom` for one carrier and a point
and `geom-brep` for carrier pairs; readings over trimmed faces and
solids live in `topo` and reach carrier facts only through those.
Exempt: kind-agnostic methods (boxes, BVH, subdivision, clearance),
identity of stored bits on which no topology decision depends, test
oracles, and reading a record (a declaration, or an edge that already
is the section). Import is not exempt: it reads the same readings with
the file's own coincidence distance as the band. C5
(`crates/geom-brep/README.md`) is this rule's instance for surface and
curve pairs.

## Decisions and verdicts

### Scalars `[scalars]`

*From:* [pure-model], [intrinsic].

- Evaluation code (evaluators, derivatives, transforms, measurements)
  is generic over a `Real` trait we define. Instantiations: `f64`,
  `Interval` (the in-house `interval-transcendentals` backend; the
  scalar and the kernel's lane impls at it compile in every build),
  `Dual<T>` (one in-house generic type; `num-dual` is a dev-only oracle
  because its std-backed transcendentals cannot satisfy the
  value-channel bit-identity contract), and `Sym`.
- **`Real` trait surface**: comparison-free by construction (no
  `PartialOrd`/`PartialEq`, plus a style rule and a CI tripwire for the
  residual channels); all operations total with poison propagation
  (NaN/empty) — poison flows through *values*, never *decisions*;
  `sin_cos` is the primitive; no order-implicit reductions. **No fused
  operations** (`hypot`, `mul_add`): a formula rounds the same way at
  every scalar type and on every platform, so the f64 build, a dual's
  value part and the determinism contract agree ([intrinsic],
  [determinism]); last-ulp accuracy is the price.
- **Interval scalar**: the *decoration as the poison channel*
  (`decoration < Def ⇒ Indeterminate(Invalid)` — silent domain clamps
  never decide); `Bounds` means only "carries a bracket" and
  `CertifiedEnclosure` is the certification half, which `Dual` does not
  implement (DUAL-DESIGN: a dual may not certify — it carries a bracket
  in its value channel, but tangent transport is not enclosure);
  poison-visible NaN brackets for empty AND NaI; tight `pown`
  (containment of the true value is the interval contract).
- **`Dual<Interval>` semantics**: *value-part delegation* — `Decide`
  classifies the value only, the derivative never influences a branch
  (tangent-space data does not decide base-space topology). Kink
  conventions: f64 tangents are branch-consistent with the value
  channel (abs′(0) = +1, ties keep self); the interval instantiation
  carries the *Clarke subdifferential enclosure* (straddle hulls).
- **Genericity boundary**: `Body<T>` = scalar-free topology arenas +
  `T`-valued geometry arenas; topology contains no `T` and never
  branches on it; keys are *body-lineage-scoped* (key identity across
  same-history builds is what lets an interval replay share topology
  with the f64 build). The certified-lane pattern is **f64 structure +
  `T` payload**: candidate generation (`ssi::jet`/`march`/`system`, the
  analytic composite's implicit form) is `f64`-only and untrusted, and
  the certificate is derived and carried at `T` (`SsiCertificate<T>`,
  `FittedLane`) — so a WIDENED analytic operand refuses typed
  rather than picking a representative surface out of the family.
- **The editor-core evaluation service is generic over `Real`.** M10's
  error-propagation UI rides the same memoization / cancelation /
  per-node-result machinery as f64 rebuilds; no parallel path.

### Verdicts `[verdicts]`

*From:* [intrinsic], [loud-failure], [one-way].

- Every topology-determining branch goes through a *named predicate
  function* returning a trilean sign plus margin, generic over `T`. No
  raw `<` on control-flow paths.
- At `T = f64` predicates are total: margins within K·ε escalate (the
  *sliver band* — semantically indeterminate even under exact
  arithmetic). K is a policy dial — refusal rate and f64 noise headroom
  — not a correctness parameter: soundness rests on
  escalate-never-guess, [tolerance.residuals] certification and interval replay, for
  any K > 1. K = 10, per-run configuration like ε (`Tolerance.k`).
  K's noise headroom presumes every margin's f64 error is a few ulp at
  the model's extent: a formula whose error is amplified by a vanishing
  quantity (a division by sin θ, |n|², a near-zero determinant) is a
  defect, re-posed or certified, never covered by K. Interval replay
  does not rescue it: its enclosure lands in the band, which is
  terminal.
- **Closeness is not identity.** `[verdicts.closeness]` Margined
  comparison is not transitive, so no decision is inferred from two
  others. Cells are one only by topology (a Zero verdict glued them,
  and from then on they are one cell) or by structure
  ([coincidence.structural]); both are transitive because neither is
  a comparison of values. A body is valid when every question its
  topology poses decides definite, or Zero and recorded.
- At `T = Interval` an indeterminate predicate aborts the operation:
  predicates return `Result<Sign, Indeterminate>` (the trichotomy is
  the primitive; bool predicates are projections) and construction code
  propagates with `?` to the **subdivision driver** (`editor_core::drive`,
  ERROR-DESIGN E6), which splits the parameter box and re-runs — a pure
  model makes re-running sub-boxes trivially correct and embarrassingly
  parallel. Leaves take definite branch paths; outcome probabilities
  are the distribution's measure on the sub-boxes. The sliver band is
  *terminal* for the driver: an enclosure wholly inside (ε, Kε) never
  refines.
- A persisted decision log is *dropped*: reified predicates are the
  load-bearing part.

- **Coincidence is a margined verdict.** Bit-equality is never the
  test — it would make topology hinge on an UNMARGINED predicate, a
  razor-thin equal-vs-one-ulp cliff with no escalation band, exactly
  what the trilean rule above forbids. A margin decided Zero is a coincidence and glues; a
  definite margin keeps the cells apart; a margin in the sliver band
  refuses typed, its resolution an explicit construction or
  repair/adoption — [import]'s machinery applied natively. Nothing is
  declared. The `unproven-coincidence` lint ([coincidence]) checks whether each
  coincidence so decided holds structurally. Consequence: topology
  depends only on recipe structure and margined verdicts, so predicate
  flips remain the *only* topology-change sites.
- **K = 10 is the permanent ratified default** (#89 CLOSED,
  `docs/K-REPORT.md`).
- **Fillet/blend validity is reified predicates, not try-and-fail.**
  Every classic failure is a margined predicate over the inputs — r vs.
  1/κ_max of the support, r vs. adjacent-face extent, spine regularity,
  corner configuration — stated in the feature definition. Payoffs:
  typed, diagnosable pre-construction errors; the predicates are [verdicts]
  predicates, so M10 can certify **fillet validity over a parameter
  box**; corner reconfigurations are enumerated predicate flips,
  extending the naming pillar to fillets. Same principle for
  shell/offset.

**What a consumer does with the answer.** `[verdicts.consumers]`
A reading of which special
case a geometry is (a class, a pose's trilean) answers *shown* (Zero),
*ruled out* (definite) or *in band*, and in band means only that the
case is not shown. Its consumer is one of two kinds, and each owes the
build and the `unproven-coincidence` lint exactly what it does with the
answer. A **lane choice** picks a form for one certified statement the
body has either way: a pcurve row's class, an edge's rest locus, a
section curve's formula where both answers give the same cells, a
carrier's recognised kind at import. Zero takes the closed form; in
band and definite take the general lane, whose certificate does not
rest on the answer, and every class a lane choice reads has one. A
lane choice records nothing, never ends as a kernel defect at a build,
and refuses only where its general lane is not yet written, with the
closed form's pose as its lever and never a tolerance offer. A closed form before the general lane is a stage of the one
reading, not a fallback: both state one certificate. A **structural
act** changes which cells the result has on the answer: a boolean's
glue, a vertex fusion, an edge join, a split's ON verdict, a section
trilean that decides how many curves meet and where. Zero acts and is
recorded ([coincidence]), in band refuses with the decision's own lever ([tolerance.recourse]),
at the reading or at the result's finished-body gate, and definite
keeps apart. At rest, only a definite contradiction of what is stored
is a defect.

### Tolerance `[tolerance]`

*From:* [intrinsic], [loud-failure], [one-way].

No per-entity tolerances that grow as operations get sloppy (the Open
CASCADE model). "Define what something is" applied to error handling.
Five commitments:


**One number, global per run.** `[tolerance.epsilon]`
a linear tolerance ε, defined once in
`geom-core` as the `Tolerance` value — once-initialized,
env-overridable per run (`CAD_TOLERANCE_EPS`), **one value per run,
shared by all bodies, never loosened mid-run** (per-model ε is
rejected: any two bodies must be boolean-combinable). Per-run
initialization lets the test suite run at several ε values.
**Angular thresholds are always derived, never a second global**: an
angle only means anything through the displacement it induces at a
lever arm (d = r·θ), so a fixed εₐ would privilege a hidden length
scale. Every angular predicate uses θ = ε/r with its lever arm named
at the call site — 1/κ_rel for tangency, the face extent for
parallelism, the session-box extent as the conservative universal
arm. ε ≈ 1e-9 m gives micron-to-kilometer coverage with ~4 orders of
f64 headroom at km scale. Import does not motivate loosening ε ([import]).


**The two-tolerance principle.** `[tolerance.recourse]`
Two roles are kernel-wide
vocabulary: **ε_precision** (this ε — certification residuals, what
gets built) and **ε_input** — the least precision a user might care
about: what counts as too-close-to-a-coincidence when interpreting
input. ε_input IS K·ε, a synonym, not a third dial; K stays the one
knob (`Tolerance.k`, env `CAD_AMBIGUITY_K`). Binding consequences:
(i) user-facing messages and recourse never fork on exactly-on vs
in-band below ε_input — ONE message, ONE recourse per **decision**
(the one question a site asks), written once in the decision's own
levers and shared by all its refused arms, with the margin riding
the payload as data; kernel semantics keep the distinction (message
policy, not predicate policy). The recourse follows from the
decision and its verdict: *make the two one construction* ([coincidence])
only where the refused side is a coincidence a shared construction
would decide; *move the geometry*, phrased as the decision's own lever, always;
*tighten the tolerance* only where the refused margin is a size the
user may intend and a smaller ε decides it — the band-decided arms
(in band, or Zero where Zero does not pass) of a decision that passes
on a nonzero sign — phrased conditionally and with the value the
margin gives ("if this size is intended, tighten the tolerance below
m/K"); never on a sign-certain arm, and never on a decision that
passes only at Zero (a residual), whose refused margin is a miss,
not a size. No refusal advises loosening ε, with one exception: a
refusal that would otherwise name no recourse at all (a kernel
approximation limit — an offset fit that stalls, a quadrature budget
spent, a built curve's residual) names loosening the tolerance as a
last resort and says the refusal may indicate a kernel bug worth
reporting. At adoption, certification runs at ε and a refusal reads
as at rest. ε_in is the file's own coincidence distance ([import]): it
governs interpretation — what the file's data is evidence of, where
healing may move geometry by up to O(ε_in) — and its refusals are
the import door's, named in ε_in with the value the margin gives; a
size below ε_in is not one the file intends, so no refusal offers to
keep it. Until adoption rebuilds its caches from the file's
descriptions ([import]), a certification refusal whose miss lies within
ε_in but beyond ε (the file's data agrees to its own declared
coincidence distance, not to this run's) names setting ε to ε_in as
a stopgap, beside re-exporting the file more precisely. A change to
a document's ε reports what it flips at the change itself. The
three-arm sentence (make them one construction / move the geometry
/ tighten the tolerance) is thus the recourse of a decision whose
refused side is a coincidence a construction could decide; an
`unproven-coincidence` finding refuses nothing, and its recourse is
one construction or an assertion at its site ([assertions]). The decision is a closed type at its site (as
`CertCheck` is), so its recourse is an exhaustive match, never a
lookup by predicate name. (ii) Error variants may stay distinct as
data; their user stories converge per decision. `Indeterminate`
carries data, not a recourse: its Display renders the payload, and
the site that knows the decision appends the one ending
(`COINCIDENCE_RECOURSE` is the sentence coincidence decisions
compose). (iii) [import]'s ε_in is an instance of ε_input, not a separate
concept. (iv) **The rule binds a predicate's DEFINITE arms too**: a
new definite outcome a user could reach by moving geometry tells the
same one story with the same one recourse as its in-band sibling. A
surface reading a body at rest may end a definite contradiction
between a stored description and its stored geometry as a defect —
no move of the geometry reaches it — and never names a lever the
decision does not.
Review checklist form: for every arm added to a decision, name which
ε_input story it belongs to, or say why it belongs to none.


**Chordal tolerance δ is not a tolerance in this sense.**
`[tolerance.chordal]`
Tessellation/export take a per-call display parameter δ (chordal
deviation), chosen per export, varying freely, participating in no
kernel validity decision. The tessellation promise is
**certified-conservative** — closed-form sagitta/deviation bounds
guarantee the mesh lies within δ of the true surface (honestly δ+ε,
plus ≤1 ulp per coordinate from STL's f32 narrowing) — an export
promise, not a kernel invariant. The mesh layer never reads ε for
sizing (the inventory is a computed pin,
`mesh/tests/all.rs::the_eps_inventory_is_pinned`); the one ε read
that is a classification — pole identification — owes a guard so an
ε flip can never move emitted coordinates with δ held fixed (#896).
**The tessellation criterion is DISTANCE-ONLY**, as ε is
([intrinsic.epsilon]): δ bounds how far any point of the mesh lies from
the true surface, and nothing else. There is no angular-deflection
criterion in the certified tessellator — every contracted consumer
is manufacturing-shaped, and a certified angular bound would cost
normal-variation enclosure machinery purchased only for visual
smoothness δ already buys. A display-mesh lane, when built, gets its
own separately-honest criterion and is never promoted into the
certified export promise.

**The margin dimensional convention.** `[tolerance.margins]`
ε is a length: the maximum
deviation from specified geometry at a single point. Four clauses
make that structural: (i) **Margins are lengths, by signature.** The
classify/Band seam takes a `#[repr(transparent)]` `Margin<T>`
newtype — no dimension algebra, no generic dimension parameter —
whose only constructors are blessed doors that make the dimension
argument explicit at the call site (a coordinate difference that IS
a length; a dimensionless quantity levered by an arm; a norm; a
measure over the lever at which it is a point's displacement, never a
mean such as 2A/P or V/A, [intrinsic.epsilon]); the sagitta and reciprocal
forms are the levered door applied at named arms; there is no raw
construction door. Consistency backstops between integral results
decide on bare `T` through `k_stats::decide_invariant`, no Margin
minted, and a certified violation is a Corrupt-class kernel-invariant
error, never a validity refusal. A site where no door honestly fits
is a finding, not a cast. The vector/linalg interior stays bare `T`.
(ii) **No dimensionally-heterogeneous uniform payloads**: a field
whose dimension depends on a runtime kind tag is illegal;
kind-dependent data lives in per-kind enum variants. (iii)
**Parameter-space values cross to model space only through per-kind
metric doors.** (iv) **Inequality gates split sign from magnitude**:
a certified sign-certain violation refuses with no ε involved; the
banded comparison governs only the near-zero region, and both arms
consume the same metered comparand. Extension is opportunistic as
signatures get touched; the migration ledger is
`docs/predicate-dimension-audit.md`, whose flagged rows ride
`k_stats::decide_flagged` — visible typed debt, not casts.

**Every derived cache carries a certified residual bound.**
`[tolerance.residuals]`
against
its intensional description ([edges]). Kernel invariant: `residual ≤ ε`
for every derived item in a valid body; the `topo` validator checks
it, at `f64` as a conservative estimate and at `Interval` as an
enclosure. A certified upper bound on a vector-valued residual is
read from the Euclidean norm of each coefficient, never from a
per-coordinate box folded into a norm: the bound is then a function
of the geometry, and a rigid map moves it only by its rounding width.

**Failure is a typed, actionable error naming the failing check and
the entity.** `[tolerance.typed-failure]`
consumable by humans and by the error-propagation
machinery. The carrier is `CertifyError::ResidualExceeded { check,
sample, margin }`, wrapped by the attachment gates and by
`ValidationError::EdgeCertification`; the residual MAGNITUDE rides
both arms as the classifier's reporting margin — the definite arm's
`margin`, the escalated arm's `Indeterminate` — because no value
projection of a generic `T` to an `f64` exists on every lane; the
classify seam's `MarginDiag` is the one projection there is, and it
is for reporting only, never a value a decision reads.
Geometry that cannot meet ε is surfaced, never absorbed, whether the
model or the kernel caused the miss ([loud-failure]).
A typed refusal is never read as a verdict: a consumer may only
take it down a path that decides the question itself.

**Fixed internal units — meters and radians — with a documented model
size range.** `[tolerance.units]`
the session box is 1 km, no coordinate larger
in magnitude; geometry outside the range is rejected at
construction. The box fixes the smallest admissible ε, ε_min =
1 km · 2⁻⁵² ≈ 2.2e-13 m, the length f64 resolves at the box's
edge: a run's ε below ε_min is refused, and no refusal offers to
tighten the tolerance below it. User-facing units are typed
newtypes at the API boundary only ([units]).

**Strictness is enforced at the boundary, not relaxed inside.**
`[tolerance.boundary]`
STEP
import is an adoption stage ([import]) that brings external geometry up to
kernel invariants *before* it becomes a kernel body; entities that
can't be adopted fail loudly in a typed import error. An imported body
is never distinguishable from a native one: no state is reachable only
through import, so anything import admits is a state some native
construction could reach, and anything else it refuses at the door.

- **ε and persistence.** A document records the ε it was authored
  under; the application pins the run's ε to the document's; an
  assembly whose referenced documents disagree on ε is a typed error
  (`ResolveFault::EpsilonSeam`). **Changing ε is a recorded
  `SetTolerance` document edit**: apply = replay at the new ε and
  structurally diff — the verdict-vector diff reports exactly which
  predicates changed verdict, escalations included, and any change is
  a typed error requiring explicit resolution. Same diff machinery as
  the naming pillar: ε changes and parameter changes are both "same
  recipe, different evaluation context."

## Geometry and topology

### Topology `[topology]`

*From:* [pure-model], [unrepresentable], [one-way].

- Topology entities (`Solid / Shell / Face / Loop / HalfEdge / Edge /
  Vertex`) live in generational arenas and reference each other by
  typed IDs — never pointers. A B-rep is a plain value: cheaply
  cloneable, serializable, diffable, validatable. Realized as Mäntylä's
  half-edge structure in typed arenas: an `Edge` is two antiparallel
  half-edges, the mate computed and never stored; the empty loop is a
  typed `LoopBoundary::Empty | Cycle`, so every half-edge field is
  non-optional and the sole `Option` in topology is the vertex's
  emanating half-edge. A face's outer loop is excluded from its ring
  list (`outer ∉ rings`), so rings coincide with the Euler–Poincaré
  r-term.
- **Manifold solids only.** Non-manifold (radial-edge) topology is added
  only if sheet/wire bodies demand it.
- Topology is built **exclusively through Euler operators**: a small
  closed set of primitives that provably preserve the Euler–Poincaré
  invariant. Each operator debug-asserts its postcondition — a per-call
  instance of the soundness theorem, never a semantic gate on
  intermediate states. The postcondition has two halves and they are
  paid at different rates: the operator's declared arena delta is O(1)
  and is checked at **every call**, while the whole-body tier-1
  re-derivation is O(body) and is checked **once per public door**, at
  the door's end, over the state the caller will see
  (`work/perf/d1-per-op-tier1-sweep-price`, Ev, PR 2305). An operator a
  consumer calls directly is itself a door and sweeps there; one inside
  a composing door's surgery scope leaves the sweep to that door. The
  guarantee is unchanged — every public mutation path preserves tier 1,
  checked at every observable boundary — and an opt-in
  `topo/per-op-postcondition` feature restores the per-operator sweep
  to name the operator behind a door-level failure. **One class needs
  the scalpel rather than merely benefiting from it**: a corruption an
  operator introduces and a later operator in the same door repairs is
  invisible at the door, because the state the door hands back is
  sound. The door-level check is a claim about that state, not about
  every state the door passed through, and the ruling's "caught one
  door later" is exactly that trade. "Exclusively" is
  realized: the operator set is the only public construction path; raw
  insertion is crate-internal test scaffolding.
- **A `Body` is never a source of truth.** `[topology.body]`
It is the materialized evaluation
  of a construction (an operator sequence; above the kernel, a recipe)
  at some scalar `T`, coherent iff bit-identical replay reproduces it
  ([determinism]). Mutation exists only as evaluator-internal linear working
  state; a body at rest is a plain value, and modification means
  deriving a successor body by further construction. Nothing about a
  body is true that is not derivable from its construction (for
  imported bodies, from the adopted descriptions plus the import
  record, [import]).
- **Non-manifold boolean results are typed errors** — "non-manifold"
  meaning **non-representable** ([topology]'s representability boundary).
  Touching via distinct entities is a typed success validating at tier
  3′. Silent splitting into separate manifold bodies is rejected as
  inexplicit; any future split behavior is an explicit ratified
  operation the user invokes, never a fallback.

**Topology conventions.**
One rule, from which everything else is a
corollary: walking any loop in `next` order with the face's outward
normal toward the viewer, the face interior lies to the **left** of
every half-edge. Corollaries: outer loops run counterclockwise viewed
from outside and rings clockwise; an edge's two half-edges are
antiparallel (`end(he) = start(mate(he))`); `Edge::he_plus` defines the
edge's intrinsic direction, and curve geometry MUST agree — increasing
curve parameter runs from `start(he_plus)` to `end(he_plus)`, pcurves
and per-face traversal senses derived from that, never stored as peers;
the vertex-orbit step `next(mate(he))` visits a vertex's outgoing
half-edges **clockwise** viewed from outside. Transcription hazard:
GWB/Mäntylä's diagrams orient face boundaries clockwise viewed from
outside — mirrored relative to us — so figures, argument orders and
traversal idioms from the book are rederived from the interior-left
rule and pinned by construction tests, never transcribed. The normative
derivations live in `crates/topo/src/entity.rs`.

**Face orientation sense.** A face carries `Face::sense: bool`: `true`
iff the face's material side agrees with its surface's chart normal, so
the outward normal at a point is `n(u, v)` where `sense` is `true` and
`−n(u, v)` where it is `false`. The analytic
chart normals admit no reversal by reparameterization (cylinder, cone
and torus normals are odd in the radius; the sphere's is even and
outward under the `radius > 0` convention — a negative-radius sphere is
rejected as a representation), so the bit on the face is the
representation of reversal. Normative consequences:
- The interior-left rule is stated against the sense-signed chart
  normal.
- Orientation reversal is **exact structure**, never a numeric decide:
  `revert` flips `sense` on every face carried by a non-plane surface,
  negates the stored normal of `Plane`-carried faces, and inverts every
  joint element, moving it onto its source predecessor (a joint's
  element read from the other side is its inverse — `topo`'s
  `JointElement`); no loop's anchor moves. The two normal encodings are
  exclusive by surface kind, so every outward normal is negated exactly
  once, inverting an element is exact integer arithmetic, and
  `revert ∘ revert` is bit-identical at every scalar backend.
- A face's `sense` is decided where its chart is decided. An Euler
  operator minting a face on its parent's chart (`same_chart`: one
  key, or keys sharing one payload) derives the bit from its own
  topology: a `mef` fragment is a piece of the parent's region and
  takes the parent's bit; the ring `mfkrh` promotes was wound
  clockwise about the parent's outward normal and now bounds its face
  counter-clockwise, so that face takes the parent's bit negated
  (`kfmrh` demotes the other way). A face minted or re-charted onto
  any other chart carries the bit its caller states beside the
  surface (`FaceSurface::New`, `Shared`); a stated bit that
  contradicts the derived one on the parent's chart is refused, typed,
  before mutating. Never a numeric compare, and never a default. Where
  the parent bounds no region yet, its bit and so the derived one are
  provisional, and the constructor states the honest bit when it
  charts the face.
- Every "which way is out" consumer (tier gates, mass-properties flux,
  boolean classification, tessellation and export winding) reads the
  signed normal, or documents in place why it is sense-invariant.
- The bit is exactly STEP's `advanced_face.same_sense`, so the exporter
  consumes it rather than deriving it. Bodies are not serialized ([determinism]),
  so persistence is unaffected.

**The operator set.** `[topology.operators]`
Ten operators in five make/kill pairs —
`mvfs`/`kvfs`, `mev`/`kev`, `mef`/`kef`, `kemr`/`mekr`, `kfmrh`/`mfkrh`
— plus the `ring_move` reparenting helper, deliberately **not** an
Euler operator (`mef` does not reclassify rings, after GWB's `ringmv`).
Addressing is by half-edge key plus per-op **site enums** whose
variants are the degenerate cases (`MevSite::{Fan, Lone}`): degenerate
sites live in the argument types, not behind null checks. GWB's id-scan
layer is dropped; arena keys are the stable O(1) handles. The uniform
per-op contract: **atomic** (typed-error preconditions fully resolve
before an infallible mutation phase; a failed op consumes no key
slots), **deterministic minting order** (documented per op — [determinism]
lineage replay), and a **debug-asserted postcondition** — the declared
arena delta at every call, the whole-body tier-1 re-derivation once per
public door ([topology]).
Association convention: **the given/first half-edge's side is the new
or affected thing** — `mef`'s `he1` side becomes the new face's outer
loop, `kemr`'s `he1` side becomes the ring, `kef` kills the given
half-edge's face, `kev` the vertex it points at. Cross-shell `kfmrh` is
the shell-fusion form (same solid), `mfkrh` its inverse; `ring_move`
reparents only within one shell (`EulerOpError::CrossShell`).

**Validity tiers.** `[topology.tiers]`

1. **Tier 1 "euler-valid"** — the structural invariant of every
   Euler-reachable state, construction scaffolding included (empty
   loops, struts, self-loop edges, laminae are mandatory
   intermediates); what each operator debug-asserts. The checklist:
   referential integrity across all arenas (orphan geometry is an
   error); half-edge chain coherence; mate involution/antiparallelism;
   vertex anchoring (every vertex is referenced by ≥ 1 half-edge XOR is
   the lone vertex of exactly one Empty loop); vertex-orbit closure
   (manifoldness — watertightness is structural in the half-edge
   form); the ownership/back-pointer partition; shell-partition/
   edge-adjacency coherence; arity floors; bidirectional [identity] provenance;
   and the **component-aware per-shell Euler–Poincaré**: per connected
   component of a shell's incidence complex, v − e + f − r = 2(1 − g)
   with g a non-negative integer, summing per shell to 2(c − Σgᵢ) over
   its c components. The naive per-body form is wrong for tier-1
   bodies — `mfkrh` on a detached ring disconnects a shell's surface
   while a single shell entity remains.
2. **Tier 2 "closed solid"** (`validate_closed`) — tier 1 plus: no
   empty loops, no valence-1 vertices, and c = 1 per shell (the third
   ban is independent: a promoted detached cycle ring disconnects a
   shell with neither an empty loop nor a strut).
3. **Tier 3 "geometric"** — [tolerance.residuals] residual certification, plus the
   **material wedge-angle predicate**: at every edge the material wedge
   ∈ (0, 2π), bounded away from the ends by θ = ε/r; wedge = π is the
   legal smooth-seam case; and the ends carry a **second-order arm**:
   wedge = 0 (a cusp) and wedge = 2π (a knife slit, the cusp's `revert`
   image — legal together or not at all) are legal iff the tangency is
   **jet-determinate**: quadratic transverse separation with κ_rel
   bounded away from zero — `TangentIntersection`'s own margin, so the
   cusp edge's honest description IS `TangentIntersection` wherever the
   tangent certificate's lane reaches. The ends are derived from the
   body exactly as the π seam is: a cusp and a smooth seam are the same
   tangent junction on one shared edge, told apart only by the sign of
   the two faces' outward normals, which the body already certifies.
   The shared edge is the structural record. A tangency is constructed
   (the profile's cusp joint, a tangency construction, [coincidence]) or decided
   Zero by its margin where an op meets it, and a tangency decided from
   values is recorded and checked by the `unproven-coincidence` lint
   like any coincidence. In-band κ_rel escalates; osculation
   refuses (`LaminaWedge`). The
   arm admits no laminae, so zero-volume bodies stay geometric defects.
   A doubled cusp (two material wedges on one tangent line) is the
   coincident-distinct-edges class, each edge classifying separately;
   where no edge lies on the line, the two wedges are a touch between
   faces, recorded like any touch, and no op cuts edges to make a
   junction of it. Consumers with no wedge-0/2π answer (fillet, offset,
   mesh sizing, sector classification), and none for a body's
   curve-touch records, refuse typed at the consumer.
   Also at tier 3: **maximal edges** (a joinable vertex at rest is a
   construction that stopped half-way, `JoinableVertexAtRest`; a
   vertex whose regularity reading lands in the sliver band, so that
   whether it is joinable is undecided, refuses too, with the
   tighten-the-tolerance recourse: a size the run's ε cannot tell from
   a pole or apex), **prefer-intrinsic enforcement** ([edges]) and the
   **orientation invariant** (a solid's faces leave infinity outside
   it, read by the reader of a shell's role, [coincidence.booleans]: a ray
   aimed at one of its faces, the certified signed volume only where no
   face offers an aim; definitely inside-out ⇒ invalid, and a solid
   whose side no reading decides refuses — an orientation probe, not a
   thinness gate, at every door). Laminae live here, not at
   tier 2: two faces glued along their whole shared boundary is a
   two-hemisphere ball's incidence structure, so a zero-volume lamina
   is a geometric defect, not a topological one. Global
   self-intersection / minimum clearance is the interval clearance
   engine's: its body-level half — cell subdivision over
   `Body<Interval>` — lives in `editor-core` today and moves into
   `topo` (SHELL-3, ruled at #1737), with the
   parameter-box outer half above it in `editor-core`, so a verb that
   must certify a boundary embedded (`shell`'s cavity clone) runs the
   same engine at a certifying scalar and refuses typed at the door.

   A **finished body** (`AtRestBody`) passes tier 3′ against its own
   contact records: none, for a body that touches nothing, where the
   census must find no coincidence at all. Every door that returns or
   consumes one pays that gate once, at the door that built it.
   The Boolean door's result meets tier 3 without the census: its
   contact records, each citing the coincidence that backs it, ride
   beside it (`BooleanBody::contacts`), and the
   census over them is parked on CONTACT's cross-solid curved lane
   (`work/reachhold/boolean-door-runs-the-census-over-its-result.md`).
   Construction state (tier 1, or tier 2 without geometric
   certification) is what Euler operators hand back, and becomes a
   finished body only through the at-rest gate.
4. **Tier 3′ "pseudomanifold"** (`validate_pseudomanifold`) — the
   honest at-rest tier for boolean results that *touch*: contacts
   limited to entirely-coincident-but-distinct edges, edge-on-face,
   vertex-on-face, vertex-on-edge/vertex — touching allowed, proper
   self-intersection not. Composition: tier 3's local battery
   verbatim (shared extraction) plus a **global coincidence census**
   plus **two-directional contact-record certification**:
   - The census is exact on the planar inventory and admits every
     carrier kind: same-key opposed-sense curved pairs certify through
     the conformal arm, face–face records through the patch
     certifier on one carrier or two planes and the jet schedule
     along the closed-form tangent line on two other carriers,
     cross-solid pairs with a curved
     side in reach refuse as undecidable, and same-solid distinct-key
     curved pairs stay undetected until C9/C6 (`topo::census` module
     docs). A record or candidate outside a certifier's lane refuses
     typed `CensusUnsupported`, never samples. Every comparison is a
     named trilean ([verdicts]); indeterminates surface as typed
     `CensusEscalated`, never a silent skip.
   - A census finding decides one number per pair of cells (two edges,
     an edge and a face, a vertex and a face, two faces), read between
     the cells themselves, never between the infinite lines or planes
     they lie on, and signed where a face gives a side. For cells that
     share nothing it is their minimum distance. Cells that share a
     vertex, a point key or an edge meet there by construction, and the
     question is whether they meet anywhere else: the number is their
     largest distance over the shorter cell, beyond what they share. For
     straight cells it falls at the shorter cell's far end, its length
     times the sine of the angle between them ([tolerance.epsilon]'s d = r·θ, with
     the cell's own extent as the arm). The verdict's three arms ([verdicts]) read it: a
     definite number is a legal configuration, however thin; one in the
     sliver band refuses; a Zero is a coincidence (the cells are one
     over their extent), which the op that made the body glues and
     records ([coincidence]), so a census Zero no record backs is that op's
     defect. Near what two cells share, every pair is within the band
     whatever its angle. That neighbourhood is not a coincidence, and
     its extent is not a number the census reads.
   - Certification runs **both directions and never scans-to-bless in
     either**: a census finding with no backing record is never
     silently blessed ((iii) below); a record with no geometric witness
     is stale and refuses typed. Structural sharing (same surface or
     point key) needs no record. Vertices an op knows or decides to be
     one point share one point key, whether they stay in one solid or
     lie in two: an op that cuts one vertex into copies hands each copy
     the original's point, as it hands a cut face's fragments one
     surface; vertices whose gap an op decided Zero share the key it
     keeps, and the op records that decision (a `VertexFusion` row,
     cited by the pair's vertex-vertex record) for the lint, not for
     the census. A Zero-gap vertex pair on two point keys in a finished
     body is its producer's defect.
   - A contact record is a pair of cells, one from each touching
     shell (or each side of a shell's pinch, whose two sides touch
     within one shell, recorded as between shells), whose interiors meet, plus its backing: a
     coincidence an op decided Zero, recorded ([coincidence]). The record is
     that pair and nothing else: an unordered pair of the body's own
     cells, in canonical order, in one list a citation addresses by
     position. It carries no operand label, no stored granularity and
     no witness. Its kind (vertex–vertex, vertex–edge, vertex–face,
     edge–edge, edge–face, face–face) is the two cells' dimensions, and
     whether it touches at a point, along a curve or over a patch is
     read from the two cells' geometry by the census (CONTACT-DESIGN
     C3), whose rungs back a subordinate vertex event. An operation's
     own working vocabulary (the boolean's operand-tagged cells) stays
     inside it and is dropped at its carry. Every pair whose interiors
     meet is recorded, an edge lying in a face or along an edge
     included; only how the census confirms a record depends on its
     geometry. A straight edge in a plane, or two collinear lines, is
     confirmed by reconstruction from its bounding vertex records
     (between two backed bounds, two lines sharing two points are one
     line; a missing bounding record is an unbacked contact, never
     inferred), a curved one as a curve touch; a crossing of two edge
     interiors has no bound, and its edge-edge record backs it. Every op carries the records through
     one lineage, which states for each cell what the op made of it:
     kept; substituted by a cell that holds its interior (a merge's
     face, a join's edge, a seam); divided, into its pieces and the
     cells the op minted between them, the division exhaustive or with
     its remainder removed; or removed. One carry, for the boolean and
     every surgery verb alike, lands a record whose two cells the op kept
     or substituted on their images by lineage, unread, their geometry
     unchanged and the record read where it was minted; it puts each
     pair drawn from a divided cell's images to the census's record
     confirm, and the record lands on every such pair whose interiors
     meet, each landing citing it. Two
     cells an op minted on a record's two cells and that meet at a point
     meet where the record puts them when one cut made both, as the
     split's one plane does, and the landing cites the record alone; an
     op that cuts with many surfaces (the boolean's tool) decides every
     such meeting at its door ([coincidence]), and the landing cites the decision
     beside the record; any other op refuses one. A record
     that lands on no pair, every pair read definitely, is consumed
     where one of its cells was removed or divided with its remainder
     removed, and is stale otherwise; a cell the lineage does not cover
     refuses. A cell that replaces another without holding its interior
     (a blend face for the edge it rounds) takes no record from it.
   - **Certification strength equals its skeleton**: a curve touch is
     certified at its jet samples plus hull bounds, a patch by
     definitely-positive region overlap in the shared chart, a
     vertex-granularity area contact via its corner/segment records.
     Nested-shell pure containment (a void, zero coincidences) is
     census-invisible and certifies — the voids story below, not a
     gap.

   **Touching is always recorded**: (i) operand coincidences are
   decided by margined verdicts ([coincidence]) — a Zero margin is a coincidence,
   and a sliver-band one is an escalated typed error, never contact;
   (ii) result-side touching arises only from those coincidences
   propagated through the op's lineage, from the op's own decision
   where two cells it minted on one meet at a point, or from an op's
   copies of one vertex at such a coincidence, which share its point; the result carries
   machine-checkable contact records (the ON-set survivors, carried
   across seam-zip/merge mints by a descendant map, never re-derived),
   each citing the coincidence ([coincidence]) that backs it, which the
   `unproven-coincidence` lint checks; (iii) a contact the
   census discovers that no record backs is never silently blessed:
   between copies at rest it is an `unproven-coincidence` finding
   ([coincidence]), and inside one op's result it is that op's defect, refused
   typed.

   **Validity class rides the result wrapper, never a mutable `Body`
   field**: a boolean result is `BooleanBody` — body + contacts — whose
   at-rest gate is `validate_pseudomanifold(&body, &contacts)` over its
   own contact list, empty or not; with the list empty, the census must
   find nothing.

   **Representability boundary**: pseudomanifold touching via
   *distinct* entities (two vertices at one point, two edges on one
   segment) is representable in the half-edge structure and is a typed
   *success* carrying its 3′ records. Genuine non-manifoldness — a
   single edge with >2 faces, a shared-entity wedge fan — is
   unrepresentable and stays a typed error at the site that would have
   needed it. "Non-manifold" means non-representable.

**Structural conventions.** `[topology.conventions]`

- **Sweeps emit single-shell primary boundaries; every CAVITY is born
  through the shared void-insertion door.** A cavity's boundary is a
  disconnected interior shell, and its bookkeeping — orientation,
  census participation, containment evidence — has exactly one home:
  `topo::insert_void`, the door the boolean owns, callable without the
  SSI pipeline for provably-no-crossing cases with caller-certified
  containment. Four producers: boolean subtraction (its probe
  verdicts); `shell`'s sealed hollow (its offset margin, OFFSET-DESIGN
  O4); the full revolve of a holed profile, DEFINED as
  `revolve(outer) − revolve(hole-as-outer)` and executed through the
  degenerate no-crossing arm (the profile's validated 2-D margins); and
  a concave blend's fill sealing a blind hole's mouth, DEFINED by the
  blend's Boolean semantics (`S ∪ F`) and executed by its surgery (the
  mouth's cycle on the face the fill covers).
  Recipe-layer sugar may wrap any of these; the door stays the one
  birthplace. `UnsupportedToroid` is permanent: a [surfaces] ring-torus
  boundary — spindle tori have no representation.
- **∅, disjoint, and voids are typed results.** ∅ is a typed success
  value (`BooleanResult::Empty`), not an error. **A solid is one piece
  of material**: the `Outer` shell around it and the `Void` shells of
  the cavities in its material, which together bound that material and
  nothing else. A cavity belongs to the piece whose material surrounds
  it, so an island inside a cavity is a solid of its own, and a cavity
  inside that island is the island's. Pieces that only touch (at a
  corner, along an edge, or across a face that contact records hold
  apart) are distinct solids, because touching is between distinct
  entities (tier 3′). A piece that touches itself stays one solid. A
  body is any number of solids, so a disjoint union is a body of
  several solids. Booleans, `shell` and `split` take bodies and return
  bodies, and each sorts its result into solids, so every output is an
  operand. A product (the copies its world placements define, [placement]) is
  not a boolean operand: the editor refuses it, naming the explicit union
  of the copies that makes them one body
  (`crates/editor-core/ASSEMBLY.md`, A2). Which solids are one part stays recipe
  structure, never body state (`crates/editor-core/ASSEMBLY.md`, A2).
  The extrude/full-revolve
  hole asymmetry is structural: extruded holes are cap-to-cap tunnels
  (one shell, genus); full-revolve holes are cavities (a second shell,
  through the door); partial revolve is extrude-shaped and carries
  holes in its one shell.
- **The minimal sphere at rest is V2/E2/F2**: tier 2's valence-1 ban
  makes a one-band wire sweep unrepresentable, so axis-touching full
  revolves sweep two π-bands and poles have valence 2. The split is
  owed only by CURVED walls, whose pole or apex must keep valence 2. A
  full revolve emits each PLANAR wall as one face: a disc's centre is
  interior to the face, not a vertex, and no revolve returns two
  adjacent faces on one plane.
- **Parameterization conventions** (authoritative text in the `geom`
  crate docs and its `curves`/`surfaces` modules): curve entities are
  complete loci; an edge's bounds derive from its vertices via the
  `he_plus`-forward contract (certification enforces forward, nonzero,
  ≤ one-period spans, the stored interval a certified cache reconciled
  against vertex authority). Shared azimuthal frame for all revolution
  surfaces (axis = +a₃, v_ref = axis × u_ref, seam at u_ref — for
  revolved bodies u = 0 IS the profile half-plane); sphere uses
  latitude; cone v = slant length with the apex a true chart
  singularity (poison normal, never sampled); normals are the chart's
  ∂u × ∂v with no "outward" contract — topology carries sense. A seam
  (a **wrap edge**) is a property of a face, not a place on its
  surface: an edge both of whose sides bound one face, across which
  that face's chart domain closes in one of its periodic directions.
  Where it sits is where the construction cut the wall — the profile
  vertex — read off the edge's carrier through the chart (a meridian
  wraps u, a torus parallel wraps v, a closed spline net's boundary
  column wraps u), so the mirror-nappe cone's chart u = 0 needs no
  spatial pin. A wall keeps the cuts its author wrote and the kernel
  adds none: a one-segment closed loop sweeps to one wall with one wrap
  edge, a k-piece loop to k walls on one surface key.
- **Profile format**: a profile loop is a vertex chain whose segments
  are each a carrier plus a signed interval on it — a line (the chord
  between its two vertices), or an arc (centre, radius and signed sweep
  Δθ, |Δθ| ≤ 2π). Vertices are stored verbatim and are authoritative; a
  full turn is ONE segment at ONE vertex (|Δθ| = 2π), so a closed
  carrier is one edge. The form is redundant (the vertices lie on the
  carrier, Δθ agrees with them mod 2π, 0 < |Δθ| ≤ 2π), and those
  consistency conditions are never trusted: each is either checked at
  validate or holds by construction, proved by what built the arc at
  the scalar it was built in, and none is decided twice (where each
  is decided: `crates/profile/README.md`); a stored carrier is carried
  verbatim, including across scalars, and never re-derived from its
  vertices. The authored shape lives in the program, which is what a
  document stores and edits; the loop is its canonical cache, rebuilt
  from it and never persisted, and each arc mode's lowering is that
  mode's one conversion into this form. Each construction lowers what
  the user wrote into this form in the
  shape's own algebra (the radius as authored, the sweep as one
  `4·atan` of a quantity algebraic in the authored data, never an
  angle difference), so the symbolic tier, replaying the program, reads
  the authored shape through the stored fields. An identity the
  algebra does not close is registered by the construction that proves
  it, on the values it built; nothing about it is stored on the arc,
  and a copied or embedded arc claims nothing. The arc carrier is one
  type shared by the profile and the B-rep sketch segment. Bulge
  (b = tan(Δθ/4), DXF-compatible) is one of the path algebra's arc
  modes, lowered into this form once, at the algebra — not the
  storage. Winding is invisible to users (roles derive from
  containment). Downstream re-inspection of arc geometry uses the
  stored carrier data, never endpoint atan2.
- **Profile tangency**: each junction's tangency is a margined
  verdict. The PATHS constructors (`.fillet(r)`, the tangent
  continuations) author exact tangency by construction, verified, never
  trusted (`TangencyContradicted`), with fit gating
  (`TangentJointOutOfRange`); a junction decided Zero that no
  constructor made is a tangent joint all the same, recorded for the
  `unproven-coincidence` lint ([coincidence]); the sliver band refuses. Every
  zero-turn joint is a tangent joint. Zero new ε: the per-junction
  classifier reuses the carrier predicates verbatim. The tangent-joint
  set is derived at lowering, from the constructors and the junction
  verdicts, never stored.
- **Curved booleans retire per arm, never wholesale.** A face kind with
  no arm refuses typed `CurvedBooleanUnsupported` /
  `CurvedPairUnsupported` naming the pair, never falling through to a
  containment verdict a curved boundary can defeat. The wired germ join
  arms are a plane against a plane, cylinder, sphere or cone, and two
  spheres or two parallel cylinders along their radical plane
  (`boolean::join`'s dispatch); a plane cuts a cone in a parabola or a
  hyperbola only outside the conic inventory, and that pair refuses
  naming the conic. Cone and torus operands answer where their face
  pairs are certified with no event on them; every other curved pair
  with an event on it refuses typed at its section frame or at the
  join. The curved
  extent test refuses typed `NurbsExtentUnsupported` on NURBS faces — a
  certified extent needs a foot point plus a bound on the patch's
  reach past it, a derivation not yet written (C12.1).
- **Coincidence discipline in the reduction.** Every
  reduction/classification comparison is a trilean ([verdicts]): definitely-off
  ⇒ clean side, decided Zero ⇒ ON, in-band ⇒ escalated typed error (a
  genuine sliver: the operand pair is ill-conditioned at this ε). An
  ON verdict that only places topology is derived; one that would make
  pieces of one result touch (a split vertex whose orbit has two or
  more runs on one side) is a coincidence decided from values, built,
  recorded and reported by the `unproven-coincidence` lint unless it is
  structural ([coincidence]). No snapping beyond the Zero verdict: a merged face
  keeps one operand's description, and nothing in the sliver band is
  moved. Booleans on independently modeled nearly-touching bodies fail
  loudly rather than guess — the design thesis; the resolution is one
  construction or an explicit [import]-style repair/adoption op.
- **Maximal-faces precondition and the merge stage.** Booleans
  precondition no two adjacent coplanar faces (`NonMaximalFaces`); the
  explicit opt-in normalization op is `merge_coplanar_faces` (merging
  is never silent), and boolean *outputs* run it as a documented final
  stage of the op's contract — the seam zip manufactures coplanar
  pairs by construction; the recipe records one boolean node, not
  hidden healing. A sweep mints no same-key adjacency to merge: it
  builds one wall per run of profile pieces on one carrier
  (`crates/sweep/README.md`, "Walls"). Merge glues on a Zero verdict
  between the carriers ([coincidence]), never on bit-equality: a boolean merges a
  cross-operand coplanar pair its margins decide a continuation, records
  it for the `unproven-coincidence` lint, and keeps one description. A boolean whose output
  stage cannot glue a planar group its verdicts decided a continuation refuses
  the step with the merge's own typed reason, so every boolean output
  is a legal boolean operand; only a curved group's skip is recorded
  and shipped. Every op's output also has **maximal edges**: no
  *joinable* vertex, meaning valence 2 with two distinct edges on one
  structural carrier between the same two faces. Two edges are on one
  structural carrier when both lie on one locus of the faces' surface
  pair, or on one iso family of one surface's chart, and that locus is
  one regular curve at their shared vertex: the surfaces are transverse
  there (tangent arms one order up), or the chart is regular there.
  Distinct components of one locus never share a point, so the vertex
  decides the branch, and nothing is compared between the two edges. A
  pole, a cone's apex or any point where the locus is not one curve is
  never joinable; a reading of the vertex in the margin band refuses
  typed. A closed edge the join makes keeps one *conventional vertex*:
  a vertex whose only edge is that one closed edge, at both its ends.
  Its position, and the edge's parameter origin and witness that hang
  on it, are conventional data the kernel owns, like a seam's place.
  The vertex has no identity of its own. It mints no name a reference
  binds to; a reference or a pick there resolves to the edge, and a
  record, census reading or later cut at its point reads the edge's
  interior. A sweep builds one rim
  edge per run, as it builds one wall, and a boolean's output stage joins
  every joinable vertex after the merge, whatever drew it. A body is then
  the unique complex with maximal faces and maximal edges over its face
  partition, up to the position of each closed edge's conventional
  vertex, so a union's body does not depend on its member order, and
  the form is checked at rest on the result alone, with no history, by
  the one predicate the join takes a vertex by: every door that
  finishes a body, import included, ends with the join, and a vertex
  whose regularity reading lands in the sliver band refuses at rest,
  naming the tolerance below which it would be decided.
  Load-bearing dependency: `merge_coplanar_faces` **never fuses two
  vertices into one**. A contact record is a pair of cells, one from each
  touching shell (vertex, edge or face), whose interiors meet, plus its
  backing. A vertex resting on an edge's interior is the record
  (vertex, edge). The output stage deletes a vertex in two shapes, both
  recorded in `killed_vertices`:
  - the free end of a seam edge the glue left dangling inside the
    merged face, which goes together with its edge;
  - a joinable vertex, whose two edges become one.

  The join carries every record citing the three cells it replaces to
  the edge it makes, by substitution with its backing unchanged, so
  records are written by the op that changes their entities and never
  re-derived at the gate (F1). Tier 3′'s strict record-drop rule (a
  contact record whose vertex pair fused into one vertex is consumed and
  drops) is correct *because* nothing is fused. Any future fusing
  re-opens the record-carriage class.

**The frontier is typed and named.** Every unbuilt case refuses with a
message naming its own blocker ([failure.taxonomy], row 2), so the
frontier is grep-able from the tree. Its prose inventory is
`docs/KERNEL-VERBS.md` — the "present today" paragraph, the crate
READMEs, and the standing frontier entries (c)–(f) older citations
name — and its schedule is the tracker.

### Edges `[edges]`

*From:* [intent.stored], [one-way], [loud-failure].

Topology and geometry live in separate arenas: faces reference
surfaces, edges reference curves, vertices reference points.

**Background: pcurves.** A surface is a map `S(u,v) → ℝ³`. A face is a
region of that surface's parameter plane, and each boundary edge is
therefore also a curve `P(t) → (u,v)` in that plane — the *pcurve*. An
edge shared by two faces classically carries three representations: a
3-D curve plus one pcurve per adjacent face, with the consistency
requirement `Sᵢ(Pᵢ(t)) ≈ C(t)`. Pcurves are not optional — trimming,
tessellation and intersection marching happen in (u,v) — and the
redundancy among peer representations is a classic bug farm.

**Our rule:** an edge's geometry is stored as an *intensional
description* of what the locus **is**; every concrete representation
(the 3-D carrier, both pcurves) is a derived cache carrying a certified
residual bound against the described locus ([tolerance.residuals]). The type is
`EdgeDescription<T>` (`crates/geom-brep/src/description.rs`):

```text
EdgeDescription =
  | Intersection        { pair, selector }   -- a curve of S₁∩S₂, `pair` the set
                                             -- {S₁, S₂}, unordered; the selector
                                             -- picks which curve: a witness
                                             -- (the branch through a point, also
                                             -- the marching seed) or a structural
                                             -- selector (a construction's
                                             -- pushforward); whether the surfaces
                                             -- cross (read `Intersection`) or touch
                                             -- to order k (read `TangentIntersection`)
                                             -- is what the certificate reads, never
                                             -- a second stored record
  | Chart(ChartCurve)                        -- a curve ONE surface's chart defines:
                                             -- (surface, Pcurve) with a `wrap` flag
                                             -- (iso-lines, seams, same-surface splits)
  | Scaffold(MappedCurve)                    -- construction-time pushforward of a
                                             -- lower-dim entity; never at rest
```

A two-surface edge is always a curve of the two surfaces' intersection,
and its selector says which: no two-surface edge is conventional. The
selector follows one question, asked of the geometry and never of where
the body came from: **is the curve known in advance?** Where it is —
the surfaces' contact or crossing has a closed form (a ruling where a
plane meets a cylinder or a swept wall, a circle where coaxial surfaces
of revolution meet, a row or column of a swept or skinned wall), or the
curve was chosen to lie in a region where the two surfaces overlap in
2-D — the selector is **structural**: that closed form, picking the
curve exactly whatever the contact order, certified by its own meter and
incidence on each surface, each read through that surface's own
chart-image certificate (the one meter every pcurve carries). A
construction that knows the curve (a profile vertex swept, a section
placed, a section vertex skinned through a loft's sections) states that
closed form directly, and an import recovers the same closed form from
the same geometry, so a native body and its imported copy carry the same
selector: the closed form is a property of the surfaces, not of the
history. Where the curve must be derived numerically from the geometry
— a crossing or tangency with no closed form — the selector is a
**witness**, the branch through a point, admitted only where its margin
certifies: transversality for a crossing, the order-k jet system's
margin for a tangency of order k, each taken across the locus, over the
faces, never along the edge. A construction therefore never classifies
its own joins by measurement, and the sketch's joint class is the one
reading of their kind. A curve no lane can yet select refuses typed,
naming the lane, native or imported alike; a coincidence of two surfaces
in band is refused at the structural act that made them adjacent. A locus one chart defines
(parameterization seams — infinite-order contact, the seam's position
pure convention; iso-lines; splits whose two faces share one surface;
two records that read as one carrier) is `Chart`: one (surface,
`Pcurve`) form covers all of them (PCURVE-UNIFY-DESIGN); the exact
classes stay as certification lanes, not variants. A `MappedCurve`
survives as the `Scaffold` payload, as a structural selector and as the
`EdgeAuthority` record — one authoritative source, never two peer
representations needing cross-reconciliation.

**Deliberately omitted: an `Explicit` (extensional) variant.** Taken as
an unconditional challenge: every edge must have an intensional
description — there is no escape hatch, so it cannot be reached for
when not absolutely necessary. This holds for imported geometry ([import]):
the intrinsic variants are checkable properties of the geometry as it
now stands, and a closed-form selector is a property of the surfaces,
so extensional input is *adopted* by recovering the selector a native
body would carry, or refuses typed where no lane recovers one. What import pressure-tests is the completeness of the
taxonomy (imported fillets force `TangentIntersection`), not the need
for a fallback.

**Validity predicates with margins.** `Intersection` requires
*transversality*: normals of S₁, S₂ linearly independent along the
locus, so the implicit function theorem makes S₁∩S₂ locally a
1-manifold; the margin (angle between normals) governs the conditioning
of every derived cache. A branch may end at a *node*, a point where
it crosses another branch of the same pair and the two surfaces are
tangent (two equal cylinders with intersecting axes: their section is
two ellipses crossing at p ± r·unit(â₁×â₂)); there the section is one
connected set, so the witness selects the branch, and transversality
holds on the open edge, which is where certification samples it. A tangency's validity sits one
differential order up: surfaces coincident within ε and normal-parallel
within the derived angular threshold ε·κ_rel ([tolerance.epsilon]: lever arm
r = 1/κ_rel) *along* the locus, separating quadratically *transverse*
to it (relative normal curvature bounded away from zero — otherwise the
surfaces osculate over a patch and the "locus" is not a curve).
Reconstructing a tangency locus is well-conditioned *despite* the
tangency because its defining system includes the first-order
normal-alignment equations: the normal angle grows linearly with
transverse distance, and the second-order margin is the
implicit-function-theorem denominator for that jet system. Order-k
contact generalizes: defining equations from the k-jet, margin at order
k+1, so a curvature-continuous join (k = 3) is read as a tangency where
its third-order separation certifies, not left under-determined. In the intrinsic variants "the locus lies on both surfaces" holds
*by definition*; only the caches need certification. Vertices
generalize the same way (three surfaces / endpoint of a locus, with a
witness). A locus in the ambiguous band — a dihedral within a few
derived angular thresholds of tangent, certifiable as neither — fails
loudly at construction exactly as at import; a conventional description
is not an escape hatch from ill-conditioned geometry. An edge whose
stations read definitely transverse at some and definitely smooth at
others is such a locus: the dihedral is continuous along it, so the two
readings bracket an in-band point the schedule did not sample, and it
refuses typed, naming the two stations.

**Prefer-intrinsic rule, tier-3-enforced.** Every two-surface edge is
intrinsic, and its selector is decided by the geometry alone: a closed
form where one exists, a witness where the curve must be derived. A
fillet we build carries its rolling ball's contact rail; a sweep carries
the ruling its profile vertex sweeps out, whatever the joint's contact
order; an imported copy of either recovers the same closed form. A
closed form that selects the curve is description, not history; [identity]
provenance stays the record of who built the edge, and native and
imported bodies carry identical descriptions. At rest, a witness
selector must certify its reading:
a definitely-transverse locus as `Intersection` (`TransverseNotIntrinsic`
otherwise), a jet-determinate tangency as `TangentIntersection`
(`TangentNotIntrinsic`, where `geom_brep::tangent_certificate_lane`
admits the class). The rule needs two surfaces and a definite reading, so
a wrap edge (one surface) and an edge whose dihedral is not read
definitely (in band, or beside a spline face) fall outside it.
An unenforced preference drifts silently — exactly the failure shape this
document exists to kill.

**Witness contract.** "Selected by the witness point" is verifiable only
if the witness is *pinned*: the stored witness IS the carrier's
mid-parameter point, enforced by certification (`WitnessMidpoint`).
Pinning removes the aliasing freedom that let any point on the
component certify, including points encoding a wrong winding; residual
freedom is documented where it is geometrically invisible (circles:
whole-period translation). Every op that mints an `Intersection`
computes the witness as carrier(mid) in the certification schedule's
own association order.

This makes [identity]'s provenance load-bearing rather than bookkeeping: the
intensional description largely *is* the provenance.

- **SSI completeness is an interval obligation, not a marching
  property.** Residual certification audits only *found* branches; the
  missed small loop is the classic silent disaster. **Marching finds,
  subdivision certifies exhaustiveness**; the outcome is "every branch
  found" or a typed failure, never silence. Certification is an
  at-rest/tier obligation; preview may march uncertified.

### Surfaces `[surfaces]`

*From:* [pure-model], [unrepresentable].

Plane / cylinder / cone / sphere / torus are first-class variants
alongside NURBS (as in Parasolid), not converted to NURBS. Most
mechanical geometry is analytic; analytic×analytic intersections have
closed forms, while NURBS×NURBS is a numerical marching problem. A
seventh variant, `Approx`, is the certified approximating class
(OFFSET-DESIGN O2): a fitted NURBS carrying the intensional description
of what it approximates plus a certificate bounding the distance,
re-derived per face and never trusted from storage.

**Extensibility:** surface kinds form a *closed enum*, not open trait
objects. Intersection requires pairwise dispatch, and a closed enum
gives compile-time exhaustiveness, so adding an analytic kind means
adding a variant and letting the compiler enumerate every dispatch
site. The `Nurbs` variant is the universal fallback: any exotic surface
is at minimum representable. Same design for curves (line / circle /
ellipse / spiric / NURBS — the spiric is the axis-parallel plane×torus
section, one oval, in the torus's own minor angle).

### Import `[import]`

*From:* [intent.stored], [loud-failure], [one-way].

Imported geometry is not second-class, and it is not distinguishable
from native geometry either: no state is reachable only through import
([tolerance.boundary]). Rather than an extensional escape
hatch, import **reconstructs** the intensional description the
extensional data satisfies — possible because the intrinsic variants
are properties of the current geometry: an imported edge recovers its
selector (a boundary iso, the analytic contact catalogue, a swept net's
structure) or refuses typed, naming the lane that would recover it. Pipeline:

1. **Surface recognition**: an imported NURBS within ε of an analytic
   surface is promoted to it, restoring [surfaces]' exactness to imported
   bodies.
2. **Edge adoption**: verify the imported curve lies within ε of the
   intersection of its adjacent surfaces with adequate transversality
   margin, then rebuild it as `Intersection { pair, witness }` — the
   imported curve demoted to witness + initial cache. Seams and
   tangency loci are recognized likewise.
3. **Healing**: where no description is satisfied within ε, repair
   (refit/nudge) or fail loudly naming the unhealable entities.
4. **The shared at-rest gate**: steps 1–3 certify each *entity*; the
   *body* is then handed to the kernel's own at-rest validator — tier
   3, or tier 3′ where solids touch — and only a body it
   passes ships from import. Same function, same tiers as a native
   caller runs; import holds no idea of validity of its own. *Per
   solid, not merely per file*: whole-body sums (the +V flux) would let
   an inside-out solid cancel against a right-side-out neighbour, so
   each solid is gated on its own body before aggregation, and the
   aggregate pass remains for the cross-solid structure. An imported
   assembly whose parts touch certifies, and its contacts — decided
   from values, since no imported cell is a construction of the
   document's variables — are `unproven-coincidence` findings ([coincidence]),
   quieted by an assertion that the gap is zero at the site: the equivalence with a natively built twin whose parts
   touch by value.

**Adoption tolerance ≠ kernel tolerance.** Adoption takes a per-import
*input tolerance* ε_in — defaulted from the STEP file's declared
`uncertainty_measure_with_unit`, overridable per call. **ε_in governs
interpretation** (recognition and classification — what the data is
evidence of); **ε governs what gets built** — an adopted entity's caches
are recomputed from its description by our own algorithms and certified
at ε like native geometry. Healing may move geometry by up to O(ε_in)
to make the chosen interpretation true — a reported model change, never
a loosened certification. Data ambiguous at ε_in scale fails with a
typed ambiguity error rather than a guess.

**Non-goal: feature recognition.** Adoption recovers *what each locus
is*, not *how the body was modeled*; imported bodies carry no
parameters, so error propagation has nothing to vary over them.
Adoption is strictly stronger than industry "shape healing" (which only
patches data into self-consistency): it must *explain* the data.

## Documents

### The recipe `[recipe]`

*From:* [pure-model], [intent.stored].

A model document is a graph of operations over typed variables ([variables])
plus a small expression sublanguage for derived quantities. The kernel interprets the recipe at any scalar `T`;
user-facing Rust is a *generator* of recipes. Consequences: the recipe
is the save format; recipe node IDs are the substrate for [identity] naming;
every value-dependent branch stays inside kernel code where predicates
are reified ([verdicts]) — user models as generic Rust functions were rejected
because `if width > 10.0` in user code would silently break interval
replay; and structural parameters (hole *count*) are explicitly
distinct from continuous ones (hole *diameter*), so parameter-driven
topology change is stated, not emergent.

**The recipe keeps the form that was written.** `[recipe.forms]`
Where people state the
same thing in more than one natural way, the recipe may hold a family
of forms for it: one enum whose variants each keep exactly the numbers
the person wrote, all lowering through one reading that every consumer
goes through. A sketch arc is the pattern: `arc_to` takes a `Bulge`,
`Via` (through a point), `Center`, `Sweep` or `ArcLen` spec, the
program stores the spec as written, and each lowers to the one
`Segment::Arc` (carrier and sweep) that every reader of a profile reads
(PATHS-DESIGN). That is how an exact authored value and a single
interface coexist. A family is not a second spelling, which is one fact
held in two places that can disagree, or a derived number stored in
place of the one written. A form joins a family only because it is how
someone authors the value, never for the kernel's convenience. Where a
form's numbers name no value it refuses typed, as `Via` does for three
collinear points (`ArcViaCollinear`).

- **The expression sublanguage is total and finite by charter.** No
  recursion, no unbounded iteration, no user-defined functions —
  anything Turing-ish lives in the host-language generator layer ([recipe]).
  Keeps interval/dual replay trivial; nearly impossible to claw back
  once one persisted model uses a loop.
- **Persisted floats round-trip bit-exactly.** Shortest-round-trip
  formatting (serde_json with `float_roundtrip`) for finite values;
  NaN/inf refuse typed (`PersistError::NonFinite`); lossy formatters
  banned; enforced by a save/load/replay-identity test. Replay never
  solves: no edit records a frame (a placement is its mates, [placement]), so load re-applies the
  edits alone, and a saved document reproduces its placements bit for
  bit with no part store in hand.

### Variables `[variables]`

*From:* [intent.stored], [intent.statements], [one-way].

Every slot that admits more than one value holds a variable whose type
suits the slot.

**Kinds.** `[variables.kinds]`

- *Scalars:* `Length`, `Angle`, `Scalar`, `Count`.
- *Discrete kinds:* a side, a half, a sense.
- *Poses:* `Point`, `Direction`, `Axis`, `Plane`, `Frame`
  ([variables.poses]).
- *Shapes:* `Body`; `Bodies`, an ordered list of bodies whose length
  is a `Count`; and `Profile`. Only an operation defines a shape. A
  `Profile` is 2-D shape whose numbers are read only against each
  other; a sweep reads a profile and, at most, a heading and a tilt in
  the profile's own axes (an extrude's slant, held to one side of the
  profile's plane) or a point and a heading (a revolve's axis line),
  never a frame.
- *Selections* of a shape: one cell of a body (`Face`, `Edge`,
  `Vertex`), or the set of them a reader takes (`Faces`, `Edges`; a
  set kind exists only for a slot that reads one).

**Free and defined.** A variable is **free** — a value, its written
unit ([units]) and optionally a distribution — or **defined**: by an
`Expr` over other variables, by a selection of a `Body` variable, or
as an output of an operation. Poses are only defined.

**No dimensioned literal.** `[variables.literals]` A dimensioned
literal stands nowhere, neither in a slot nor inside a formula. The
only constants are dimensionless rationals, rational fractions of a
turn, and a lone `0`, which carries no unit and so stands at any
dimension as the omission it marks (`0 mm` is a dimensioned literal,
so writing it mints a variable, VR6). Each is the shape of a formula
rather than a dimension. A lone `0` is the one written value that is a
constant rather than a variable.

**Names.** A variable without a name is read by exactly one slot or
definition, which is how it is spoken; a variable two readers share
has a name the person gave it; an output is spoken by its operation.

**Poses.** `[variables.poses]` A pose is only ever defined: read off a
body's geometry (a face's plane, a carrier's axis or centre, a
vertex's point); moved from another pose by a slide along, or a spin
about, a direction or axis the definition reads; by a construction
over other poses; as a pose of a copy; or as an output of an
operation. So no pose is free, none is defined from nothing, and no
construction reads one.

A pose is a frame known up to its kind's symmetry, a subgroup of the
rigid motions and the same `Subgroup` the mates fold (A11 (1)): a
plane forgets in-plane motion, an axis slide and spin along itself.
The kinds are ordered by which determines which. A slot holds its own
kind; a finer value is read through its projection, and the role it is
read through (a frame's x, y or z axis, origin or coordinate plane; a
plane's normal; an axis's line or direction) is part of the read. An
incidence between poses (an axis in a plane, a point on an axis) is a
construction over one variable, never a check between two.

Every number a pose, a construction's axis or a placement holds is a
`Length` along one freedom or an `Angle` about one: no slot holds a
component of a direction or of a frame's axis, and a 2-D direction is
an angle. A face reads as a plane and a vertex as a point; no reader
takes a carrier's reference direction. A 2-D value (a profile's step,
a revolve's axis line) lives in the node that holds its frame, as
length and angle slots, and is never a variable of a 2-D kind; its
lift to a 3-D pose is how it leaves.

### Operations `[operations]`

*From:* [intent.statements], [pure-model], [one-way].

A node is an operation: it reads variables and defines the variables
its signature states, a fixed list of named, typed ports set by its
variant, possibly none. A split defines two bodies; a revolve its body
and its axis; an instance of a part is a placement, defining one
output per world placement of the part, named by that placement (and
a family where that placement is one), all of its targets' space. An
assertion defines none. A mate or a value is a constraint of a
placement, not an operation.

**Reading is the only dependency.** Nothing consumes anything, so an
operand stays a first-class value after a boolean reads it.

**Selections.** A `Face`, `Edge` or `Vertex` variable, or a set of
them, is a selection of a `Body` variable by `StableName`: a
definition, not a node, stating its body once, and the N5 resolution
ladder lives there. A selection authored at two sites is two variables
(the GUI offers the existing one), and a repair is addressed by body
and name. Deleting a variable leaves its readers unresolved, typed,
never silently re-pointed.

**Observed values.** A `Measure` defines an *observed* variable, a
function of the built geometry rather than of what was written. An
observed variable, and any definition reading one, is read only by an
assertion: a construction reads what was written.

**The product.** The product is every copy whose space reaches the
world, in placement order ([placement]). Building, combining or
placing shapes adds nothing else to it: a body appears in the product
only as a copy whose space reaches the world, and a document with
nothing related to its world has an empty product.

### Repetition `[repetition]`

*From:* [intent.statements], [one-way].

**Repetition.** A document repeats by an **index**: `k = index(N)`
defines a `Count` variable over `0..N`, `N` any `Count` expression. A
definition or operation whose reads reach `k` is evaluated once per
value of it, and each of its outputs is a **family** of that kind (a
`Bodies` is the family of `Body`), one member per value, keyed by the
tuple of the indices it reaches, outer first. Membership is derived
from reads, as dependency is: no node holds a template, and there is
no pattern operation. A variable defined by reading `k` is, to a
reader that reaches the same `k`, its member at the same value, and to
every other reader the whole family: `xs[i, j]` reads one member, and
`union` and `intersect` take a `Bodies`: a family, or its members spelled
as reads; a subtract's tool is one `Body`, so cutting by a family is
`subtract(a, union(holes))`.
Lockstep is one index read twice. An index enters a placement as a value, a slide or a
spin that is a `Length` or `Angle` expression over it, or through a
mate whose target is a member of another family
(`bolt.axis ≡ holes[k].axis`); no pose is constructed from an index, so
an `Axis` mate with `spin = scalar(k)·turn/N` is a ring and a slide of
`scalar(k)·pitch` a row. Two indices meet in one reader only when one is
declared `within` the other, which is implied when its count reads the
other; a reader of two unrelated indices refuses. `Count` arithmetic is
exact and includes `mod`, so a relation between a ring's neighbours is
one statement (`assert(gap(c[k].r, c[(k + 1) mod N].l) = −b)`), while a
ring stepped by `turn/N` closes by construction and needs none. There
is no list literal: an irregular family is its members written out,
each saying where it is. A definer that refuses at one index refuses
the whole family, naming the index; a read whose index leaves the
range is unresolved and typed, never re-pointed. A member's names are
`Member { (i, j), of }`, keyed by the index variables' ids and the
integers. Copies of one body are built once and mapped; a construction
whose scalar inputs read an index is built per member, and no
construction reads a frame. A mirror image is not a copy, because a
reflection is not a pose: `Mirror { body, plane }`
is a construction defining a new `Body` in its source's root, the
plane a `Plane` pose whose reads reach that root alone (a face read as a
plane, or a plane constructed from the body's geometry), and every pose
is proper. A symmetric part is `union(body, Mirror { body, plane })`,
one construction whose seam is the plane's face read twice; a chiral
twin elsewhere is a placement of the image, and there the plane carries
no shape.
Linear, circular, grid and bolt-circle patterns and mirroring are the
façade's functions, which write this program; the document stores no
preset, and the GUI's reading of one ("circular, 12 about A") is
recognised from the program for display only.

### Spaces and placement `[placement]`

*From:* [intent.statements], [intrinsic], [one-way].

**A part has no location.** No body has coordinates anyone reads.
Every construction is built in coordinates of its own, and frames
enter only at placement: a feature on a face is a construction placed
against the face by a bundle of mates, then combined, and which side
of the face its material lies on is the mates' to say. No construction
reads a frame (FORK-S3P), so a feature at several positions is one
body placed several times, each placement a copy.

**Spaces.** `[placement.spaces]` A **root** is a construction that
reads no body, the world, or a copy whose bundle pins less than its
body needs. **Spaces are kinds**, decided from the recipe: a body's
kind is its root, a placement's copy takes its targets' root once its
bundle pins it, and a copy pinned, transitively, against the world is
of the product's kind. A loose copy is of its own kind with everything
pinned to it, solved among itself, read at a pose by nothing outside
it, and drawn from display state no logic reads; an instance of a part
enters a document only as a placement. Only a mate reads across kinds,
so relating two raw coordinate systems cannot be written: a read
across two spaces is a kind mismatch at the door, and a kernel bug
anywhere else.

**A placement** `[placement.placement]` is an operation reading a list
of shapes of one space, as a union reads its operands (a list of
reads, not a list literal), and a bundle of constraints — mates and
values — and defining a copy of each shape under the one rigid motion
its constraints pin. Two placements of a part are two copies. Nothing
moves a body: a copy is defined by its one placement and never moved
after, and an operation that would move one (today's `Transform`) is a
placement. A pattern is a placement whose reads reach an index
(FORK-PAT, [repetition]): one copy per value, the index entering as a
value or through a mate's target. A copy is pinned when its mates and
values together leave
nothing free, and a body's symmetry pins nothing (a written value it
makes unobservable, a placement's or a pose move's, is reported, never
refused). A placed copy's frame is not a variable: it is the
construction its bundle states, which the coincidence door replays
like any other.

**Constraints are on equal footing.** A placement's mates and values
are on equal footing, and a constraint any of whose equations the
others already fix refuses as an overconstraint, pinned or not,
decided by subgroup algebra (A11 (1)) without measuring. A mate that
would take a freedom a value sets refuses as well. Where two copies
meet beyond what their mates fix is a contact like any other, recorded
and linted, and its recourse is an assertion ([assertions]).

**Mates.** `[placement.mates]` Each mate equates two poses of one
kind, modulo that kind's symmetry: a pose read off the copied shapes'
geometry with a pose read off geometry of the space the copy joins,
never a frame standing for a part's coordinates. It holds no number of
its own, and its sense is a construction (`Flip`) on one side; a
number a kind's own equation fixes (a standoff along a plane's normal)
is a construction on the target, never a value. Which copy is defined
is which placement reads it, and a pose of another copy is read as
that copy carries it. A mate places and never checks.

**Values.** `[placement.values]` A **value** sets one freedom the
bundle's mates leave, a slide or a spin, to a `Length` or `Angle`
variable, charted on the two bodies' own coordinates as the placement
carries them: a slide is the copy's origin measured from the target's
along the freedom, a spin the angle between their reference
directions about it, each a function of the relative pose alone, so no
order of the values is chosen. Zero is always a valid value and says
the two bodies' own coordinates agree as far as the mates allow; the
façade writes the constant `0` for each freedom a gesture leaves
unnamed. A rotation left by a lone point mate has no such chart and is
lowered by a direction mate first. A placement is the only reader of a
body's own coordinates, and reads them only through the freedoms its
mates leave.

**The world** `[placement.world]` is one undeletable node that copies
may be related to like a part. It is one frame among many; it defines
no pose variable, so only a placement's mates and export read it, and
construction never does. A body enters the product by a placement of a
copy against the world, or against what is related to it.

**The computing frame.** `[placement.frame]` An operation computes in
a frame that is a function of what it reads and of nothing else,
chosen so its arithmetic is well conditioned near the geometry it
builds, and never in the world's; the frame is keyed with its inputs,
so an edit that leaves an operation's reads alone moves none of its
bits ([determinism]). The frame is no part of the operation's
meaning: the body up to that rigid map, its names and every verdict
outside the sliver band are the same in any frame, and a minted
reference direction is a function of the inputs, not of the axes.
Where conditioning does not decide, a construction computes in its own
coordinates and an operation over copies in its first operand's as the
author lists it. A check over a space, the at-rest census, is defined
order-free: each pair's verdict is the same in either member's frame,
or the sliver band refuses.

### Coincidence `[coincidence]`

*From:* [intent.geometry], [intent.checked], [intent.structural],
[intrinsic], [one-way].

**A coincidence is a margined verdict.** Whether two cells coincide is
decided like any other question ([verdicts]): a margin decided Zero
glues them, a definite one keeps them apart, and one in the sliver
band between refuses. Structure decides something else: whether the
coincidence holds across the family of the variables' values.

**Structural.** `[coincidence.structural]` Two cells are structurally
one when the document establishes them as one, decided by two rungs at
one door. The first is the same construction read twice, directly or
as projections of one construction. The second reads each output
carrier as the function of its construction's inputs that the
construction's own code computes, by evaluating that code with every
variable a symbol: no operation describes its outputs a second time
(an extrude's end cap comes out as `plane(frame, direction, depth)`
with no profile variable in it, and a side wall with no depth). A
coincidence is structural there when the kernel's own carrier-pair
verdict, run on the two cells' symbolic carriers, or the margin that
decided it, is zero as a polynomial in those symbols with exact
rational coefficients: a theorem for every value at which the
construction takes the decisions it took here (E12). The verdicts read
each kind modulo its own symmetry (a plane forgets in-plane motion and
folds a shift along its normal into its offset; an axis forgets slide
and spin along itself), so that reduction is written once, and a
projection of a construction reduces to what it was built from.
Polynomial identity is an equivalence relation, so a chain of blocks
each built on its neighbour's floor closes into a loop, and a brick
laid across two of them sits on both, with nothing more said. A
computed value never re-enters the evaluation as a constant; a
numerical routine's output enters as one opaque symbol per call. The
symbolic tier decides only whether a Zero is structural; a margin in
the sliver band is not decided by it, since the band is indeterminate
under exact arithmetic too ([verdicts]). "Structurally" is decided at
that door and nowhere else, and a rung added there may only turn a
finding quiet ([intent.structural]).

**Constructing a coincidence.** Coaxiality is one `Axis` read twice,
directly or as projections of one construction; tangency is
constructed (a sketch may read another surface's trace in its plane
and continue tangent to it).

**The door and the lint.** `[coincidence.door]` Every coincidence the
kernel infers from values — a boolean's glue, a split's ON verdict
that makes pieces of one result touch, a contact the at-rest census
finds — is recorded at the one door where structure is decided, and
the **`unproven-coincidence` lint** checks whether each one holds
structurally. One that does not holds only at the current values of
the variables: the f64 build glues it all the same, so no setting
changes a built body, and the lint reports it with the edit that would
make it one construction. The analysis lanes, which run over a
parameter box, see such a coincidence as the point it is and escalate
there. A recorded coincidence names its cells as the reads they
entered the deciding operation through and their names there, never
by a stamp the kernel carries; a contact record a result carries cites
the coincidence that backs it. A per-construction table of output
forms is only ever a cache generated from the symbolic evaluation and
checked against it, never written by hand. A reading that only chooses
a cell's form (a lane choice, [verdicts.consumers]) infers no
coincidence and records nothing; a door that makes cells one on a
class it read (the boolean's edge join) records it.

**Booleans.** `[coincidence.booleans]` A boolean's operands must already be in one space;
otherwise it refuses. It glues what its verdicts decide Zero where an
arm exists for the carrier pair ([topology]'s frontier), keeping one fixed
operand's description for a merged face, and refuses what falls in the
sliver band. Its result holds no in-band pair, a pair of result cells
whose one census number lies in band. A solid's only shell is its outer
shell and poses no role question; a shell's role among several is read
off a ray aimed from outside the shell at one of its faces, and off the
certified sign of its volume only where none of its faces offers an
aim. A ray's reading lies in band only where a pair does, so how thin a
result is gets judged by its pairs alone. The door's
finished-body gate decides this on the result alone and refuses it as
the operands' ill-conditioning (`Escalated`, recourse: tighten ε or move
the geometry), never silently; a definite finding there is a kernel
defect (`ResultInvalid`).

### Assertions `[assertions]`

*From:* [intent.statements], [intent.checked].

**An assertion checks and never places.** `Assert { measure,
relation, bound }` (`≤`, `≥`, `=`, the bound a variable) states a
relation the author claims and the kernel checks.

**The at-rest census.** At rest the census examines the copies of each
space pairwise, and nothing it finds refuses. Contact between copies
is an `unproven-coincidence` finding unless it is structural (a
mate-placed face is); an overlap of their material is an interference
finding; and a pair the census has no lane for is a finding too,
saying it could not look, so no outcome of the census is silence. A
contact or interference finding observes the sign of the gap
(CONTACT-DESIGN C5) between two copies: a contact is `g = 0` at two
cells, an interference `g < 0` over one connected overlap of their
material.

**What quiets a finding.** `[assertions.quiet]` Holding assertions
quiet a finding when they say the same: each reads a `Gap`'s output
directly, over an opposed pair of faces of the two copies, and admits
only values of the finding's sign (`= 0` for a contact; `≤ b` or
`= b` with `b` negative for an interference). A contact is quiet when
such an assertion's two faces are the two cells the census found
coincident; an interference, when the assertion's two faces bound the
overlap and every face bounding it lies between the carriers of an
asserted pair. An assertion speaks for nothing else, so a new contact
or overlap anywhere else is loud until something says otherwise, and
an overlap the kernel cannot bound is loud and nothing quiets it.

**Families.** The members of a family are examined like any other
copies, and an assertion about each of them is written once, as an
assertion whose reads reach the family's index ([repetition]): one
assertion per member, each reading the one bound variable, as many as
the index's `Count`. That quiets each member against another copy. An
overlap between two members is quieted by an assertion reading two of
them: `c[k]` and `c[(k + 1) mod N]` for neighbours in a ring, or two
indices, one `within` the other, for all pairs.

### Identity and naming `[identity]`

*From:* [pure-model], [intrinsic].

Every topological entity carries a provenance record from the moment it
is created: a typed per-operator **birth record** — the operator plus
its argument keys — on every entity of all seven arenas. Kills remove
the record with the entity; survivors keep theirs; reparenting or
demotion (`ring_move`, `kfmrh`'s loop demotion) is not a re-birth. The
validator enforces the record bidirectionally. This does not solve the
topological naming problem — the parametric layer builds its stable
references on top of it (NAMING-DESIGN) — but recording identity at
birth is cheap and retrofitting it is nearly impossible.

- **Naming is localized to reified predicate flips.** Topology is a
  function of the recipe and can change only where a structural
  parameter ([recipe]) changed or a trilean predicate ([verdicts]) flipped. Within a
  flip-free parameter region, replay is history-identical and
  lineage-scoped key identity makes name resolution provably trivial;
  at a flip, the flipping predicate itself names what changed and why.
  Resolution is trivial where provable, a loud typed failure carrying
  the flip's diagnosis where not; re-binding cleverness only as
  ratified opt-in policies.

### Assemblies and document identity `[assemblies]`

*From:* [one-way], [intrinsic.meaning], [pure-model].

The rules here bind every component; how editor-core keeps them, door
by door, is `crates/editor-core/ASSEMBLY.md` and
`crates/editor-core/IDENTITY.md`.

- **An assembly is a document.** `[assemblies.document]` An assembly is
  an ordinary document whose leaves instantiate other documents. It
  uses the same node, edit, naming, undo and memo machinery as a part,
  and its evaluation is one kernel `Body`, generally multi-solid and
  not connected: the kernel has no assembly type. Which solids are one
  part, and which mates hold, is recipe structure, never body state;
  fusing copies is an explicit boolean, never implied.
- **A reference pins content.** `[assemblies.pins]` A cross-document
  reference is a `DocRef`: a `DocumentId`, which says which document
  and survives every edit, and a `ContentPin`, the SHA-256 of the
  document's canonical semantic bytes, which says which version. An
  edit to the referenced document never retargets a reference: a
  document is served only when its bytes hash to the pin, and moving a
  pin is a recorded edit. Since the pin is the content, the memo is a
  pure function of the document, and whether a store still holds the
  pinned bytes is the mounting session's question, never the memo's.
- **A held value names the document it came from.**
  `[assemblies.pairing]` An evaluation, a solve or a pick carries the
  identity of the document it is of. A door that takes a document
  together with a value of it refuses a value of another document,
  typed, before reading anything of it. Node ids cannot decide this:
  they are minted from the edits that inserted them, so two documents
  built from one recipe carry the same ids. A node id names one
  authored node wherever it is minted, and whoever holds one keeps the
  bare id and asks the document in hand whether it is foreign, deleted
  or live.
- **Saving keeps identity; forking is its own act.**
  `[assemblies.saving]` Identity is the document's, not the file's.
  Saving at a path keeps the id, and refuses where that directory
  already holds the id under another filename; saving as a new document
  mints a fresh id, and every inbound reference stays on the original.
- **Split and inline are pure.** `[assemblies.split]` Moving content
  into a document of its own, and back, is a pair of pure refactors
  that compute no frame and return values plus edit lists.
  Split-then-evaluate equals the unsplit evaluation at structural and
  name-resolution identity, and inline-of-split returns the document
  split was given, up to minted ids and order.

### Units `[units]`

*From:* [intent.stored], [unrepresentable].

Kernel-internal code is raw `T` in meters/radians by convention — no
dimensional types inside. The public API uses hand-rolled newtypes
(`Length`, `Angle`, …, the `quantity` crate) that convert on entry.
Hand-rolled rather than `uom`: uom's dimensional generics fight the
scalar-type parameter and we need ~five quantities, not the SI lattice.

**A stored value always names its notation.** Units erase at the
accessor doors because the kernel wants them gone. One consumer wants
them kept: a document records what a person *wrote*, so it can be read
back that way. Every free continuous variable's value ([variables]) therefore
carries a display unit — a row of `quantity::UNITS`, presentation metadata excluded from
expression identity, keys and evaluation — and that unit is **not
optional**: the table carries a dimensionless row (`ONE`, empty symbol,
factor 1.0) so a `Scalar` value names its notation rather than
declining to. `Count` needs no row: a count is an integer, not a
quantity. A value crossing into a document carries the unit it was
written in, never a bare number — which is why a GUI creation op mints
a free variable carrying the written unit ([variables]), and what `quantity::WrittenLength`/`WrittenAngle` are the
library spelling of.

## Failure, determinism and engineering

### Failure `[failure]`

*From:* [loud-failure], [unrepresentable].

- **The kernel never panics on any INPUT** — every failure an input can
  reach is a typed error. **A panic is therefore never a refusal: it
  reports that a bug has already happened.** Read it as *a firing panic
  is evidence of a bug*, never as *a panic in the source is a defect to
  remove*. **The converse is a positive obligation: a state that can
  only be a kernel bug MUST panic**, as loudly and as early as it is
  detectable (`unreachable!` / `debug_assert`, the taxonomy below);
  downgrading one to silence, or to a typed error, launders a bug into
  a supported outcome. The two halves are separate rules over disjoint
  state classes. Every traversal is bounded: never a hang.
  The closure property behind the first half: every public mutation
  path preserves tier 1, **checked at every observable boundary** —
  the Euler operators by the soundness theorem, the non-operator
  structural mutators by declaring the same debug postcondition, or by
  opening a surgery scope and closing it with that check, or by being
  composed of doors that do; the attach/metadata setters by
  re-certifying under the same assertion. Where the check runs is the
  door rather than the operator ([topology], and
  `work/perf/d1-per-op-tier1-sweep-price`, Ev, PR 2305); what is
  checked is unchanged. The claim is that property, not a count of
  doors;
  `topo`'s `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
  checks it against the real surface, both spellings. **What holds a
  scope closed is the borrow, not that walk**: a scope opened through
  the RAII guard is released only by a close or a drop, and both
  decrement. The walk adds a lexical read on top — a `pub fn … &mut
  self` door of `topo/src` that opens a scope and closes nothing reds
  by name — and the two sites where a borrow forbids the guard are
  held instead by a runtime depth read at the next phase boundary. The
  reach of each, and what none of them sees, is
  `work/perf/door-scopes-outside-topo-are-unguarded`. **`instance`'s graft is held to the
  property by staging** (S14, Ev, PR 4006): it transplants into a fresh
  body and commits only on success, the shape `merge_coplanar_faces`
  already uses, so a refused graft leaves the caller's destination as
  it was and no public door leaves a body tier-1-invalid.
- Essentially no unsafe Rust outside vetted dependencies.

**The bug-vs-invalid-state taxonomy.** `[failure.taxonomy]`
Silent discard
is never an answer: a state that cannot occur is announced, not
swallowed.

| # | State class | Mechanism |
|---|---|---|
| **0** | **Can this state be made unrepresentable?** — asked of every state, before the rows below | **change the type.** Preferred over every row below whenever available |
| 1 | Reachable by input, **invalid** | typed error |
| 2 | Reachable by input, **valid but unbuilt** | typed `Unsupported*` error |
| 3 | **Value-domain degeneracy** | poison — NaN / empty |
| 4 | **Kernel bug**, observable in a branch | `unreachable!` |
| 5 | **Kernel bug**, detectable only by re-derivation | `debug_assert` |

- *Row 0 is a question, not a class.* A lane that files a state under
  any row owes the reason row 0 did not apply. It is answered against
  the cost of the type change: yes when the change is local to the type
  and its constructors (a private field and a constructor signature,
  no public API change); no when it propagates into signatures that do
  not otherwise care (a brand lifetime on `Body` would infect every
  signature naming a body — that no is the precedent for where the
  line falls). A "no" is a complete answer, recorded as the reason a
  row below applies.
- *Row 1 absorbs the terminal indeterminates*, but the axis is
  curable-vs-terminal, not bug-vs-invalid: an `Indeterminate` whose
  `MarginDiag` is of kind `Value`, or an `Enclosure` wholly inside a
  sliver band, is a statement about the input and reaches the user
  through its decision's recourse ([tolerance.recourse]); a straddling `Enclosure` is generally
  curable by subdivision, and an `Invalid` margin from a domain clamp
  may cure as the violating sub-box shrinks (a NaI never does). The
  subdivision driver exists (`editor_core::drive`, ERROR-DESIGN E6): a
  curable indeterminate unwinds to it and is not reported as invalid
  input.
- *Row 2 is a naming rule.* `Unsupported*` means "valid input, the
  kernel has not built this yet" and nothing else, which makes the
  frontier inventory grep-able. A panicking `not_implemented!` macro is
  rejected: these refusals are reachable by valid input and must stay
  recoverable. Where a frontier branch genuinely cannot be reached it
  is row 4, with a message.
- *Row 3:* poison flows through **values**, never through decisions
  ([scalars]). `sup_norm_bound` returning NaN on every poison path is the
  pattern.
- *Rows 4 and 5 split on re-derivation, not on cost.* `unreachable!` is
  for a state the current best understanding says only a kernel bug
  reaches, however many interacting parts that understanding rests on
  (Ev, PR 4006): a failed lookup of a key minted or proven live in the
  same call (`topo`'s `Live` brand; a shared helper takes a proven-live
  key as its argument type and is `#[track_caller]`), and equally a
  failed read through a record of a body that every door keeps
  tier-1-valid. `debug_assert` is for a check that *re-derives* the
  invariant (`assert_euler_postcondition`, O(body)). Row 4's message
  states WHY the state cannot occur, not merely what was violated, and
  carries the values a reader debugging it would want; this stays
  prose, not a gate (`topo`'s `d18_no_unreachable_message_can_impersonate_the_postcondition`
  forbids one spelling and that is all a shape gate can do).
- *A torn body is a kernel bug.* A failed read through a record the
  body holds (a `next`, a `parent_loop`, a mate slot, a face's surface
  key) is row 4: `unreachable!`, its message naming the record and why
  it cannot dangle. A key the caller passed that does not resolve is
  row 1, a typed refusal stating the fact, and never the same variant.
- *Row 5's boundary:* `debug_assert` also serves the expensive check
  whose failure PROBABLY indicates a bug — a tripwire, not a proof.
  Its contract: (i) the assertion's absence never changes shipped
  semantics; (ii) an input-reachable failure still gets its row-1/2/3
  disposition; (iii) each such assertion documents its calibration
  in-file — the population measured and the margin observed — so a
  firing one reads as evidence (`geom_brep::props::quad`'s area gauge
  is the precedent; an uncalibrated ceiling was once off by nine
  orders). Where the measured condition is reachable by input, the
  preferred disposition is a reported value, not a tripwire
  (`topo::coherence`).
- *Boundary rule.* `pncad-py` re-types at the FFI edge — anything the
  Python layer can trigger is validated into a typed error before the
  kernel call, so an `unreachable!` never crosses into a
  `PanicException`.
- *Lint state.* `unreachable` is outside the banned clippy family in
  both the workspace manifest and `crates/pncad-py/Cargo.toml` (kept in
  step by that crate's `crate_lints_match_the_workspace_minus_unsafe_code`
  test); `panic`, `todo` and `unimplemented` stay banned. The
  `unreachable!` population and its per-site proofs live in the code;
  the unconverted exception (`merge_coplanar_faces`' ring re-homing,
  which reads its key from a loop back-pointer and refuses typed) is
  named in `topo::euler`'s module docs. The class is not confined to
  `crates/topo`; instances elsewhere are tracker items.

### Determinism `[determinism]`

*From:* [pure-model], [intrinsic].

- Same build + same inputs → bit-identical outputs. No hash-map
  iteration order may influence geometry; parallelism only in fixed
  reduction shapes (addendum below).
- Transcendentals via the pure-Rust `libm` crate: system libm differs
  across platforms in the last ulp — enough to flip a marginal
  predicate.

**Replay with kills.** The determinism contract holds with destructive
operators in the history: identical histories replay bit- and
key-identically, kills included; a failed operator consumes no key
slots. Convergence with a kill-free history is **per-arena**: a balanced
kill/make pair re-converges the half-edge, edge and curve arenas
immediately and the loop arena one loop-mint later; an unbalanced kill
history offsets the killed arenas' allocation cursors permanently —
untouched arenas stay aligned forever, killed arenas never re-align.

**Deterministic parallelism.** (PERF-PLAN §2.2; every use
cites these two idioms instead of re-deriving):

1. **Indexed parallel map**: results written to slot *i* of a
   pre-sized buffer. Combination is positional, not arithmetic;
   bit-deterministic at any thread count.
2. **Fixed-shape reduction**: FP sums/mins are **never**
   `par_iter().reduce()` (rayon's reduction tree is schedule-dependent).
   Idiom 1, then a *sequential* fold in arena order — or, if that fold
   profiles hot, a fixed-arity block tree with a named chunk size and a
   documented combine order.

Targets in value order: the subdivision driver, certification
sampling, mass properties, independent DAG nodes. Euler-op sequences
stay serial.

*GPU boundary (PERF-PLAN §3.3), ratified:*

| Work | Home | Why |
|---|---|---|
| Rendering, LOD, ID-buffer picking | GPU (viewer) | no kernel coupling |
| Preview (uncertified) surface evaluation | GPU-eligible display lane | never re-enters the kernel |
| Certified tessellation, export meshes | CPU forever* | the export promise needs certified bounds |
| Booleans, splitting, SSI, predicates | CPU forever* | D9 + certification; a GPU pre-filter is not worth the audit |
| Euler ops, validators, arena surgery | CPU forever | pointer-chasing, serial, already cheap |
| Interval lane / subdivision driver | CPU (rayon) | embarrassingly parallel on CPU already |

\* "forever" = for this project's plannable horizon; PERF-PLAN §3.2's
grounds (rounding control, f64, portability) are re-checkable facts.

- **Content-keyed cache transfer.** [determinism] bit-determinism makes any
  derived artifact keyed by the bit-content of its geometric inputs
  transferable across rebuilds by equality check — the key *is* the
  correctness proof; no dirty-flag invalidation. The key shape is
  shipped (mesh back-references, the content/naming keys); a
  finer-grained per-artifact transfer service remains future.

**Semantic equivariance where it is free — with the premise
UNAUDITED.** `[determinism.equivariance]`
Kernel constructions and selection rules should commute
with rigid motions *and reflections* at the semantic level (in ℝ),
unless equivariance is provably impossible for the case or costs
something real. This concerns DESIGNED rules — no left-hand rules,
no absolute-orientation tie-breaks — not bitwise f64 equivariance,
which [determinism]'s fixed evaluation orders forgo. Prefer intrinsic
quantities (arc lengths, distances, angles) over enumeration or
construction order; where a candidate-swapping symmetry makes
equivariance impossible, fall back deterministically and DOCUMENT
the residual. **The "the kernel is currently equivariant" premise is
UNVERIFIED; an audit is banked, not assumed. Do not cite the kernel
as equivariant without checking the claim at the site in question.**

### Engineering conventions `[engineering]`

*From:* [one-way], [unrepresentable].

1. **Sentinel-free tagged encodings.** Internal byte/key encodings never
   use in-band magic values (sentinel indices, marker floats); any
   stream mixing kinds is TAGGED TOKENS — tag byte + typed payload — so
   collisions are unrepresentable by construction.
2. **Save/load validation is ONE shared validator, not two mirrored door
   sets.** Every direction-independent document check lives in
   `persist::check::validate_document`, invoked by BOTH doors — at save
   on the in-memory document before bytes are written, at load after
   parse — so a document that would refuse to load is impossible to
   save by construction. The wire keeps only the genuinely load-only
   residue (parse/position errors, the canonical-set rule).
3. **A stale shorter matrix never gates a merge.** Any merge-gating
   checks watcher asserts a minimum green-row count equal to the current
   full CI matrix, bumped in the same PR that grows the matrix. CI
   carries a three-tier change filter, implemented once in
   `scripts/ci-filter.py` and called by `ci.yml`'s filter job:
   tier `docs` (only `*.md`/`memories/`) skips every build row and
   gates on the `docs-only` marker job; tier `all` (any workspace-level
   file, any member `Cargo.toml`, anything the allowlist does not
   recognise) runs the whole matrix; tier `closure` (crate sources
   only) scopes the cargo rows to the changed members plus every member
   that transitively depends on them. Classification fails CLOSED.

## Layering

Each layer depends only on the layers below it.

| Crate | Contents |
|---|---|
| `test-utils` | The shared test scaffolding several suites would otherwise each hand-roll: the fuzz/property harness (seed + effort dial), the `Display`-contract predicate, the anti-vacuity floor and its tightness companion, the shared Rust lexer, and the header-roster weld. A dev-dependency with ZERO dependencies — a leaf below every crate, which is what lets the excluded `interval-transcendentals` workspace depend on it too |
| `geom-core` | The `Real` scalar trait (`f64`, `Interval`, `Dual<T>`, `Sym`), points/vectors/transforms (hand-rolled, fixed-dim), the predicate vocabulary (`Decide`, `Margin<T>`, `MarginDiag`), `Tolerance`, root finding, the bisection walk the kernel's localizations share (`bisect`), spline hulls |
| `interval-transcendentals` | The interval scalar's backend beneath `geom-core`: proven per-function libm error pads, MPFR-differential-certified. A separate workspace root on purpose (root `Cargo.toml`'s `exclude`), so its gmp-backed oracle never enters the kernel's graph |
| `bvh` | Deterministic AABB tree: arena-order build, fixed split rule with total tie-breaks, conservative-superset contract — the tree prunes, exact predicates decide. Below the geometry crates (only `geom-core` under it) so SSI subdivision can consume it; certified box constructors live beside their invariants in `geom` |
| `geom` | Analytic + NURBS types, evaluators, closest-point and the other one-carrier readings (D3): a point's foot on a curve (`Curve3::foot`) and whether it lies on the carrier (`Curve3::on_carrier`), and a carrier's rate bounds (`carrier_speed`). Curves and surfaces are two modules of one crate, so the parameterization conventions and the totality/poison policy are stated once |
| `geom-brep` | The B-rep geometry layer: [edges]'s `EdgeDescription`, certified carrier caches, the dihedral classification predicate, Newell face equations, pcurve caches, SSI, the surface-pair dispatch table, the curve-pair meeting reader (the point-on-span reading, composing `geom`'s one-carrier readings with the span's reading of its ends; one total curve-kind-pair table) and the machinery under it (the half-angle ladder and its subdivision, `half_angle`; the depressed quartic's sign ladder, `quartic`), the carrier-pair pose and the curve×surface crossing (D3), certified mass properties, offset surfaces |
| `profile` | 2-D sketch profiles: the PATHS authoring algebra and the profile-program it records, lowering to the `Profile` of verbatim vertices and canonical segments, and its trilean validation |
| `topo` | Arenas, entities, Euler operators, the validation tiers; plane splitting, the boolean engine and its census/contact-record machinery (sibling modules at the crate root), shell/offset surgery, the kernel query seat |
| `sweep` | Solids from validated profiles: extrude, revolve, loft, sweep, tube; the blend family (fillets, chamfers) and its composition surgery |
| `verbs` | The kernel verb vocabulary seat (VERB-SEAT-DESIGN §2): one closed `Verb` enum reifying an operation's parameters as data, run dispatch, and the parameter→field flow; a layer guard keeps serde, `Expr`, `StableName` and recipe ids out |
| `mesh` / `stl` | Certified tessellation (watertight triangle meshes with source-`Face`/`Edge` back-references); STL export (binary + ASCII) |
| `step-export` / `step-import` | STEP (AP214) analytic-subset export, and import of that subset as adoption ([import]) |
| `quantity` | Typed quantities at the API boundary ([units]): `Length`, `Angle`, the unit table and the written forms |
| `editor-core` | Headless document/editor layer AND the parametric layer: document-as-value (recipe + metadata), typed edit vocabulary (`DocEdit` + `apply`, pure over the document and, where an inserted mate's datum needs them, the mated parts' reach — a function of the parts' pinned content; no edit records a frame, so the log is the edits themselves and replay is pure over the log), parameter expressions, feature DAG evaluation, persistent naming, stable-reference/selection model, incremental evaluation (preview/commit, epochs, cancelation), assemblies, distributions and the subdivision driver, the checks registry. No rendering dependency. See `crates/editor-core/README.md` |
| `pncad` / `pncad-py` | The authoring façade (LIBRARY-DESIGN U1 — one crate to depend on, a prelude, f64-first signatures) and its PyO3 bindings, which speak the document layer |
| `viewer` | The interaction layer over `editor-core`: `Camera`/`CameraOp` and `DocSession`/`SessionOp` as values with one `apply`/`perform` each, feature tree, property panel, selection, open/save, scene extraction — renderer-free and headless-tested; the eframe/wgpu application lives behind the non-default `app` feature. See `crates/viewer/README.md` |

Every layer below `viewer` *is* the product, exercised by tests and
code-driven models. The regression suite is mass-property checks
(volume/centroid vs. closed forms), watertightness validation, and
randomized parameter fuzzing — the fuzzing infrastructure is itself a
precursor of the error-propagation feature.

## Difficulty ranking (sequence around this)

1. **Fillets/blends, shelling/offsets** — hardest; even ACIS/Parasolid
   still get these wrong. Scope-boxed: constant-radius edge fillets on
   analytic geometry first.
2. **General SSI + robust NURBS booleans** — second hardest; deferred by
   [surfaces].
3. **Booleans on analytic geometry** — hard but tractable;
   classification is the main challenge.
4. **2-D sketch constraint solver** — a real subproject but
   well-trodden; possibly bind an existing solver ([open.sketch-solver]).
5. Everything else is careful engineering, not research.

## Milestones

Milestones M0–M9 are complete; each exit walk has a note in
`docs/doc-ledger/`. M0 scalar trait/arenas/harness; M1 topology +
Euler operators; M2 analytic geometry, extrude/revolve, tessellation,
STL; M3 analytic intersections, booleans, mass properties; M4 the
parametric layer, naming, STEP export; M5 NURBS depth, SSI,
constant-radius fillets; M6 the SSI generic-`T` lift, loft/sweep
assembly, composition surgery; M7 STEP import as adoption; M8 the
kernel residuals the demos raised; M9 the declared-contact join lane. Open work is the tracker's (`work/STATUS.md`); the
missing modeling verbs are registered in `docs/KERNEL-VERBS.md`.

## Beyond the kernel: the usability gap

A scoping section, not a milestone plan: it names the work between "the
kernel exists" and "a person can actually use this," so that none of it
gets invented ad hoc or discovered late.

**Sequencing stance: "usable as a library" ships before GUI work.** The
kernel has parametric models, mass properties, and STEP in both
directions; language bindings (Python — the CadQuery/build123d
audience), documentation and feature breadth yield a usable code-first
tool without an interactive application. The GUI is a separate layer
and effectively a second project of comparable size to the kernel
(Fornjot's postmortem and Zoo's app-team scale are the evidence); its
architecture is `crates/viewer/README.md` — the G1 three-layer split
(kernel / headless `editor-core` / interaction), egui as the toolkit,
the ray path authoritative for picking with the id pass advisory.
Multi-select UX and filter presentation stay deferred to GUI time.

### Band 1 — kernel-side services an interactive client requires

The "any GUI is a thin client" claim is true only if the kernel exports
these. All are shipped in `editor-core` except where noted:

- **Incremental recompute**: memoized per-node evaluation keyed on
  128-bit content/naming keys (op kind, structural params, evaluated
  expression bits beside each slot's f64 nominal, upstream keys,
  ambient ε/K, witness), evaluation
  epochs, deterministic level-parallel scheduling; a targeted mid-DAG
  edit recomputes only its downstream cone (pinned on the corpus
  documents). Remaining: partial re-tessellation, and a resident cache
  service — today the memo is the caller-threaded prior `Evaluation`.
- **Picking back-references**: tessellation output carries per-patch
  source-`Face` and per-polyline source-`Edge` keys, and
  `editor_core::resolve::pick::pick_face` is the `ray → StableName`
  service (`bvh::Bvh::ray`, exact ray/triangle tests, each admitted
  candidate answering a certified `t` INTERVAL, the `resolve::hit`
  inversion). One candidate is in front of another only when the whole
  of its interval is; candidates the geometry cannot order are a
  certified tie, answered as ONE face when they name one — the hull of
  their intervals — and REFUSED with all of them
  (`HitTestError::Ambiguous`) when they name several. Nothing else
  decides a pick: not a claim's width, not where the model sits, not
  the order the targets were offered in. `NodePick` pairs a mesh with
  its node by construction.
- **Cancelation** (`CancelToken`, yielding between nodes/levels; a
  canceled run returns the completed prefix as a typed outcome).
  Remaining: progress reporting (nothing exists) and in-op yield points
  — a long single boolean or fillet is still uninterruptible.
- **Selection stability across edits** — the user face of [identity]'s
  persistent naming: ONE `StableName` type for recipe references and
  selections, with resolution, the diagnosis ladder, tombstones, and
  `Rebind` with suggestion affordances (offers, never auto-repair).
- **Appearance attributes**: per-face/body display attributes live in
  the document layer keyed by stable names — never arena keys — survive
  recompute via post-pass resolution and report losses loudly;
  appearance-only edits recompute zero nodes.
- **Node labels**: a node's human label is document data — a map from
  node id to label beside the node, never inside it, so it is in
  neither the id's mint nor any content key. One edit sets or clears
  it; it is not unique and never identity (references hold ids, and
  nothing resolves a label). Kernel sentences speak a node as its
  kind, label and tag, `Extrude "base plate" (3fa9c1d2a0b1)`, or kind
  and tag when it has none. A spoken label is never one its value can
  outlive: a value the evaluation memo reuses holds a label only when
  its memo key fixes that label. A document's own labels are outside
  its evaluation key (a rename re-evaluates nothing), so what it
  memoizes keeps bare ids and the label is read off the document when
  the sentence is made; a part's labels are in its pin, and the pin is
  in the instance's key, so a fault from inside a part carries the
  part's nodes as the pinned part says them.
  The kernel mints no label; the viewer's create forms propose an
  editable "Kind N", stored only when the person commits it. Faces and
  bodies carry the same `Label` text as an appearance attribute.

### Band 2 — the interactive application (a second, kernel-sized project)

Architecture ratified (`crates/viewer/README.md`) and v1 built; what v1
delivered against these is partial, and the items keep their full-size
framing because that is what they still cost.

- **Viewport**: real-time tessellation with LOD, edge/silhouette
  rendering, section views, snapping, navigation.
- **The interactive sketcher** — the largest single item: dragging,
  dimension placement, constraint inference, and visual
  over-/under-constraint feedback. [open.sketch-solver]'s ecosystem gap (no
  DOF-diagnosis solver in Rust) becomes user-facing here, converting
  that solver from optional to mandatory.
- **Feature tree UI**: rollback, reorder, suppress, edit-in-place —
  [recipe]'s recipe-as-DAG is the substrate.
- **Error UX.** [tolerance.typed-failure]'s typed errors are correct for a kernel and brutal
  in a GUI if presented raw; `ResidualExceeded { check, sample }` must
  become "this fillet fails *here*" with the entity highlighted.
- **Direct manipulation** (drag a face → parameter change) is an
  inverse problem on top of everything above; optional except dragged
  sketch dimensions.

### Band 3 — the subsystems beyond the kernel proper

- **Assemblies — shipped at v1 scope** (`crates/editor-core/ASSEMBLY.md`):
  an assembly document is a recipe of the same formalism ([operations],
  [placement]) — a copy of a part is a placement, its mates and values
  are the placement's constraints, and a pattern is a placement whose
  reads reach an index, so the editor and solver machinery transfers
  unchanged; binding is
  pinned-with-explicit-update, the Cargo.lock model. Interference
  checks fall out of booleans / M10 clearance.
- **Engineering drawings.** Dimensioned 2-D drawings require projection
  plus hidden-line removal; HLR on curved B-reps is SSI-grade and
  belongs on the difficulty ranking near fillets. Near-term dodge:
  export STEP, make drawings elsewhere.
- **Feature breadth.** What the kernel has and what daily use still
  assumes (variable-radius fillets, draft, hole features, mirror,
  helixes, rib/text) is `docs/KERNEL-VERBS.md`; the long tail dominates
  "why can't I model my part."
- **Interchange breadth**: 3MF, DXF in/out, OBJ. Each small; STEP
  remains the only hard one.


**Flags banked.**
: mate solving needs witnesses/interval contraction
  on SE(3), not ℝⁿ; recipe-level provenance carries **pattern indices**
  explicitly so references into indexed families never degrade to
  positional guessing; rebuild latency is an architectural property,
  measured by the corpus and latency suites in `editor-core`.

### Band 4 — product-grade infrastructure

- **Recipe schema versioning/migration from the first RELEASED file**,
  autosave/crash recovery, and embedded derived caches so opening a
  model isn't a full rebuild. *Ruled (Ev, 2026-09-01): pre-release
  there is NO hand-maintained schema version, no migration chain and no
  bump coordination — nothing is released and every checked-in document
  is a regenerable artifact. The one door that stays: a file this build
  cannot read refuses TYPED, by the deserializer's own rejection of
  unknown or missing vocabulary wrapped in the regenerate recourse, so
  an additive vocabulary change invalidates nothing and a breaking one
  names the field. Versioning returns the day a document ships to
  someone.*
- **Performance at scale**: hundreds of features / thousands of faces;
  the parallel-evaluation story under [determinism]'s fixed reduction shapes.
- **A real-model corpus** as the usability regression suite: "these N
  parts rebuild in < T seconds with identical topology."
- **Docs and onboarding** for the API-as-product: `docs/GUIDE.md`,
  examples, and the Python bindings.

## Open questions

### Are findings part of the model's output? `[open.lint-output]`

The checks that report findings (the `unproven-coincidence` lint, the
at-rest census's interference and could-not-look findings, and checks
from outside the tree, DISCIPLINES DS9) are meant to be pluggable. How
their findings relate to [pure-model]'s function is open: the central
pipeline should not know which checks are in place, and findings
should not travel through a side channel.

### The sketch constraint solver: build or bind `[open.sketch-solver]`

No solver is in the tree. Ecosystem: **libslvs bindings** are dead and
GPLv3 (avoid); **planegcs** has no Rust bindings; **ezpz** (Zoo, MIT) is
a pure-Rust solver in production, pre-1.0 and product-driven, the
strongest option; **ISOtope** (archived, non-OSI) is the best free
writeup of the constraint-as-energy math — reference only. **Gap**: no
DCM-style graph-decomposition/DOF-diagnosis solver exists in Rust;
over/under-constrained diagnosis is ours to build regardless. Leading
answer: adopt ezpz when constraint-driven sketches have a consumer, with
"roll our own LM solver on `levenberg-marquardt`/`faer` using ISOtope's
math as tutorial" as the fallback. ezpz sits *upstream* of the certified
core (its output is numbers that then pass through our construction and
checks), so an arm's-length dependency stays principled.

- **Sketch DOF diagnosis is two named layers, never conflated.** The
  **structural layer** (DOF counting, graph decomposition — exact,
  combinatorial, float-free) diagnoses over/under-constraint; the
  residue — generically-well-constrained but configuration-degenerate
  sketches — is a Jacobian-rank fact at the witness, caught by GQ1's
  bifurcation-margin predicate with its own vocabulary ("degenerate
  configuration" ≠ "over-constrained"). "Solver didn't converge" is
  never reported as a diagnosis.

### The project's name `[open.name]`

License **resolved**: dual MIT OR Apache-2.0. Name: pending
(`docs/NAME-CANDIDATES.md`); placeholder workspace acceptable,
pre-publish renames are cheap.

### Before publishing (listed so they don't get lost)

Things deliberately in a pre-publication state; nothing goes red when
the project publishes with one still in the shipped state.

- **Roll the version numbers.** No member manifest carries a `version`
  field, so every crate is cargo's default `0.0.0`, and
  `[workspace.package]` says `publish = false` until the project has
  its name ([open.name]). Rolling them back is what un-does a premature publish.
- **Turn release debug-assertions back off.** The root `Cargo.toml`'s
  `[profile.release]` sets `debug-assertions = true`, so a release build
  runs every row-5 postcondition. That is the right posture for a
  kernel nobody depends on yet; deleting the stanza is a real reduction
  in what a release build checks, so it is a decision to take at
  publish rather than a chore (**S65**, `work/pred/S65.md`, is
  the worked example).
- **The name ([open.name]).**
- **Post-publish schema discipline for the verb tags** (VERB-SEAT-DESIGN
  §2 V2, folded here at the SEAT program's close). The document
  content tags `verb_content_tag` commits to (`editor-core/src/eval/
  mod.rs`) and the `Node` tag space are stable identifiers a
  published document format would freeze: after publication a tag
  number is never reused or renumbered, a kernel-only verb keeps
  `None` until a node arm exists, and the three tag censuses are the
  guard. Before publication they are pinned but movable by a
  deliberate re-baseline.

## Companion documents

Ratified design lives in this document AND in per-topic companions.
Companions whose programs have closed live as README pages beside the
code they govern and keep their clause ids: a citation such as
`CURVED-DESIGN C3` or `ASSEMBLY-DESIGN A6` resolves to that clause in
the row whose scope names the family. The design conversations those
pages condense, and every deleted `docs/` file, have a note in
`docs/doc-ledger/`. Live work is never listed here: `work/STATUS.md`
is the board and `work/README.md` its contract.

| Document | Status | Scope |
|---|---|---|
| `crates/geom-brep/README.md` | Ratified (#85) | Curved geometry, CURVED-DESIGN C1–C12: locus ladder, certificates, SSI, pcurves, dispatch, fillets, NURBS scope |
| `crates/geom-brep/README.md` | Ratified (#907) | Offset & shell, OFFSET-DESIGN O1–O6: analytic offsets by struct-update, the approximating-surface lift (`Surface::Approx`), the offset certificate, what shell IS (Q8's resolution) |
| `crates/editor-core/src/names/README.md` | Ratified (#74) | Persistent naming, NAMING-DESIGN N1–N7: derivation-path names, split/merge policy, name table |
| `crates/editor-core/README.md` | Ratified (#79) | GQ1 witness mechanism, SOLVER-DESIGN W1–W9 |
| `crates/editor-core/README.md` | Ratified with a recorded hedge (#1151) | The profile-parameter lift, PROFILE-LIFT-DESIGN PP1–PP6: guided replay — structure f64-once as the witness, geometry at the lane scalar with every consumed decision re-verified at `T` |
| `crates/editor-core/README.md` | Ratified (#496, option A′) | Group boolean in the recipe layer, GROUP-BOOLEAN-DESIGN: `PlacedUnion`, a Pattern that fuses — one prototype, one body out |
| `crates/editor-core/ASSEMBLY.md` | Ratified (#333); v1 shipped | How editor-core keeps [assemblies], ASSEMBLY-DESIGN A1–A13 + AQ1–AQ8: the evaluation seam, the pairing doors, pins and split/inline, mirror, the product, the constructive solve; the pre-refactor clauses (A3, A5, A9, A11 (2)–(4)) gathered until INTENT stages 3–5 retire them |
| `crates/topo/README.md` | Ratified (#178, #965) | Contact census & declared contact, CONTACT-DESIGN C1–C8 (the C7 join lane is shipped); at-rest census structural identity, the CENSUS-REST-CLOSURE-DESIGN clauses |
| `crates/geom-core/README.md` | Ratified (Ev, 2026-09-05) | The spline layer's pairing rule, SPLINE-DESIGN S1: a `Span` borrows the `KnotVector` it indexes, a `CurveWindow` its curve and a `SurfaceWindow` its surface, so every span-restricted door takes one structure and reads everything from it and the mismatch is unrepresentable; one level down a `SplineCoeffs` borrows the `KnotVector` its coefficients were fitted against and `hull`'s doors read through it, `InteriorKnot` the deliberate crate-private exception |
| `crates/sweep/README.md` | Ratified (#992) | ARMS-3, ARMS3-DESIGN A3-1…A3-3: the sphere×sphere fillet arm, the valence-4 seam vertex that is not a corner, what a run-out IS; the blend-vocabulary clauses V1–V4 |
| `crates/profile/README.md` | Ratified (V1–V8; enclosing tangency #1210) | Profiles as programs (PROFILES-V2-DESIGN V1–V8); the enclosing (ρ < 0) fillet tangency is permanently unreachable and a radius demanding it refuses typed (ENCLOSING-TANGENCY-DESIGN) |
| `crates/viewer/GUI-DESIGN.md` | Ratified; GUI v1 shipped | GUI architecture G1–G5, GQ1–GQ7: the three-layer split, egui as toolkit, what v1 ships. `crates/viewer/README.md` beside it is the implementation record, which the program maintains itself |
| `docs/ERROR-DESIGN.md` | Ratified (#110); M10 closed 2026-09-13 | Error propagation E1–E12: duals, stackups, the subdivision driver, trichotomy. M10 built it and closed 2026-09-13; the analysis lane is PROPS' and E12's symbolic tier continues as SYM |
| `docs/DUAL-DESIGN.md` | Ratified (#1146) | The Dual contract DL1–DL6: a Dual is tangent transport and never certifies; ContentBits feeds both channels; the delegation rule; poison-vs-widen in certified lanes |
| `docs/PATHS-DESIGN.md` | Ratified (#124) | The PartialPath authoring algebra |
| `docs/LIBRARY-DESIGN.md` | Ratified (#229); program open | Usable-as-a-library L1–L8: façade, Python bindings via the document layer, v2-fronted PATHS, the authoring-ergonomics unit ladder |
| `docs/DISCIPLINES-DESIGN.md` | WIP, provisionally accepted | Disciplines/checks registry DS1–DS9: identification criterion, severity invariant, the four grades, the recording dial, out-of-tree checks; residents live in `editor_core::checks` |
| `docs/PCURVE-UNIFY-DESIGN.md` | Ratified (#514); executed | Pcurve unification: the conventional edge variants collapse to ONE (surface, `Pcurve`) form, the exact variants kept as certification lanes; `MappedCurve` demotes to an authority record. P-2 residue stays open |
| `docs/RECIPE-DOORS-DESIGN.md` | Ratified (D2–D5) | Recipe doors for the surgery verbs: `Node::Chamfer` is `Node::Fillet`'s twin; `Node::Tube` and `Node::HollowTube` (wall REQUIRED on the hollow kind, `Option` nowhere in the recipe vocabulary); `Node::Shell { target, thickness, open }` (D5) with `open` ORDERED — the first designated face carries its chart's rim — resolved through the N5 ladder to the seat's `Verb::Shell` |
| `docs/MIRROR-DESIGN.md` | Ratified (#909); unbuilt | Patterns & mirror P1–P6: the chart-handedness convention (u ↦ −u), mirror's own door beside rigid transform, the boundary of A6's equivariance audit |
| `docs/DRAFT-DESIGN.md` | Ratified (#908); unbuilt | Draft DR1–DR6: plane walls only at v1, a certified re-geom pass, the pull-direction selector as a SELECT-DESIGN amendment, survivor naming |
| `docs/SELECT-DESIGN.md` | Ratified | Selection: filters, heterogeneous sets, vanishing entities; the contact-site recourse (§3d) |
| `crates/editor-core/VARIABLES.md` | Ratified (designer pair, converged); built | Variables, VARIABLES-DESIGN VR1–VR9: minted identity with a separate unique optional name, a slot holds a `VarId`, `Expr` holds no float (rationals and `turn`), `Formula` authored and lowered at the edit door, lifecycle, id-keyed tokens and analysis seeds |
| `docs/AXIS-DECLARATION-DESIGN.md` | Ratified (Ev, 2026-09-12, #2404); unbuilt | Retired by [coincidence] (header): coaxiality is one `Axis` read twice, or a margin decided Zero and recorded. The body keeps the conversation that ratified the channel |
| `crates/verbs/README.md` | Ratified (#1388; S3 corrected #1983, VS-Q4 revised #1870); SEAT closed, walk ratified #1997 | The kernel query seat, one verb vocabulary, lowered parameter identity, VERB-SEAT-DESIGN S1–S4, V1–V4, P1–P3: §1 query doors at `topo`; §2 the per-verb kernel `Verb` declaration; §3 the parameter→field flow |
| `docs/MATE-7-TANGENCY-DESIGN.md` | Ratified | Torus×torus rim tangency; the kissing arm banks on it |
| `crates/editor-core/REFERENCES.md` | Ratified (in-chat 2026-09-04; DM4 amended 2026-09-06; DM7–DM8 on the `[ev]` PR of 2026-09-16); DOCM closed 2026-09-13, EDIT its successor until 2026-10-02, then RECIPE (`docs/doc-ledger/edit-leaves-the-tracker.md`) until 2026-10-11, then DOCTAIL (`docs/doc-ledger/recipe-leaves-the-tracker.md`) | What a recipe reference may be, DM1–DM8: `Datum::FaceFrame`, the carrier-kind read, `Node::Part`, the n-ary `Node::Union` with `DocEdit::SetMembers`, all built (DOCM-1…DOCM-8) |
| `crates/editor-core/IDENTITY.md` | Ratified (in-chat 2026-09-04); DOCM closed 2026-09-13 | A held value names the world it came from, DI1–DI5: history-branch validity of node ids, the memo as a pure function of the document, `Evaluation` carries its document's identity, forking is its own act |
| `scripts/gates/README.md` | Ratified (Ev, 2026-09-06) | The CI gate directory: one home per gate, both CI halves, and the greps-vs-lints evaluation `S13` commissioned — `dylint`, `clippy::disallowed_*`, a proc-macro and a `syn` binary against the four grep gates. The four stay greps and the compound-bound-through-alias gap stays registered where it is disclosed. **The first design page outside `crates/<crate>/README.md`**: it sits beside the code it governs, which for these invariants is `scripts/gates/` and not a crate |
| `tools/README.md` | Ratified (Ev, 2026-09-08) | The instrument crates' shared rule, clauses `CC1`–`CC5`: where a check owed on what a file says belongs, and in which voice it speaks. The subject is the **reading boundary** — Ev's scope ruling at ratification — of which the cross-column admission is the largest instance: `tess-lint` and `k-lint` both police a CSV column by column, and a property spanning two columns has no entry to live in; the class had eight instances in the tree and no statement. `CC1` (the check goes at the reading boundary and only there) and `CC5` (the harness voice, and the owed-test forwarded to `tess_lint::Report` and on to its module docs — the forwarding is what licenses a citation across cargo roots) are stated over readings generally; `CC2`, `CC3` and `CC4` are labelled as the per-column admissions table's own and do not generalise past it, `CC4` being that a producer-side entailment is **not** a disposition — this page once stated its opposite, and the row proving it wrong is now a test. **The second design page outside `crates/<crate>/README.md`**, and the first governing two sibling crates rather than one directory of scripts |
| `docs/KERNEL-VERBS.md` | Reference register | The modeling verbs the kernel does not yet have, each with prerequisites, and the "present today" inventory. The register never schedules |
| `docs/K-REPORT.md` | Reference | K-constant evidence record (K = 10 permanent) |
| `work/perf/plan.md` | Merged-and-advisory ([determinism], deterministic parallelism) | Performance plan and Q-P answers |
| `docs/CURVO-AUDIT.md` | Reference | curvo vendor audit behind Q5 |
| `docs/LONGTERM-IDEAS.md` | Parked, non-binding | Idea bank with a graduation rule |
| `docs/MODEL-AB-LOG.md` | Experiment log | Model A/B protocol + data; process, not design |
| `docs/NAME-CANDIDATES.md` | Reference | Q9 name candidates (re-sweep before ratifying) |
| `docs/predicate-dimension-audit.md` | Live audit | Dimensional-analysis sweep of predicate comparands against D4; its *Findings* section is the live list |

## Crate landscape

Dependencies are for the *substrate*, not the modeling core.

| Area | Crate | License | Status |
|---|---|---|---|
| ID arenas | `slotmap` | Zlib | **Adopted**: typed keys per entity kind, `SecondaryMap` for attributes |
| Persistent collections | `imbl` (or `rpds`) | MPL-2.0 / MIT | candidate only — nothing has needed it. `im` is unmaintained with an open soundness advisory; use the `imbl` fork if ever adopted |
| Interval arithmetic | `interval-transcendentals` (in-house) | MIT/Apache | **Adopted** as the `T = Interval` backend: proven per-function libm error pads, MPFR-differential-certified via the optional `oracle-inari` feature of its own workspace, libm-only, D9-clean. **The kernel is copyleft-free in every build configuration**: `inari` and its gmp/MPFR LGPL stack appear nowhere in the root lockfile, and the hosted `interval-backend` CI row runs the crate's fast suites gmp-free and tripwires the graph |
| Robust predicates | `robust` (georust) | MIT/Apache | nothing of ours calls it; rides in transitively under `spade` |
| Dual numbers | `num-dual` | MIT/Apache | dev-only derivative oracle; duals are the in-house `Dual<T>` ([verdicts]) |
| CDT / mesh refinement | `spade` | MIT/Apache | **Adopted** (`mesh`): Delaunay + constrained + Ruppert refinement in UV space; exterior classification is ours (even-odd flood fill), spade supplies the CDT only. Insertion is quadratic for faces bounded by nested near-cocircular loops (`mesh` §Performance) |
| Serialization | `serde` + `serde_json` | MIT/Apache | **Adopted** (persistence); the `float_roundtrip` feature is LOAD-BEARING. Kernel crates stay serde-free (`scripts/gates/kernel-serde-free.sh` checks the dependency edge of every manifest; `profile` is additionally sealed by `profile/tests/seal.rs`). Where a kernel type must persist, its bytes are described above the boundary, not by a mirror enum (CONTACT-DESIGN C4) |
| 2-D polygon booleans | `i_overlay` | MIT/Apache | candidate only (present in the lockfile only transitively under the curvo oracle's `geo`) |
| Display triangulation | `earcut` (georust) | MIT/Apache | candidate only (same) |
| Sketch constraints | `ezpz` (Zoo) | MIT | not adopted; see Q3 |
| STEP | `truck-stepio`/`ruststep` | Apache | dev-dependency parse-back oracles for the in-house AP214 writer only — ruststep cannot write STEP; truck-stepio's writer has unfixable conformance defects |
| GUI toolkit | `egui`/`eframe`/`egui-wgpu`, `winit`, `egui_tiles`, `bytemuck`, `rfd` | MIT/Apache | **Adopted** (`viewer`), every entry optional behind the non-default `app` feature so the toolkit graph never enters a kernel PR's compile closure |
| Python bindings | `pyo3` | MIT/Apache | **Adopted** (`pncad-py`), `abi3-py38`, optional so a kernel build never links Python |
| Hashing | `sha2` | MIT/Apache | **Adopted** (`editor-core`): content pins are SHA-256 over canonical semantic bytes (ASSEMBLY-DESIGN A4) — the pin IS version identity; the in-process FNV `ContentKey` stays a separate, weaker vocabulary |
| OS randomness | `getrandom` | MIT/Apache | **Adopted** for interactively-authored document ids in `pncad` (and the wasm32 backend in `viewer`); `editor-core` stays deterministic by construction |
| NURBS oracle | `curvo` (git-pinned) + `nalgebra` | MIT / Apache | `geom` DEV-dependency at the audited commit; never a runtime dependency (Q5) |

Reference-only (read, don't depend): **truck** (the only living Rust
B-rep kernel; booleans demo-grade), **vcad** (young Apache-2.0
half-edge kernel), **Fornjot** (archived June 2026 after multiple
rewrites without robust booleans — required reading as a postmortem),
**opencascade-rs** (the only production-grade-boolean route in Rust;
LGPL + C++ build tax; a *test oracle* for boolean results).

**`curvo`: a study and a dev-dependency oracle; vendoring rejected.**
`docs/CURVO-AUDIT.md`. The core invariants (certified residuals,
trilean predicates, generic `T`, no hidden tolerance decisions) live
*in the algorithms*, and in every candidate routine the
invariant-relevant surface is exactly what a retrofit would rewrite;
curvo has no SSI at all. Oracle scope, pinned at the audited commit:
evaluation/derivatives/basis/degree-elevation/interpolation; not
bit-exact (std-routed math); not SSI/booleans (opencascade-rs/truck
remain those oracles).

## Prior art / references

Local copies live in `references/` (git-ignored): the NURBS Book (2nd
ed.), Mäntylä complete, Hoffmann complete, the GSD06 discrete
differential geometry notes, and Vida–Martin–Várady 1994 (the canonical
blending survey; source for the fillet scope-boxing and [edges]'s
`TangentIntersection` treatment).

- **Mäntylä, *An Introduction to Solid Modeling*** — the
  Euler-operator B-rep reference; `topo` is essentially this book (with
  the mirrored orientation hazard noted under [topology]). One erratum on
  record: Program 11.6's `lmev` `addhe` order (PLUS-half first) breaks
  both `he1 == he2` cases; MINUS-first is coherent.
- **Hoffmann, *Geometric and Solid Modeling*** — intersections,
  robustness.
- **Piegl & Tiller, *The NURBS Book*** — canonical NURBS algorithms.
- **Grinspun–Schröder–Desbrun, DDG course notes** — the
  discrete-exactness philosophy framing how stackup design treats
  kinks/subdifferentials ([scalars]).
- **Fornjot** and **truck** — the two serious Rust B-rep attempts;
  study the topology/geometry split.
- **Open CASCADE** source — a catalog of what every subsystem must do,
  and a cautionary tale on tolerance philosophy.
- **Parasolid XT format spec** — the cleanest picture of a production
  kernel's data model.
- **Shewchuk's robust predicates** + CGAL literature — for the
  polyhedral/predicate corners where exactness is achievable.

## Old numbering

Where the clauses of this document at commit `95c77e152a` now live.

| Old | Now |
|---|---|
| The central commitment | [pure-model] |
| D1 | [topology]; D1's frontier (c)–(f) moved to `docs/KERNEL-VERBS.md` |
| D2 | [edges] |
| D3 | [surfaces]; "One reading per question" is [one-way.readings]; "What a consumer does with the answer" is [verdicts.consumers] |
| D4 | [tolerance]; ¶1 is [tolerance.epsilon] with [tolerance.recourse] (the two-tolerance principle and its (i)–(iv)), [tolerance.chordal] and [tolerance.margins]; ¶2 [tolerance.residuals]; ¶3 [tolerance.typed-failure]; ¶4 [tolerance.units]; ¶5 [tolerance.boundary] |
| D5 | [identity] |
| D6 | [units] |
| D7 | [import] |
| D8 | [recipe]; "The recipe keeps the form that was written" is [recipe.forms] |
| D9 | [failure] (the never-panic rule and the bug-vs-invalid-state taxonomy, [failure.taxonomy]); [determinism] (bit-identity, libm, replay with kills, deterministic parallelism, the GPU boundary, equivariance as [determinism.equivariance]); [engineering] (the other engineering conventions) |
| D10 | the principle is [intent]; "Variables" is [variables]; "Operations" is [operations]; "Repetition" is [repetition]; "Spaces and placement" is [placement]; "Coincidence" is [coincidence]; "Booleans" is [coincidence.booleans]; "Assertions" is [assertions]; the retirement list is in [State of the build](#state-of-the-build) |
| Q1 | [scalars] and [verdicts] |
| Q2, Q4, Q6, Q7, Q8 (resolved) | [tolerance], [tolerance.units] and [units], [recipe], [determinism], OFFSET-DESIGN O1–O6 |
| Q3 | [open.sketch-solver] |
| Q5 | the `curvo` paragraph under [Crate landscape](#crate-landscape) |
| Q9 | [open.name]; the license is dual MIT OR Apache-2.0 |
| Roadmap | [Milestones](#milestones); its standing outcomes: bit-identity coincidence checking's retirement is dropped as long settled (the `bit-identity-consumer` gate keeps it), and K = 10 is in [verdicts] |
| Banked principles | naming localized to predicate flips → [identity]; content-keyed cache transfer → [determinism]; coincidence is a margined verdict → [verdicts]; the evaluation service generic over `Real` → [scalars]; ε and persistence → [tolerance]; fillet validity → [verdicts]; SSI completeness → [edges]; non-manifold results → [topology]; the total expression sublanguage and bit-exact persisted floats → [recipe]; sketch DOF diagnosis → [open.sketch-solver]; flags → [Beyond the kernel](#beyond-the-kernel-the-usability-gap) |
