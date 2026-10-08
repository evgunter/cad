# FORK-S4U — a union's pairwise judgement after declarations retire (designer B)

## For Ev

**Recommendation (likely).** Keep the pairwise pass, as the union's **only source of coincidence
records between two members**. Every pair of members whose bounding boxes meet records its
contacts and refuses on its sliver band, even where a third member covers the contact. Also
**stop honouring member order**: the members are a set, and the fold runs in ascending id order.
Then nothing a union produces depends on the order a caller listed its members: not the body's
bits, its names, its refusals or its records. The cost is unchanged: up to n(n−1)/2 two-member
unions, skipped where boxes are disjoint, plus the fold (its first step is the first judged
pair, so it can be reused).

**The premise, corrected.**
1. *The pass has a third job the brief omits:* **naming**. N2's union naming takes its face
   links (which member faces are one parent) from the pairwise judgements' merges and covers
   (`emit_union.rs`, `Links`; names README "A union's face is named for its PARENT"). That is
   what makes a covered or swallowed member face named alike in every order, and what keeps
   removing one member from renaming the others' faces. So the pass survives stage 4 whatever we
   decide about records and refusals (sure). The question is only what else it carries.
2. *Today the pass makes refusals order-free only because it feeds its verdicts to the fold*
   (the declarations). After F nothing is fed, and the fold judges its own pieces again. A plane
   verdict's margin depends on the extent it is read over (the parallelism margin is the
   normals' cross product times the extent's radius, `plane_eq.rs`). So a fold step on a piece
   can read in-band where the pair read Zero over the whole face. If the fold runs in list
   order, that refusal depends on member order again (sure that margins depend on extent; likely
   that this is reachable).
3. *A pairwise pass cannot see every coincidence.* An edge where two members meet can lie in a
   third member's face, or a seam vertex can fuse with a third member's vertex. No pair holds
   that coincidence; only the fold meets it, and it meets it spelled by its path (which seam
   lies on which face). Making that order-free with judgements would take all triples, then all
   quadruples. A canonical fold order makes it order-free for nothing (sure on the geometry;
   unsure whether the kernel's record emits such rows or only escalates on them).
4. *The body's bits already depend on order after E.* A Zero glue keeps operand A's description
   (D10). In a fold, operand A is the accumulation, so the earliest-listed member's description
   survives. DM4's "the fold's body is the same in every member order" holds for the topology
   and fails for the bits (likely).

**Ev's rule this follows.** On #3200 you rejected "a contact another member covers is not a
contact", because it "lets a set get out of declaring a contact just because no single pair is
blamed for it". Your reason survives the redesign with *recording* in place of *declaring*. A
covered contact is still load-bearing under pairwise judgement: at nearby values it can fall
into the pair's sliver band, and the union refuses. So a lint that kept quiet about it would
hide a coincidence that a parameter change turns into a refusal.

### The options as final states

**A (recommended): pairwise records, canonical fold.**
- *Members are a set*, held in ascending id order (VarId order once stage 2 makes members `Body`
  reads). An authored duplicate still refuses at the door (DM5), and is never silently
  deduplicated.
- *Judgement.* Each pair with meeting boxes runs as `m ∪ n`, with the lesser id as operand A.
  Every refusal it raises is the union's, naming member cells, `Escalated` included, covered or
  not.
- *Records.* The union publishes its judgements' rows. Each names its cells in the members'
  tables, with the pair's own margin, sorted by pair and then by decision. A fold-step row whose
  cells are both (pieces of) member cells is a re-reading of a pair's row and is not published.
  A fold-step row that names a cell an earlier step minted (a coincidence among three or more
  members) is published.
- *Fold.* It glues on its own Zero verdicts (no routing, no feed). Its refusals are the union's.
  With canonical order, operand A is always the lesser id, exactly as in the pair judgement, so
  the description a glue keeps is the earliest-minted member's.
- *Representable:* records for cells absent from the result (by design: every face contact goes
  interior, and covered contacts are still recorded). *Unrepresentable:* any result that differs
  by member order. What remains is dependence on mint order, for higher-order rows and for
  sub-tolerance description bits. That is D9's own edit-sequence dependence. It is not something
  the person can change by reordering.
- *Reversible:* easily; going back to list order is one sort removed.

**A′: A without the canonical order** (members stay an ordered list).
- Pair rows are order-free.
- These still depend on member order: higher-order rows and refusals, a fold piece escalating
  where its pair read Zero, and the body's bits.
- It is a smaller DM4 edit. It keeps "D9: the order is the list's", a phrase whose provenance I
  could not find (shallow history; it reads as agent-written), while the clause's own goal is
  order-freedom. Lean against.

**B: retire the pass; the union is the fold** (canonical order, n−1 steps).
- Records are the fold's rows. A covered contact is recorded only if the mint order lets the
  fold meet it before the cover, which reverses your #3200 rule.
- N2's links must come from the fold. Then removing a member can relink others (a contact the
  removed member covered starts linking), which weakens DM4's stability under removal.
- Cheapest. Lean against.

**C: result semantics, as a true n-ary kernel union.** A coincidence counts only where the
result's boundary depends on it, so a covered contact neither records nor refuses. This is your
rejected rule 2, and it needs a new kernel verb. Not recommended.

### DM4 text that changes (`crates/editor-core/REFERENCES.md`)

- **Replace** "It evaluates as a fold of the kernel's pair verb in member order (D9: the order
  is the list's, and the list is data). The fold builds the body; contact is judged pairwise
  before it (below), not by it. The fold's body is the same in every member order: …" **with**
  "Its members are a set, held in ascending id order whatever order they were written in, and it
  evaluates as a fold of the kernel's pair verb in that order, so its body, names, refusals and
  records are functions of its members."
- **`SetMembers`** "by naming the whole new list" becomes "the whole new set". It still refuses
  a duplicate as written.
- **Retitle** "Contact is judged pairwise, in member space, before the fold" **as** "Coincidence
  is judged pairwise, in member space". Keep its first sentence with "contact" read as
  "coincidence". **Replace** its bullets and the "A certified pair authorizes …" paragraph
  **with**:
  - "Each pair of members whose closed bounding boxes meet is judged as the two-member union `m
    ∪ n`, the lesser id as operand A; a pair whose boxes are disjoint cannot touch and is
    skipped. [cost sentence kept]"
  - "Every refusal a judgement raises is the union's, naming member cells: a pair the sliver
    band leaves undecided refuses `Escalated` even where a third member covers it."
  - "The union's coincidence records are its judgements' rows, named in the members' tables. A
    row stands whether or not its cells reach the result: a contact a third member covers is
    still its pair's."
  - "The fold decides its own pieces. Its refusals are the union's, and of its rows only those
    naming a cell an earlier step minted are published: a coincidence among three members that
    no pair holds."
  - "The faces each judgement merged or covered are the union's face links (N2)."
- **Delete** "The declaration channel, sited at the members" whole, "Merges and order" whole,
  and "The refusal names member faces" (its one surviving thought is in the second bullet
  above). F already deletes the declaration clauses. These two go because they describe the
  routing of fed pairs, which no longer exists.
- **N2** (names README): "their members are declared coincident on them, or share a recipe
  source (N6), with the same orientation; the pairwise judgement certifies the pair (DM4); and
  that judgement consumed the pair" **becomes** "the pairwise judgement of their two members
  (DM4) consumed them".
- **DM5**: a union's members join `Loft`'s list only as written, since the stored form is a set.

**Confidence:** recommendation *likely*; naming dependence and extent-dependent margins *sure*;
the kernel recording higher-order coincidences *unsure*.

## For the orchestrator

- Read D10, VARIABLES-DESIGN, DM4–DM7, the spec §0–§13, `wire_union`/`judge_pairwise_contact`,
  `emit_union.rs` `Links`/`Parents`, N2/N3, `plane_eq.rs`, and the `union-refuses-in-some-member-orders…` row.
- **Brief errors.** (1) It omits the naming role (`UnionLinks`). That role alone keeps the pass.
  (2) "the records a union emits — a fold records only what it meets" assumes the fold's rows
  are the union's. The spec's §1 names cells "at the operation that decided it"; for a pair
  judgement that is the members' tables, which already resolve.
- **Risk for E on unions (spec §6), independent of this fork.** `Parents::of` raises an Emission
  bug ("a union's face descends from member faces no judged pair links") on the premise that a
  fold merge happens only under declaration or shared source. After E the fold glues on its own
  Zero over a piece. A pair read Distinct over the whole faces (grazing angle, large arm) while
  a piece reads Zero would hit that "bug" on numerics. Either the fold's merges join the links,
  or the parent rule refuses typed. Worth a probe on the union corpus at E.
- **Feeding pair verdicts to the fold was considered and rejected.** It is the declaration
  channel with the union as author: it brings back `route_declarations` / `look_through_fold` /
  `ConsumedByFold`, the source of all 22 `KNOWN_MIXED` cases. Glue on Zero without routing
  dissolves that class. With a canonical fold, what is left is order-free anyway.
- Stage 2 interaction: members become `Vec<S>` (VarIds, stage 2 spec §1), so "id order" means
  VarId order and `FromMember` keys move with it. Check that split/inline re-minting preserves
  relative order. If it does not, an inline can reorder a union's fold (bits only).
- Measure before E: count the higher-order rows (edge or vertex of two members on a third's
  cell) on the union corpus (`emit_union_rim_piece_ranks.rs`), to settle the *unsure* claim.

## Round 2

**1. Member order: hold (sure on the argument; a choice for Ev, because it changes a ratified sentence).**
A's own orchestrator notes list what its design leaves order-dependent: the fold's non-verdict
refusals (`RingHomingAmbiguous`, `JoinDesync`), the pinch vertex's name, and, after E, the
description a Zero glue keeps (operand A is the accumulation) and a piece's margin read over its
own extent. A canonical fold makes every one of those a function of the members, with no
mechanism added. A's design keeps them, and each is an open row today. A's argument for list
order is only that DM4 says so. The phrase "D9: the order is the list's" has no provenance I
could find, and D9 itself says nothing about lists. The cost: `SetMembers` takes a set, an
authored duplicate still refuses as written, and the members sort by variable id after stage 2.
Inline and split must keep relative mint order; if they don't, an inline can change a union's
bits but nothing it publishes beyond them. Reversal either way is one sort, added or removed.

**2. A Zero the fold decides that no pair row backs: hold that it is never a bug; refine the
criterion.** A's backing claim is false for the three-way case. Take faces `a.f`, `b.g` and
`c.h`, pairwise transversal, meeting along one line L. Pair `(a, c)` cuts along `a.f ∩ c.h`
transversally, and transversal is not a coincidence, so it emits no Zero. The same holds for
`(b, c)`. The fold then decides the seam `a ∩ b` ON `c.h`: a Zero no pair holds, in a legal
union. A vertex where four member planes meet behaves the same way. A's assertion would refuse
such a document as an emission bug. Two cases remain to keep apart:
- That ON may be one spec B does not record (it "only places topology"). Then its only effect
  is a band refusal, which is the union's refusal like any other.
- The piece-reads-Zero, pair-reads-Distinct case is numerics, not a bug.

Final state: a fold row is dropped when a pair row states it (the same two member cells, after
mapping pieces to their member parents, and the same relation). Every other fold row is
published. There is no assertion. This replaces my round-1 test ("names a cell an earlier step
minted"), which missed the numeric case.

**3. Face links: move to A, conditionally (likely).** One source is better than two. A
`covered` pair or a `merge_groups` pair is a same-oriented coplanar face pair decided Zero in the
same build, and after E deletes the kernel's rung 1 every such pair is margin-decided. The
condition is that unit B emits a `SameOriented` row only where the faces merge or one covers
the other. A row for coplanar faces that touch only at a vertex would link them, and change
names. The implementer asserts links-from-rows equal links-from-`BooleanNaming` over the union
corpus before `BooleanNaming` leaves the pass. If they differ, emission is incomplete or too
wide, which is A's point. Reversal is cheap: `BooleanNaming` is still built.

**Remaining choices for Ev.** (1) Canonical order or list order. Cost of reversing later:
re-baselined bits and names in reordered unions, nothing structural. (2) Nothing else: point 2
is a correctness fix to A's assertion, not a preference.

**DM4 text now proposed.** Take A's bullets for the pairwise clause (its opening, the Zero-glue
bullet, "A row is a fact about two members…", "A row names member cells", N2 as "recorded a
same-oriented coincidence between them"), with three changes:
- Replace "It evaluates as a fold of the kernel's pair verb in member order (D9: the order is
  the list's, and the list is data). … The fold's body is the same in every member order: …"
  with "Its members are a set, held in ascending id order whatever order they were written in,
  and it evaluates as a fold of the kernel's pair verb in that order, so its body, names,
  refusals and records are functions of its members." `SetMembers` names "the whole new set".
- Replace A's "a fold step whose Zero no judged row backs is an emission bug", and the old
  "the fold mints no contact verdict of its own", with: "The fold decides its own pieces; its
  refusals are the union's. A fold row that a judged row states (the same member cells, the
  same relation) is not published again; any other, such as a seam of two members lying in a
  third's face, is the union's row too."
- Keep A's deletions of the declaration channel, "Merges and order", and the declared and
  certified bullets.
