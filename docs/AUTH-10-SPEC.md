# AUTH-10 — a held face pick is drawn

**Row**: `work/author/held-face-pick-is-invisible-in-the-viewport` (P1).
Read it in full, and its sibling `face-pick-cannot-name-which-face`,
which is **not** this unit: that row is about what a face is *called*,
this one is about what is *drawn*.

**Branch** `author/held-face-mark`. **Never merge; I merge.**

## What a person gets

The add-datum form's frame-on-face kind latches the viewport's face pick
into `Drafts::datum_face`, and only a second face pick clears it. The
latch is deliberate: it lets an author pick a face, type a spin, and
click elsewhere without losing the pick. But nothing draws the held
pick. Once the selection moves on, an author can press Add datum
against a face the viewport shows no sign of.

After this unit, a held face pick has a mark of its own, distinguishable
from the live selection. The mark is there for as long as the pick is
held and gone when it is released.

## Check these first, because I'm not sure of them

- **How many held face picks there are.** I found two by reading: the
  add-datum form's `datum_face`, and the mate tool's picks (`matetool.rs`,
  `FaceSelection`s). `pane/viewport.rs` marks the blend tool's held
  EDGE set (`tools.blend()` → `edges.selected`) and, as far as I can
  see, nothing for the mate tool. Census every latched pick (face,
  edge, node seat) and say which are drawn. The face picks are this
  unit. If held node seats are also undrawn, that is a row, not this
  unit.
- **What the face mark can be.** `marks::Highlight` carries ONE selected
  and ONE hovered patch id for the shader. `marks::focus` already
  answers a SET of patch ids with its own theme colour
  (`Theme::marks`, `Uniforms::focus`). The blend precedent draws held
  edges in the selected edge lane. Find out what the renderer can
  already mark before you add anything to it. If the honest mark needs
  a shader or uniform change, say so and say what it costs.
- **Where the pick is resolved.** `face_frame_seat` already resolves the
  held pick against the landed pair for the Add button's gate. A pick
  whose body is no longer drawn must mark nothing rather than the wrong
  copy: `ids_of_target` narrows by (node, body) for exactly this reason
  (see `marks::highlight`'s doc).

## Design calls: decide and say

1. **What the held mark looks like, and what it means.** The blend
   precedent uses "selected", on the reading that a held set is a choice
   the user has made. A held pick while something else is selected is a
   different state from the selection, and the row asks for it to be
   distinguishable. Decide, and put the meaning where
   `marks.rs`/`theme.rs` say meanings and colours live.
2. **One home for "held picks to mark".** If both the form and the
   mate tool get marks, they should come from one place, not a second
   and third `if let` block in `viewport.rs` beside the blend one.
   Decide whether the blend block moves into it too, and say why or why
   not.

## Traps

- Eight of nine AUTHOR units have minted a fresh duplication while
  closing one. The ninth minted a second *mechanism* beside an existing
  one. Before you build a mark, look for the mark that exists.
- A mark is a value, recomputed and never retained (`marks.rs` header).
  Don't add a "currently highlighted" field.
- A composition no test holds is this program's recurring defect. The
  mark must be asserted as a value (`tests/focus_highlight.rs` and
  `tests/edge_pick.rs` do this without a GPU), and the viewport must
  be shown to compose it, not just a helper unit-tested.
- **Before merging, I run editor-core's `node_standing` census myself.**
  If you read evaluation results, run it locally:
  `cargo test -p editor-core --test all node_standing::`.

## Scope, verification, deliverable

In: `marks.rs`, `pane/viewport.rs`, `pane/create.rs`, `drafts.rs`,
`matetool.rs`, `theme.rs`, and `gpu.rs` only if the mark needs it (say
so in the PR). `marks.rs`, `theme.rs` and `gpu.rs` are VGEOM's and
CHROME's; post seam notes on their logs.

- Rows that go red:
  - a held datum face pick is marked after the selection is cleared;
  - the mark is gone once the pick is released;
  - a pick of one placement of a doubly-drawn body marks that copy only;
  - a pick whose body is no longer drawn marks nothing;
  - the mate tool's held picks likewise, if they are in scope.
  Name the mutations, restore each from a byte copy, and **touch**
  afterwards.
- Local: fmt; clippy (both feature sets, `-D warnings`); `doc-gate.sh`;
  every `scripts/gates/*.sh`; `--lib` and `--test all` as separate runs
  (two NO-WGPU rows fail in `--lib` on this box); `work.py lint`; the
  `work.py territory` output in the PR.
- `CARGO_TARGET_DIR=/root/auth-10-target` on every invocation, including
  excluded roots. Scratch in `/root/auth-10-scratch/`. Wrap `cargo` in
  `local-scripts/with-build-slot.sh -- <cmd>`.
- CI: confirm the head SHA, read `gate ok`, and say what the change
  filter selected.

PR titled `AUTH-10: a held face pick is drawn`. Report to me; don't
merge.
