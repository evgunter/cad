# Persistent naming & selection stability (pre-M4 design doc)

Status: **RATIFIED** (Ev, PR #74, 2026-07-23 — N1–N7 as proposed,
including keeping F7 per the N3 analysis; the elaboration rounds on
merge policy and N2 alternatives are in the PR conversation).
Grounding:
`references/notes/naming-constraint-inventory.md` (every ratified
constraint, quoted, with the five tensions T1–T5 this doc must
resolve) and `references/notes/persistent-naming-litreview.md`
(15-source mechanism review; taxonomy (a)–(d); failure modes 1–7).
Citations like [B05] resolve there. This doc answers the inventory's
open questions Q-a…Q-i; it does NOT cover GQ1 mechanism details
(shipped and ratified separately — `docs/SOLVER-DESIGN.md`, #79),
margin-based pre-flip warnings (far-future), or assembly pin
representation.

*IMPLEMENTATION STATUS (added at the M4 8c exit sweep, 2026-07-27;
record above unchanged):* N1–N7 are IMPLEMENTED across M4 PRs 3/4/5 —
N1/N2/N3/N4 at #87 (StableName/RolePath, eager bidirectional name
tables, N2 discriminators through reified predicates, the D5-style CI
invariant); N5/N7 at #96 (resolution, Diagnosis/TieWitness/Tombstone,
the diff engine, Rebind with the Declare carve-out); N6 at #102
(GeomSource identity, Declare threading, production bit-identity
retirement EXECUTED — empty allowlist, debug-only). The #95
memo-naming staleness hole was closed by the recursive naming key
(ratified on the #95 thread; landed #102).

## 0. The problem, stated in our terms

A recipe (D8 operation DAG) references entities of intermediate
bodies ("fillet the edge born of this extrude∩that pocket"). GUI
selections are the same references (G1: one stable-name type, solved
once). Arena keys cannot be those references: they are
body-lineage-scoped and permanently diverge after any edit that
changes kill history (T1) — key identity is a proof device inside
flip-free, edit-free replays, nothing more. The literature's framing
[B05]: (A) how to *name* an entity at construction; (B) how to
*re-resolve* names after re-evaluation. Every prior system solves (B)
with matching heuristics — neighborhood scoring, fingerprints,
best-effort provenance walks — because their kernels cannot promise
that re-evaluation is a specified function of the recipe. Ours can
(D9 bit-identical replay + D5 total birth records). The design's one
big move: **make (B) a lookup, not a search** — names are derivation
paths, the evaluator emits the name↔entity table as part of
evaluation, and "matching" disappears entirely. What remains — and
what most of this doc decides — is making names *denote the right
thing under edits*: split discriminators, merge policy, and tie
handling, which no amount of determinism decides for you (lit modes
1, 2, 4: they are design decisions, not infrastructure).

Theoretical anchor (lit mode 7): beyond the BR-deformation regime
[Raghothama–Shapiro] "the same face" has no ground truth — any
re-resolution is convention. Our convention is *intensional*: a name
denotes a construction role, not a point set. "Same entity" means
"same role in the same derivation," which is well-defined across the
entire parametric family because it is a function of recipe syntax
plus reified predicate verdicts — and the regime boundary surfaces
exactly as verdict flips, which are the pillar's loud sites. The
name table depends on nothing else (N4 states this as the invariant
that CI pins).

## N1 — A stable name is a derivation path

```
StableName<K> = { node: RecipeNodeId, path: RolePath }   // K ∈ {Body, Face, Edge, Vertex}
RolePath      = [RoleSeg]                                 // op-typed, closed enums
```

- `RecipeNodeId` is a **stable identifier minted at node insertion**
  (Onshape's lesson, lit mode 3): never positional, never reused,
  survives reorder/insert/delete of other nodes. Serialization is the
  node-ID type's (GQ3 schema discipline); **names contain no floats**
  (see N2 — geometry enters only through margined predicate verdicts
  during table construction), so bit-exact persistence is trivial.
- `RoleSeg` is an operation-specific combinatorial role, a closed enum
  per feature-op type: extrude — `Cap(Top|Bottom)`, `Lateral(profile
  edge ref)`, `RimEdge(...)`; revolve — bands/poles/seam per the M2
  taxonomy; booleans — `FromA(name)`, `FromB(name)`, `Seam(fA, fB)`;
  split — `Fragment(parent, Qualifier)`; pattern — `Instance(i)`
  (Q-d: the index is D8-structural data, defined by the pattern's own
  indexing expression — pattern references never degrade to
  positional guessing, discharging DESIGN.md's banked requirement).
  Role vocabularies are part of each op's contract, versioned with it.
- Composition: role arguments are themselves names (or profile-level
  combinatorial refs), so boolean-born entities carry terms over
  operand names — [CCH]'s generic naming, made total by our closed op
  set instead of their never-completed qualifier zoo.
- Entity kinds: faces, edges, vertices AND bodies get first-class
  names (Q-h): body count changes only explicitly (typed refusals
  otherwise, ratified), so a body's name is the node+role that minted
  it (e.g. which profile region of a multi-region extrude). Edges and
  vertices are NOT reduced to face-intersection names (the
  literature's manifold shortcut): Euler ops give them birth identity
  natively — one of our few genuine structural advantages, kept.
- Document-local per GQ4; the (document identity × local ref) wrapper
  is the sanctioned extension at exactly these seams (Q-i): table
  keys, rebind edits (N6), appearance attachment, hit-test returns.

## N2 — Split discriminators are covariant margined predicates

The split-face problem (lit mode 1): one source, n fragments; history
supplies no ordering. Value discriminators (indices, (u,v) data) are
the documented trap — deterministic but not *covariant*: an edit
permutes them silently and a reference mis-binds without error (lit
mode 5, the costliest failure). Decision:

- `Fragment` qualifiers are **sign vectors of named trilean
  predicates against recipe-covariant references** — e.g. a face
  split by a slot gets qualifiers = side-of(splitting feature's own
  oriented plane); fragments along an edge get order-along(oriented
  parent carrier). [B05]'s constraint-solver-positioned half-spaces,
  except our "solver" is the recipe itself: the discriminating
  geometry is already recipe data, so covariance is free.
- Because qualifiers are Q1 predicates (named, margined, through
  `k_stats`), fragment identity can change **only at a recorded
  predicate flip** — the naming pillar's flip-localization now covers
  discriminators too, by construction rather than by hope. A sliver
  margin escalates (typed) instead of silently swapping fragments.
- Where no covariant qualifier discriminates — congruent symmetric
  candidates (equal-radius crossing holes, lit mode 4) — the table
  records the entities with an explicit **tie mark**; *naming* them is
  fine, *referencing* a tied name is `Ambiguous{candidates}` (N5)
  until the user records a disambiguation in the recipe (an explicit
  declared choice, same ethos as the coincidence ladder; Onshape's
  `setExternalDisambiguation` is the precedent). Never auto-pick among
  equally admissible candidates — the single clearest lesson of the
  literature ([B05] doctrine, TNaming's fail-don't-guess).

**Alternatives considered (all four fill the same slot — what
disambiguates n fragments of one source):**
1. *Deterministic enumeration indices* (Kripac, realthunder, Han
   SFI): D9 makes them reproducible, but deterministic ≠ covariant —
   an edit can permute which fragment is "1" while both survive, and
   the reference re-resolves *successfully* to the wrong fragment:
   silent mis-binding with NO recorded flip, a denotation change the
   pillar cannot even diagnose. The literature's "old trap of
   enumeration", rejected on Q1 grounds as much as on its record.
2. *Geometric value fingerprints* ((u,v) extents, centroids, bboxes;
   nearest-match): values move with the parameters being edited;
   nearest-match is an unmargined comparison (Q1 violation by
   construction); floats would enter names, breaking N1's float-free
   persistence.
3. *Neighborhood matching at resolve time* (Chen–Hoffmann scoring,
   Kripac graph match): reintroduces the search N1 eliminates,
   silently accepts best-scoring wrong answers, and reconstructs
   information we already own.
4. *Ask-always* (every fragment reference → user disambiguation):
   maximally honest, but "loud everywhere" erodes loud's signal —
   most splits are provably unambiguous via a certified-margin
   qualifier, and refusing provable answers violates
   trivial-where-provable. N2 degrades to ask-always exactly at
   genuine ties, where it is correct.
N2 is taxonomy-(d) hybridization domesticated: derivation is the
name's spine; geometry enters only as margined predicate verdicts —
never raw values — so the "geometry tie-breaker" exists but every
use of it is named, margined, and diffable.

## N3 — Merge policy: names retire into the merge, loudly

merge_coplanar_faces (F7) merges only structural/declared-coincident
faces — by the coincidence ladder those share a recipe source, so:
the merged face's name is `Merged{sorted set of constituent names}`
(canonical order = name order, not enumeration). Constituent names
**retire**: a reference to one fails typed with the merged face as
the offered candidate (N5 payload) — resolving it silently to the
merge would change the denotation (different area, different
boundary) without the recipe saying so. Symmetric on unmerge: if an
edit removes the coincidence, `Merged{a,b}` vanishes and references
to it fail with candidates {a,b}.

**Why F7 stays (Bidarra's merge ban considered and rejected).** [B05]
forbids owner-crossing merges because his scheme has no merge
denotation — a merged face would have no name. N3 supplies one, so
the ban's motivation evaporates. What makes keeping F7 *safe* here is
our own coincidence ladder: numeric coplanarity NEVER merges, so a
merge can never appear or disappear from continuous parameter drift
alone. Merge/unmerge events are confined to the pillar's recorded
change sites — a recipe edit or structural parameter creates/removes
the shared source, or a named adjacency predicate flips (same-source
faces sliding into/out of contact) — every one diagnosable in the N5
payload. So an N3 failure fires exactly when the flat region
*genuinely changed cardinality*, which any honest scheme must
surface. Forbidding merges would not remove that event, it would
hide it: the reference keeps resolving to the a-fragment while the
visible flat region has grown — a silent denotation mismatch, lit
mode 5's cousin. The ban's real costs: interior bookkeeping edges
accumulate unboundedly through op chains (each later boolean
fragments along them); union-of-two-halves and the directly-built
whole stay topologically distinct forever; and it reopens ratified,
adversarially-reviewed PR 4/5/5.5 output behavior including the
bitwise-pinned two-brick trace. Its sole benefit — names never
retire — is already covered loudly and one-edit-repairably.
Softener for the common case: tooling may *suggest* the follow-merge
`Rebind` (an explicit DocEdit, one click); if experience shows it is
always right for appearance-like references, it graduates to a
ratified opt-in policy via the N5 menu — same convenience as
auto-follow, without the silent semantics.

## N4 — The name table: eager, per-node, cache-transferable

Resolution machinery (Q-b, T3):

- Each feature-op **emits names for every boundary entity it
  produces** as part of evaluation (eager). Rationale: the role
  vocabulary makes it mechanical (linear in entity count — no search);
  lazy naming ([TNaming] selectors) reintroduces on-demand B-rep
  inspection, i.e., matching. The per-node **name table**
  (`StableName ↔ arena key`, per body) is part of the node's result
  in the GQ2 result DAG; kills drop entries (a name whose entity was
  consumed simply isn't in the table — historical birth records stay
  untouched, T3 resolved: the resolver never chains dead keys, it
  reads the table the replay just built).
- **The invariant CI pins**: the name table is a function of (recipe
  structure, structural parameters, predicate verdict vector) ONLY.
  Same recipe + same verdicts ⇒ identical table (this is the pillar's
  "provably trivial" made checkable — and it must hold at f64 AND
  Interval: same verdicts ⇒ same names, the Q1 genericity boundary
  respected). A name-table golden test joins the replay-identity CI
  family.
- Resolution = table lookup in the target node's result. Cost model:
  O(entities) build during evaluation (amortized into the op), O(1)
  resolve, zero regen-time matching passes — vs [realthunder]'s ~30%
  recompute overhead for application-side reconstruction of what our
  evaluator knows natively.
- Content-keyed cache transfer (banked principle) applies to tables
  as to any derived artifact: the table is data keyed by the node's
  input content; a cache hit transfers names with the geometry —
  identical inputs cannot disagree about names (the key is the proof).
- Home (G1 layering): tables are **produced by the kernel evaluation**
  (only ops know their roles) and **held/queried by editor-core**;
  hit-testing (Q-g) is the same table read backwards — mesh back-refs
  end at arena keys, editor-core inverts key→name against the
  evaluation the mesh came from. The GUI still never sees an arena
  key.

## N5 — Typed resolution failure

```
ResolveError =
  | Vanished  { name, diagnosis: Diagnosis, last_good: Option<Tombstone> }
  | Ambiguous { name, candidates: Vec<StableName>, tie: TieWitness }
  | NodeGone  { name, edit: RecipeEditRef }
Diagnosis =
  | PredicateFlip { predicate: &'static str, from: Sign, to: Sign }   // the pillar's promise
  | StructuralParam { node, param }
  | RecipeEdit { edit }
  | Cascade { through: StableName }                                    // operand vanished upstream
  | WitnessBifurcation(WitnessBifurcation)                             // SOLVER-DESIGN W3's payload
```

- The **diagnosis is computable** because both evaluations' verdict
  logs exist (k_stats names + D9 replay): diff the verdict vectors,
  attach the flip(s) on the name's derivation path. Same diff
  machinery as SetTolerance's ε-change audit — ratified to be shared.
- **Verdict logs record what the evaluation RAN** (amended 2026-07-29,
  with M5 PR 8): candidate pruning — the C10 BVH sweep — legitimately
  shrinks that population, so a vanish whose flip evidence lived on a
  pair the realized sweep pruned (interaction-boundary edits:
  overlapping ↔ disjoint operands) diagnoses to the documented
  evidence-free fallback `RecipeEdit { NodeChanged(minting node) }` —
  a site, not a claim that an edit happened. Results are unaffected
  (the differential suite pins them bit-equal); only diagnosis
  RICHNESS degrades, and only where the evidence genuinely was never
  computed. The recovery rung — on-demand shadow re-execution of the
  vanished pair to mint the missing verdicts at diagnosis time — is
  banked as #134.
- `Tombstone` carries the last-good table entry (enough for GQ7's
  ghost rendering: entity kind, owning body name, the mesh patch key
  of the last evaluation). Selection tools survive vanishing entities
  by holding the name + tombstone, never a key.
- **Rebinding: the v1 ratified policy menu is EMPTY.** The error
  carries candidates; the only repair is an explicit `Rebind(name →
  selection)` DocEdit — recorded user intent, GQ3-persisted,
  diffable. Any future automatic policy (e.g. "follow the merge")
  enters as its own ratification with this doc's failure cases as its
  test corpus.

## N6 — Recipe-source identity retires bit_identity

The declared-coincidence rung and the M4 retirement (T2, Q-e):

```
GeomSource = { node: RecipeNodeId, expr: ExprPath, orient: Or }   // Or ∈ {Id, Rev}
```

- Every surface/curve/point description carries the recipe expression
  that produced its parameters (through transforms: the transform
  node composes into `expr`; through `revert`: `orient` flips —
  `rev ∘ rev = id`). Same-source is *syntactic* identity of
  `GeomSource` — a provenance lookup, no numerics.
- **Theorem (the retirement's soundness): same GeomSource ⇒
  bit-identical descriptions**, by D9 determinism of expression
  evaluation. The converse is deliberately NOT claimed — equal bits
  without shared source stay unglued, exactly the ratified rung (b).
  So the declared rung becomes "same GeomSource", and the bit
  comparison survives only as `debug_assert!(same_source ⇒ eq_bits)`
  — the "records agree with bits" assertion DESIGN.md's M4 entry
  promises, now with a definition that can actually hold.
- The allowlisted consumers: `merge_faces.rs` and `plane_eq.rs`
  (oriented_plane_eq) compare `(GeomSource, orient)`;
  `bit_identity.rs` is debug/test-only; `interval.rs`'s use is scalar
  plumbing unrelated to coincidence and keeps its own allowlist entry
  with a renamed justification. The CI tripwires stay ARMED over an
  empty production-consumer allowlist — a new production consumer is
  a regression against the retirement, not a step in a migration.

## N7 — What the pillar now says, exhaustively

Topology-change sites, complete for *edits* as well as parameter
motion (T4): (i) structural parameter change (D8), (ii) reified
predicate flip (Q1) — including N2 discriminator predicates, (iii)
recipe edit (node insert/delete/reorder). Names localize (iii) by
construction: node IDs are stable, so an edit renames nothing outside
derivation paths that actually pass through the edited node. Within
a flip-free, edit-free replay, arena-key identity remains the
optimization/proof device (M0's lemma); everywhere else, the name
table carries resolution. T1 through T5 discharge: T1 (names are
recipe-level, keys are per-evaluation), T2 (N6), T3 (N4's
replay-built table), T4 (N2/N3 + this exhaustiveness claim), T5 (N5's
contract).

## Open after this doc

- Out-of-family detection (lit mode 7): a typed resolution failure
  says the name broke, not that the edit left the design family; a
  family-membership predicate (Raghothama–Shapiro necessary, Wang
  sufficient — nothing tight exists) is far-future, and honestly may
  never be decidable in the useful generality.
- Which (if any) rebinding policies to ratify after v1 experience;
  the menu starts empty by decision.

## The split-naming walls (2026-08-15 survey; the G14 disposition
## — RATIFIED: Ev 👍 on #512, A2 + the B1 alignment)

**EXECUTED (LIB-G14).** Both walls are down as ratified; the tenses
below are the survey's and describe the state before the fix. One
measured residue: A2's disambiguation clause reads "the selector
layer already narrows ties geometrically, so a specific chord stays
reachable via `select_where`". Measured, `select_where` is
all-or-nothing PER NAME by its own GS-Q4 rule — a tie whose
candidates disagree under a filter is `SelectRefusal::TiedDisagrees`,
never a narrowing. The tie is reachable and honestly escalated;
picking ONE chord needs a per-candidate narrowing door the SEL layer
does not have. Filed, not worked around. The escalation is itself the
evidence such a door would work: a `DatumDistance` atom separates the
two cap chords 1-of-2 today (executed row), so what is missing is the
door, not the signal.

The LIB audit's "G14" turned out to be TWO disjoint M4-era
deferrals (one logged sentence, M4-LOG:311), disentangled by
measurement (cad-work/g14-survey.md; the conflation persisted
because NamingError had no Display until #380, so Python saw one
opaque refusal):

**Wall B — tied-upstream over-strictness: ALIGN TO RATIFIED
(dominant argument, executed without a fork).** emit_topo's
upstream_name (and name_boolean's shared guard) refuse the WHOLE
op if ANY operand-table entry is Tied — even pass-through
entities far from the tie. That is stricter than this doc's own
ratified N2 text ("naming a tie is fine"), and three shipped
emitters already do it right (name_pattern, name_in_part,
graft_names propagate tied as tied). Disposition: propagate
Entry::Tied through splits/booleans as tied (option B1); the
refusal narrows to entities whose OWN name genuinely needs a
unique upstream. This is alignment with ratified text plus
precedent, not new design.

**Wall A — chord multiplicity: THE FORK (Ev's call).**
RoleSeg::SectionEdge{side, face} names a section chord only by
the operand face it crosses, so a face crossed twice would mint
one name twice — refused today ("multiple section chords across
one operand face"), and it fires on boolean-free scenes (a plain
L-shaped single-loop extrude). Options: A1 rank chords by
order-along-the-section-line (deterministic, no new vocabulary;
names shift if an edit adds a chord); A2 the chords become TIED
SectionEdge entries (one tie story everywhere — B1's direction;
the selector layer already narrows ties geometrically, so a
specific chord stays reachable via select_where); A3 index by
the minted section face (new coupling to face identity).
RATIFIED: A2 — it reuses the ratified tie vocabulary instead
of inventing an ordering, and composes with the SEL1 selector
story for disambiguation. #380 (a Display for NamingError) rides
the implementation unit as a mechanical fail-loud rider; it
changes no disposition.
