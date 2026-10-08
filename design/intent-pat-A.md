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

## Round 2 (after reading B's "For Ev"; checked against Ev's 2026-10-03 words and the brief's quotes)

**1. The construct: I move to B's index variable (likely).** In my `map`, every map states its
own range. A multi-step family (pose, then placement, then assertion) is three maps that each
restate `0..N`, and they line up only because those three ranges happen to be equal: one domain
stored three times and kept in step by hand. With `k = index(N)`, the domain is one variable.
Lockstep is that variable read twice, and "inside the family" is derived from reads. That is
D10's one way to say dependency (reading a variable) and Ev's "the role played by edges is
replaced by sharing variables". The brief's "map higher order function" is what results; the
quote doesn't need a stored `map` node to say it. A stored template is the rival B rejects,
because the scope represents membership a second time.

One rule must be written down or B's model becomes implicit lifting. A variable defined by
reading `k` has kind `K` to readers that reach the same `k`, which read the member at the same
value, and kind `[K]` to every other reader, which reads the whole list. So `union(pockets)`,
read outside `k`, is one union; `assert(gap(copies[k].f, …))` is N assertions. Two indices meet
only by nesting (`within`), and otherwise the read refuses, as B says. With that rule, the
multi-step family is
`k = index(N); seat = rotate(s, A, scalar(k)·turn/N); c = place(b) where b.seat ≡ seat;
assert(gap(c.f, base.g) ≤ x)`.

**2. Wraparound pair: keep `mod`, a minor point (likely).** B is right that a full ring needs no
closing assertion for *contact*: N−1 meeting 0 is structural with `turn/N`, so the lint is
quiet. Interference is never quiet structurally, though. A press-fit ring asserting `gap = −b`
between neighbours needs the N−1/0 pair too. Without `mod`, that pair is a second statement
written out beside `index(N−1)`; with exact `Count` `mod`, the ring is one statement,
`k = index(N); assert(gap(c[k].r, c[(k+1) mod N].l) = −b)`. `mod` is one total operator on
exact integers. I would include it, but would not fight it.

**3. Grids: B's nesting, with one change (likely).** Nesting is what `index … within` gives, and
dependent bounds (all pairs, `j = index(k) within k`) need it anyway. My product domain was
nesting under another name. Names are the same either way (`Member{i}/Member{j}`). The change:
`union` and `subtract` should read a `[[Body]]` directly as its members, rather than a union of
unions, so a grid of cutters is one boolean and not Nx+1. That is a property of the union's
operand kind, not a second form.

**4. Remaining differences.**
- **(a) E12.** B says "E12 needs nothing new" for ring closure. `N·turn/N = turn` is exact as an
  angle, but a rotation's matrix entries (cos turn/12 = √3/2) are not rational. Polynomial
  identity over ℚ proves closure only if the tier carries a rotation by its angle expression,
  modulo a turn, rather than by its matrix. Both reports lean on closure being structural, so
  this is a shared unchecked load, not a settled point.
- **(b) My `F[k]` versus B's `xs[i]`.** These are the same thing, and I adopt B's spelling and
  `Member{k, of}` keyed by the index variable's id. That replaces my "`Instance{i}` stays".
- **(c) Everything else in B's "For Ev" I now agree with.** That covers mirror as an operation
  with improper poses unrepresentable, the circular form defaulting to `turn/N`, P4 reversed, and
  P5's layout going.

## Round 3 (grids)

**I hold flat now (likely). This was my round-1 position, and B's round 2 is the better form of
it.** A flat family keeps one list kind, `[K]`: no `[[K]]` kind, and no rule that `union`
flattens nested lists. `union` reads one family, and a member is one `xs[i, j]` read named
`Member{(i, j), of}`. `within` still gives dependent bounds and the outer-first order, so
nesting adds nothing except a second shape for the same family.
