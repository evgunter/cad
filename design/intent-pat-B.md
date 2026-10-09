# FORK-PAT — patterns are programs: the index variable

## For Ev

**Recommendation (likely).** Retire `PatternKind`, `Node::Pattern` and `Node::PlacedUnion`. The one construct is an **index
variable**: a `Count` variable defined as `index(N)` ranging over `0..N`, where `N` is any `Count` expression. Everything
whose reads reach an index is evaluated once per value of it, and its outputs are **lists** (a kind `[K]` for every kind
`K`, length a `Count`; D10's `Bodies` is `[Body]`). That is the whole of `map`: no node holds a template, no scope is
stored, and "inside the pattern" is derived from reads exactly as dependency is. A circular pattern is
`k = index(N)`, `seat_k = rotate(seat, axis, scalar(k) · turn/N)`, `copy = place(bolt) where bolt.seat ≡ seat_k`:
the index is shared by the pose and the placement, so the placement is `N` copies. Ev's `map(e^(2πik/N), k ∈ {0..N−1})`
is this with the exponential spelt as a rotation and `turn/N` as the exact constant it is.

**Premise check.** The fork is evidence of two things, and the presets are the symptom. (1) `Linear` and `Circular` are
one thing said twice: the powers `g^k` of one rigid motion read off geometry, differing only in which motion; `Explicit`
is not that thing at all but a list of raw frames, each a position (D10's third defect), and #4326 already dissolves its
one use (the die's pips are constructions). (2) Nothing represents "the same statement, once per member": a count on a
node does it for placements only, and an assertion over the members cannot be said at all, which is the interference
question this fork grew from. Ev's words carry it: "nodes disappearing into a fancy operation on these variables (one
that may produce many more variables, not just one)" and "the role played by edges is replaced by sharing variables".
An index is a shared variable, and a list is the many variables. D10's text names only `Bodies`; Ev's words win and are
more general. This is one answer with one defensible rival (B below), not a fork between presets and programs.

**The vocabulary** (sure on each item; unsure only that nothing is missing).
- `index(N) [within j]`: a `Count` definition; `N` is a `Count` expression and may read `j`. `within` names the index this
  one nests in; it is implied when `N` reads `j`. A node reading two indices neither of which nests in the other refuses
  (two independent repetitions, nest one). Lockstep over two lists is one index read twice; the outer product is nesting.
- Lists `[K]`, nested as `[[K]]`, defined only by a read reaching an index. No list literal: an irregular family is its
  members written out, each a placement or construction that says where it is in words. `xs[i]` for a `Count` expression
  `i` picks one member (REFERENCES DM3's pick becomes this definition); out of range refuses typed at evaluation.
- Pose constructions from #4324: `rotate(pose, axis, angle)`, `translate(pose, direction, length)`, their composition, and
  `scalar(k)` (today's `CountToScalar`) to put an index into an angle or length.
- `place(body) where mates` (#4326) and the constructions, unchanged; `union(xs)` and `subtract` read lists as well as bodies.
- `mirror(body, plane)`: an **operation** defining a `Body`, not a pattern and not a placement. Reflection is not in SE(3), so
  the mirror image is not a copy and no pose is improper; a symmetric part is `union(body, mirror(body, plane))`.

What presets cannot say and this says: a screw (angle and length both in `k`); spacing that varies with `k`; copies placed
into another family's members (`bolt_k` coaxial with `holes[k]`); a 2-D grid (`i = index(Nx)`, `j = index(Ny) within i`,
a `[[Body]]`, unioned by a union of unions); a family of assertions; a family whose shape, not only pose, varies with `k`.

**Q2 — assertions (sure).** The same index. `k = index(N); assert(gap(copies[k].face, base.face) ≤ b)` is `N` assertions
with the sign D10 requires. Neighbouring pairs: `k2 = index(N − 1); assert(gap(copies[k2], copies[k2 + 1]) = 0)`, which is
why an index is a `Count` expression. All pairs: `j = index(k) within k`, an assertion reading both, `N(N−1)/2` of them,
total and bounded. A ring closed by `step = turn/N` needs no closing assertion: copy `N−1` meets copy `0` structurally
(`N · turn/N` is the exact constant `turn`), so the lint is quiet, and a step typed as `60 deg` is what the lint reports,
which is D10's teaching. Interference between members of one pattern stops being the special case INTENT-STAGE5 keeps
(an overlap inside "one node's `Bodies`" is a kernel error): copies under an index are ordinary copies of one space, the
census sees them pairwise, and a `[Body]` of constructions fed to a union is the boolean's business like any operand.

**Q3 — what the kernel loses (likely).** Nothing it uses. No code reads a pattern's group today (the census, the solve and
the key tags do not); the presets never declared a group either, they spelt a generator by variant, and `rotate(_, _, k·θ)`
spells the same generator in one form a reader can recognise instead of three. A property that would be declared and
should be derived stays derived. Evaluation reuse holds where it is true: a placement is a rigid image of a finished body
whatever placed it, so one body is built and `N` are mapped; a construction through `frames[k]` is built `N` times, which
#4326 already decides. D9 caching: a node not reading `k` is computed once; one reading it is keyed by its content key and
the values of the indices it reaches. A11's algebra is per copy, each pinned by its own mates; `k` enters only as values.
Names: `RoleSeg::Instance { i, of }` becomes `Member { k, of }`, keyed by the index variable's id and the integer, nesting
for nested indices; one list per output port, so P5's placement-major layout (`j·M + i`) goes. A reference to member `k`
is a read of `xs[k]`: when `N` shrinks below it the reader refuses typed and is never re-pointed, as DM3 says today, and
`xs[N − 1]` ("the last") follows `N`, which no preset could say. The symbolic tier binds each integer in turn, so E12 needs
nothing new; a family-level theorem ("for all k") is a later recogniser, not a requirement.

**Q4 — presets (sure).** The façade writes the program: `linear_pattern`, `circular_pattern`, `grid`, `mirror` are functions
that mint the index, the pose definition and the placement, and the GUI's forms author through them. Reading back, a
**recogniser** in the façade matches the stored shape (an index whose only placing pose is `rotate(seat, axis, scalar(k)·θ)`)
and the form shows "circular, N, θ"; anything else shows the general form: the index, the pose formula, the repeated nodes
bracketed. The document stores no preset tag: a tag would describe the program a second time and drift. A document is
readable without the preset because it prints what it is: `k = index(N)` and the formula in `k`. The circular form should
default the step to `turn/N` as a definition, minting a free `Angle` only when the person types one, so closure is
structural by default. The viewer's `PatternKindChoice` and `PatternRuleSpec` become this recogniser's output and input.

**Q5 — constructions over a family (sure).** The same index: `f = index(6)`, `frame_f = ...` off face `f`,
`pocket_f = extrude(pip_profile, frame_f, depth)`, `die = subtract(cube, union(pockets))`. The pips themselves are irregular
(face `f` carries `f+1` pips in a layout that is a table, not a formula), so without conditionals they are twenty-one
constructions sharing `pip_profile`, `depth` and the offset `a`, each through `InFrame(face_frame, (±a, ±a))`; one family of
copies would be the pattern of the pattern. That is #4326's ruling made concrete and stays readable: every pocket says
which face and which corner. `PlacedUnion`'s certified disjointness survives as the union's fast path when its operands are
rigid images of one body (visible in the reads), never as a node of its own.

**The rival, B (defensible, not preferred).** A `Map` operation holding a template: nodes stored inside a scope with the
index as the scope's variable. Same expressiveness, a visible boundary for the GUI. Against it: membership is represented
twice, by the scope and by the reads, and the two must agree (a node in the scope reading no index is a constant computed
`N` times; one outside reading the index is ill-formed); moving a node across the boundary is an edit that means nothing
beyond what the node reads. A's boundary is the GUI's to draw from the reads. Rejected outright: implicit lifting (a
placement handed a `[Frame]` broadcasts): zip against outer product is ambiguous and D10 wants nothing implicit; and a
kernel `Pattern` over a `[Frame]`, which under #4326 must carry mates and is then `place` under an index said twice.

**Ratified text this changes.** D10 Variables: kinds gain `[K]` (replacing the one-off `Bodies`) and the `Count` definition
`index`. D10 Operations (#4326's sentence): "`Pattern` is a placement of several copies, one per member of a pose family"
becomes "there is no pattern operation: a placement whose reads reach an index is one copy per value of it". ASSEMBLY A3's
`Node::Pattern` sentence and A11 (3)'s pattern clause retire; A6's "mirror is a pattern whose frame is improper" becomes
"mirror is a construction; a pose is proper" (improper unrepresentable in the pose kinds rather than refused at doors).
REFERENCES DM3 (the pick is `xs[i]`; `Instance` → `Member`), DM4 (`Union` members may be lists; the two-member floor moves
to evaluation). editor-core README "The group boolean: `PlacedUnion`" (A′, #496) retires into `union` over a list.
MIRROR-DESIGN: P1–P3 and P6 stand as the mirror operation's design; **P4 reverses** (a repeated construction is the general
form, a copy pattern is `place` under an index; no second instancing semantics exists because there is one); P5's layout
item goes, its `SegPat` index predicate stays as `Member`. VARIABLES VR3 (kinds, `index`), VR4's "a pattern's count or
index". INTENT-STAGE2 §Pattern, STAGE3 A ("explicit frames become `Frame` variables"; `Direction`'s first reader is now
`translate`), STAGE5 B's one-node overlap sentence. Provenance: I found no Ev words choosing the three presets (the checkout is grafted, so
`git log -S` does not reach their birth); `PatternKind`'s own doc cites the F4 feature spec, `Explicit` came with A′ (#496),
and P1–P6 Ev signed (#909), of which only P4, P5 and A6's sentence are touched.

**Sites and migration** (cost, stated apart from the ranking). editor-core src: `node.rs`, `eval/wire.rs` and `eval/stepped.rs`
(`wire_pattern`, `wire_placed_union`, `stepped_map`), `eval/mod.rs` key tags 12/13/19–22, `names/emit.rs` (`name_pattern`,
`name_placed_union`), `names/role.rs`, `mate/member.rs`, `refactor.rs`, `persist/check.rs`, `assembly.rs`; ~60 editor-core
test files; viewer 12 src files (`forms.rs`, `session/author.rs`, `combine.rs`, `matetool.rs`) and 17 tests; pncad re-exports
and `guide.rs`; pncad-py `place.rs` (`PatternKind`), `doc.rs` (`pattern`, `placed_union`, `part`), `pncad.pyi`, 11 Python
tests; the tour's `assembly.rs` bench-layout and `bool_bodies.rs`. One persisted corpus document holds a pattern,
`die_tool.pncad`. Migration is a one-time regenerate: `Linear` → `index`, `translate(seat, dir, scalar(k)·spacing)`, `place`;
`Circular` → `rotate(seat, axis, scalar(k)·step)`; every migrated pattern needs the anchor pose off the joined space that
#4326 requires anyway; `PlacedUnion(Explicit)` → constructions through `InFrame` poses and `union`. Names and ids move;
geometry is bit-equal for copies and within rounding for constructions. Reversible: the façade functions are the presets,
so reinstating kernel presets later would be sugar over this, not a rewrite.

Confidence: presets retire and the general form is a bounded index over a `Count`, sure; A over B, likely; mirror is an
operation, likely; no list literal, unsure (lean no, add on a real need); nothing today reads the family's group, sure.

## For the orchestrator

- I read #4326 and #4324 from their branches' `docs/DESIGN.md` diffs and commit titles only, no PR bodies or comments.
- Checked: no symbol in `editor-core/src` reads a pattern's symmetry or group (`assembly.rs`, `eval/*`, `mate/solve.rs`);
  `place_each` maps one built master per member; the only `.pncad` with a pattern is `tests/corpus/die_tool.pncad`.
- Off-question: (1) `Count` has no `div`/`mod`; a grid needs none (nesting), a ring's wraparound pair would, but the
  structural closure makes that assertion unnecessary, so file nothing yet. (2) A6 stores improper frames and refuses them
  at every door; under stage 3 A the pose kinds can make them unrepresentable, which stage 3's spec does not say. (3)
  INTENT-STAGE3 §Readers item 6 re-homes `Direction`'s first reader to `PatternKind::Linear`; under this answer it is the
  `translate` construction. (4) DM4's "fewer than two members refuses" at the door cannot hold for a list of variable length.
- The brief's "A11 subgroup algebra" loss does not arise: no mate folds across members today or under #4326.

## Round 2 (after reading A's `For Ev`)

Ev's clarification ("maps over numbers + some facade helper functions") is what both reports say; the differences below
are representation, not expressiveness, and I re-read the 2026-10-03 transcript before each answer.

**1. The construct: I hold, with one concession.** A's `map` holds one definer over a domain, so a pose, then a placement,
then an assertion over the same members are three maps that each restate `0..N` and correspond by equal index values:
the range is represented three times and kept in step by hand, and "lockstep" is a convention about equal ranges. An
index variable states the range once and the three statements read it; lockstep is one index read twice; the members
correspond because they are the same variable's values. That is Ev's "edges replaced by sharing variables" applied to
repetition, and it is the one-source-of-truth test. The concession: A's domain `a..b` says a bolt circle missing one hole
directly (`1..N`) where I write `index(N−1)` and `k+1`; if Ev wants two bounds, `index(a..b)` is harmless sugar the
façade can own. I also move to A on faults: a family whose definer refuses at one index refuses whole, naming the index;
a list is one value and no reader can hold a list with a hole.

**2. Wraparound: I move halfway.** When the ring closes by `turn/N` the contact between member `N−1` and member `0` is
structural, so no assertion is needed and none should be written (D10: construct the coincidence). An assertion over all
adjacent pairs is needed only for a value-level relation, such as an intended press fit between neighbours, and there
`(k+1) mod N` says it once where I needed `N−1` pairs plus one more statement. So: admit `mod` on `Count` (exact integer
arithmetic, already closed under the other operators), and say in the same breath that the structural ring is the normal
case and the façade's full-ring helper writes `turn/N`, never a mod.

**3. Grids: I move to flat families.** The output of a node that reads indices `i` and `j` is one family keyed by the
index tuple `(i, j)`, outer first; there are no nested list kinds, `xs[i, j]` is the member read, and a union reads one
family. Names are `Member { (i, j), of }`, stable under either count changing. What I keep from my report is the rule
that gives the tuple its order: a node may read two indices only when one is declared `within` the other (implied when
its count reads it, as for pairs `j within k`); two unrelated indices in one reader refuse rather than take an order
from document position. Without that rule A's "outer first" has to come from the order of the domain's spelling.

**4. Else.** (a) A says repeated constructions through different frames share one cache entry if keys are taken in the
frame they read, so the die's pip balls are one entry. I disagree: #4326 distinguishes copies (rigid images of one body,
reused) from constructions through frames precisely because equivariance is audited per site, never assumed (D9 conv.
4, MIRROR-DESIGN P3). A construction is built per member until a site is proven equivariant. (b) A keeps the name
segment `Instance` as the map's index; I keep `Member` keyed by the index variable's id, since a document may hold
several families and the integer alone names nothing. (c) Agreed, and worth saying together: orbit structure for the
census is derived from the pose formula being affine in the index, never declared.

Where we now stand: one answer in two spellings. Ev's call is the construct of point 1; everything else has converged.

## Round 3 (grids)

**I hold flat (likely), and A's round 3 holds flat too, so this is converged.** One list kind `[K]`, a family keyed by
its index tuple outer-first, `xs[i, j]` the member read, `Member { (i, j), of }` the name. `within` supplies both the
dependent bound (pairs `j within k`) and the tuple's order, so nesting adds a second shape, `[[K]]`, for the same family
and a flattening rule in every list reader. A grid of cutters is then one `union` over one family, which was A's reason
for nesting, and it comes for free from flatness rather than from a special case in the union.

## Round 4 (alignment with the settled FORK-S3M and FORK-S3P text)

**1. #4341's text.** D10 Repetition says nothing about how an index enters a placement, which is the gap. The work item
fills it the old way: its Migration writes `rotate(seat, axis, scalar(k)·step)` and `translate(seat, dir, scalar(k)·spacing)`,
target poses constructed to carry the number, which is the retired `Offset` target in another spelling; its Evaluation and
MIRROR P4 say "a construction through a different frame per member", which S3P now forbids ("no construction reads a
frame"); and `Mirror { body, plane }` reads a pose, which S3P says no construction does. S3M's own sentence, "`Pattern` is a
placement of several copies, one per member of a pose family read off geometry", is the one to retire: there is no pose
family, only values. My round-1 `rotate`/`translate` pose constructions leave the vocabulary with it.

**2. Yes (sure).** A pattern is a placement whose reads reach an index, and the index enters through the two constraint
forms that exist and no third: as a **value**, a slide or spin that is a `Length` or `Angle` expression over `k`, or through a
**mate** whose target is a member of another family. Ring: `Axis` mate, `Plane` mate, `spin = scalar(k)·turn/N`. Row: two
`Plane` mates, `slide = scalar(k)·pitch`. Grid: one `Plane` mate, `slide = scalar(i)·p` on one freedom, `scalar(j)·q` on the
other, spin `0`. Helix: `Axis` mate, both a slide and a spin over `k`. Bolts into holes: `bolt.axis ≡ holes[k].axis`. Copy `0`
has spin `0`, "the two bodies' own coordinates agree", which is Ev's "0 is always valid" said per member, and
`(N−1)·turn/N + turn/N = turn` is exact, so the ring closes structurally with no rotation matrix anywhere: #4341's
"unchecked requirement" narrows to the spin value's chart reducing modulo a turn. Q5 moves with S3M: the pips are one
cutter body placed 21 times (`Plane` mate to the face, slides `±a`, spin `0`), then `subtract(cube, union(copies))`, not 21
constructions; "built per member" survives only for a construction whose scalar inputs read `k` (`depth = scalar(k)·d`).

**3. Mirror changes in one word; names and helpers in what they write.** `Mirror { body, face }`: the plane is a face of
its own body (S3P: a face reads as a plane), the image is built in the body's own coordinates like a fillet is, and where it
sits is a placement; a "mirror across the assembly's plane" helper places the image against the targets' mirror-image
faces, which exist only when the targets are symmetric, and that is a true statement, not a loss. `Member { (i, j), of }`
is unchanged. The helpers write mates and values, never a pose, and the recogniser matches a placement under an index
whose values are affine in it.

**Exact text.** D10 Operations (S3M's sentence) → "A pattern is a placement whose reads reach an index (Repetition): one
copy per value, the index entering as a value or through a mate's target." D10 Repetition, after "there is no pattern
operation.": "An index enters a placement as a value, a slide or spin that is a `Length` or `Angle` expression over it, or
through a mate whose target is a member of another family; no pose is constructed from an index, and nothing else carries
one." Its "a construction whose own inputs differ per member is built per member" → "a construction whose scalar inputs
read an index is built per member; no construction reads a frame". Its `Mirror { body, plane }` sentence → "`Mirror { body,
face }` is a construction defining a new `Body` in its body's own coordinates, reading one of its own faces as the plane;
where the image sits is a placement; every pose is proper." MIRROR P4: the same two substitutions. Work item: Evaluation
and Mirror bullets as above; Façade bullet adds "the helpers write mates and values: `circular_pattern` an `Axis` mate, a
`Plane` mate and `spin = scalar(k)·turn/N`; `linear_pattern` two `Plane` mates and `slide = scalar(k)·pitch`"; Migration
replaces the `rotate`/`translate` lines with those forms, and `PlacedUnion(Explicit(frames))` → "one placement per frame,
a `Plane` mate against the face with the frame's in-plane offsets as slides and spin, read off the stored frame once at
migration, gathered by `union`", dropping "through `InFrame` poses" and "within rounding for constructions"; Sites' "
`Direction`'s first reader is `translate`" → "the `Linear` `Direction` slot has no successor: a row's direction is the
freedom a `Plane` mate leaves"; Unchecked requirement → the narrowed form above; Sequencing's "#4326's `Pattern`
sentence" → the Operations wording above.

## Round 5 (does `Mirror` read a plane?)

**1. No, `Mirror { body }` does not hold (likely).** The premise is right for the image alone: reflections about any two
planes differ by a rigid motion, so for a chiral twin that goes to another space the plane carries no shape, exactly as the
extrude's side does, and one convention would suffice. It is wrong for the image **fused to its source**, which is what
mirror is for in part modelling: `union(body, image)` depends on the plane, since the plane is the seam. With
`Mirror { body }` the image is a root of its own, so fusing it needs a placement: a `Plane` mate (flipped) on the seam face
and three values. Reflection across that face fixes everything in the plane, so the right values are constants, but which
constants (zero, or a half-turn spin) depends on the fixed reflecting plane of the image's own coordinates and on which
reference direction the spin chart reads, a convention the person has to know and a fudged invariant. The plane says it
without a number.

**2. The case: the symmetric part.** `union(body, Mirror { body, plane })` is one construction in one root; the seam is the
plane's face read twice, the same construction, so the glue is structural with nothing more said and the census proves
nothing. Round 4's `face` was too narrow: the plane is any `Plane` pose whose reads reach the body's root alone, a face
read as a plane usually, or a plane constructed from the body's own geometry (through its axis) for a body with no planar
face; S3P's root rule is the whole restriction, and "no construction reads a frame from elsewhere" is kept. The image lives
in its source's root; a chiral twin elsewhere is a placement of a copy of it, where the plane is shape-free and harmless.

**3. Text (for `Mirror { body, plane }`).** D10 Repetition: "`Mirror { body, plane }` is a construction defining a new `Body`
in its source's root, the plane a `Plane` pose whose reads reach that root alone (a face read as a plane, or a plane
constructed from the body's geometry); a reflection is not a pose, so the image is not a copy and every pose is proper. A
symmetric part is `union(body, Mirror { body, plane })`, one construction whose seam is the plane's face read twice; a
chiral twin elsewhere is a placement of the image, and there the plane carries no shape." MIRROR P4: replace "Mirror is the
construction `Mirror { body, plane }`, defining a new `Body`: a reflection is not a pose, so no pose is improper" with the
same two sentences, and keep "P1–P3 and P6 are that construction's design".
