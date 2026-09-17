---
id: a-ruled-carve-has-no-editor-fixture-so-emit-fillet-s-band-end-roles-are-unexercised
kind: issue
title: No editor row drives a ruled carve, so emit_fillet's band-end roles (EndArc, FootVertex, BandCut) have never been minted through the document layer
status: closed
opened: 2026-09-15
branch: edit/ruled-carve-fixture
pr: 2778
closed: 2026-09-16
---


## Finding

`crates/editor-core/tests` contains **no ruled-carve fixture**, and
`TransverseCap` appears nowhere in `editor-core` at all — every `grep`
hit for "ruled" under those tests is the English word in a sentence about
a ruling. So the roles a ruled band's ends mint through `emit_fillet` —
`RoleSeg::CornerArc`, `RoleSeg::FootVertex`, `RoleSeg::BandCut` — have
never been exercised from the document layer.

Measured by the WIRE orchestrator on 2026-09-15 while reading
`cut-off-arc-persists-as-a-corner-arc` against the tree. That row's own
argument for leaving the vocabulary alone in FILLET-H7 rested on this
gap — *"no editor row exercises a ruled carve today… so the shape of the
name has no consumer to be wrong for yet"* — and the gap is still open,
so the argument is still load-bearing.

## Why it is its own row rather than a rider

`cut-off-arc-persists-as-a-corner-arc` was ruled (Ev, 2026-09-15) as
option (b′): the roles stay, their doc comments stop making a geometric
claim, and `CornerArc` is renamed to a structural word. **That work does
not need this fixture** — it is a rename and three comments. Attaching
the fixture to it would have made a test nobody had asked for the price
of a vocabulary correction, and would have left the fixture dead if the
ruling had gone the other way.

It is worth having on its own merits: it is the only thing that would
have caught the mis-description in the first place, and it is the only
row that would notice if the rename broke an emitting arm.

## What a taker owes

A document-layer row driving a **rod with a flat** — a cylinder meeting a
plane along a ruling — through `Node::Fillet`, asserting the names minted
at each transverse cap: the cut-off arc, the two cap feet, and the
surviving rim piece. Write the assertion so it can go red: name the
runtime value that would make it false. Asserting merely that some name
exists is documentation.

Read beside `crates/sweep/src/blend/open/ruled.rs`'s module header, which
states the carve step by step (split each rim edge at its foot, `mef` the
cut-off arc across the cap between the two feet, the sliver, the
trimlines, the `kef`/`kev` cleanup) and is the map of what the fixture
should see.

## Filed from outside the fence

Filed by the WIRE orchestrator under
`docs/prompts/implementer-discipline.md` §6, on EDIT's slate beside the
vocabulary row it was split from. `crates/editor-core/tests/*` is claimed
by TCOST and TINT, but their claim on `*/tests/*` is the suite-cost and
suite-integrity dimension of every crate's tests; a missing fixture for
an unexercised emitting path is the vocabulary's own coverage gap, not a
cost row. Re-home if that reading is wrong.

## Spec (2026-09-16, EDIT orchestrator) — middle tier: one opus style review with a correctness arm, no A/B row

Branch `edit/ruled-carve-fixture`. A document-layer row that drives a
ruled carve through `Node::Fillet` and asserts the names it mints,
written so it can go red.

1. **The fixture.** A rod with a flat — the convex side the ruled
   module's header pins through the extrude door
   (`crates/sweep/src/blend/open/ruled.rs`, "the D-profile rod" and
   its boolean-built twin `rod ∖ box`) — authored as a DOCUMENT: the
   profile program, the extrude (or the union that cuts the flat), and
   a `Node::Fillet` on the crease between the flat and the cylinder.
   Read the module header first; it states the carve step by step and
   is the map of what the fixture should see. Put it in
   `crates/editor-core/tests/corpus` if the corpus is where documents
   that exercise a name-minting path belong (say why, and what
   censuses gain a document); otherwise a suite of its own.
2. **The assertions.** At each transverse cap: the cut-off arc's name
   (the role `cut-off-arc-persists-as-a-corner-arc` renamed; read
   `names/role.rs` for its current spelling), the two cap feet
   (`RoleSeg::FootVertex`), the surviving rim piece (`RoleSeg::BandCut`).
   Every assertion names the runtime value that would make it false —
   the role, the operand name it carries, the cap it is on — and asserts
   the geometry the name attaches to (the arc's endpoints are the two
   feet; the feet lie on the cap plane at the trimlines). Asserting that
   some name exists is documentation, not a row.
3. **Redness by mutation.** Break one emitting arm in
   `names/emit_blend.rs` (drop the `FootVertex` mint; mislabel the
   `BandCut`) and report which assertions red; a mutant no assertion
   sees is a gap to close before reporting.
4. **Both material sides if cheap**: the concave twin (a rod's section
   standing on a block's top edge) is the same walk with the cap
   gaining the region under the arc; if a second document costs little,
   pin it too, else name it as the next row.
5. Nothing in `crates/sweep` changes; `crates/editor-core/src/names/*`
   is EDIT's ground and changes only if the fixture proves a name
   wrong — then that is the finding, filed or fixed with the reason.

## Built (2026-09-16)

`crates/editor-core/tests/edit_ruled_carve.rs` — a suite of its own,
not a corpus registration (the reason is in its module header: the
registry buys an interval lane, a round trip, a latency baseline row
and name-digest goldens that this row asserts nothing about, and
`blend5_rim_support.rs` already set the shape for the annulus). Two
documents, both authored as recipes through `DocEdit`: the D-profile
rod (convex) and a rod's section standing on a block's top edge
(concave), each a frame, a chain profile with one bulge arc, an extrude
and a `Node::Fillet` naming the two ruling creases by their extrude
`LateralEdge` names. Both are the same chord on the same circle at the
same standoff — the rod extrudes the arc the flat leaves standing, the
sunk rod the arc it cuts away — derived once in
`sweep::test_support::rod_chord_at`, which the two sweep-side copies of
that arithmetic now call as well.

Six rows. Five read a role's ARGUMENTS against the runtime entity the
name resolves to: the cut-off arc (`RoleSeg::EndArc`) runs between the
two feet of the cap vertex it is keyed by; a foot
(`RoleSeg::FootVertex`) lies at the stored height of the source cap
vertex its `vertex` argument names and is a vertex OF the carve's
survivor of the wall its `support` argument names, not merely on that
wall's surface; a trimline (`RoleSeg::TrimEdge`) runs between its own
support's two feet, one per (crease, support); the surviving rim piece
(`RoleSeg::BandCut`) carries the cap rim it was cut from and runs
between whatever cut it — foot to foot on a rim both creases reached,
foot to surviving source vertex (`RoleSeg::FromTarget`) on one only a
single crease reached (the concave twin's segments 2 and 4); and the
block's top's two COPLANAR faces are told apart by the support face
rather than by its plane. The sixth reads the material side: `ΔV < 0`
for the convex fixture, `ΔV > 0` for the concave one, off the evaluated
bodies.

Redness measured by mutating `names/emit_blend.rs` four ways and the
fixture once, restoring after each:

| mutant | rows red |
| --- | --- |
| drop the `FootVertex` mint | all six, at `check_total` |
| permute a foot's `support` across the two feet of one cap | the foot, trimline and rim rows, on geometry; the arc row blind (the pair of feet is unchanged) |
| permute `BandCut`'s source argument across the remnants | the rim row only |
| swap the argument pairs of the sunk rod's two COPLANAR plane feet | the foot row **on its face check**, with the plane and residual checks passing; also the arc, trimline, rim and coplanar rows |
| swap `sunk_rod`'s bulge for the groove's | the material-side row (and the separation row, which pins a measured number); the four name rows all green |

No mutant went unseen. The last two are what the style review measured
as blind spots of the first draft: a foot on the far coplanar wall, and
a concave fixture that is not concave.

Not done here, filed instead:
`bandfoot-and-bandcross-arguments-are-read-by-no-document-row` — the
ladder rim phase's other two mints still ride `check_total` with no row
reading their arguments.

## Closed (2026-09-16, EDIT orchestrator)

Built and merged as PR #2778 after one opus style review (MERGEABLE:
four MINOR, four NOTE, every one taken in the fix pass, one widened by
the lane's own measurement). Six rows drive a ruled carve through
`Node::Fillet` on two documents — the D-profile rod (convex) and a
rod's section on a block's top edge (concave, two plane supports, cap
rims cut by one crease each) — and read each band-end role's ARGUMENTS
against the entity the name resolves to: the cut-off arc between its
two feet, a foot in its cap plane (exact, a stored coordinate) and on
the face its support names (by the face's own boundary, which is what
tells two coplanar walls apart), the trimline between the feet on its
support, the surviving rim piece's ends, the material side by `ΔV`'s
sign, and the closest separation a row must resolve, from which the
one window the suite uses is derived. Five mutants, each named with the
rows it reds; the arc row's blindness to a swapped support is stated at
the claim site. The copied D-profile derivation has one home
(`sweep::test_support::rod_chord_at`, S-BOOL/FILLET's test support,
value-identical, disclosed). Residue in its own file:
`bandfoot-and-bandcross-arguments-are-read-by-no-document-row`
(widened to four mints).
