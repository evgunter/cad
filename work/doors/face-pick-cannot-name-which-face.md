---
id: face-pick-cannot-name-which-face
kind: issue
title: The add-datum form's face pick names a body scope, and no label in the tree can name WHICH face
status: parked
opened: 2026-09-21
priority: P1
cost: D
refs: [2955]
blocked_on: [names-render-a-faces-leaf-role-in-words]
---

## What

`add_profile_mints_no_frame`'s half 2 has three sites. AUTH-3 closed
the first two — the add-profile form's frame picker and the feature
tree's rows now say which frame a node is, from one home
(`tree::frame_pose`) — and did **not** close the third, because the
third is not the same defect and does not have the same answer.

The third is the add-datum form's held FACE pick
(`crates/viewer/src/pane/create.rs`, `datum_face_frame_rows`), which
renders as `BlendTarget::of_face(face)` said from the landed document
(`Say for BlendTarget`) and therefore reads
the same for **all six faces of a box**. AUTH-1's reviewer filed that
as NOTE-6.

## Why AUTH-3 did not fix it at the home the spec named

`docs/AUTH-3-SPEC.md` says the third site "now has ONE home — fix that
home and this site and the blend sites move together." That is true of
the NODE half of the sentence and false of the face half, and the
face half is the whole complaint.

- `BlendTarget` is deliberately a BODY SCOPE and nothing else — two
  fields, `node` and `body`, with a doc saying so and a destructure in
  its `Say` whose comment exists to make a third field a compile
  error. A blend's accumulator opens on a body; it does not want a
  face and could not use one.
- So putting face identity into `Say for BlendTarget` would give
  every blend refusal a sentence about something the blend is not
  about, to fix one caller. AUTH-3 routed only the NODE half through
  the crate's one spelling (`tree::node_number`), which is what the
  blend sites genuinely share.
- The face's own identity is its role path, and the tree has no
  renderable form of one: `RoleSeg` carries no `Display` at all
  (`crates/viewer/src/idpass.rs`, `Say for Disagreement`, says so in
  as many words — "the path rides as `Debug` because `RoleSeg` has no
  `Display`"), and
  `Display for StableName` deliberately renders the kind and the
  minting node and stops, with a comment ruling that "the role path is
  a derivation, not something a person reads mid-sentence, so prose
  never renders it".

## The two candidate answers, neither cheap

1. **A renderable face descriptor in the names layer** — a prose noun
   for a `RoleSeg` (`top cap`, `side wall 3`) beside the typed path.
   That reverses the `Display for StableName` comment above, which is
   a decision in `editor-core` and not a viewer lane's to take alone.

2. **Read the pose off the landed evaluation.** A face PICK is unlike
   a frame NODE here: the pick was made against the landed run's
   picture, so reading that same run is not a stale guess — it is the
   only non-guess. `names::interrogate::face_frame` already answers
   "where is this face", and `add_datum_ui` already resolves the pick
   once through `face_frame_seat` for the button's gate. What it
   costs: the outward normal has exactly one mint,
   `geom_brep::OutwardNormal::from_chart`, which the `pncad` facade
   does not re-export, and hand-negating `Pose::axis` by `Pose::sense`
   at the form would be a second spelling of the type that exists to
   prevent it. So this answer starts with a facade re-export.

Answer 2 is the smaller of the two and does not reverse anyone's
ruling. It is filed rather than taken because it needs a facade
change and a decision about what a pick readout may read, which is
the same question AUTH-3 answered for NODE labels and deliberately
did not answer for PICK readouts.

## The other half of NOTE-6 is a separate row

The reviewer also noted the pick "stays on screen after the selection
is cleared, so an author can commit against a pick nothing in the
viewport is showing". That is a different defect with a different fix
on different ground — a mark in the viewport, not a sentence in the
form — so it has its own file:
`work/author/held-face-pick-is-invisible-in-the-viewport`.

## Both answers here land outside AUTHOR's ground

Worth knowing before this row is dispatched: neither candidate above
is a change AUTHOR can make alone. The first reverses a ruling written
into `crates/editor-core/src/names/role.rs`; the second needs a
re-export on the `pncad` facade. Either way the unit that takes this
row opens with a question for whoever owns that ground.

## A fourth site since AUTH-9 (2026-09-30): the declare offer

The boolean tool's offer to declare a refused contact names each side
of the pair as `a face of feature N` (`combine::DeclareOffer::pair_line`,
drawn by `pane::create::declare_offer_rows`). The node half goes
through `tree::node_number`; the face half is the same gap as the
held face pick (the line's composer is now
`Refusal::declare_pair_wording`, `session/refuse.rs`). For the
boss-on-a-face union the two operands are enough to know the contact,
but a pair of bodies that touch at two faces gets two lines that read
alike.

**Measured by AUTH-9's review:** a block and a cylinder through it
whose two caps sit flush with the block's top and bottom. The offer's
second round shows `a face of feature 2 against a face of feature 5 —
Rest contact` twice, once for the cylinder's `Cap(Start)` and once for
its `Cap(End)`, and the author cannot tell which line is which. Answer 2 above reaches this site too: each side is a `SitedRef`
at an operand the landed run has drawn, so the pose read is the same
question asked of the offer's pair instead of the add-datum pick.

## Which face, in words (2026-09-30)

Two designers weighed this independently and reached the same final state in their first reports. Evidence: AUTHOR's log (`git show 29b8874a1:work/author/log.md`), 2026-09-30, "face-naming fork".

**Two premises above are wrong.**
- **Words for a face already exist.** `editor_core::resolve::role_words` (private, with `piece_words` and `descent_leaf`, PR #3205) renders every `RoleSeg` exhaustively ("the end cap", "the side wall over the leg of the profile step …"). `Cutter`'s `Display` already puts one in a sentence a person reads.
- **"Prose never renders the role path" is not a ruling.** It is a code comment on `Display for StableName` written by a smell-fixing lane (commit `4067a3ef7`, PR #1454), whose point was keeping `Debug` dumps out of sentences. #3205 already contradicts it, and `display_contract.rs` asserts both the ban and its breach.

**The answer both recommend.** A face is told apart by its **leaf role in words**: the entity the author made, reached by looking through the carry-over wrappers (`FromA`, `FromB`, `FromMember`, `FromTarget`). It is not told apart by its pose. The words and the picture divide the work: the words say *which* face, and the picture (AUTH-10's held mark, plus a hover mark on each line of the declare offer) says *where* it is.
- **In the names layer.** One public renderer for a name's leaf role, promoted from `role_words` and re-exported on the facade, exhaustive over `RoleSeg`. `Display for StableName` becomes kind, minting node and leaf role, the shape `Cutter` already prints. Every refusal that forwards a name then tells faces apart. The comment becomes: "the path as a structure is the machine channel; a person reads its leaf in words". `idpass`'s `Debug` path stays, as the one operator diagnostic.
- **In the viewer.** One composer for a face in a sentence, beside `tree::node_number`. The add-datum pick, the declare offer, the mate panel line and its drop notice, the property header and a face-frame datum's tree row all route through it. `BlendTarget::of_face` goes.
- **"Face of feature N" means different features today.** At the property header it is the minting feature; at the mate tool and the add-datum form it is the node whose body the click hit. The composer settles this: the minting feature, plus "on feature M" where the drawn body differs.
- **Worked example.** The flush block and cylinder read "feature 2's end cap against feature 5's end cap — Rest contact" and "… start cap against … start cap …".

**Rejected by both: the pose.** It needs a landed run, so unevaluated and failed rows would fall back to a second spelling. It is the carrier's frame, so two coplanar faces share it, and that is the flush case itself. It means nothing for a curved face. And it drifts with every edit, while the face stays the same face.

**What stays imperfect.**
- A cylinder's wall is two pieces.
- Pieces of a cut face read long.
- A tie (two faces sharing one name) cannot be told apart in words, because the kernel refuses it as `Ambiguous` too.
- "Start/end" is the sweep's vocabulary, and a person may not hold that in their head. The mark closes that gap.

**Whether the name's own `Display` carries the role.** Both designers say yes. The alternative is an adapter that each composer opts into. That keeps refusal sentences short, but the five gaps above show that opt-in recurs.

## Ruled 2026-10-01 (Ev, #3571)

1. **A face is told apart by its leaf role in words, not by its pose.** The words come from one public renderer in the names layer, promoted from `resolve::role_words`. The "prose never renders the role path" comment becomes "the path as a structure is the machine channel; a person reads its leaf in words".
2. **(a): `StableName`'s own `Display` carries the role**, as kind, minting node and leaf role, so every refusal that forwards a name tells faces apart.

**Where the work is:**
- **EDIT `names-render-a-faces-leaf-role-in-words` (P1).** This is the names layer: the public renderer, `StableName`'s `Display`, the comment and the `display_contract` test, `SelectRefusal`'s `named` forwarding to it, `descent_leaf` derived from `SegOrigin`, and how a profile step is spelled. It also re-baselines the goldens that embed "name minted by node".
- **This row: the viewer half, blocked on EDIT's.**
  - One composer for a face in a sentence, beside `tree::node_number`. It names the minting feature, plus "on feature M" where the drawn body differs, which settles the three meanings of "feature N".
  - The five sites route through it: the add-datum pick, the declare offer, the mate panel line and drop notice, the property header, and a face-frame datum's tree row.
  - `BlendTarget::of_face` goes.
  - A hover mark lights each line of the declare offer.
- `committed-nodes-do-not-light-the-faces-they-reference` (P3) stays its own row: it is the picture half for committed nodes.
