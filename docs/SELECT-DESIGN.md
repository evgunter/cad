# SELECT-DESIGN: geometric selectors and the GQ7 re-homing

Status: **RATIFIED** (design conversation PR #286, 2026-08-09: Ev
approved the recommendations round 1; Ev: "sounds good"). GS-Q3
retired with D10's declarations (§3). GS-Q1 (sel_* K-census
participation), GS-Q2 (convexity reserved-not-built), GS-Q4 (mixed-
Tied refuses), GS-Q5 (this doc), GS-Q6 (datum-relative position) all
as recommended. §1's exact-vs-decided reframing signed off.

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
- **The coincidence door** (DESIGN.md D10, Coincidence): a margin
  decided Zero glues, and structure is decided at the door; a
  selector decides no coincidence.
- **The #256/#250 precedent**: degenerate coincidence has NO absorb
  arm; the repair is to move the geometry.
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
tour's flush pairing. The alphabet records what the demand was, not
where the code is:

1. *carrier kind of an edge's curve* ("the straight edges of the
   pipped die" — `matches!(carrier, Curve3::Line{..})`);
2. *adjacent-surface-kind pair across an edge* ("the plane/sphere
   rims" — face kind on `he_plus`/`he_minus`);
3. *cosurface face pairing* (alignment + co/anti-orientation + zero
   coincidence margin — spelled per carrier kind, and planar in the
   hand triple this alphabet was read off). A boolean decides it now,
   at its glue door (`carrier_pair_relation`), so no selector does;
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
      -- decided (funnel site sel_plane_distance)
  -- reserved, unbuilt: Convex / Reflex (GS-Q2)

select_where<T: Decide>(
    ev: &Evaluation<T>, node: RecipeNodeId,
    sel: &Selector, geom: &[GeomPred],
    params: &VarEnv<T>, tol: Tol,
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

## 3. No detect / declare protocol

A boolean glues every coincidence its margins decide Zero and refuses
the sliver band (D10, Booleans), so there is nothing for an author to
declare and nothing for a detector to offer: no boolean takes a
declaration, and no refusal offers one. A
coincidence a boolean infers from values is recorded at the coincidence
door and checked by the `unproven-coincidence` lint (D10, Coincidence).
An in-band refusal's recourse is the geometry, and the tolerance where
the margin gives one (D4 ¶1 (i)).

## 4. GQ7 re-homing

Per Ev's LB7-note ruling ("a bunch of general-usefulness stuff
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
structural materializer returns, a selection definition stores (the
`names` of the `Select { body, names }` a fillet reads). G3's "selection feeds the
existing edit doors" is this rule stated from the GUI side: click →
ID-buffer hit → key→name inversion (U7's doors) → `StableName`,
and from that point the GUI is indistinguishable from a library
caller. No parallel "GUI selection object", no conversion layer,
no second staleness story.

## 5. What this design does not depend on

**No representation dependency.** Selectors interrogate EVALUATED
bodies through the name table; nothing here touches profile
representation, the persisted schema, or Expr binding.

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

**GS-Q3** retired with the detect/declare protocol (§3).

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
