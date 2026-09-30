# AUTH-1 — the face-frame seat: chrome that mints a `Datum::FaceFrame` (spec)

Unit of the `author` program. Item
`work/author/add-profile-placement-on-picked-face-frame.md` — read it
in full, including its 2026-09-15 premise re-cut, before you start.
**Track:** a chrome build, no design revision and no ratified clause
touched; `crates/editor-core/REFERENCES.md`'s DM1 chrome-consequence
bullet already names this row and rules the multi-node question, so
nothing here waits on Ev. One implementer, then two reviewers
(§Review). No A/B row (the account has no Fable budget this week).

Branch `author/face-frame-seat`, off `main`.

## The gap, in one sentence

`Datum::FaceFrame` is a node the document evaluates, the tree labels
(`"Datum frame (on face)"`), the add-profile plane seat admits
(`session::refuse::admits`, `NodeKindWanted::Frame` matches both frame
arms) and the sketch drawer places — **and that no chrome can create.**
Placing a sketch on a face you picked is the most ordinary gesture in
CAD and the viewer cannot do it at all.

## What the tree says now

1. **The authoring vocabulary has five seats and no sixth.**
   `DatumSpec` (`crates/viewer/src/session/author.rs`) is `Plane`,
   `Axis`, `Point`, `Frame`, `AxisInPlane`; `datum_node` lowers exactly
   those five and is total. `DatumKindChoice` (`crates/viewer/src/forms.rs`)
   is the form's five-way offering, held to `DatumSpec` in both
   directions — `Drafts::datum_spec`'s match compiles only over the
   offering, and the `partial_mirror!` invocation classifies every
   `DatumSpec` arm as offered or deliberately absent. Its `absent`
   section is empty today.

2. **The node wants two picks and one field.** `Datum::FaceFrame`
   (`crates/editor-core/src/node.rs`): `at: RecipeNodeId` — "the
   body-denoting node the face is read out of, a DAG input exactly as
   `Datum::AxisInPlane::plane` is"; `face: StableName` — "a frozen name
   resolved through `at`'s value under the N5 ladder"; `spin: Expr`
   (Angle, `SlotId::Spin`) — "the rotation of sketch +x about the
   outward normal", and the node's ONLY slot, because origin and normal
   are read off the face.

3. **The viewport already answers both picks.** `Selection::Face` holds
   a `FaceSelection { name: StableName, node: RecipeNodeId, body: u32 }`
   (`crates/viewer/src/session/select.rs`). Its `node` field is
   documented as "whose body did the ray meet" — the body the name
   resolves in — and `feature()` answers the different question "which
   feature minted it".

4. **The planarity question is a shipped tag read.**
   `editor_core::names::interrogate::face_carrier_kind(ev, node, name)
   -> Result<SurfaceKind, InterrogateError>`, re-exported through
   `pncad::select`, which `crates/viewer/src/matetool.rs` already
   imports `face_frame` from. "Is this face planar" is
   `face_carrier_kind(..)? == SurfaceKind::Plane` and consults no
   number. `Datum::FaceFrame` refuses a non-planar face at evaluation
   (DM1b), so the chrome gate is an AFFORDANCE, never the safety.

5. **The evaluation is in hand.** `DocSession::landed_pair() ->
   Option<(&Doc<ProfileProgram>, &Evaluation<f64>)>`; the create pane
   already calls it (`ViewerBehavior::frames`).

6. **Nothing mints one today, and a test says so.**
   `crates/viewer/tests/docm1_face_frame.rs`'s header reads: *"No
   chrome MINTS one here — that is CHROME's build
   (`add-profile-placement-on-picked-face-frame`); the node arrives in
   the document through the document door."* That sentence is this
   unit's, and it stops being true when you land.

## What the unit builds

**1. The seat.** A sixth `DatumSpec` arm in
`crates/viewer/src/session/author.rs`:

```rust
FaceFrame {
    /// The body-denoting node the face is read out of — a PICK.
    at: RecipeNodeId,
    /// The picked face, frozen — a PICK.
    face: StableName,
    /// Sketch +x's rotation about the outward normal (`Angle`).
    spin: Expr,
},
```

lowered in `datum_node` (which stays total for the reason its note
gives). **Two picks and one field — the `AxisInPlane` shape, not the
`Plane` one**, which is what the item's "What such a seat has to
carry" concluded. Document the arm the way its five neighbours are:
say which components are picks and which are numbers, and say what
dimension the field is.

**`at` is the picked face's `node`, NOT its `feature()`.** The name is
read out of the body the ray met, and that is the body whose table
holds it; a flat swept by an extrude and shrunk by a later fillet is
picked on the fillet's body, and reading it through the extrude would
name a face of a different size. State this at the seat, because the
two fields sit beside each other in `FaceSelection` and the wrong one
compiles.

**2. The offering.** `DatumKindChoice::FaceFrame = "frame on face"` in
`crates/viewer/src/forms.rs`, declared in FORM order — beside `Frame`,
for the reason that doc already gives about form order ("the frame sits
next to the plane because that is the choice a reader is actually
making"): a frame you type and a frame you pick are the same choice,
differently sourced. Add the `offered` entry to the `partial_mirror!`
invocation. Wording of the label is yours; it has to distinguish itself
from `"frame"` at a glance in a radio row.

**3. The draft seats.** `Drafts` (`crates/viewer/src/drafts.rs`) gains
what the arm needs: the held face pick and the spin number. Reuse
`angle_unit` and `angle_picker` / `ANGLE_DRAG_SPEED` — a spin is an
Angle and the form's other angles are authored through that door, so a
second spelling here would be the duplication `reviewer-style-lane.md`
Q1 exists to catch. Spin defaults to zero.

**How the face reaches the draft is yours to choose, with one
constraint**: the pick must survive the author then interacting with
the form (typing a spin, clicking the unit picker, opening another
section). The established idiom is the blend tool's — a HELD seat
falling back to the live selection
(`BlendTarget::of_selection`, `crates/viewer/src/pane/create.rs`) — and
it is the one to beat. If you take the live selection unheld, say in
the PR why the held seat is not needed; if you hold it, say what
clears it.

**4. The gate and the message, as VALUES.** `Drafts::datum_spec`
returns `Result<Option<DatumSpec>, DimensionError>` where `Ok(None)`
means "a pick is still missing", and `add_datum_ui` renders that with a
HARDCODED `"pick a frame to write the axis in"` — a sentence that is
already only true of one kind and would be false of two. Fix that as
part of this unit: the unmet-seat sentence follows the kind.

**The planarity gate does not belong inside the widget function.**
`add_datum_ui` is untestable headlessly; a gate buried in it is a claim
nothing can go red on. Put the question — *given this evaluation and
this face pick, may a face frame be authored on it, and if not, why
not* — in a named function in a module the tests reach
(`session/author.rs` or `session/refuse.rs`; `refuse.rs` already hosts
`admits`, the one classification behind every creation seat's gate, and
is the closer neighbour). `add_datum_ui` renders its answer and decides
nothing.

That function's refusals are at least: no face selected; the selected
face's name no longer resolves (`InterrogateError` — carry it, do not
flatten it to a string at the raising site, per `CommitFault`'s note);
and the carrier is not a plane, naming the `SurfaceKind` it actually
is. **A multi-body `at` is a real case**: `FaceSelection` carries a
`body: u32` that `Datum::FaceFrame` has no seat for, so decide and say
in the PR whether `face_carrier_kind` can answer `Ambiguous` for a name
under a multi-body node, and make the form surface that refusal rather
than mint a node that refuses later.

**5. The commit.** One `SessionOp::AddDatum { datum }`, exactly as the
other five kinds. **This unit does NOT touch `commit_action`, the
add-profile form, or the frame picker's labels.** "Mint the frame and
the profile in one gesture" and "a picker that names frames by node
number" are `work/author/add-profile-mints-no-frame.md`, the next unit;
after AUTH-1 a person picks a face, adds the datum, and draws on it in
the add-profile form, which already admits it. Say in the PR that the
two-form trip is the known residue and name the row that carries it.

## Claims to falsify (for the reviewers, and for you first)

- **C1.** After this unit a `Datum::FaceFrame` reaches the document
  through `SessionOp::AddDatum` alone, and the node it mints is
  bit-identical to the one `docm1_face_frame.rs` hand-builds through the
  document door for the same face and spin.
- **C2.** `at` is the node whose evaluated body carried the picked
  name. A face carried through a later feature authors a frame on the
  body it was picked on.
- **C3.** The form declines to author on a non-planar face, and says
  which kind of carrier it found. The refusal is a value a test reads,
  not a string a widget draws.
- **C4.** The form declines to author on a face whose name no longer
  resolves, carrying the interrogation error rather than a rendered
  sentence.
- **C5.** The unmet-seat sentence is true of the kind that is showing.
  No kind renders another kind's sentence.
- **C6.** `partial_mirror!` still holds: a seventh `DatumSpec` arm
  cannot arrive with no form edit and nothing saying so.
- **C7.** Nothing outside the datum path changed behaviour — the
  add-profile form, the frame picker, `commit_action` and the tree
  labels are as they were.

## Tests

`crates/viewer/tests/docm1_face_frame.rs` is the natural home and its
header comment is this unit's to correct (item 6 above) — a row there
now mints through the chrome. `creation_ops.rs` is where the
`SessionOp::AddDatum` replay rows live; a headless acceptance row
(document → box → pick its top cap → author the frame → draw a profile
on it → extrude) belongs there and is the end-to-end evidence the
standing goal asks for.

**Write assertions a bug could break** (implementer discipline §2). A
row asserting that a planar face is admitted cannot go red on the gate;
the rows that can are the non-planar one, the unresolved one, and the
`at`-is-the-hit-node one, which needs a body whose face was carried
through a later feature. `docm1_face_frame.rs` already builds a box with
`common::{session_insert, inserted, len}` and names its cap
`RoleSeg::Cap(CapEnd::End)`; reuse those helpers rather than minting a
second fixture vocabulary.

Note `PR 2929` (`dup/viewer-shared-doors`) is live on
`crates/viewer/tests/common` — merge `main` before you land and re-run.

## Review

**Two reviewers, no dual.** This unit's failure mode is a confident
wrong answer — a frame minted on the wrong node, at the wrong spin
reference, or on a face the gate wrongly admitted — and not a refusal,
which is CHROME's inherited posture for exactly that case and is
adopted here as AUTHOR's (`work/author/plan.md`, Review posture; this
spec answers the question it left open).

- **Correctness lane**: the claims C1–C7 above, to falsify.
- **Style lane**: `docs/prompts/reviewer-style-lane.md` in full. Q1 and
  Q8 are the live ones — a sixth arm across four files is the shape that
  mints a near-duplicate, and `pane/create.rs` is 1109 lines and grows
  one titled section per unit.

## Discipline

`docs/prompts/implementer-discipline.md` in full, by path, before you
start. Hosted CI is the verification of record. `crates/viewer` is a
workspace member, so `cargo clippy --workspace --all-targets` covers
it; the excluded roots do not consume this vocabulary, but
`scripts/doc-gate.sh --print-roots` is how you check rather than
trusting that sentence.

File anything you find outside this fence on the owning program's
slate, in this PR (§6 of the discipline). `python3 scripts/work.py
territory --files -` says who owns a path.
