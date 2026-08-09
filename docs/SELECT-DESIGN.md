# SELECT-DESIGN (draft): geometric selectors, the detect/declare protocol, and the GQ7 re-homing

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

## 1. The predicate vocabulary (PROPOSED FIRM core; convexity OPEN)

**The demand evidence is the P10 alphabet** — the two demo filters
that stayed hand-written against the kernel body because U7 was
structural-only (`demos/tour/src/diefillet.rs:203-244`), plus the P9
flush helper's decision triple (`demos/tour/src/booleans.rs:60-118`):

1. *carrier kind of an edge's curve* ("the straight edges of the
   pipped die" — `matches!(carrier, Curve3::Line{..})`);
2. *adjacent-surface-kind pair across an edge* ("the plane/sphere
   rims" — face kind on `he_plus`/`he_minus`);
3. *flush-plane face pairing* (parallel + co/anti-oriented + zero
   offset — the P9 triple, §3's detector);
4. *convexity* — NOT actually used: P10's comment says the kind-pair
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
  (`sel_*` prefix proposed, mirroring the funnel's naming
  convention), and a ledger row if any comparand cannot honestly be
  a length (the #214 standing rule applies to selector sites exactly
  as to kernel sites — public API is not a discount lane).

**Position predicates (PROPOSED FIRM form, the equivariance-shaped
part).** Raw world-coordinate filters ("z ≈ 1") violate the
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

**Convexity (OPEN — recommendation: defer out of v1).** No 3D
edge-convexity predicate exists in the kernel (the only shipped
convex/reflex classification is the 2D vertex-sector one in
`boolean/sectors.rs`). A dihedral-sign predicate is a genuinely new
decided predicate: comparand design (the honest margin is not
obviously a length — likely a flagged-lane ledger conversation),
sample-point choice along the edge, behavior on smooth (tangent)
edges, and an equivariance check. Demand evidence is the weakest of
the alphabet (P10 got "concave rim" via kind-pairs). Recommend:
ship kind, adjacent-kind-pair, and datum-relative position in v1;
bank convexity with a named design note so the vocabulary slot is
reserved (`GeomPred::Convex`/`Reflex` naming reserved, unbuilt).

**Verdict recording** — deferred to ledger GS-Q1 (§7); short form:
selector decisions go through the same funnel, so K-census
participation is automatic and desirable, and the N4 replay pins
are unaffected because materializers run outside evaluation.

## 2. The selector algebra extension (PROPOSED FIRM)

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

## 3. The detect / declare / menu protocol (PROPOSED FIRM shape; sugar arity OPEN)

**The census finding this section retires (LB11)**: the P9 demo
helper `flush_declarations` infers declarations from values —
C4's forbidden pattern, legal only as fixture code, and its three
decision sites (`demo_flush_parallel` bare sine, `demo_flush_orient`
bare cosine, `demo_flush_offset` proper `Margin`) are the flagged
bare-gate family the library version must not replicate. The honest
library form is three separated pieces:

**(a) Detect — findings, never declarations.**

```text
find_flush_candidates<T: Decide>(
    ev: &Evaluation<T>, a: RecipeNodeId, b: RecipeNodeId,
) -> Result<Vec<FlushFinding>, SelectRefusal>

FlushFinding {
    pair: (StableName, StableName),   -- names, never keys (G1)
    class: ContactClass,              -- Rest, in v1
    evidence: ...,                    -- the definite margins found
}
```

A finding is a REPORT: "this cross-body face pair would verify as
`Rest` if declared." It glues nothing, changes no topology, and is
never stored in a recipe — it is the coincidence ladder's rung-(b)
affordance ("these faces coincide exactly — declare the relation?")
given an API. Indeterminate margins inside the detector follow §2:
an in-band pair is refused into the result honestly
(`FlushFinding` is only ever definite; in-band pairs surface in the
`SelectRefusal` alternative), never silently dropped.

**(b) Detector = the C4 verifier run in candidate-generation mode
(PROPOSED FIRM — the anti-twin rule).** The detector does NOT get
its own predicate triple. C4's `Rest` verify table already names
the ladder (the kind-generalized `oriented_plane_eq` — shipped,
`topo/src/boolean/plane_eq.rs` — plus sense opposition and overlap);
the detector enumerates candidate pairs and asks the SAME doors the
declared rung will later verify with. Consequences: detect-then-
declare can never disagree with verify-at-use (no twin drift — the
demo twins' "kept in step BY HAND" comment in `eval/wire.rs` is the
warning label); the flagged bare-gate family is retired on the
public path rather than promoted (the demo fixture keeps its twins,
per LB11 "fixture twins stay put"); and no new ledger rows are
minted for detection — the interpretation-discipline contract is
"the detector interprets nothing the verifier doesn't".

**(c) Declare — sugar over the shipped vocabulary.** `Node::Declare
{ pairs }` exists; the sugar is a document-layer convenience that
takes explicitly-passed pairs and appends/creates a Declare node
wired into the consuming Boolean. Thin by design. The arity
question is GS-Q3 — RULED (round 2, amended): the boundary is
FUSION, not arity. Both `declare(finding)` and
`declare_all(findings: Vec<FlushFinding>)` ship; what stays
forbidden permanently is a fused detect-and-declare door (P9's
original shape), because the enforceable intent-recording property
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

**What ships first**: flush/`Rest` planes is the whole v1 detector —
it is the only demand-evidenced case (P9, the boolean test suites,
the M4 declarer), and `Rest`'s verify ladder is the most mature.
`Tangent` and `Fit` findings reuse the same `FlushFinding`/
`ContactClass` shape when their demand arrives; the type is built
for that from day one (the `class` field, not a `flush: bool`).

## 4. GQ7 re-homing (PROPOSED FIRM)

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

**The one-type rule (PROPOSED FIRM).** A GUI selection is the SAME
value as a recipe reference: `Vec<StableName>` — the exact type the
structural materializer returns, `Node::Fillet`'s selection stores,
and `Node::Declare` pairs are built from. G3's "selection feeds the
existing edit doors" is this rule stated from the GUI side: click →
ID-buffer hit → key→name inversion (U7's doors) → `StableName`,
and from that point the GUI is indistinguishable from a library
caller. No parallel "GUI selection object", no conversion layer,
no second staleness story.

## 5. Sequencing and sizing

**No SWITCH dependency.** Selectors interrogate EVALUATED bodies
through the name table; nothing here touches profile representation,
schema v4, or Expr binding. `Node::Declare` is shipped vocabulary,
so §3's sugar adds no schema change. The one soft ordering:
implementation extends `names/select.rs` and `pncad::select`, which
U7's R2/fix-pass is still churning — start after U7's merge settles
(days, not units).

**Position in the ladder**: parallel with SWITCH-P/E, before U9 —
U9 (Python bindings) wants this surface bound once, not rebound
(the same reason U7 preceded it), and the GUI's G3 minimum consumes
§4's one-type rule. The datum-distance predicate takes an `Expr`
value; if U8b's unit storage lands first it inherits units for
free, but a plain `Expr` is correct either way — soft, not a gate.

**Sizing** (house scale): the whole design is **L, staged as two
PRs / one A/B unit**:
- PR-1 (**M**): `GeomPred` + `select_where` + Tied/in-band refusals
  + pncad doors + tour demo rework (P10's hand-written filters
  become the acceptance evidence — diefillet's two filters rewrite
  to one `select_where` call each).
- PR-2 (**M**): `find_flush_candidates` on the C4 verify ladder +
  declare sugar + the `UndeclaredContact` menu carrying
  `FlushFinding` + P9 demo rework. (If the verifier needs
  refactoring to expose candidate-generation mode, PR-2 leans L —
  the named spec risk.)

## 6. Out of scope, recorded

- **P5's declared-offset** (derived table = base + stated deltas):
  expression-layer / Expr-shared-subtree territory, re-homed to
  post-SWITCH-E per LB11(c). Noted so this doc is the pointer trail.
- **"Nearest/first entity" ordering selectors**: §2's recorded
  future S8-ladder site; not v1.
- **`Tangent`/`Fit` detectors**: the finding type reserves the slot
  (§3); demand decides when.
- **Fixture twins**: the demo/test declarers stay as they are —
  LB11's ruling; the ledger rows continue to document them.

## 7. Question ledger

**GS-Q1 — Do selector-predicate decisions enter the K census?**
The funnel records automatically (one `Cell` write; `MarginSample`s
at Probe; `Verdict` pushes only when a test installs a log).
*Alternatives*: (a) selectors use `decide` verbatim — decisions are
K-census participants; (b) a separate `sel_*` recording lane; (c)
suppress recording for selector queries. *Recommendation: (a),
with the naming convention doing the separation.* Selector margins
are real topology-adjacent margins — their ε-band behavior is
exactly what the K telemetry wants more of; a second funnel is
machinery without a customer. The N4 replay concern dissolves on
inspection: verdict logs are installed by tests around specific
operations, and materializers run OUTSIDE evaluation, so no shipped
pin sees selector verdicts unless a test asks to (and the `sel_*`
prefix lets any consumer filter). The honest cost of (a): a
selector query between two evaluations perturbs a thread-local
verdict log spanning both — rule it as "logs bracket operations,
not sessions", which is how every existing pin already uses them.

**GS-Q2 — Does convexity ship in v1?** No kernel predicate exists
(checked: only the 2D vertex-sector convex/reflex classifier);
comparand design is genuinely open (dihedral sign at a sample —
likely flagged-lane, needs a ledger conversation); demand evidence
is a comment, not a call site. *Alternatives*: build it now (the
alphabet feels complete; fillet users say "concave edges") vs
reserve the slot. *Recommendation: reserve, don't build.* The P10
evidence shows kind-pairs covered the real case; a wrong margin
design shipped into public API is much more expensive than a
follow-up unit. Reserving the enum name keeps the door visibly
open.

**GS-Q3 — Detector findings vs LB7's "values never verdicts" line —
and how much declare-sugar is legal?** The apparent tension: a
detector RETURNS finding objects that look verdict-shaped. The
resolution this draft takes: the line's real content is that values
never become TOPOLOGY without a structural/declared rung — and the
ladder itself blesses detection as "a diagnostic/affordance only".
A `FlushFinding` is a value about values; only `Node::Declare`
(explicit, recipe-recorded, verified-at-use) crosses the line, and
C4 polices that crossing. What KEEPS this honest is the arity rule:
*Alternatives*: (a) per-finding declare sugar only; (b) also ship
`declare_all(findings)`. *Recommendation: (a).* A blanket
detect→declare composition in the library reconstructs the P9
fixture pattern with extra steps — value-inferred declaration
laundered through an API seam. Per-pair enumeration at the call
site is what records intent ("value equality is not evidence of
intent" — the ladder, verbatim). Fixtures keep their loops; the
library asks the author to point at each pair once. This is the
section I most want Evan's read on: (b) is genuinely convenient for
imported/pattern-heavy models with dozens of honest flush pairs,
and one could argue C4's verify-at-use makes (b) safe enough. I
think (a) is right for v1 because widening later is additive.

**GS-Q4 — Tied-name × geometric-filter semantics.** §2 proposes the
trilean (all-match include / none-match exclude / mixed refuse).
*Alternative*: exclude Tied names from geometric selection entirely
(simpler, but silently shrinks results — the staleness-shaped sin).
*Recommendation: the trilean* — it is Q1's shape applied to
name-level ambiguity, and mixed-tie refusal messages name the
entities, which is actionable.

**GS-Q5 — Where does the ratified text live?** *Alternatives*: (a)
a new `docs/SELECT-DESIGN.md` (this file, promoted); (b) a section
in LIBRARY-DESIGN.md. *Recommendation: (a)* — three consumers
(library, GUI, contact) already cross-reference it, matching the
CONTACT-DESIGN precedent for a cross-cutting concern; LIBRARY-
DESIGN stays the program/ladder doc.

**GS-Q6 — Datum-relative-only position: too strict?** §1 forbids
raw world-coordinate filters. A user with one part and no datum
must reference the origin datum explicitly. *Alternative*: allow a
world-frame convenience door. *Recommendation: keep datum-only* —
the origin datum exists in every document, the explicitness is one
argument, and the equivariance discipline says selection rules
should not privilege a frame silently. Cheap to relax later;
impossible to retract.
