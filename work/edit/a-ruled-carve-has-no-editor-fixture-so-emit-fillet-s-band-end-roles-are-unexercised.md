---
id: a-ruled-carve-has-no-editor-fixture-so-emit-fillet-s-band-end-roles-are-unexercised
kind: issue
title: No editor row drives a ruled carve, so emit_fillet's band-end roles (CornerArc, FootVertex, BandCut) have never been minted through the document layer
status: dispatched
opened: 2026-09-15
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
