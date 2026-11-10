# Fork: a cut whose tool is a seated ⊔ — designer A, round 1

## For Ev

**Recommendation (likely): a cut reads its tool as material, not as pieces.** `X − (bolt ⊔ plate)` is `X − (bolt ∪ plate)`. At the cut's door the kernel replaces the tool by its material — its solids united, glued on the pairs the tool's own face–face records already state, nothing measured again — and runs the ordinary cut against that. The crack is never minted, no reader of the finish changes, and `ToolSeatCrossed` is deleted in both places it is raised.

**Premise check.** The brief asks whether the crack should be "never minted" or "dropped by the finish", and how a wall's patch is divided where the wall turns one-sided. Both questions assume the cut must section `X` along the seat's two faces. It must not. Terms:

- *Seat*: the area where two pieces of one body rest on each other, stated by a face–face contact record (here the bolt's foot on the plate's top).
- *Wall*: the kernel's reading of a seat as one two-sided surface, material on both sides, each face owning its side.
- *Crack*: the zero-thickness part of `X` between the two cuts made along the seat's two faces.

In every other role the pieces of a ⊔ survive into the result: `S − X` and `X ∩ S` keep two pieces, and `S ∪ X` keeps the seat as a record ([operations.operators]). There the seat's faces are faces of the result, so `X` has to be read against each, and the wall reading is right. A cut's tool is the one role where nothing of the operand survives: the result is `X` less the tool's material, and the seat lies in the interior of that material. Its two faces are not boundary of anything the cut removes. The fault is that the cut sections `X` along faces that are not the tool's material boundary; the crack, its order-dependent class, the broken seam bookkeeping and the undivided patch are all downstream of that one section. A region whose class flips with minting order is not a region to classify.

"How is the wall's patch divided where it turns one-sided" has the same answer: the plate's top, as stored, describes the plate, not the tool's material. In the material, the top is the top less the foot's outline. That imprint is exactly what `∪` does when it glues two faces at rest; it is not a new reader.

**Measured** (main `571615865d`, the row's fixtures, at rest and turned, unrecorded inputs; every body tier 3 and 3′):

| cut part | today | `X − (bolt ∪ plate)` | volume |
|---|---|---|---|
| post | `ToolSeatCrossed` | typed empty | — |
| pin | `ToolSeatCrossed` | 1 solid, 6 faces | 0.020 |
| bead | `ToolSeatCrossed` | 1 solid, 6 faces | 0.006 |
| slab (faces only) | `ToolSeatCrossed` | 1 solid, 14 faces | 2.276 |
| cap (faces only) | `ToolSeatCrossed` | 1 solid, 10 faces | 0.168 |
| over | `ToolSeatCrossed` | 1 solid, 8 faces | 0.030 |
| collar | `ToolSeatCrossed` | 1 solid, 10 faces | 0.096 |
| above / below (a face on the seat) | `VertexReadTwice` | typed empty | — |
| ledge, corner | — | build | 0.060, 0.018 |

`bolt ∪ plate` is one solid of 11 faces in either order: the foot is gone and the top carries its outline. The intended bodies in the row's "Done when" are all met, and the faces-only rows and the face-on-seat rows come with them. (sure, for these fixtures.)

**The answers as final states.**

1. **Tool as material (recommended).** One sentence of semantics; the cut's classification, sections and finish know nothing of seats in the tool.
   - Deleted: `ToolSeatCrossed`, `boolean::seat_crossed`, the cut-tool branch in `vtxfac::classify_vertex_on_face`, the "touches the wall in a cut by the seat" exception, the cut arm of `ops::Held::apart`.
   - The wall reading (`vtxfac::Wall`, `WallRead`) stays, for the roles where pieces survive, and is no longer asked a question it has no answer to.
   - Nothing is decided twice: the record is the decision, handed to the union as a given pair, the way an n-ary union's fold steps are handed their verdicts (`boolean::nary`, `Verdicts::Given`). (likely — see below.)
   - Result faces cite the tool's original faces through the union's lineage; the tool's non-area records ride it.
   - Leaves possible: a refusal where `∪` has no arm for the seat's carrier pair (below). That is the frontier's per-arm refusal, honest about what is unbuilt, not a seat rule.
   - Reversible: it is a door in front of an unchanged cut.
2. **Fold at the door: `(X − bolt) − plate`.** Also measured: every row above builds, in both orders, to the same bodies. Defensible. Worse as a final state: it forms intermediate bodies of `X` (`pin − plate` is two solids), so an intermediate sliver or self-touch can refuse in one order and not another, which [operations.operators] calls a kernel defect; the seat reappears at step two as a flush pair between operands and has to be handed along by lineage; and the tool's material is recomputed for every cut instead of once per value. Today's refusal text already tells the user to do this by hand ("cut by each piece of the tool in turn") — a recourse that is an exact rewrite the kernel can perform does not earn a refusal.
3. **Mint no crack inside the single pass** (the row's second direction). Same bodies. But every reader of the cut (vertex pass, face–face stage, edge-in-face, finish) gains "unless this is a cut and the wall is the tool's", and the patch division re-implements the union's imprint inside the cut. The same thing represented twice, kept in step by hand.
4. **The finish drops the crack** (the row's first direction). Reject: it keeps the section that should not exist and compensates after it — a class rule for a thing with no class, seam bookkeeping taught to lose a completed pair, and a patch divider that still has to be built. (sure)
5. **Keep the typed refusal.** Not acceptable as a final state for any cut: the result is well defined, representable, and built by an exact rewrite. (sure)

**Worked example, `bead − seat`.** The bead straddles the bolt's side and dips into the plate. Today the top's section runs under the bolt's foot, so the crack over the foot opens into the bead's outer piece. Against the material, the top ends at the foot's outline, the foot does not exist, and the bead is cut by three faces — the bolt's side, the top outside the foot, and nothing between — leaving the block `[0.2, 0.35] × [−0.1, 0.1] × [0.2, 0.4]`, volume 0.006.

**What it does to the rest.**
- `S − X`, both `∩`, `∪`: unchanged; they do not pass this door. The bridged union's `TouchesItself` stays, as the ratified limit it is.
- Faces-only seats (slab, cap): covered, measured above; the result gate is no longer their only guard.
- Curved seat (a shaft in a bore): the silent wrong body cannot arise in a cut, because the cut never sees the two walls. It builds if `∪` glues the coincident cylinders, else refuses naming the pair. The gate no longer has to tell a curved area record from a tangency for a cut: the union takes the record under its class. (unsure — no coaxial fixture exists; not measured.)
- The editor: `t − (p ⊔ q)` stays one kernel call with the ⊔ whole, so pieceless bodies (imports, severed parts) are reached too.

**Ratified text.** [operations.operators] says what `−` and `∩` do to a ⊔ they read as the cut part and that `∪` does not absorb one, but says nothing of a ⊔ as a cut's tool. It was written in the `[ev] D′` PR (2026-10-22), a design conversation, so I weigh it as Ev's. I would add one sentence there, for sign-off: *"A cut removes its tool's material, so the tool's pieces and the meetings between them do not reach the result: `t − (p ⊔ q)` is `t − (p ∪ q)`."* This does not contradict "a meeting inside an operand is never changed by an operator above it": the ⊔ value is untouched and nothing of the meeting is in the result to be changed. The closed row's design for PR 6613 ("`X − S` refuses typed") is agent-written sequencing, not a ruling, and is replaced.

**Confidence.** Recommendation: likely. Semantics of `X − S`: sure. The measured table: sure. That 1 beats 2 as a final state: likely. Curved seats and the given-pair plumbing: unsure.

## For the orchestrator

- **Probe.** `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-crack-A-probe.rs` (a module added to `crates/topo/tests/all.rs` in my clone, not committed). It unites the bolt and plate as two bare bodies and cuts each block by the result, and runs both fold orders. One run, `cargo test -p topo --test all zz_probe_fused_tool -- --nocapture`, 1 passed; all rows at rest and turned.
- **Not checked.**
  - The brief's crack facts (order-dependent class, `LaminaWedge`, `JoinDesync`, `EdgeInFace { gap: Crossed }`) live on the prototype branch `ed87ae56c3`; I did not rebuild it. My recommendation does not rest on them beyond "the crack exists".
  - My probe united unrecorded bodies, so `∪` decided the seat by its own margin. Feeding a body's own area records to a union of that body's solids as given pairs is not an existing entry point: `nary_boolean` takes member bodies and judges pairs itself. Whether a record carries enough (its class, a margin for `GivenDecision`) to stand as a given pair is unread. If not, the first build measures instead, which is sound (same verdict, by [coincidence.booleans]) but is the re-decision the last fork objected to in the editor; say so in the row.
  - Reading one body's solids as separate union members needs a solid-extraction step I did not find a helper for.
  - Lineage composition (result face → fused face → tool face) and the carry of the tool's line and point records through the union: assumed, unread.
  - A tool holding an area self-touch inside one solid, or a slit: its material needs a unary heal, not a union of solids. No such value builds today (`TouchesItself`); I did not probe a slit-bearing tool.
- **Brief.** It says to delete the target dir; the dispatch says the orchestrator will. I left `/home/user/fork-crack-target` in place. The clone needed a background fetch once (three clones ran at once and hit the 120 s limit); nothing is running now.
- **Off the question.** The tally row `the-editors-distribution-over-a-seat-retires-for-the-kernels-whole-body-boolean` says a ⊔ as a cut's tool is something "the distribution never built either"; under this answer it needs no editor work at all, and the row's note should be updated when this lands. The curved-seat row's "Done when" (the gate telling area from tangency) becomes moot for cuts and remains only for a bridged `∪`.
- **Rows that flip under answer 1:** `a_cut_across_its_tools_seat_refuses_typed` (all seven build or are empty), the `above`/`below` half of `a_tool_touching_its_seat_from_one_piece_is_cut_away` (to typed empty), and the editor's `a_seat_as_a_cuts_tool_is_read_whole_and_refuses_where_the_cut_crosses_it`.
