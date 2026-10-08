# FORK-PAT — designer A: should patterns be programs?

## For Ev

**Recommendation (likely).** Yes. Retire every pattern preset from the kernel, and add one
construct, **`map`**: apply one definer to every key of a finite **domain**. The definer can be
an `Expr` definition, a construction, a placement, a selection or an assertion. Each output
port of the definer becomes a **family**: a finite, keyed collection of that kind, so `Bodies`
becomes the family of `Body`, and pose families get a kind too. The same `map` places a pattern,
builds the die's pips and states assertions over a pattern's members or pairs of members (Q2,
Q5). Presets live in the façade and the GUI. Each preset lowers to a `map` and is recognised
from one, and none is ever stored (Q4). Mirror stops being a pattern: a reflection is not a
pose, so mirror becomes its own construction.

**Premise check.** Your two questions are one question. A `PatternKind` is a value-level map:
it computes N poses from a few slots. "Declare over all the outputs" is an operation-level map:
it applies one operation per member. D10 already treats a definition and an operation as the
same thing, something that reads variables and defines variables. So one higher-order construct
covers both, and the presets were a closed, hand-enumerated subset of it. A second finding comes
from your own defect list. Today a member's identity is its position: `Instance(i)`, the order
of `Explicit`'s list, and D10's "`Bodies`, an ordered list". Deleting pip 3 silently re-points
"pip 5" at what was pip 6, which DM6 forbids. So keys must not be positions where the author did
not write positions.

### The vocabulary (Q1)

- **Domain.** One of two forms:
  - a product of `Count` ranges `a..b`. A later bound may read an earlier index, so `0..N` is a
    ring and `0..M × 0..N` is a grid. `i in 0..N, j in i+1..N` gives every unordered pair.
  - the keys of an existing family.

  A **key** is either an index tuple or the `VarId` of an element of a literal family. There are
  no nested families: a grid is a product domain, never a family of families. Keys compose outer
  first, so a placement of a multi-body master keys its copies as (member, body), which is P5's
  placement-major layout.
- **Literal family.** `[p, q, r]`: each element is a variable defined as usual (under #4324,
  read off geometry), and its key is its own `VarId`. Appending, deleting or reordering an
  element moves no other member.
- **Body.** One definer that may read the bound key: index `Expr`s, a member `F[key]` of a
  family with the same key set (a mismatch refuses typed), and selections whose name segment
  carries the key.
- **`Expr` grows** a bound-index leaf, exact `Count` arithmetic with `mod`, and `Count/Count`
  as an exact `Scalar`. **Poses grow** `Offset(pose, direction, length)`, which stage 3 already
  plans, and `Rotate(pose, axis, angle)`.
- **Totality.** The domain is finite at evaluation, because counts are read at nominal (A11
  (5)). A body that refuses at one key refuses the whole `map`, naming the key. The `map` never
  returns a partial family.

Today's cases, written as maps (`p` is the base pose, read off geometry):

| case | family |
|---|---|
| linear | `map(k in 0..N, Offset(p, d, k*s))` |
| circular, fixed step | `map(k in 0..N, Rotate(p, A, k*step))` |
| circular, full ring | `map(k in 0..N, Rotate(p, A, k*turn/N))` |
| grid | `map(i in 0..M, j in 0..N, Offset(Offset(p, d1, i*s1), d2, j*s2))` |
| explicit | a literal family |
| bolt circle on a PCD with one tooth missing | `map(k in 1..N, …)` |
| helix | `Rotate(Offset(p, A, k*h), A, k*θ)` |

The pattern itself is `map(x in F, Place(body, mate(pose of body, F[x])))`. Under #4326,
`PlacedUnion` becomes `Union` reading a family, `Part{Instance(i)}` becomes the member read
`F[k]`, and `Transform` retires.

**The full ring is the case the presets cannot say.** With `Circular{step, count}`, twelve teeth
at 30° close the ring only at the current values, so their closure would be an
`unproven-coincidence`. The constant `turn/N` makes the closure one construction. For that to be
proved structurally, the symbolic tier has to treat `Rotate` by a rational fraction of a turn
exactly (unsure how hard this is; see the orchestrator section). That is D10's "construct the
coincidence" applied to patterns, and it is the strongest reason to make this change.

**Mirror (P1–P6).** A pose is a rigid frame up to a subgroup of the rigid motions (D10). A
reflection is not one, and a mirrored body is a different shape, not a copy of the original.
So `Mirror{body, plane}` is a construction defining a new `Body` in the plane's space.
- P1–P3 (chart convention, own door, the boundary of the equivariance audit) stand unchanged,
  applied to `Mirror`.
- A symmetric part is `Union(A, Mirror(A, P))`.
- Mirroring a family is `map(x in F, Mirror(F[x], P))`.

### Q2: assertions over a pattern's members

Yes, it is the same `map`, with `Assert` as the body. A map whose body is `Assert` defines
nothing.

For bolt k against hole k:

`map(k in 0..N, Assert(gap(bolts[k].shank, plate.hole[k]) = -b))`

For adjacent teeth in a ring:

`map(k in 0..N, Assert(gap(T[k].right, T[(k+1) mod N].left) = 0))`

Every member assertion is checked on its own, so the D10 rule still holds: an assertion speaks
only for its own pair, and a pair it names that does not meet is loud. When N grows, the new
pairs come with their assertions, which is "said once for the pattern". **Will it suffice?**
- Yes where the pairing is index arithmetic or the same key in two families.
- No for irregular adjacency, such as which pip neighbours which. Those pairs are written out,
  as a literal family of key pairs or one assertion at a time.

This also settles "pattern interference" at its root. Under #4326 a pattern is several copies,
so an overlap between members is an at-rest finding between copies, quieted by a mapped
assertion. It is not the in-one-value kernel error that the stage-5 spec keeps for it today.

### Q3: what the kernel loses, and how it is recovered (sure on "today")

Today nothing in the kernel exploits group structure. I checked `wire_pattern`, `stepped_map`,
the `PlacedUnion` certificate (which already tests all N² pairs through `M_i⁻¹M_j`), the census
and the mesher. So the loss is only to optimisations not yet built, and each one is recovered by
deriving it from the stored `map`, never by declaring it.
- **Group or orbit.** A range map whose pose is `Offset`/`Rotate` by an affine function of the
  indices is an orbit, and the symbolic evaluation of `g_i⁻¹g_j` reduces to a function of `j−i`.
  The census and the union certificate can then test one pair per difference instead of all N²
  pairs (likely).
- **Reuse.** Copies of one body share that body's evaluation and tessellation whatever the
  family is. Constructions repeated over a family reuse each other's work if a construction's
  content key is taken in the frame it reads (#4324's computing frame). Then the 21 pip balls
  are one cache entry, and D9 caching works per member (likely).
- **A11.** The coset algebra reads pose kinds, not families. The member walk through `Pattern`
  already retires under stage 3 / #4326. A mate reads the pose of copy `F[k]` like any other
  pose.
- **Names.** `RoleSeg::Instance{i}` becomes `Member{key}`.
- **Member k after a `Count` change.** A reference means the member at key k, and it follows
  the formula: in a full ring, member k moves when N changes, as written. A key that has left
  the domain strands its readers typed (DM7) and re-points nothing. An author who wants a member
  to keep its identity across insertions writes a literal family. That is the only design in
  which the banked flag "references into indexed families never degrade to positional guessing"
  is true.

### Q4: where the presets live

The façade has `linear`, `circular` (step or full ring), `grid` and `bolt_circle`, each emitting
the `map`. The GUI has the same forms plus a **recogniser**: a display-only function that
matches a stored `map` against the preset templates and shows "circular, 12 about A" with its
slots. Anything it does not match shows as formula text through `unparse`. Nothing stores a
preset tag, because a tag would say the same thing twice. So a document written without the
presets reads exactly like one written with them, since both store the same definition.

### Q5: constructions repeated over a family

They use the same `map`, with a construction as the body. In the die example, pip frames are a
literal family of 21 poses, each read off a die face's frame:

```
P      = [InFrame(frame(f1),0,0), InFrame(frame(f2),a,a), …]
cutter = map(x in P, Revolve(halfdisc, through P[x]))
die    = Subtract(cube, Union(cutter))
```

Pip x's cavity faces are named `Member(x)/…`. Copy versus construction is decided by the body
of the `map` (#4326's distinction), not by the `map`. This replaces P4's ruling that "feature
patterns are sugar over body patterns". P4 rejected a second instancing semantics, and there is
still only one.

### Ratified text this changes

- **D10 Variables:** `Bodies` as "an ordered list" becomes keyed families of any kind.
- **D10 Operations (#4326's wording):** `Pattern` becomes a `map` of a placement, and
  `Pattern`/`PlacedUnion` stop being nodes.
- **VARIABLES VR5:** the bound-index leaf and `Count` operators.
- **VR4:** "a pattern's count or index" becomes a domain's bounds and a key read.
- **MIRROR-DESIGN:**
  - P4 is replaced.
  - P5's `SegPat` instance predicate is read as a key predicate.
  - P5's layout becomes compound keys.
- **ASSEMBLY:**
  - A6: "mirror is a pattern whose frame is improper" becomes "mirror is a construction".
  - A11 (3): drop the pattern clause.
  - A11 (5): drop the pattern level of the member walk.
- **REFERENCES DM3:** the index operation becomes a member read.
- **editor-core README group boolean:** `PlacedUnion` and `Explicit` retire, and its
  certificate becomes `Union`'s pre-check on a family of copies.
- **NAMES N1:** the `Instance` segment becomes `Member`.
- **Stage-5 spec Q5:** pattern overlap moves to the at-rest census.

**Alternatives.**
- **(B) Keep the presets and add `map` for assertions only.** This leaves two ways to say a ring,
  closure is never structural, and constructions get no `map`. Rejected.
- **(C) A value-level map for pose families only, with a separate `forall` for assertions and a
  separate repeat for constructions.** This is three constructs for one idea. Smaller to build,
  worse final state.

Reversing the recommendation means re-adding preset nodes. That is easy because the recogniser
already names them.

## For the orchestrator

- **Sites.**
  - `editor-core`:
    - `node.rs`: `PatternKind`, `Node::Pattern`, `Node::PlacedUnion`, `PartSelect::Instance`,
      the placement-rule slot table, `placement_rule_fault`, `CountMismatch`.
    - `eval/wire.rs`: `stepped_map`, `wire_pattern`, `wire_placed_union`, `stepped_rule_map`.
    - `eval/mod.rs`: content tags 12/13/19/20–22.
    - `names/role.rs`: `lift`'s `placer_axis`, `name_pattern`, `name_placed_union`.
    - `mate/member.rs`.
  - Python bindings: `pncad-py` `py/doc.rs` (`pattern`, `placed_union`, `part`) and
    `py/place.rs`.
  - Façade: `pncad`.
  - Viewer: `forms.rs:53`, `session/author.rs:224`, `combine.rs`, `session/op.rs`.
  - Corpus: `die_tool` (and its `.pncad` bytes), `heatsink_union`, `sink`, `part_select`.
  - About 40 test files reference the presets.
- **Migration.**
  - `Linear` and `Circular` map to the forms in the table, keeping `k*step`. Do not infer a
    full ring: the closure lint will propose `turn/N`.
  - `Explicit` becomes a literal family. Stage 3 A already turns its frames into `Frame`
    variables.
  - `Part{Instance(i)}` becomes `F[i]`.
  - `PlacedUnion` becomes `Union(map …)`.
  - Index-keyed names keep their numbers. Explicit-list names move from positions to element
    ids, which is a naming re-baseline, and that is deliberate.
- **Sequencing.** This builds on #4324 and #4326 and should land after them. It would replace
  stage 3's plan to give `Linear` a `Direction` slot. Check the INTENT program's unit list before
  that slot is built.
- **Unchecked.**
  - Whether the E12 symbolic tier can decide `Rotate(k*turn/N)` closure exactly (cyclotomic
    constants). If it cannot, the ring-closure benefit is lint text, not a theorem, and the
    recommendation still stands on unification alone.
  - Whether a construction's content key can be taken frame-relative under #4324's computing
    frame.
- **Brief.** No errors found. Ev's 2026-10-03 words agree with D10 on everything this fork
  touches. The one tension is that D10's "`Bodies`, an ordered list" is the "by position" defect
  in miniature. I did not fetch any PR comments.
