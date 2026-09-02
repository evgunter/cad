# SELECT-DESIGN: geometric selectors, the detect/declare protocol, and the GQ7 re-homing

Status: **RATIFIED** (design conversation PR #286, 2026-08-09:
Evan approved the recommendations round 1; GS-Q3 AMENDED round 2 —
the ruled boundary is FUSION, not arity: `find_flush_candidates ->
Vec<FlushFinding>` with both `declare(finding)` and
`declare_all(Vec<FlushFinding>)` acceptable, a fused
detect-and-declare door forbidden permanently (findings must pass
through user-visible hands as values); Evan: "sounds good").
GS-Q1 (sel_* K-census participation), GS-Q2 (convexity
reserved-not-built), GS-Q4 (mixed-Tied refuses), GS-Q5 (this doc),
GS-Q6 (datum-relative position) all as recommended. §1's
exact-vs-decided reframing signed off.

## Grounding (committed; this doc does not re-litigate)

- **The materializer doctrine** (M6-5 freeze, U7 as shipped):
  selection evaluates against ONE evaluation, returns
  `Vec<StableName>`, and the caller STORES it. No live queries in
  recipes; the growth path is `Rebind`. Geometric selectors change
  nothing here.
- **The margins discipline** (DESIGN.md ~:1215-1237): designed
  selection/tie-break rules prefer intrinsic quantities, commute with
  rigid motions and reflections where free, and document residuals
  (the S8 rung-3 precedent). Every numeric decision goes through the
  `k_stats` funnel with a named site.
- **The coincidence ladder** (DESIGN.md ~:1563-1587): coincidence is
  structural or declared, never value-inferred; description-equality
  *detection* is "a diagnostic/affordance only ('these faces coincide
  exactly — declare the relation?'), never semantics". That sentence
  is this design's charter for §4.
- **C4** (CONTACT-DESIGN): declarations are recipe data by stable
  name, verified never trusted, per-class tables, four typed
  failures. `Node::Declare { pairs: Vec<(StableName, StableName)> }`
  is SHIPPED and consumed by `Node::Boolean`'s `declare` input.
- **The #256/#250 precedent**: degenerate coincidence has NO absorb
  arm; the repair menu is declare-it (verified) or move-the-geometry.
  Keep-as-spline is banned; exact analytic geometry has exactly one
  native representation.
- **#214 / the flagged lane**: no new `decide_flagged` site ships
  without a ledger row in `docs/predicate-dimension-audit.md`; the
  census assertion pins the count.

## 1. The predicate vocabulary

**The demand evidence is the fillet-selection alphabet** — the two
demo filters
that stayed hand-written against the kernel body because U7 was
structural-only (`demos/tour/src/diefillet.rs:203-244`), plus the
flush helper's decision triple (`demos/tour/src/booleans.rs:60-118`):

1. *carrier kind of an edge's curve* ("the straight edges of the
   pipped die" — `matches!(carrier, Curve3::Line{..})`);
2. *adjacent-surface-kind pair across an edge* ("the plane/sphere
   rims" — face kind on `he_plus`/`he_minus`);
3. *flush-plane face pairing* (parallel + co/anti-oriented + zero
   offset — the flush helper's triple, §3's detector);
4. *convexity* — NOT actually used: the demo's comment says the kind-pair
   test "stands in for concave rim". Demand is inferred, not
   measured.
5. *position* — no corpus site uses it; it is GQ7's "filter" language
   and general CAD expectation.

**The load-bearing observation: the alphabet splits into EXACT and
DECIDED, and only the DECIDED half is a margins-discipline site.**

- **EXACT (representation facts, no margin, no funnel).** Carrier
  kind and adjacent-kind pairs read the surface/curve enum tag —
  and post-#256 (always-promote; "exact analytic geometry has
  exactly one native representation") the tag IS the semantic kind:
  there is no plane-shaped NURBS hiding from a tag match on the
  main path. These predicates are total, deterministic, trivially
  equivariant (kinds are motion-invariant), and scalar-independent.
  They are NOT decided predicates and must not be dressed as ones —
  minting a fake margin for a tag match would be dimension-laundering
  in the other direction. (Residual, documented: geometry that is
  value-analytic but declared/structured as spline does not tag-match
  — that is #256's intended semantics, not a defect; the recipe said
  spline, the selector believes the recipe.)
- **DECIDED (funnel sites, named, margin-audited).** Flush-pair
  detection (§3's three margins), position comparisons, and — if it
  ships — convexity. Each is a `k_stats::decide` site with a
  `Margin` comparand where the dimension is honest, a static name
  (the `sel_*` prefix, mirroring the funnel's naming convention),
  and a ledger row if any comparand cannot honestly be
  a length (the #214 standing rule applies to selector sites exactly
  as to kernel sites — public API is not a discount lane).

**Position predicates (the equivariance-shaped part).** Raw
world-coordinate filters ("z ≈ 1") violate the
intrinsic-quantities preference: they bake an absolute frame into a
selection rule. The house-consistent form: position predicates are
**relative to a referenced datum** — signed distance to a datum
plane, distance to a datum axis/point — with the datum a
`RecipeNodeId` reference like every other input. This keeps the
selection rule equivariant *as a rule* (move the datum with the
part and the selection commutes), makes the frame explicit recipe
structure, and reuses the existing datum vocabulary. Comparands are
genuine lengths (`Margin` doors, no ledger debt). The comparison
vocabulary is the sign trilean against a stated value: `≈ v` /
`> v` / `< v` at the document ε-band — never a bare float equality.

**Convexity — RESERVED, NOT BUILT (GS-Q2).** No 3D
edge-convexity predicate exists in the kernel (the only shipped
convex/reflex classification is the 2D vertex-sector one in
`boolean/sectors.rs`). A dihedral-sign predicate is a genuinely new
decided predicate: comparand design (the honest margin is not
obviously a length — likely a flagged-lane ledger conversation),
sample-point choice along the edge, behavior on smooth (tangent)
edges, and an equivariance check. Demand evidence is the weakest of
the alphabet (the demo got "concave rim" via kind-pairs). So v1
ships kind, adjacent-kind-pair and datum-relative position, and
convexity keeps only its vocabulary slot — `GeomPred::Convex` /
`Reflex` named and unbuilt.

**Verdict recording** — deferred to ledger GS-Q1 (§7); short form:
selector decisions go through the same funnel, so K-census
participation is automatic and desirable, and the N4 replay pins
are unaffected because materializers run outside evaluation.

## 2. The selector algebra extension

**Shape: the structural `Selector` is unchanged; geometry is a
FILTER at the materializer, not a new pattern leaf.**

The shipped algebra (`editor-core/src/names/select.rs`) is
deliberately union-only: `Selector = union of NamePat`, each
`NamePat` a conjunction of its own fields, matching **on the name
value alone** (`matches(&StableName) -> bool` — pure, no evaluation
in sight). That purity is worth keeping: `NamePat` is reusable
anywhere a name exists (appearance, resolve diagnostics) precisely
because it never needs a body.

A geometric predicate is different in kind: it interrogates the
**entity the name resolves to** in one evaluation. So the extension
is a second stage on the materializer, not a new `NamePat` field:

```text
GeomPred =                       -- a CONJUNCTION of atoms
  | CurveKind(CurveKindSet)      -- exact (edges)
  | SurfaceKind(SurfaceKindSet)  -- exact (faces)
  | AdjacentKinds(SurfaceKindSet, SurfaceKindSet)  -- exact,
      -- unordered pair across an edge (equivariant by symmetry)
  | DatumDistance { datum: RecipeNodeId, cmp: Cmp, value: Expr }
      -- decided (funnel site sel_datum_distance)
  -- reserved, unbuilt: Convex / Reflex (GS-Q2)

select_where<T: Decide>(
    ev: &Evaluation<T>, node: RecipeNodeId,
    sel: &Selector, geom: &[GeomPred],
    params: &ParamEnv<T>, tol: Tol,
) -> Result<Vec<StableName>, SelectRefusal>
```

Structural narrows the names; geometric filters the survivors by
resolving each through the name table to its entity and testing the
atoms as a conjunction. Union-of-conjunctions stays the whole
algebra (run `select_where` twice and concatenate for a geometric
union — same posture as U7: growth is additive later, nothing
measured needs more). The result is sorted/deduped canonical order,
stored by the caller, frozen thereafter — the materializer doctrine
verbatim, one new door beside `select`.

**Why `Result` where `select` is infallible**: decided predicates
introduce two honesty obligations the structural matcher never had.

- **Indeterminate is a refusal, not a filter outcome.** An in-band
  margin on a candidate (its datum-distance sits in the ambiguity
  band) must not silently include OR exclude it — that would be a
  razor-thin selection cliff, exactly what the funnel exists to
  forbid. `select_where` refuses with the typed
  `SelectRefusal::InBand { name, predicate, .. }` listing the
  in-band candidates. (Exact atoms alone can never refuse; a
  purely-exact filter is total, and the type honestly says so only
  in docs — collapsing to two doors was considered and rejected:
  one door, one contract.)
- **Tied names meet geometry as a trilean.** The structural `select`
  passes `Entry::Tied` names through (the tie is the name table's
  fact; referencing one refuses downstream as `Ambiguous`). A
  geometric filter must evaluate ALL tied candidates: all match ⇒
  the name is included (still tied — downstream still owns the
  refusal); none match ⇒ excluded; mixed ⇒
  `SelectRefusal::TiedDisagrees { name, .. }` — the filter cannot
  half-select a name, and silence in either direction lies.

**Tie-breaks and margins in the S8 sense**: `GeomPred` has no
ordering, no "nearest", no "first" — every atom is a pointwise
predicate, so the S8 selection-ladder problem (choosing ONE of
several) does not arise in v1; selection returns the full matching
set and the freeze does the rest. Any future "nearest entity to X"
door is where the S8 ladder precedent (intrinsic ordering, Tied on
in-band ties, documented residual) would bind; recorded here so it
is designed, not improvised.

## 3. The detect / declare / menu protocol

**The census finding this section retires (LB11)**: the demo
helper `flush_declarations` infers declarations from values —
C4's forbidden pattern, legal only as fixture code, and its three
decision sites (`demo_flush_parallel` bare sine, `demo_flush_orient`
bare cosine, `demo_flush_offset` proper `Margin`) are the flagged
bare-gate family the library version must not replicate. The honest
library form is three separated pieces:

*(Status, SEAT-3: retired in fact as well as in principle. The
body-seat detector below gave the demo and test-common declarers a
library door to call, both hand declarers are gone with their six
flagged sites, and the fixture-twin disposition recorded in (b) and
§6 has closed.)*

**(a) Detect — findings, never declarations.**

```text
-- the document seat
find_flush_candidates<T: Decide>(
    ev: &Evaluation<T>, a: RecipeNodeId, b: RecipeNodeId,
    tol: Tol,
) -> Result<Vec<FlushFinding<(StableName, StableName)>>, SelectRefusal>

-- the body seat (SEAT-3), the same finding over the other pair
topo::flush::find_flush_candidates<T: Decide>(
    a: &Body<T>, b: &Body<T>, tol: Tol,
) -> Result<Vec<FlushFinding<(FaceKey, FaceKey)>>, FlushRefusal>

FlushFinding<P> {
    pair: P,                          -- the SEAT's pair vocabulary
    class: ContactClass,              -- Rest, in v1
    evidence: ...,                    -- the definite margins found
}
```

**The pair field is the seat's vocabulary (amended, SEAT-3):**
findings are **names at the document door, keys at the body door, one
verifier under both**. The document detector above answers
`FlushFinding<(StableName, StableName)>` — names, never keys, which is
G1 for everything above the kernel line; the kernel's own detector
(`topo::flush::find_flush_candidates(&Body, &Body, tol)`, the
producer `BooleanDeclarations` lacked) answers
`FlushFinding<(FaceKey, FaceKey)>`, because arena keys ARE the body
seat's vocabulary and a stable name cannot be spelled below G1. This
does not weaken the anti-twin rule of (b) — it is that rule one layer
down: both doors enumerate over the SAME per-pair verify rung, so
neither seat can report a finding the other's verifier would refuse,
and the type is literally one type over two pair vocabularies rather
than two types kept in step.

A finding is a REPORT: "this cross-body face pair would verify as
`Rest` if declared." It glues nothing, changes no topology, and is
never stored in a recipe — it is the coincidence ladder's rung-(b)
affordance ("these faces coincide exactly — declare the relation?")
given an API. Indeterminate margins inside the detector follow §2:
an in-band pair is refused into the result honestly
(`FlushFinding` is only ever definite; in-band pairs surface in the
`SelectRefusal` alternative), never silently dropped.

**(b) Detector = the C4 verifier run in candidate-generation mode
(the anti-twin rule).** The detector does NOT get
its own predicate triple. C4's `Rest` verify table already names
the ladder (the kind-generalized `oriented_plane_eq` — shipped,
`topo/src/boolean/plane_eq.rs` — plus sense opposition and overlap);
the detector enumerates candidate pairs and reaches the SAME verdict
ladder the declared rung will later verify with — not by calling the
identical entry point (SEAT-3 measured that: detection enters at
`flush_pair_relation`, verify-at-use at `carrier_pair_relation`), but
because `carrier_eq`'s `(Plane, Plane)` arm delegates to the very
`oriented_plane_eq_verdict` the detection door wraps. One verdict
function, one set of `decide` sites, two spellings of the same three
inputs. Consequences: detect-then-declare can never disagree with
verify-at-use, and there is no second implementation to keep in step
by hand; the flagged bare-gate family is retired on the
public path rather than promoted — and, since SEAT-3 gave the body
seat the same door, retired at the fixtures too: the demo and
test-common declarers call the library rather than mirroring it, so
LB11's "fixture twins stay put" has closed rather than been waived;
and no new ledger rows are
minted for detection — the interpretation-discipline contract is
"the detector interprets nothing the verifier doesn't".

**(c) Declare — sugar over the shipped vocabulary.** `Node::Declare
{ pairs }` exists; the sugar is a document-layer convenience that
takes explicitly-passed pairs and appends/creates a Declare node
wired into the consuming Boolean. Thin by design. The arity
question is GS-Q3, RULED: the boundary is FUSION, not arity. Both
`declare(finding)` and
`declare_all(findings: Vec<FlushFinding>)` ship; what stays
forbidden permanently is a fused detect-and-declare door (the demo
helper's original shape), because the enforceable intent-recording
property
is that findings pass through user-visible hands AS VALUES — an
arity restriction is defeated by a two-line for-loop, while the
no-fusion rule is structural. C4's verify-at-use backstops lies
either way (`ContactContradicted`, never silent).

**(d) The refusal menu — two arms, ratified shape.** The boolean's
`UndeclaredContact { finding }` refusal carries the SAME
`FlushFinding` the detector produces (one vocabulary end-to-end),
and its recourse menu has exactly two arms: **declare the named
class** (→ (c)) or **move the geometry**. NO absorb arm — the #256
ruling ("degenerate coincidence has no 'absorb it' arm") applied to
contact, matching C4's failure table verbatim. The error message
renders the finding; the GUI renders the same finding as its
declare-affordance dialog (§4's one-type rule at work).

**The v1 detector is flush/`Rest` planes** — the only
demand-evidenced case (the flush helper, the boolean test suites,
the M4 declarer), and `Rest`'s verify ladder is the most mature.
`Tangent` and `Fit` findings reuse the same `FlushFinding`/
`ContactClass` shape when their demand arrives; the type is built
for that from day one (the `class` field, not a `flush: bool`).

**(e) Where the vocabulary lives, and what M9-1 changed.** The
`ContactClass` this section names is the KERNEL's
(`topo::contact::ContactClass`, M9-1 PR-1): the boolean's own
`UndeclaredContact` / `ContactContradicted` refusals must carry the
same words the detector produces, and `topo` cannot depend on
`editor-core`, so the enum is defined lowest and re-exported upward
through `editor_core::names::flush` → `editor_core` → `pncad::select`
→ the prelude. There is no parallel enum at any level, and the same
door re-exports the rest of the vocabulary a rendered refusal needs
(`CONTACT_RECOURSE`, `FIT_DEFERRAL`, `ContactVerdict`,
`ContactRefusal`, `DeclaredContact`) so a message quotes the kernel's
sentence rather than paraphrasing it.

SEAT-3 put the FINDING vocabulary on the same chain and for the same
reason: `FlushFinding`, `FlushEvidence` and `FlushRung` are defined in
`topo::flush` beside the detector that produces them, and
`editor_core::names::flush` re-exports them (its `FlushFinding` is the
kernel type at this seat's pair vocabulary — the amendment in (a)), so
every path above is spelled as it was. The kernel doors themselves
reach the prelude as a MODULE (`pub use topo::flush;`, the `topo::query`
precedent): all three names — `find_flush_candidates`, `declare`,
`declare_all` — exist at both seats, answering names from an evaluation
above and keys from a body below, and a prelude must not make one
shadow the other.

M9-1 PR-2 closed the gap between (a) and (c): `Node::Declare`'s pairs
each carry their class, so the class a finding reports is the class
the declaration records and the class the boolean verifies against —
one vocabulary end-to-end, now as data and not only as a type. Before
that change `declare_node` dropped `finding.class` on the floor, which
was invisible while `Rest` was the only class and would have become a
silent mis-verification the moment a second one existed.

**What the detector still does NOT do.** The `Tangent` arm of
`find_flush_candidates` is not built. Detecting a tangency needs the
contact LOCUS — the kernel's `Tangent` table verifies *along* a curve
— and the detector holds two faces from two unevaluated-together
bodies with no shared edge between them. Supplying the locus in closed
form is possible for exactly the certified-lane configurations (a
plane and a cylinder tangent along a ruling; parallel cylinders), and
that is geometry, so it belongs beside `carrier_pair_relation` in the
kernel rather than in the recipe layer. Recorded here as the named
next step rather than approximated: a detector that reported tangency
candidates without a locus would be reporting something the verifier
cannot check.

Nor does it report a CURVED conformal (`Rest`) pair, and that one is
a scope decision rather than a missing capability — measured at
SEAT-3 and recorded so the next unit does not re-derive it. The
`Rest` verify ladder covers the carrier inventory today
(`carrier_pair_relation`: plane, sphere, cylinder, torus), and asked
in its DETECTOR posture it already answers a cylindrical cosurface
pair with the same "would verify if declared" encoding the planar
door answers a flush plane pair with
(`demos/tour/src/twopeg.rs`'s `seat3_measurements` runs both
postures on the peg/bore mate). So widening the detector is a door
swap — `flush_pair_relation` → `carrier_pair_relation`, with the
carrier verdict/refusal in place of the plane one — and no verify
table moves. What it is NOT is free: it widens both seats at once
(the document detector's answers change), and every caller of the
demo helper that must keep REFUSING on a curved contact
(`demos/tour/src/lily.rs`'s stem glue) is downstream of it. Filed as
**#1537**, which carries the one-identifier door swap together with
the re-baselining it forces on the scenes downstream — the unit that
takes it decides both halves, or neither.

## 4. GQ7 re-homing

Per Evan's LB7-note ruling ("a bunch of general-usefulness stuff
got originally mentioned in GUI-DESIGN even though it's more
broadly applicable" — the GUI becomes a consumer, not the owner):

**Moves to the library docs** (this document, ratified, becomes
`docs/SELECT-DESIGN.md` — placement is GS-Q5):

- *Selection filters* (GQ7's "selection filters" clause): they are
  §§1-2 of this design — library surface with the GUI as one
  caller.
- *Heterogeneous selection sets as values*: a selection is
  `Vec<StableName>` and `StableName` already spans entity kinds —
  the library type IS the heterogeneous set; nothing GUI-specific
  remains in the value.
- *Survive-the-vanishing-entity semantics*: already owned by GQ4 /
  the naming doc's resolution-failure semantics (typed refusal on a
  dead name, `Rebind` as the growth path); GQ7's clause becomes a
  cross-reference, not a GUI decision.

**Stays in GUI-DESIGN** (a slimmed GQ7, still deferred to
sketcher/tree design time):

- Multi-select UX: click/drag/modifier mechanics, hover, filter
  *presentation* (which filters are offered where, pick-priority when
  a click hits several entities).
- *Selection does not participate in document history*: undo never
  changes what is selected. Purely a GUI-state convention (display
  layer, G1 layer-3) — the library never sees it because…

**The one-type rule.** A GUI selection is the SAME
value as a recipe reference: `Vec<StableName>` — the exact type the
structural materializer returns, `Node::Fillet`'s selection stores,
and `Node::Declare` pairs are built from. G3's "selection feeds the
existing edit doors" is this rule stated from the GUI side: click →
ID-buffer hit → key→name inversion (U7's doors) → `StableName`,
and from that point the GUI is indistinguishable from a library
caller. No parallel "GUI selection object", no conversion layer,
no second staleness story.

## 5. What this design does not depend on

**No representation dependency.** Selectors interrogate EVALUATED
bodies through the name table; nothing here touches profile
representation, the persisted schema, or Expr binding.
`Node::Declare` is shipped vocabulary, so §3's sugar adds no schema
change.

**The datum-distance predicate takes an `Expr` value**, never a bare
float: a selection rule written against a named document parameter
is the whole point of the value type. Unit storage rides along
wherever it exists; a plain `Expr` is correct either way.

## 6. Out of scope, recorded

- **The declared-offset relation** (derived table = base + stated
  deltas, LIBRARY-DESIGN U6):
  expression-layer / Expr-shared-subtree territory, re-homed to
  post-SWITCH-E per LB11(c). Noted so this doc is the pointer trail.
- **"Nearest/first entity" ordering selectors**: §2's recorded
  future S8-ladder site; not v1.
- **`Tangent`/`Fit` detectors**: the finding type reserves the slot
  (§3); demand decides when.
- **Fixture twins**: CLOSED at SEAT-3, not carried. LB11's ruling
  ("stay put") held while no library door existed to replace them;
  the body-seat detector is that door, so both declarers and their
  six flagged sites are gone.

## 7. Question ledger — the rulings and their grounds

**GS-Q1 — Selector-predicate decisions DO enter the K census**, via
`decide` verbatim, with the `sel_*` naming convention doing the
separation. Ground: selector margins are real topology-adjacent
margins, and their ε-band behaviour is exactly what the K telemetry
wants more of; a second funnel would be machinery without a
customer. The N4 replay concern dissolves because verdict logs are
installed by tests around specific operations and materializers run
OUTSIDE evaluation, so no shipped pin sees selector verdicts unless
a test asks. Honest cost, ruled acceptable: a selector query between
two evaluations perturbs a thread-local verdict log spanning both —
logs bracket operations, not sessions, which is how every existing
pin already uses them.

**GS-Q2 — Convexity does NOT ship in v1: the slot is reserved, not
built.** Ground: no kernel predicate exists (only the 2D
vertex-sector convex/reflex classifier), comparand design is
genuinely open (dihedral sign at a sample — likely flagged-lane),
and the demand evidence is a comment rather than a call site, since
the demo's kind-pairs covered the real case. A wrong margin design
shipped into public API costs far more than a follow-up unit.

**GS-Q3 — The detect/declare boundary is FUSION, not arity.** The
apparent tension is that a detector returns finding objects that
look verdict-shaped; the resolution is that LB7's line forbids
values becoming TOPOLOGY without a structural/declared rung, and the
coincidence ladder itself blesses detection as "a diagnostic/
affordance only". A `FlushFinding` is a value about values; only
`Node::Declare` crosses the line, and C4 polices that crossing. So
both `declare(finding)` and `declare_all(findings)` ship, and a
fused detect-and-declare door is forbidden permanently: the
enforceable property is that findings pass through user-visible
hands AS VALUES. A per-finding-only restriction was considered and
dropped — a two-line user loop defeats it, so it penalizes the
legitimate many-pair case without preventing anything.

**GS-Q4 — Tied names meet geometric filters as a trilean** (§2:
all-match include / none-match exclude / mixed refuse). Ground: it
is Q1's shape applied to name-level ambiguity, and mixed-tie
refusals name the entities, which is actionable. Excluding Tied
names outright was rejected as the staleness-shaped sin — it
silently shrinks results.

**GS-Q5 — The ratified text lives here**, as its own
`docs/SELECT-DESIGN.md`, rather than as a LIBRARY-DESIGN section:
three consumers (library, GUI, contact) cross-reference it, matching
the CONTACT-DESIGN precedent for a cross-cutting concern, and
LIBRARY-DESIGN stays the program/ladder doc.

**GS-Q6 — Position predicates stay datum-relative only**; no
world-frame convenience door. Ground: the origin datum exists in
every document so the explicitness costs one argument, and the
equivariance discipline says selection rules must not privilege a
frame silently. Cheap to relax later, impossible to retract.
