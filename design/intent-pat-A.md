# FORK-PAT — designer A: should patterns be programs?

## For Ev

**Recommendation (likely): yes. Maps over numbers are the kernel's only pattern; presets are
façade helpers.** Retire every pattern preset and add one construct, **`map`**: apply one definer
(an `Expr` definition, a construction, a placement, a selection or an assertion) to every index
of a finite range of `Count`s. Each output port becomes a **family**, a finite indexed
collection of that kind; `Bodies` is the family of `Body`, and pose families get a kind too. The
same `map` places a pattern, builds repeated constructions and asserts over members or pairs of
members (Q2, Q5). Presets lower to a `map` in the façade and the GUI, are recognised from one,
and are never stored (Q4).

**Two things are not maps over numbers.**
- **Mirror.** A reflection is not a pose, so mirror is its own construction.
- **An explicit list of placements** (the die's pips). It is computed from no index, and
  positional identity there is a defect: deleting pip 3 silently re-points "pip 5", which DM6
  forbids. So it retires into what it is: N separate operations, which a façade helper emits from
  a list and an n-ary `Union` (DM4, named by member) gathers.

**Premise check.** Your two questions are one: a `PatternKind` is a value-level map; "declare
over all the outputs" is an operation-level map. D10 already treats definitions and operations alike (they
read variables and define variables), so one higher-order construct covers both, and the presets
were a hand-enumerated subset of it.

### The vocabulary (Q1)

- **Domain.** A product of `Count` ranges `a..b` (a later bound may read an earlier index:
  `0..N` a ring, `0..M × 0..N` a grid, `i in 0..N, j in i+1..N` every unordered pair). No
  nested families; indices compose outer first, so copies of a multi-body master are indexed
  (member, body), P5's layout; the index tuple, which the formula reads, is the identity.
- **Body.** One definer, which may read the bound indices: index `Expr`s, `F[k]` from a family
  a `map` defined (an index outside its range refuses typed), and selections whose name segment
  carries the index.
- **New vocabulary.** `Expr` gains a bound-index leaf, exact `Count` arithmetic with `mod`, and
  `Count/Count` as an exact `Scalar`. Poses gain `Offset(pose, direction, length)` (already
  planned in stage 3) and `Rotate(pose, axis, angle)`.
- **Total.** Counts are read at nominal (A11 (5)), so the domain is finite. A body that refuses
  at one index refuses the whole `map`, naming it; there is no partial family.

**Today's presets as maps** (`p` a base pose read off geometry): linear
`map(k in 0..N, Offset(p,d,k*s))`; circular by step `Rotate(p,A,k*step)`; full ring
`Rotate(p,A,k*turn/N)`; grid `map(i in 0..M, j in 0..N, Offset(Offset(p,d1,i*s1),d2,j*s2))`;
explicit: separate operations, not a `map` (above). **Beyond the presets:** a bolt circle missing one hole (`1..N`), a
helix (`Rotate(Offset(p,A,k*h),A,k*θ)`), and the full ring (below). **The nodes:** a pattern is
`map(k in 0..N, Place(body, mate(pose of body, F[k])))`; `PlacedUnion` becomes `Union` reading a
family; `Part{Instance(i)}` becomes `F[k]`; `Transform` retires (#4326).

**The full ring is the strongest reason.** Twelve teeth at a free 30° step close only at the
current values (an `unproven-coincidence`); with `turn/N` the closure is one construction, D10's
"construct the coincidence". It needs the symbolic tier to take `Rotate` by a rational turn
exactly (unsure).

**Mirror (P1–P6).** A pose is a rigid frame up to a subgroup of the rigid motions, and a
mirrored body is a different shape, not a copy. So mirror is `Mirror{body, plane}`, a
construction defining a new `Body` in the plane's space. P1–P3 stand, applied to it. A
symmetric part is `Union(A, Mirror(A, P))`; a mirrored family `map(k in 0..N, Mirror(F[k], P))`.

### Q2: assertions over members — the same `map`
The body is `Assert`, and the `map` defines nothing. Each bolt against its hole is
`map(k in 0..N, Assert(gap(bolts[k].shank, plate.hole[k]) = -b))`; adjacent teeth of a ring,
`map(k in 0..N, Assert(gap(T[k].right, T[(k+1) mod N].left) = 0))`. Each member assertion is checked on its own, so an assertion still speaks only for its own pair,
and a named pair that does not meet is loud. When N grows, the new members bring their own
assertions: it is said once for the pattern. **Will it suffice?** Yes, when the pairing is index arithmetic or the same index in two families.
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
- **Reuse (likely).** Copies share their body's evaluation and tessellation, as today.
  Repeated constructions share a cache entry if content keys are taken in the frame they read
  (#4324's computing frame): the 21 pip balls are one entry, and D9 caches per member.
- **A11 and names.** The coset algebra reads pose kinds, not families; the member walk through
  `Pattern` already retires (stage 3, #4326), and a mate reads `F[k]`'s pose like any pose.
  `RoleSeg::Instance{i}` stays, as the map's index.
- **Member k when `Count` changes.** Member k is the one at index k, and it follows the formula:
  in a full ring it moves when N changes, as written. An index that leaves the range strands its
  readers typed (DM7); nothing is re-pointed. Members that must keep identity across insertions
  are separate operations, so an index is only ever what a formula reads (the banked flag holds).

### Q4: presets live in the façade and the GUI
The façade offers `linear`, `circular` (by step or full ring), `grid` and `bolt_circle`, each
emitting a `map`. The GUI adds a display-only **recogniser**: it matches a stored `map` against
those templates and shows "circular, 12 about A" with its slots. Anything else shows as formula
text through `unparse`. Nothing stores a preset tag (that would say the same thing twice), so a
document written without the presets reads exactly like one written with them.

### Q5: constructions over a family — the same `map`

Yes where the frames are computed (a row of holes: `map(k in 0..N, Extrude(hole, through
F[k]))`). The die's 21 pips are not computed: each frame is read off a face by hand. So they are
21 `Revolve`s (a façade helper loops over a Python list), and
`die = Subtract(cube, Union(pip_1, …, pip_21))`, each cavity face named by its member. Whether members are copies or constructions is
decided by the `map`'s body (#4326's distinction), not by the `map`. This replaces P4 ("feature
patterns are sugar"): P4 rejected a second instancing semantics, and there is still only one.

### Ratified text this changes

- **D10.** Variables: `Bodies` becomes the family of `Body`, and families exist of any kind.
  Operations, as #4326 words it: `Pattern` becomes a `map` of a placement, and
  `Pattern`/`PlacedUnion` are no longer nodes.
- **VR4 and VR5.** The domain bounds, the member read, the bound-index leaf, and the `Count`
  operators.
- **MIRROR-DESIGN.** P4 is replaced. P5's layout becomes compound indices; its `SegPat`
  instance predicate stands.
- **ASSEMBLY.** A6 says "mirror is a construction", not "a pattern whose frame is improper".
  A11 (3) loses its pattern clause, and A11 (5) loses the walk's pattern level.
- **Other pages.** REFERENCES DM3: the pick becomes a member read. The editor-core README's
  group boolean: `PlacedUnion` and `Explicit` retire (an explicit list is separate operations), the certificate becoming `Union`'s
  pre-check on a family of copies. NAMES N1: `Instance` becomes `Member`. Stage-5 spec Q5:
  overlap between pattern members becomes an at-rest finding.

**Alternatives.** (B) Keep the presets, add `map` for assertions only: two spellings of a ring, closure never
structural, no `map` for constructions. (C) A pose-family map plus a separate `forall` and a
separate repeat: three constructs for one idea. Either way, easy to reverse: the recogniser
already names every preset.

## For the orchestrator

- **Sites.** `editor-core`: `node.rs` (`PatternKind`, `Node::Pattern`, `Node::PlacedUnion`,
  `PartSelect::Instance`, the placement-rule slot table, `placement_rule_fault`,
  `CountMismatch`); `eval/wire.rs` (`stepped_map`, `wire_pattern`, `wire_placed_union`);
  `eval/mod.rs` tags 12/13/19/20–22; `names/role.rs` (`placer_axis`, `name_pattern`,
  `name_placed_union`); `mate/member.rs`. `pncad-py` `py/doc.rs`, `py/place.rs`; the `pncad`
  façade; viewer `forms.rs:53`, `session/author.rs:224`, `combine.rs`, `session/op.rs`. Corpus
  `die_tool` (and its `.pncad` bytes), `heatsink_union`, `sink`, `part_select`; ~40 test files.
- **Migration.** `Linear`/`Circular` map as listed, keeping `k*step` (never infer a full ring;
  the closure lint proposes `turn/N`). `Explicit(frames)` becomes one operation or placement per
  frame, gathered by an n-ary `Union` where it fused. `Part{Instance(i)}` becomes `F[i]`; `PlacedUnion`,
  `Union(map …)`. Explicit-list names move from `Instance(i)` to member names: a deliberate
  naming re-baseline.
- **Sequencing.** After #4324 and #4326; supersedes stage 3's `Linear` `Direction` slot.
- **Unchecked.** Whether E12 can decide `Rotate(k*turn/N)` closure exactly (cyclotomic
  constants; if not, the ring argument weakens, the unification stands), and whether content
  keys can be made frame-relative.
- **Brief.** No errors. Ev's 2026-10-03 words agree with D10 wherever this fork touches it. Revised
  after Ev's clarification ("maps over numbers + some facade helper functions"): literal
  families are dropped, since they were the one non-numeric domain. No PR
  comments were fetched.
