# FORK-PAT — designer A: should patterns be programs?

## For Ev

**Recommendation (likely): yes.** Retire every pattern preset from the kernel and add one
construct, **`map`**: apply one definer to every key of a finite **domain**. The definer can be
an `Expr` definition, a construction, a placement, a selection or an assertion. Each output port
of the definer becomes a **family**, a finite keyed collection of that kind. `Bodies` becomes
the family of `Body`, and pose families get a kind too. The same `map` places a pattern, builds
the die's pips and asserts over members or pairs of members (Q2, Q5). Presets live in the façade
and the GUI: each lowers to a `map`, is recognised from one, and is never stored (Q4). Mirror
leaves patterns: a reflection is not a pose, so mirror is its own construction.

**Premise check.** Your two questions are one question. A `PatternKind` is a value-level map
(N poses computed from a few slots). "Declare over all the outputs" is an operation-level map
(one operation per member). D10 already treats definitions and operations alike, as things that
read variables and define variables, so one higher-order construct covers both. The presets were
a hand-enumerated subset of it.

A second defect, from your own list: today a member's identity is its position (`Instance(i)`,
`Explicit`'s list order, D10's "`Bodies`, an ordered list"). Deleting pip 3 silently re-points
"pip 5" at what was pip 6, which DM6 forbids. Keys must not be positions where the author did
not write positions.

### The vocabulary (Q1)

- **Domain.** Either a product of `Count` ranges `a..b`, or the keys of an existing family. A
  later range bound may read an earlier index: `0..N` is a ring, `0..M × 0..N` a grid, and
  `i in 0..N, j in i+1..N` every unordered pair.
- **Key.** An index tuple, or the `VarId` of an element of a **literal family** `[p, q, r]`.
  Each element of a literal family is a variable defined as usual (read off geometry, #4324).
  Appending, deleting or reordering an element moves no other member.
- **No nested families.** A grid is a product domain. Keys compose outer first, so copies of a
  multi-body master are keyed (member, body): P5's placement-major layout.
- **Body.** One definer, which may read the bound key: index `Expr`s, `F[key]` from a family with
  the same key set (a mismatch refuses typed), and selections whose name segment carries the key.
- **New vocabulary.** `Expr` gains a bound-index leaf, exact `Count` arithmetic with `mod`, and
  `Count/Count` as an exact `Scalar`. Poses gain `Offset(pose, direction, length)` (already
  planned in stage 3) and `Rotate(pose, axis, angle)`.
- **Total.** Counts are read at nominal (A11 (5)), so the domain is finite. A body that refuses
  at one key refuses the whole `map`, naming the key; there is no partial family.

**Today's presets as maps** (`p` is a base pose read off geometry):

- linear: `map(k in 0..N, Offset(p,d,k*s))`
- circular with a fixed step: `Rotate(p,A,k*step)`
- full ring: `Rotate(p,A,k*turn/N)`
- grid: `map(i in 0..M, j in 0..N, Offset(Offset(p,d1,i*s1),d2,j*s2))`
- explicit: a literal family

**Cases the presets cannot say:** a bolt circle missing one hole (`1..N`), a helix
(`Rotate(Offset(p,A,k*h),A,k*θ)`), and the full ring itself (below).

**What the existing nodes become.**
- A pattern is `map(x in F, Place(body, mate(pose of body, F[x])))`.
- `PlacedUnion` becomes `Union` reading a family.
- `Part{Instance(i)}` becomes `F[k]`.
- `Transform` retires, per #4326.

**The full ring is the case presets cannot say.** Twelve teeth at a free 30° step close the ring
only at the current values, so the closure is an `unproven-coincidence`. With `turn/N` the
closure is one construction. That is D10's "construct the coincidence", applied to patterns, and
the strongest reason for the change. It needs the symbolic tier to treat `Rotate` by a rational
turn exactly (unsure).

**Mirror (P1–P6).** A pose is a rigid frame up to a subgroup of the rigid motions, and a
mirrored body is a different shape, not a copy. So mirror is `Mirror{body, plane}`, a
construction defining a new `Body` in the plane's space.
- P1–P3 stand, applied to it.
- A symmetric part is `Union(A, Mirror(A, P))`.
- A mirrored family is `map(x in F, Mirror(F[x], P))`.

### Q2: assertions over members — the same `map`

The body is `Assert`, and the `map` defines nothing. Two examples:

- Each bolt against its hole:
  `map(k in 0..N, Assert(gap(bolts[k].shank, plate.hole[k]) = -b))`
- Adjacent teeth of a ring:
  `map(k in 0..N, Assert(gap(T[k].right, T[(k+1) mod N].left) = 0))`

Each member assertion is checked on its own, so an assertion still speaks only for its own pair,
and a named pair that does not meet is loud. When N grows, the new members bring their own
assertions: it is said once for the pattern.

**Will it suffice?** Yes, when the pairing is index arithmetic or the same key in two families.
No for irregular adjacency (which pip neighbours which); those pairs are written out.

**Pattern interference itself.** Under #4326 a pattern is several copies. An overlap between
members is therefore an at-rest finding between copies, quieted by a mapped assertion. It is not
the in-one-value kernel error the stage-5 spec keeps for it.

### Q3: what the kernel loses (sure: nothing today)

Nothing in the kernel exploits group structure today. I checked `wire_pattern`, `stepped_map`,
the census, the mesher, and the `PlacedUnion` certificate, which already tests all N² pairs
through `M_i⁻¹M_j`. What could be lost is future optimisations, and each is recovered by
deriving it from the stored `map`, never by declaring it:

- **Orbit structure (likely).** A range map whose pose is `Offset`/`Rotate` by an affine
  function of the indices is an orbit. `g_i⁻¹g_j` reduces symbolically to a function of `j−i`,
  so the census and the certificate test one pair per difference instead of N².
- **Copies.** Copies share their body's evaluation and tessellation, as today.
- **Repeated constructions (likely).** They share a cache entry if content keys are taken in the
  frame the construction reads (#4324's computing frame). The 21 pip balls would then be one
  entry, and D9 caches per member.
- **A11.** The coset algebra reads pose kinds, not families. The member walk through `Pattern`
  already retires (stage 3, #4326), and a mate reads `F[k]`'s pose like any pose.
- **Names.** `RoleSeg::Instance{i}` becomes `Member{key}`.
- **Member k when `Count` changes.** Member k is the one at key k, and it follows the formula:
  in a full ring it moves when N changes, as written. A key that leaves the domain strands its
  readers typed (DM7); nothing is re-pointed. An author who wants a member to keep its identity
  across insertions writes a literal family. Only with this design is the banked flag
  ("references into indexed families never degrade to positional guessing") true.

### Q4: presets live in the façade and the GUI

The façade offers `linear`, `circular` (by step or full ring), `grid` and `bolt_circle`, each
emitting a `map`. The GUI adds a display-only **recogniser**: it matches a stored `map` against
those templates and shows "circular, 12 about A" with its slots. Anything else shows as formula
text through `unparse`. Nothing stores a preset tag (that would say the same thing twice), so a
document written without the presets reads exactly like one written with them.

### Q5: constructions over a family — the same `map`

The die's pip frames are a literal family `P` of 21 poses, each read off a die face's frame. The
pips are then:

```
cutter = map(x in P, Revolve(halfdisc, through P[x]))
die    = Subtract(cube, Union(cutter))
```

and each cavity face is named `Member(x)/…`. Whether members are copies or constructions is
decided by the `map`'s body (#4326's distinction), not by the `map`. This replaces P4 ("feature
patterns are sugar"): P4 rejected a second instancing semantics, and there is still only one.

### Ratified text this changes

- **D10.** Variables: `Bodies` as "an ordered list" becomes keyed families of any kind.
  Operations, as #4326 words it: `Pattern` becomes a `map` of a placement, and
  `Pattern`/`PlacedUnion` are no longer nodes.
- **VR4 and VR5.** The domain bounds, the key read, the bound-index leaf, and the `Count`
  operators.
- **MIRROR-DESIGN.** P4 is replaced. P5's `SegPat` instance predicate becomes a key predicate,
  and its layout becomes compound keys.
- **ASSEMBLY.** A6 says "mirror is a construction", not "a pattern whose frame is improper".
  A11 (3) loses its pattern clause, and A11 (5) loses the walk's pattern level.
- **Other pages.**
  - REFERENCES DM3: the pick becomes a member read.
  - The editor-core README's group boolean: `PlacedUnion` and `Explicit` retire, and the
    certificate becomes `Union`'s pre-check on a family of copies.
  - NAMES N1: `Instance` becomes `Member`.
  - Stage-5 spec Q5: overlap between pattern members becomes an at-rest finding.

### Alternatives

- **(B) Keep the presets and add a `map` for assertions only.** A ring then has two spellings,
  ring closure is never structural, and repeated constructions have no `map`.
- **(C) A pose-family map, plus a separate `forall` for assertions and a separate repeat for
  constructions.** Three constructs for one idea: smaller to build, worse final state.

**Reversibility.** Easy to reverse: the recogniser already names every preset.

## For the orchestrator

- **Sites.**
  - `editor-core`:
    - `node.rs`: `PatternKind`, `Node::Pattern`, `Node::PlacedUnion`, `PartSelect::Instance`,
      the placement-rule slot table, `placement_rule_fault`, `CountMismatch`.
    - `eval/wire.rs`: `stepped_map`, `wire_pattern`, `wire_placed_union`.
    - `eval/mod.rs`: tags 12/13/19/20–22.
    - `names/role.rs`: `placer_axis`, `name_pattern`, `name_placed_union`.
    - `mate/member.rs`.
  - `pncad-py`: `py/doc.rs` and `py/place.rs`.
  - The `pncad` façade.
  - Viewer: `forms.rs:53`, `session/author.rs:224`, `combine.rs`, `session/op.rs`.
  - Corpus: `die_tool` (and its `.pncad` bytes), `heatsink_union`, `sink`, `part_select`.
  - About 40 test files.
- **Migration.**
  - `Linear` and `Circular` map as listed above, keeping `k*step`. Never infer a full ring: the
    closure lint proposes `turn/N`.
  - `Explicit` becomes a literal family (stage 3 A already makes its frames `Frame` variables).
  - `Part{Instance(i)}` becomes `F[i]`, and `PlacedUnion` becomes `Union(map …)`.
  - Explicit-list names move from positions to element ids. That is a deliberate naming
    re-baseline.
- **Sequencing.** This lands after #4324 and #4326. It supersedes stage 3's plan to give
  `Linear` a `Direction` slot, so check the INTENT unit list before that is built.
- **Unchecked.**
  - Whether E12 can decide `Rotate(k*turn/N)` closure exactly (cyclotomic constants). If it
    cannot, the ring argument weakens but the unification still stands.
  - Whether content keys can be made frame-relative.
- **Brief.** No errors. Ev's 2026-10-03 words agree with D10 wherever this fork touches it. The
  one tension: "`Bodies`, an ordered list" is the "by position" defect in miniature. No PR
  comments were fetched.
