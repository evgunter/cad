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
state, data rather than behaviour. The seven principles below are that
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
  an assertion that says what it decided quiets it ([coincidence],
  [assertions.quiet]). A coincidence between quantities only the
  operation computes (a blend turn's two feet), or between written
  values (two radii typed alike), has no such assertion: the
  construction is its recourse, and its finding never refuses. Where a
  construction could also say it (one variable written for both
  sides), the assertion quiets all the same, and the finding names
  the edit that would make it one construction. A
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
  never the lines or planes they lie on ([topology.census]); coincidence
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
[placement], [edges], [identity] (the names' N0, `crates/editor-core/src/names/README.md`).

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

### The code is as simple as it can be `[simple]`

The code is as simple as the problem allows. Two instantiations carry
most of the weight:

- **One uniform rule, not special cases.** `[simple.uniform]` Where one
  rule covers what a set of special cases would, the one rule is the
  design. For example, in the names
  (`crates/editor-core/src/names/README.md`): what several members of
  a union hold at once, a face, an edge or a vertex, is named `Merged`
  of all of them, where each kind once had its own rule for picking
  one; and nothing is ranked, so names that do not tell two crossings
  apart tie. Orientation is one rule ([topology]), and so is what a
  document establishes ([intent.structural]).
- **Checks serve the code.** `[simple.checks]` Tests, debug assertions
  and CI are tools for finding defects in the code, not a ritual:
  nothing is added to the code for a check's sake, and no
  representation, stage or decision exists only so a check can read
  it. Where a check needs one, the check changes or goes.

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
| Contact records across an op | one lineage, one carry | [topology.records] |
| Which description a glued pair keeps | decided once, at the glue | [coincidence.booleans] |
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
a second method for the same question exists only as a test oracle.
A reading may be built of several stages, always run together in a
fixed order, when together they decide one rule: the stages are the
parts of one proof of that rule, not a list of heuristics tried in turn,
and each stage decides soundly at the reading's band. A box cull before
exact roots is such a stage. So is the interior-loop guard's loop-free
test before C3's whole reading of a plane × spline section: the rule is
that no closed loop of the section lies inside both faces, which the
test proves where it holds and the whole reading proves or refuses
where it does not. What is forbidden is a method that decides a
different rule, a second and similar predicate for the same fact,
reached because the first declined: that is a fallback.
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
  raw `<` on control-flow paths. Nor does an instrument represent a
  tie-break the band forbids it to make: where its reading would turn
  on a choice inside the band that its answer cannot depend on, it is
  posed so that the choice does not arise (a ray within the band of an
  edge meets the edge, half a crossing for each face the edge bounds;
  within the band of a vertex it reads nothing).
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
scale. **An angle is metered in whatever way is required to make all
geometry correct to within ε**: every angular predicate uses θ = ε/r
with its lever arm named at the call site, the reach at which the
angle displaces the geometry it governs — 1/κ_rel for tangency, the
distance of each face's own points from the other's carrier for
parallelism, the session-box extent as the conservative universal arm
only where no such geometry is in hand. ε ≈ 1e-9 m gives micron-to-kilometer coverage with ~4 orders of
f64 headroom at km scale. Import does not motivate loosening ε ([import]).


**The two-tolerance principle.** `[tolerance.recourse]` Two roles are
kernel-wide vocabulary: **ε_precision**, this ε (certification
residuals, what gets built), and **ε_input**, the least precision a
user might care about (what counts as too close to a coincidence when
interpreting input). ε_input *is* K·ε, a synonym and not a third dial:
K stays the one knob (`Tolerance.k`, env `CAD_AMBIGUITY_K`), and
[import]'s ε_in is an instance of ε_input (iii). Binding consequences:

- **(i) One message, one recourse per decision.** User-facing messages
  and recourse never fork on exactly-on vs in-band below ε_input: each
  **decision** (the one question a site asks) has one story, written
  once in its own levers and shared by all its refused arms, with the
  margin riding the payload as data. Kernel semantics keep the
  distinction; this is message policy, not predicate policy.
- **The recourse follows from the decision and its verdict.** *Make the
  two one construction* ([coincidence]) only where the refused side is
  a coincidence a shared construction would decide; *move the
  geometry*, phrased as the decision's own lever, always; *tighten the
  tolerance* only where the refused margin is a size the user may
  intend and a smaller ε decides it, phrased conditionally and with the
  value the margin gives, never on a sign-certain arm and never on a
  decision that passes only at Zero. No refusal advises loosening ε,
  except one that would otherwise name no recourse at all (a kernel
  approximation limit), which names it as a last resort and says the
  refusal may indicate a kernel bug. An `unproven-coincidence` finding
  refuses nothing; its recourse is one construction or an assertion at
  its site ([assertions]).
- **(ii) The decision is a closed type at its site**, so its recourse is
  an exhaustive match, never a lookup by predicate name. Error variants
  may stay distinct as data; their user stories converge per decision.
- (iv) **The rule binds a predicate's DEFINITE arms too**: a
new definite outcome a user could reach by moving geometry tells the
same one story with the same one recourse as its in-band sibling.
"Definite" means the reading certainly lies on a refused side,
whether the band, a number or a proof decided it, so a proved zero
tells the story of its exact and in-band siblings. Each decision
says what its refused side is, and every definite reading of it ends
by that at each door. A side the lever reaches tells the lever story
everywhere. A contradiction between a stored description and its
stored geometry is the kernel's defect at a build, since every build
door that holds a carrier and a face is the kernel's own: a verb that
takes a user's curve refuses it at its own door, with its lever. At the
import door
it is the file's: the import refuses typed, naming the file. Every
defect ending a bad file reaches there past ε_in names the file,
whether the reading compares a value the file stored or one adoption
derived from it; a sound file such an ending reaches is a kernel bug to
fix. Past the import door no state is reachable only through import
([import]), so only a kernel bug reaches it, and it panics ([failure]).
A side no kernel construction produces panics at a build and past the
import door, and at the import door is the file's like any other. No
refusal names a lever the decision does not.

How each door applies these (the import door's ε_in sentences and its
stopgap, `Indeterminate`'s rendering, `COINCIDENCE_RECOURSE`, the review
checklist) is `crates/geom-core/README.md`, "The recourse rules at each
door".


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

**A measurement refines until a predicate over its enclosure resolves.**
`[tolerance.measure]`
A certified measurement (volume, area, moments) is an enclosure, never a
number alone, and its width is part of the value. There is one door. It
refines the measured surface where its enclosure is widest, until a
predicate over the enclosure resolves, until refinement stops paying (a
floor no round removes), or until the work budget the caller states with
its predicate, in one unit for every lane, as a multiple of reading the
measured surface once. The kernel's own questions (the validity gate, a
boolean's decisions, a document's assertions) state a fixed constant per
question. What a measurement spends
depends on what it asks and on the geometry, never on how its surface is
cut into faces. It returns the narrowest enclosure reached and why it
stopped.

A consumer that decides something passes its predicate: a sign, a
relation against a bound as an assertion states it, or a containment. At
the public doors the predicate is closed data, not a callable. A deciding
predicate is a named predicate ([verdicts]). It settles when definite,
refines while its enclosure straddles the threshold, stops when the
enclosure lies wholly in the band, and is recorded at every round it is
asked.

A consumer that hands the number on passes a resolution, the width
predicate: the mean boundary displacement it needs over the measured
surface as a whole. That is a sizing,
decides nothing against ε ([intrinsic.epsilon]), and is not recorded. It
is for a consumer that wants the number and nothing else: one that would
compare the result against anything passes that comparison as its
predicate instead, so a decision is never made on a bracket sized by a
guess.
The model defines its measures only to ε of boundary displacement, so no
finer resolution is offered. There is no default: every caller states
its predicate, a resolution included, and its budget at the call, so the
precision a measurement buys, and what it may spend buying it, are always
the ones its consumer asked for.

A deciding predicate still undecided at the budget is the one
quadrature refusal, and it carries the enclosure. A resolution unmet
there is the bracket, with the resolution it reached.

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
construction door. A site where no door honestly fits
is a finding, not a cast. The vector/linalg interior stays bare `T`.
(ii) **No dimensionally-heterogeneous uniform payloads**: a field
whose dimension depends on a runtime kind tag is illegal;
kind-dependent data lives in per-kind enum variants. (iii)
**Parameter-space values cross to model space only through per-kind
metric doors.** Extension is opportunistic as
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

*From:* [pure-model], [unrepresentable], [one-way], [loud-failure].

The rules a body keeps, which every other crate relies on. How `topo`
keeps them (the operator set's addressing, each tier's checklist, the
census's lanes and certifiers, the carry's landings, the join and merge
mechanics) is `crates/topo/README.md`, "The body, its operators and its
tiers".

- **A body is a value of typed arenas.** Topology entities (`Solid /
  Shell / Face / Loop / HalfEdge / Edge / Vertex`) live in generational
  arenas and reference each other by typed ids, never pointers, so a
  B-rep is a plain value: cheaply cloneable, serializable, diffable,
  validatable. It is Mäntylä's half-edge structure, an `Edge` being two
  antiparallel half-edges.
- **A `Body` is never a source of truth.** `[topology.body]` It is the
  materialized evaluation of a construction (an operator sequence; above the kernel, a recipe)
  at some scalar `T`, coherent iff bit-identical replay reproduces it
  ([determinism]). Mutation exists only as evaluator-internal linear working
  state; a body at rest is a plain value, and modification means
  deriving a successor body by further construction. Nothing about a
  body is true that is not derivable from its construction (for
  imported bodies, from the adopted descriptions plus the import
  record, [import]).
- **Manifold solids only.**
  Non-manifold (radial-edge) topology is added only if sheet or wire
  bodies demand it. Touching via *distinct* entities (two vertices at
  one point, two edges on one segment) is representable and a typed
  success carrying its tier-3′ records; a single edge with more than
  two faces, or a wedge fan sharing one entity, assembles faces into
  something that is not a solid, and is not represented. Where more
  than two sheets meet on a line, the material decides the edges
  ([topology] tier 3). Silent splitting
  into separate manifold bodies is rejected as inexplicit; any future
  split behaviour is an explicit operation the user invokes, never a
  fallback.
- **Topology is built only by the Euler operators.**
  `[topology.operators]` A small closed set of primitives that provably
  preserve the Euler–Poincaré invariant is the only public construction
  path. Each is atomic (typed preconditions resolve before an
  infallible mutation, and a failed op consumes no key slots), mints in
  a documented deterministic order ([determinism]), and asserts its
  postcondition: its declared arena delta at every call, and the
  whole-body tier-1 re-derivation once per public door, at the door's
  end (Ev, PR 2305). So every public mutation path preserves tier 1,
  checked at every observable boundary. Degenerate sites live in the
  argument types, never behind null checks ([unrepresentable]).
- **Orientation is one rule, and reversal is exact.** Walking any loop
  in `next` order with the face's outward normal toward the viewer, the
  face interior lies to the left of every half-edge, stated against the
  sense-signed chart normal. A face carries `Face::sense` (STEP's
  `same_sense`), decided where its chart is decided, from topology or
  stated by the caller, never by a numeric compare or a default.
  Reversal is exact structure, so `revert ∘ revert` is bit-identical at
  every scalar, and every "which way is out" consumer reads the signed
  normal.
- **Validity tiers.** `[topology.tiers]` Each tier is what a body at it
  promises:
  1. **Tier 1, euler-valid:** the structural invariant of every
     Euler-reachable state, construction scaffolding included; what each
     operator asserts.
  2. **Tier 2, closed solid:** tier 1, with no empty loops, no
     valence-1 vertices but a wrap edge's tip, and one connected
     component per shell.
  3. **Tier 3, geometric:** tier 2, with every derived cache within ε
     of its description ([tolerance.residuals]); the **material wedge**
     at every edge in (0, 2π), its ends (a cusp, a knife slit) legal
     only where the tangency is jet-determinate; **maximal edges** (no
     joinable vertex at rest); **one region per face**: a face's
     loops bound one region of its surface. An edge the face lies on
     both sides of (a seam, or a wrap edge: the seam that ends at a
     chart singularity) bounds nothing on the surface, so the face's
     **surface boundary** is its loops with those edges dropped; an
     empty one is the whole surface. Its cycles are simple and
     disjoint, each lies on the face's side of every other, and an
     open surface's ends lie off the face. Whether a point lies in a
     face has one reading, the point-in-face doors' (on the face's
     edge curves), and tier 3 asks those doors what they presume. A
     reader in the chart's own coordinates (measurement in a lifted
     coordinate, a loop's windings, the mesher's domain) reads the
     loops as stored, seams and wrap edges as the chart's cuts and a
     wrap edge's tip through the lift's reset at its singular
     vertex. A chart-free reader (Gauss–Bonnet, 3-D flux) reads the
     surface boundary, keeping each wrap tip's singular vertex as the
     datum of the face's side; an empty one on a closed surface is
     the whole surface, signed by the face's sense. No reader reads a
     turn at a wrap tip. **Chart singularities as vertices** are an instance (every one in a face's closure is a
     vertex of it, and every wrap edge's tip lies on its face's chart
     singularity);
     **prefer-intrinsic** descriptions ([edges]); and the **orientation invariant**, a solid's faces
     leaving infinity outside it, read by the one reader of a shell's
     role ([coincidence.booleans]). A zero-volume lamina is a geometric
     defect.
  4. **Tier 3′, pseudomanifold:** the honest at-rest tier for results
     that touch through distinct entities: tier 3, a global coincidence
     census, and certification of contact records in both directions,
     never scanning to bless.

  A **finished body** (`AtRestBody`) passes tier 3′ against its own
  contact records, none for a body that touches nothing, once, at the
  door that built it; construction state becomes a finished body only
  through that gate. The validity class rides the result wrapper
  (`BooleanBody`, body and contacts), never a mutable `Body` field.
- **The census reads one number per cell pair.** `[topology.census]`
  A census finding decides one number per pair of cells (two edges,
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
- **Touching is always recorded.** `[topology.records]` A contact
  record is a pair of the body's cells, one from each touching shell
  (or two of one shell, where a piece touches itself), whose interiors
  meet, plus its backing: a coincidence an op decided
  Zero, recorded ([coincidence]). It carries no operand label, stored
  granularity or witness. Result-side touching arises only from operand
  coincidences carried through the op's lineage or decided by the op
  where two cells it minted meet, and a touch the census finds that no
  record backs is never silently blessed. Between two operands of a
  `⊔` at rest ([assertions]) the census is the deciding operation: it
  records each such touch as an `unproven-coincidence` finding and
  leaves on the result a contact record citing it, which every reader
  and every placement of that body carries, so the touch is never
  decided again. Inside one op's
  result it is that op's defect, refused typed. Every op, the boolean
  and every surgery verb alike, carries records through one lineage
  stating what it made of each cell (kept, substituted by a cell
  holding its interior, divided, or removed), and records are written
  by the op that changes their cells, never re-derived at the gate.

**Structural conventions.** `[topology.conventions]`

- **Every cavity is born through one door.** A cavity's boundary is a
  disconnected interior shell, and its bookkeeping (orientation, census
  participation, containment evidence) has exactly one home,
  `topo::insert_void`. Recipe-layer sugar may wrap a producer; the door
  stays the one birthplace.
- **∅, disjoint, and voids are typed results.** ∅ is a typed success
  value (`BooleanResult::Empty`), not an error. **A solid is one piece
  of material**: the `Outer` shell around it and the `Void` shells of
  the cavities in its material, which together bound that material and
  nothing else. A cavity belongs to the piece whose material surrounds
  it, so an island inside a cavity is a solid of its own, and a cavity
  inside that island is the island's. Pieces that only touch (at a
  corner, along an edge, or across a face that contact records hold
  apart) are distinct solids, because touching is between distinct
  entities (tier 3′). A piece that touches itself stays one solid,
  over an area too: `(bolt ⊔ plate) ∪ bead` is one connected solid
  whose bolt and plate faces lie on each other, held apart by a record
  that cites the `⊔` that decided it. Until the census and tier 3′
  read an area self-touch inside one shell, an operator whose result
  holds one refuses typed, naming the pair. A
  body is any number of solids, so a disjoint union is a body of
  several solids. Booleans, `shell` and `split` take bodies and return
  bodies, and each sorts its result into solids, so every output is an
  operand, a body `⊔` assembled among them: an operator above it
  decides only the meetings between its own operands
  ([coincidence.booleans]). Which solids are one part stays recipe
  structure, never body state (`crates/editor-core/ASSEMBLY.md`, A2).

- **Curved booleans retire per arm, never wholesale.** A face kind with
  no arm refuses typed (`CurvedBooleanUnsupported`,
  `CurvedPairUnsupported`), naming the pair, never falling through to a
  containment verdict a curved boundary can defeat. Which arms are
  wired is the frontier's (`docs/KERNEL-VERBS.md`).
- **No snapping.** Every reduction comparison is a trilean
  ([verdicts]), and nothing beyond a Zero verdict moves geometry: a
  merged face keeps one operand's description, and nothing in the
  sliver band is moved. Booleans on independently modelled
  nearly-touching bodies fail loudly rather than guess; the resolution
  is one construction or an explicit repair or adoption ([import]).
- **Maximal faces and maximal edges.** Merging is never silent: a
  boolean's output stage merges, on a Zero verdict between the
  carriers and never on bit-equality, and joins every joinable vertex,
  and every door that finishes a body ends with the join. A body is
  then the unique complex with maximal faces and maximal edges over its
  face partition, up to the position of each closed edge's
  conventional vertex and the place of each wrap edge that reaches a
  chart singularity, and the form is checked at rest on the result
  alone. Within one n-ary operator, member order is not precedence: the
  written order decides only which description a glued face keeps and
  which fold step a refusal names, and an order that refuses where
  another builds is a kernel defect ([operations.operators]). Merging
  **never fuses two vertices into one**; the record carry rests on it.
- **A seam is a property of a face**, not a place on its surface, and a
  wall keeps the cuts its author wrote: the kernel adds none, except
  where a face or edge carrier is not C¹ by its knot structure, and at
  a chart singularity (a sphere's pole, a cone's apex), which is a
  vertex of every face whose closure holds it. Where no edge the author
  wrote reaches that vertex, the kernel adds one wrap edge to it, and
  only one: the singularity is that edge's tip, where the face's loop
  walks the edge out and back, and the edge's place round the axis is
  conventional, like a seam's. That wrap edge is the face's seam
  (D1), kept to its end at the singularity: a native construction's
  seam already is it, and the kernel mints one only where a file
  states none. The join never removes a cut either
  exception requires.

**The frontier is typed and named.** Every unbuilt case refuses with a
message naming its own blocker ([failure.taxonomy], row 2), so the
frontier is grep-able from the tree. Its prose inventory is
`docs/KERNEL-VERBS.md` — the "present today" paragraph, the crate
READMEs, and the standing frontier entries (c)–(f) older citations
name — and its schedule is the tracker.


### Edges `[edges]`

*From:* [intent.stored], [one-way], [loud-failure].

Topology and geometry live in separate arenas: faces reference
surfaces, edges reference curves, vertices reference points. How
`geom-brep` keeps the rules below (the pcurve background, the
derivations behind each validity predicate, the witness contract) is
`crates/geom-brep/README.md`, "Edge descriptions".

**An edge stores what its locus is.** An edge's geometry is an
*intensional description* of the locus; every concrete representation
(the 3-D carrier, both pcurves) is a derived cache carrying a certified
residual bound against it ([tolerance.residuals]), never a peer of the
description. The type is `EdgeDescription<T>`
(`crates/geom-brep/src/description.rs`):

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

- **A two-surface edge is always a curve of the two surfaces'
  intersection**, and its selector says which. The selector follows one
  question, asked of the geometry and never of where the body came
  from: *is the curve known in advance?* Where it is (a closed form of
  the surfaces' contact or crossing, or a curve chosen to lie where they
  overlap in 2-D) the selector is **structural**, that closed form,
  certified through each surface's chart-image certificate; a native
  body and its imported copy carry the same one. Where the curve must
  be derived numerically, the selector is a **witness**, the branch
  through a point, admitted only where its margin certifies
  (transversality for a crossing, the order-k jet system's margin for a
  tangency of order k). A construction never classifies its own joins by
  measurement, and a curve no lane can yet select refuses typed, naming
  the lane.
- **A locus one chart defines is `Chart`**: seams, iso-lines, splits on
  one surface, one (surface, `Pcurve`) form for all of them
  (PCURVE-UNIFY-DESIGN). A `MappedCurve` is the `Scaffold` payload, a
  structural selector and the `EdgeAuthority` record: one authoritative
  source, never two peer representations.
- **No extensional variant.** There is no `Explicit` escape hatch, for
  native or imported geometry: import adopts extensional input by
  recovering the selector a native body would carry, or refuses typed
  where no lane recovers one ([import]).
- **Validity is margined.** `Intersection` requires transversality along
  the locus; a tangency of order k is read through its k-jet, with its
  margin at order k+1, transverse to the locus. A locus in the ambiguous
  band, certifiable as neither, fails loudly at construction exactly as
  at import, and so does an edge read definitely transverse at some
  stations and definitely smooth at others.
- **Prefer-intrinsic, enforced at tier 3.** Every two-surface edge is
  intrinsic and its selector is decided by the geometry alone; at rest a
  witness selector must certify its reading (`TransverseNotIntrinsic`,
  `TangentNotIntrinsic`). An unenforced preference drifts silently.
  Since the closed form is description and not history, the intensional
  description largely *is* the provenance ([identity]).
- **SSI completeness is an interval obligation, not a marching
  property.** Marching finds, subdivision certifies exhaustiveness: the
  outcome is "every branch found" or a typed failure, never silence.
  Certification is an at-rest obligation; preview may march
  uncertified.

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
- *Discrete kinds:* a side, a half, a sense, an orientation
  ([placement.values]).
- *Poses:* `Point`, `Direction`, `Axis`, `Plane`, `Pivot`, `Flag`, `Frame`
  ([variables.poses]).
- *Shapes:* `Body`; `Bodies`, an ordered list of bodies whose length
  is a `Count`; and `Profile`. Only an operation defines a shape. A
  `Profile` is 2-D shape whose numbers are read only against each
  other; a sweep reads a profile and, at most, a heading and a tilt in
  the profile's own axes (an extrude's slant, held to one side of the
  profile's plane) or a point and a heading (a revolve's axis line). A
  profile drawn on a `Frame` may read traces in its plane
  ("Constructing a coincidence").
- *Spaces:* `Space`, a coordinate system ([placement.spaces]). A
  space holds nothing and contains nothing: no pose, no number, no
  member list, and no origin, axis or plane that a reader takes as a
  pose. It is defined only by minting: the world, `new_space()`, and
  each construction, whose body is born in a space of its own
  (`a.space`). A placement reads a space as its target, and the
  values of placements into a space are the only readers of its axes.
- *Selections* of a shape: one cell of a body (`Face`, `Edge`,
  `Vertex`), or the set of them a reader takes (`Faces`, `Edges`; a
  set kind exists only for a slot that reads one).

**Free and defined.** A variable is **free** — a value, its written
unit ([units]) and optionally a distribution — or **defined**: by an
`Expr` over other variables, by a selection of a `Body` variable, or
as an output of an operation. Poses are only defined, and spaces only
minted.

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
A name is also the mark that a value is wanted for itself.

**Poses.** `[variables.poses]` A pose is only ever defined: read off a
body's geometry (a face's plane, a carrier's axis or centre, a
vertex's point); moved from another pose by a slide along, or a spin
about, a direction or axis the definition reads; by a construction
over other poses; as a pose of a copy; or as an output of an
operation. So no pose is free and none is defined from nothing. A
construction reads poses as it reads any variable, only those of the
space it is built in, and is in that space; relating two spaces is a
placement's alone ([placement.spaces]).

A pose is a frame known up to its kind's symmetry, a subgroup of the
rigid motions and the same `Subgroup` the mates fold (A11 (1)): a
plane forgets in-plane motion, a flag (a plane, a line in it, and the
side of the line its face lies on, read off a face and one of its
bounding edges, or built as a plane cut by a plane) slide along its
line, a pivot (an axis with a point on it, read off a circular edge, a
cone, a torus, or a face with a vertex) spin about its axis, an axis
slide and spin along itself. There is a kind for each subgroup that is
the symmetry of something a cell determines, alone or with one cell
incident to it, and only for those.
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
and its axis; an instance is a placement whose source is another
document's product body ([placement.world]). An
assertion defines none. A mate or a value is a constraint of a
placement, not an operation.

**Reading is the only dependency.** Nothing consumes anything, so an
operand stays a first-class value after a boolean reads it.

**Bodies and operators.** `[operations.operators]` A body variable is
defined by exactly one of: a construction; a one-input operation (a
fillet, a shell, a split, a mirror); an operator over bodies; or a
placement ([placement.placement]). There are four operators, REFERENCES
DM4's `Union`, `Subtract` and `Intersect` and one more: `∪` union
(n-ary) fuses its operands into one material, `−` cut is binary, `∩`
meet is n-ary, and `⊔` assemble (n-ary, `Assemble`) keeps its operands
as separate pieces. They differ only in what they do where two
operands' material meets: `∪` and `∩` glue there and merge an overlap,
`−` removes, and `⊔` records a touch and reports an overlap as an
interference finding ([assertions]). `−` and `∩` distribute over `⊔`,
so `(p ⊔ q) − t` cuts both and keeps two pieces. `∪` does not absorb
`⊔`: a meeting inside an operand is never changed by an operator above
it ([coincidence.booleans]), so `(bolt ⊔ plate) ∪ bead` fuses the bead
to each part and leaves the bolt–plate meeting a record.

Within one n-ary operator, member order is not precedence: the
written order decides only which description a glued face keeps and
which fold step a refusal names, and an order that refuses where
another builds is a kernel defect. Between operators, order is the
tree. A written list, feature-tree style, is the façade's sugar for
the left-nested tree. Where every two of the bodies touch, a pattern
of fused pairs is writable exactly when it is a cograph: four bodies
fused in a chain, each apart from the ones it does not neighbour, is
the smallest pattern no tree writes. No statement about one contact
overrides the tree.

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
assertion: a construction reads what was written, and a carrier read
in closed form (a pose, a trace) is what was written, not a
measurement.

**The product.** A document has one product slot, which reads a body
in its world space ([placement.world]): `product: p ⊔ bolt ⊔ nut`.
Nothing is derived from which bodies go unread: the slot says what
the product is, and being the product is a use, as being what an
assertion states is. What the person made and nothing uses is a
finding: an operation they inserted none of whose outputs is used,
and a variable they named that nothing uses. What a door minted on
their behalf (an output nobody named, an anonymous variable) is a
byproduct of what it was minted for, never a finding of its own. A
document whose slot reads nothing has an empty product.

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
`union`, `intersect` and `assemble` take a `Bodies`: a family, or its
members spelled as reads; a subtract's tool is one `Body`, so cutting by a family is
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
ring stepped by `turn/N` whose member spans its step meets each
neighbour, the last the first among them, structurally
([coincidence.structural]), and needs none. There
is no list literal: an irregular family is its members written out,
each saying where it is. A definer that refuses at one index refuses
the whole family, naming the index; a read whose index leaves the
range is unresolved and typed, never re-pointed. A member's names are
`Member { (i, j), of }`, keyed by the index variables' ids and the
integers. Copies of one body are built once and mapped; a construction
whose scalar inputs read an index is built per member. A mirror image is not a copy, because a
reflection is not a pose: `Mirror { body, plane }`
is a construction defining a new `Body` in its source's space, the
plane a `Plane` pose whose reads reach that space alone (a face read as
a plane, or a plane constructed from the body's geometry), and every
pose is proper. A symmetric part is `union(body, Mirror { body, plane })`,
one construction whose seam is the plane's face read twice; a chiral
twin elsewhere is a placement of the image, and there the plane carries
no shape.
Linear, circular, grid and bolt-circle patterns and mirroring are the
façade's functions, which write this program; the document stores no
preset, and the GUI's reading of one ("circular, 12 about A") is
recognised from the program for display only.

### Spaces and placement `[placement]`

*From:* [intent.statements], [intrinsic], [one-way].

**A part has no location.** No body has coordinates anyone reads
until a placement relates them to something: a part has no location,
so how two parts relate is always stated explicitly. Every
construction is built in coordinates of its own, which are its space,
and spaces are related only at placement: a feature on a face is a
construction placed into the space the face's body is in, against the
face by mates and values, then combined, and which side of the face
its material lies on is the mates' to say. Only a placement relates
two spaces (FORK-S3P), so a feature at several positions is one body placed
several times, each placement a copy.

**Spaces.** `[placement.spaces]` A **space** is a coordinate system:
the coordinates a construction is written in, the world, or a space
`new_space()` mints ([variables.kinds]). The three are one kind and
differ only in what is written in them. A space holds nothing and
contains nothing, and which space a body is in is derived from the
recipe alone: a construction is in its own space, or in the space of
the poses it reads (a mirror, a profile drawn on a frame), a placed copy in its
placement's target, an operator's result in the space its operands
share, and a one-input operation's result in its input's, so every
body is in exactly one space. Each placement is a copy of its own, so
one source placed twice into one space under two bundles is two bodies
there, at two places. **Only a placement relates two spaces**:
without one, two spaces' coordinates say nothing of each other.

An operator's operands share one space. The recipe decides it, with no
value and no evaluation, as it decides a dimension mismatch: the
façade's builder and the Python binding refuse the call that would
write such a node (`SpaceMismatch`), the GUI does not offer the
gesture, and a file holding such a read loads
(`persist::check::validate_document` does not refuse it): the node
refuses at evaluation, naming both spaces.
An edit that re-targets a placement so that reads of its copy
land in another space leaves those reads unresolved and typed, as
deleting a variable does ([operations]): the document stays valid and
loads, and each such node refuses at evaluation, naming both spaces. A
file edited by hand or merged into that state loads with them marked.

**A placement** `[placement.placement]` is an operation reading a
**source**, a **target** and a bundle of constraints, mates and
values. The source is always a body, of this document or another
document's product (an instance), never a space; the target is a
space. The placement defines one body, a copy of the source in the
target space under the one rigid motion its constraints state, which
carries every pose of the source as the source carries it, and the
source's contact records and census decisions, which stay the
source's. What reads a copy downstream reads a body, never a
placement: a body is the same to its readers whether it was built in
its space, placed there, or placed through other spaces. A
sub-assembly is a body written with `⊔` in its space, placed as many
times as it is wanted, so no placement's data is ever repeated. A
**pick** is a `Body` variable defined as one piece of a copy: it names
the placement and the piece's variable in the source, through the
placements inside the source down to the first variable it reaches,
which is its one spelling. A copy's faces, edges and vertices keep
their names in the source, and an operation reading two copies of one
source keys each operand's names by the read it came through. Like a
selection, a pick authored at two sites is two variables (the GUI
offers the existing one), and a pick whose body the source no longer
has is unresolved, typed, never re-pointed. Two placements of a part
are two copies. Nothing moves a body: a copy is defined by its one
placement and never moved after, and an operation that would move one
is a placement. A pattern is a placement whose reads reach an index
(FORK-PAT, [repetition]): one copy per value, the index entering as a
value or through a mate's target. A placement's mates and values
together set every freedom: a bundle that leaves one unset is refused
at the door, so every placement is pinned and an unwritten freedom
cannot be represented. A body's symmetry pins nothing (a written value
it makes unobservable, a placement's or a pose move's, is reported,
never refused). A placed copy's frame is not a variable: it is the
construction its bundle states, which the coincidence door reads by
what its mates state and replays where they state nothing.

**Constraints are on equal footing.** A placement's mates and values
are on equal footing, and a constraint any of whose equations the
others already fix refuses as an overconstraint, pinned or not,
decided by subgroup algebra (A11 (1)) without measuring. A mate that
would take a freedom a value sets refuses as well. Where two copies
meet beyond what their mates fix is a coincidence like any other,
recorded and linted, and its recourse is an assertion ([assertions]):
a relation the placement can still carry is a mate, of a richer kind
where none says it yet, and one the mates already fix is claimed,
never mated twice.

**Mates.** `[placement.mates]` Each mate equates two poses of one
kind, modulo that kind's symmetry: a pose read off the source's
geometry with a pose read off a body in the target space that does
not read this placement. A mate reads only bodies' geometry, never a
space and never a frame standing for a part's coordinates, and no
space has roles a mate can read. A mate states a physical relation; a
placement whose source touches nothing has none, and its values state
the whole motion. A mate holds no number of its own, and its sense is
a construction (`Flip`) on one side (a flag's `Flip` keeps its side:
the half-turn about the line in its plane perpendicular to the flag's
line); a number a kind's own equation fixes (a standoff along a
plane's normal) is a construction on the target, never a value. Which
copy is defined is which placement reads it, and a pose of another
copy is read as that copy carries it. A mate places and never checks.

**Values.** `[placement.values]` A **value** sets one freedom of a
placement's motion that its mates leave, a slide or a spin, to a
`Length` or `Angle` variable. Where the mates fix a direction (a
plane's normal, an axis, a lone direction; a bundle holds at most one
mate fixing a direction, since a second would state the angle between
them, an equation the first already fixes, so its values have one
chart and a value names no mate), the placement writes an
**orientation**: one of the 24 ways the source's axes lie along the
target's (`x: +x, z: −z`), a discrete kind, the identity unless
written; the gesture writes the one nearest where the copy stands.
The orientation says where the copy's mated direction should point,
and the **least turn** carries that onto the mated direction. Every
value beside the mate is read through it: a **slide** runs along one
of the target space's axes as the least turn carries it, so a slide a
plane mate leaves runs in that plane however the plane is tilted, and
where the face is where the orientation puts it the least turn is the
identity and a slide is a coordinate on the target's own axes; the
one **spin** is the angle about the target's own direction (its
outward normal for a plane), whatever the contact's sense, left after
the least turn, so no reference axis is chosen on either side, a
flip written on either side of a rest turns the spin the same way, and
zero says the two spaces agree, up to the orientation, as far as the mates
allow. Where the source's own mated direction is one of its axes, one
carried axis lands on the mated direction and the mate takes that
slide; otherwise the written slides are admitted only where they span
the plane (or line) the mate leaves, decided once from the source's
geometry. Where the mated direction is opposite to where the
orientation puts it (the orientation written backwards) the least
turn has no answer, and within the band about that one pole the chart
refuses, typed at evaluation and naming the orientation to write, as
a construction refuses at its degenerate values; so no edit that keeps
the face short of a half turn from where the orientation puts it
moves a held value's meaning, or the copy's place on the face. The
orientation and a mate's `Flip` are two facts, the copy's attitude and
the contact's sense; only the pole band refuses. Where no direction is
fixed (no mate, or a lone point mate), there is no orientation: a
slide is a coordinate on the target space's own axes, and the
rotation is a written chain of at most three spins, each about the
target space's x, y or z (through the mated point, for a point mate),
no two in a row about one axis; turns do not commute, so the author
writes the order and the kernel chooses none. A slide is the copy's
origin measured from the target's, a function of the relative pose
alone, so no slide is ordered. Every freedom is set explicitly, as a
written value: zero is always a valid value, a placement whose values
are all zero and which has no mate is the identity, and the façade
writes the constant `0` for each freedom a gesture leaves unnamed. A
bundle with no mate is legal. A space's axes are read only by values,
so setting a part on the world's xy is written as values (`z = 0`, the
two tilts `0`, and the slides and spin the author chooses). A
placement is the only reader of a source's own coordinates, and reads
them only through the freedoms its mates leave.

**The world** `[placement.world]` is a space like any other, the one
every document holds and cannot delete: it sets the coordinates. Its
axes chart the values of placements into it, and it is nothing else.
It has no datum plane, origin or axis as a recipe object: a viewer
may draw its axes or a grid as a display aid, and a gesture that drops
a part on the floor writes values; neither is a recipe object, and no
mate or construction reads the world. Export and the viewer's drawing compose a body into
the coordinates of a space it reaches, the world's or any other shown
space's, through one door that nothing else calls, and what they
produce never re-enters the model. The **product** is the body the
document's product slot reads in the world space ([operations]). One
document enters another as an instance, a placement whose source is
its product body; that body's contact records and census decisions
are carried with it and reported once, as that document's.

A plate with a pocket, a rib and a hole, held by a bolt and a nut:

    base   = extrude(profile, t)                       -- in base.space
    pocket = place pocket_tool {target base.space; plane ≡ base.top (flip);
                                x = px, y = py, spin = 0}
    rib    = place rib_body    {target base.space;
                                plane ≡ (base − pocket).From{pocket}.floor (flip);
                                x = rx, y = 0, spin = 0}
    hole   = place drill       {target base.space; axis ⟂ base.top;
                                x = hx, y = hy}
    part   = fillet((base − pocket) ∪ rib, edges, r) − hole
    p      = place part     {target world; zeros}
    bolt   = place bolt_doc {target world; pivot ≡ p.From{hole}.rim (flip);
                             spin = 0}
    nut    = place nut_doc  {target world; axis ≡ bolt.axis;
                             face ≡ p.bottom (flip); spin = 0}
    product: p ⊔ bolt ⊔ nut

The rib mates to the floor of `base − pocket`, a body in the target
space that does not read the rib, and the `∪` glues it there,
structurally by that mate. The bolt's head on the plate and the nut
under it are each one face pair the `⊔` records, structural by the
mates, and the thread inside the nut is an interference finding that
an assertion quiets. Writing `p ∪ bolt` instead welds the head to the
plate.

**The computing frame.** `[placement.frame]` An operation computes in
a frame that is a function of what it reads and of nothing else,
chosen so its arithmetic is well conditioned near the geometry it
builds, and never in the world's; the frame is keyed with its inputs,
so an edit that leaves the bodies an operation reads where they stand
in their space moves none of its bits ([determinism]). The frame is no
part of the operation's meaning: the body up to that rigid map, its
names and every verdict outside the sliver band are the same in any
frame, and a minted reference direction is a function of the inputs,
not of the axes. Where conditioning does not decide, a construction
computes in its own coordinates and an operation over copies in its
first operand's as the author lists it. The census at a `⊔`
([assertions]) is defined order-free: each pair's verdict is the same
in either operand's frame, or the sliver band refuses.

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
as projections of one construction. A mate is that rung's statement:
the two poses a placement's mate equates, read with the sense it
states, are one construction read twice, wherever the placement
evaluates; a pose a value moves off the mate's target (a standoff) is
not. Being one construction is the equivalence the document's
constructions and mates generate, so a wall each part builds through
the edge of a flag that a flag mate equates is one construction read
over both, and that mate establishes the flush pair the two walls
make. A traced profile is that rung's statement too: a piece drawn
along a face's trace and swept by a motion in the symmetry that face's
construction states (an extrude's wall slides along its generators, a
revolve's band turns about its axis) is tangent to the face along the
swept curve, and one whose frame's plane contains an axis of that
symmetry is tangent at the crossing. The second reads each output
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
constructed: a profile drawn on a frame reads another surface's trace
in its plane and continues tangent to it, and a sweep of it stays
tangent to the surface wherever its motion follows the symmetry the
surface's construction states.

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

**Booleans.** `[coincidence.booleans]` The booleans are the four
operators ([operations.operators]), and a boolean's operands must
already be in one space ([placement.spaces]). Each operator decides
every meeting between its operands at the coincidence door; a meeting
inside one operand is that operand's decision, carried. Cells that
are one construction read twice carry one description, so the door
proves their meeting by its first rung, as it proves any such pair
(two poses a mate equates among them), and measures nothing. Nothing
is decided twice because nothing could come out otherwise. So `A ∪ A`
and `A ∩ A` are `A`, `A − A` is the typed empty result, and `A ⊔ A` is
an interference finding; the halves of a split, whose cut faces both
lie on the split's one plane, meet structurally, so `h1 ⊔ h2` records
the cut and `h1 ∪ h2` glues it; and in `(A ∩ B) ∪ (A ∩ C)` the cells
of `A` both operands carry are glued structurally.

Where its verdicts decide Zero, a union, a cut or a meet glues where
an arm exists for the carrier pair ([topology]'s frontier), keeping
one fixed operand's description for a merged face (which one is
arbitrary; the kernel keeps the first-written operand's, the simplest
fixed rule, where a rule that read values would choose by rounding
between two descriptions within the band of each other), and an assemble
records the touch ([topology.records]); each refuses what falls in the
sliver band. Its result holds no in-band pair, a pair of result cells
whose one census number lies in band. A solid's only shell is its outer
shell and poses no role question; a shell's role among several is read
off a ray aimed from outside the shell at one of its faces, and off the
certified sign of its volume where no ray reads. A ray that passes
within the band of an edge meets it, half a crossing for each face the
edge bounds ([verdicts]), and reads the role as the sign of its count
of crossings; it refuses only on two faces that do not meet whose
crossings lie in band of each other, a pair, and a ray that meets a
corner or a wedge reads nothing. The volume's sign is certified by its
enclosure alone, never held against the band, so how thin a result is
gets judged by its pairs alone. The door's
finished-body gate decides this on the result alone and refuses it as
the operands' ill-conditioning (`Escalated`, recourse: tighten ε or move
the geometry), never silently; a definite finding there is a kernel
defect (`ResultInvalid`).

### Assertions `[assertions]`

*From:* [intent.statements], [intent.checked].

**An assertion checks and never places.** `Assert { measure,
relation, bound }` (`≤`, `≥`, `=`, the bound a variable) states a
relation the author claims and the kernel checks.

**The at-rest census.** The census is each `⊔`'s door: at rest it
examines the meetings between that operator's operands pairwise, and
nothing it finds refuses. An **arrival** is one operand of an operator
at one value of its indices. Between two arrivals the
census decides each pair, once, at the highest-dimensional cells that
touch: two faces meeting over an area are one pair, and a touch of a
cell bounding one of them against a cell bounding the other is part
of that pair where it lies in the closure of their overlap, and leaves
the same contact; elsewhere it is a pair of its own. Any further
coincidence such a meeting implies is a face pair of its own. Two faces on one carrier,
facing the same way, whose boundaries meet while their interiors do
not are a flush pair: no contact, but a coincidence the census infers
from values, so a finding unless established, by a flag mate or by
the replay (two faces' offsets written with one variable), or
asserted, by a `Gap = 0` over the two faces read on their sides. Contact is an `unproven-coincidence` finding
unless it is structural (a mate-placed face is); an overlap of their
material is an interference finding; and a pair the census has no
lane for is a finding too, saying it could not look, so no outcome of
the census is silence. Inside one arrival it decides nothing and reads
nothing: the arrival's verdicts are its source's or its door's, carried
with it. A finding belongs to the `⊔` whose census decided it and is
reported once there, however many times its body is placed or read,
and a `⊔` the product does not read is checked all the same. A contact
or interference finding observes the sign of the gap (CONTACT-DESIGN
C5) between two bodies: a contact is `g = 0` at two cells, an
interference `g < 0` over one connected overlap of their material.

**What quiets a finding.** `[assertions.quiet]` Holding assertions
quiet a finding when they say the same: each reads a `Gap`'s output
directly and admits only values of the finding's sign (`= 0` for a
contact; `≤ b` or `= b` with `b` negative for an interference). A
contact is quiet when such an assertion's `Gap` is taken between the
finding's two cells as its record carries them, at whatever dimension
they are (two faces, a vertex or an edge on a face, an edge on an
edge, two vertices) and wherever they lie, in two arrivals or in two
solids one arrival brought in (an imported file's instances). An
interference is quiet when the assertion's `Gap` is over an opposed
pair of faces of the two bodies, those faces bound the overlap, and
every face bounding it lies between the carriers of an asserted pair.
A `Gap` reads bodies of one space ([placement.spaces]), so an
assertion quiets only findings between bodies of the space it is in.
A contact is a touch on one carrier or on two, opposed, so a tangency
between opposed faces (a rod resting on a plate) is a contact, and its
`Gap` reads each face's material side. A coincidence of poses the
document reads off its cells (a centre on an axis, two axes one or
meeting, a vertex on a split's plane) is quiet under a holding
`Distance = 0` between those poses, read off the row's cells or the
split's tool. Only a relation against zero quiets: a bound is a
tolerance, not the row. An assertion speaks for nothing else, so a new contact or overlap
anywhere else is loud until something says otherwise, and an overlap
the kernel cannot bound is loud and nothing quiets it.

**Families.** Each member of a family is its own arrival, examined
like any other, and an assertion about each of them is written once, as an
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
  name-resolution identity, and inline-of-split, with the identity
  placement it lands flattened into its target, returns the document
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

- **The kernel panics on kernel bugs, and only on kernel bugs.** A state
  that can only be a kernel bug MUST panic, as loudly and as early as it
  is detectable (`unreachable!` / `debug_assert`, the taxonomy below);
  downgrading one to silence, or to a typed error, launders a bug into a
  supported outcome. **No input should be able to lead to a known panic,
  since bad input is not a kernel bug**: every failure a correct kernel
  lets input reach is a typed error. Some input triggers every bug, so a
  check that fails on the kernel's own result reports a bug and panics;
  it is not a refusal of that input. **A panic is therefore never a
  refusal: it reports that a bug has already happened.** Read it as *a
  firing panic is evidence of a bug*, never as *a panic in the source is
  a defect to remove*. The two halves are separate rules over disjoint
  state classes. Every traversal is bounded: never a hang.
  The closure property behind the no-known-panic half: every public mutation
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
- *A site whose firing a correct kernel can reach is not a defect* but
  a precision limit the model controls (row 1, a lever and
  `KERNEL_LIMIT_RECOURSE`) or a case not built yet (row 2,
  `NOT_YET_ENDING`), and it takes that row's ending. A file is input, so
  a document check, or a check at the import door, is row 1.
- *A load-bearing check panics in release.* A re-deriving check that
  ships, because removing it would ship a wrong result (a validate
  over a Boolean's result), is not row 5: row 5's contract (i) is
  that a `debug_assert`'s absence changes nothing shipped. It runs in
  release, and its failing branch is `unreachable!` with row 4's
  message: what the check re-derived and why a correct kernel always
  passes it.
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

- Same build + same inputs → bit-identical outputs. A body's storage
  (the order of its arenas) is part of the input: outputs are not
  promised identical across two storages of one solid, and claims
  across storages are made on the structure, or as computed bounds.
  No hash-map iteration order may influence geometry; parallelism only
  in fixed reduction shapes (addendum below).
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
   Idiom 1, then a *sequential* fold in the walk's own fixed order (the
   order its slots were mapped in), and non-arithmetic combination
   (naming a first refusal, splicing recordings) positional in that
   same order — or, if that fold profiles hot, a fixed-arity block tree
   with a named chunk size and a documented combine order.

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
| `geom` | Analytic + NURBS types, evaluators, closest-point and the other one-carrier readings ([one-way.readings]): a point's foot on a curve (`Curve3::foot`) and whether it lies on the carrier (`Curve3::on_carrier`), and a carrier's rate bounds (`carrier_speed`). Curves and surfaces are two modules of one crate, so the parameterization conventions and the totality/poison policy are stated once |
| `geom-brep` | The B-rep geometry layer: [edges]'s `EdgeDescription`, certified carrier caches, the dihedral classification predicate, Newell face equations, pcurve caches, SSI, the surface-pair dispatch table, the curve-pair meeting reader (the point-on-span reading, composing `geom`'s one-carrier readings with the span's reading of its ends; one total curve-kind-pair table) and the machinery under it (the half-angle ladder and its subdivision, `half_angle`; the depressed quartic's sign ladder, `quartic`), the carrier-pair pose and the curve×surface crossing (D3), certified mass properties, offset surfaces |
| `profile` | 2-D sketch profiles: the PATHS authoring algebra and the profile-program it records, lowering to the `Profile` of verbatim vertices and canonical segments, and its trilean validation |
| `topo` | Arenas, entities, Euler operators, the validation tiers; plane splitting, the boolean engine and its census/contact-record machinery (sibling modules at the crate root), shell/offset surgery, the kernel query seat |
| `sweep` | Solids from validated profiles: extrude, revolve, loft, sweep, tube; the blend family (fillets, chamfers) and its composition surgery |
| `verbs` | The kernel verb vocabulary seat (VERB-SEAT-DESIGN §2): one payload type per run door reifying an operation's parameters as data, the closed `VerbKind` vocabulary they project onto, run dispatch, and the parameter→field flow; a layer guard keeps serde, `Expr`, `StableName` and recipe ids out |
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
- **Cancelation** (`CancelToken`, yielding between nodes/levels and
  before each member of a family; a canceled run returns the completed
  prefix as a typed outcome, a family canceled part-way holding no
  result). Remaining: progress reporting (nothing exists) and in-op
  yield points — a long single boolean or fillet, a union folding a
  family's members among them, is still uninterruptible.
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
from outside the tree, CHECKS DS9) are meant to be pluggable: the
central pipeline should not know which checks are in place, and
findings should not travel through a side channel. Today a report is a
second pure function, `run_checks(document, evaluation, config)`, over
an evaluation that knows nothing of which checks exist; reports are
never persisted and enter no content keys (DS9). What keeps this open is
what the evaluation carries for the checks: each node's value holds the
coincidences its op decided from values (`NodeValue::coincidences`).
Those a contact record cites are part of the body's own validity, but
some (a merge's glue, a split's ON verdict) back no record, and the
`unproven-coincidence` lint is their only reader. Whether that record is
part of the model's output, or lint evidence the pipeline should not be
producing, is the question.

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
  publish rather than a chore (**S65**, `work/predtail/S65.md`, is
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
| `crates/editor-core/README.md` | Ratified (#496, option A′) | Group boolean in the recipe layer, GROUP-BOOLEAN-DESIGN: a union over a family's rigid images of one body, pairs certified apart not judged |
| `crates/editor-core/ASSEMBLY.md` | Ratified (#333) | How editor-core keeps [assemblies], ASSEMBLY-DESIGN A1–A13, AQ1–AQ8: the evaluation seam, the pairing doors, placements as mate bundles into a target space with the world a node (A3), pins, split and inline, the constructive solve, the at-rest check (A5) |
| `crates/topo/README.md` | Ratified (#178, #965) | Contact census & glued contact, CONTACT-DESIGN C1–C8 (the C7 join lane is shipped); at-rest census structural identity, the CENSUS-REST-CLOSURE-DESIGN clauses |
| `crates/mesh/README.md` | Ratified (#5708) | The mesher's sizing rule, `[mesh.sizing]`: each lane meshes as coarse as its own pinned worst-triangle theorem allows, with no safety factor; what kind each other mesh constant is |
| `crates/geom-core/README.md` | Ratified (Ev, 2026-09-05) | The spline layer's pairing rule, SPLINE-DESIGN S1: a span-restricted door takes one structure (a `Span` borrows its `KnotVector`, a window its curve or surface, `SplineCoeffs` its knot vector), so a mismatch is unrepresentable |
| `crates/sweep/README.md` | Ratified (#992) | ARMS-3, ARMS3-DESIGN A3-1…A3-3: the sphere×sphere fillet arm, the valence-4 seam vertex that is not a corner, what a run-out IS; the blend-vocabulary clauses V1–V4 |
| `crates/profile/README.md` | Ratified (V1–V8; enclosing tangency #1210) | Profiles as programs (PROFILES-V2-DESIGN V1–V8); the enclosing (ρ < 0) fillet tangency is permanently unreachable and a radius demanding it refuses typed (ENCLOSING-TANGENCY-DESIGN) |
| `crates/viewer/GUI-DESIGN.md` | Ratified; GUI v1 shipped | GUI architecture G1–G5, GQ1–GQ7: the three-layer split, egui as toolkit, what v1 ships. `crates/viewer/README.md` beside it is the implementation record, which the program maintains itself |
| `docs/ERROR-DESIGN.md` | Ratified (#110); M10 closed 2026-09-13 | Error propagation E1–E12: duals, stackups, the subdivision driver, trichotomy. M10 built it and closed 2026-09-13; the analysis lane is PROPS' and E12's symbolic tier continues as SYM |
| `docs/DUAL-DESIGN.md` | Ratified (#1146) | The Dual contract DL1–DL6: a Dual is tangent transport and never certifies; ContentBits feeds both channels; the delegation rule; poison-vs-widen in certified lanes |
| `docs/PATHS-DESIGN.md` | Ratified (#124) | The PartialPath authoring algebra |
| `docs/LIBRARY-DESIGN.md` | Ratified (#229); program open | Usable-as-a-library L1–L8: façade, Python bindings via the document layer, v2-fronted PATHS, the authoring-ergonomics unit ladder |
| `crates/editor-core/CHECKS.md` | Ratified (2026-10-18) | Checks, DISCIPLINES-DESIGN DS3, DS6, DS8, DS9: no knob changes a body, certified vs heuristic, `error` only where a finding can be quieted, out-of-tree checks |
| `docs/PCURVE-UNIFY-DESIGN.md` | Ratified (#514); executed | Pcurve unification: the conventional edge variants collapse to ONE (surface, `Pcurve`) form, the exact variants kept as certification lanes; `MappedCurve` demotes to an authority record. P-2 residue stays open |
| `docs/RECIPE-DOORS-DESIGN.md` | Ratified (D2–D5) | Recipe doors for the surgery verbs, D2–D5: `Node::Chamfer` beside `Node::Fillet`, `Node::Tube`/`HollowTube`, and `Node::Shell` with its ordered open faces |
| `docs/MIRROR-DESIGN.md` | Ratified (#909); mirror built | Patterns & mirror P1–P6: the chart-handedness convention (u ↦ −u), mirror's own door beside rigid transform, the boundary of A6's equivariance audit |
| `docs/DRAFT-DESIGN.md` | Ratified (#908); unbuilt | Draft DR1–DR6: plane walls only at v1, a certified re-geom pass, the pull-direction selector as a SELECT-DESIGN amendment, survivor naming |
| `docs/SELECT-DESIGN.md` | Ratified | Selection: filters, heterogeneous sets, vanishing entities; the contact-site recourse (§3d) |
| `crates/editor-core/VARIABLES.md` | Ratified | How a variable is represented, VARIABLES-DESIGN VR1–VR9: minted identity and optional name, `Expr` holds no float, `Formula` lowered at the edit door, lifecycle, id-keyed tokens |
| `crates/verbs/README.md` | Ratified (#1388, #1870, #1983, #1997; V1's shape #5457) | The kernel query seat and one verb vocabulary, VERB-SEAT-DESIGN S1–S4, V1–V4, P1–P3: query doors at `topo`, the per-verb declaration (one payload type per door, `VerbKind`), the parameter→field flow |
| `docs/MATE-7-TANGENCY-DESIGN.md` | Ratified | Torus×torus rim tangency; the kissing arm banks on it |
| `crates/editor-core/REFERENCES.md` | Ratified (in-chat 2026-09-04; DM4 amended 2026-09-06; DM7–DM8 2026-09-16) | What a recipe reference may be, DM1–DM8: a sketch on a face placed against its plane, the carrier-kind read, `Node::Part`, the n-ary `Node::Union` |
| `crates/editor-core/IDENTITY.md` | Ratified (in-chat 2026-09-04) | How editor-core keeps [assemblies.pairing] and [assemblies.saving], DI1–DI5: node ids across history, the memo and the store, an evaluation's identity stamp, saving vs forking |
| `scripts/gates/README.md` | Ratified (Ev, 2026-09-06) | The CI gate directory: one home per gate, both CI halves, and why the four grep gates stay greps (the S13 evaluation of lint alternatives) |
| `tools/README.md` | Ratified (Ev, 2026-09-08) | The instrument crates' shared rule, CC1–CC5: a check owed on what a file says goes at the reading boundary and only there (CC1), and speaks in the harness voice (CC5); CC2–CC4 are the per-column admissions table's own |
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
| D4 | [tolerance]; ¶1 is [tolerance.epsilon] with [tolerance.recourse] (the two-tolerance principle and its (i)–(iv)), [tolerance.chordal], [tolerance.measure] and [tolerance.margins]; ¶2 [tolerance.residuals]; ¶3 [tolerance.typed-failure]; ¶4 [tolerance.units]; ¶5 [tolerance.boundary] |
| D5 | [identity] |
| D6 | [units] |
| D7 | [import] |
| D8 | [recipe]; "The recipe keeps the form that was written" is [recipe.forms] |
| D9 | [failure] (the never-panic rule and the bug-vs-invalid-state taxonomy, [failure.taxonomy]); [determinism] (bit-identity, libm, replay with kills, deterministic parallelism, the GPU boundary, equivariance as [determinism.equivariance]); [engineering] (the other engineering conventions) |
| D10 | the principle is [intent]; "Variables" is [variables]; "Operations" is [operations]; "Repetition" is [repetition]; "Spaces and placement" is [placement]; "Coincidence" is [coincidence]; "Booleans" is [coincidence.booleans]; "Assertions" is [assertions]; the clauses it superseded are retired (`docs/doc-ledger/intent-leaves-the-tracker.md`) |
| Q1 | [scalars] and [verdicts] |
| Q2, Q4, Q6, Q7, Q8 (resolved) | [tolerance], [tolerance.units] and [units], [recipe], [determinism], OFFSET-DESIGN O1–O6 |
| Q3 | [open.sketch-solver] |
| Q5 | the `curvo` paragraph under [Crate landscape](#crate-landscape) |
| Q9 | [open.name]; the license is dual MIT OR Apache-2.0 |
| Roadmap | [Milestones](#milestones); its standing outcomes: bit-identity coincidence checking's retirement is dropped as long settled (the `bit-identity-consumer` gate keeps it), and K = 10 is in [verdicts] |
| Banked principles | naming localized to predicate flips → [identity]; content-keyed cache transfer → [determinism]; coincidence is a margined verdict → [verdicts]; the evaluation service generic over `Real` → [scalars]; ε and persistence → [tolerance]; fillet validity → [verdicts]; SSI completeness → [edges]; non-manifold results → [topology]; the total expression sublanguage and bit-exact persisted floats → [recipe]; sketch DOF diagnosis → [open.sketch-solver]; flags → [Beyond the kernel](#beyond-the-kernel-the-usability-gap) |
