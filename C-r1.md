## For Ev

**Recommendation: a cut reads its tool as material.** `t − (bolt ⊔ plate)` is computed as `t − (bolt ∪ plate)`. Before the reduction, the boolean door unites the tool's pieces wherever the `⊔` recorded a meeting between them, citing those records rather than deciding the meetings again. The seat is then interior to the tool, and the existing pipeline cuts an ordinary solid. No crack is minted, there is nothing for the finish to drop, and no reader needs to divide a seat patch. `ToolSeatCrossed` is deleted, along with every branch that raises it. Confidence: **likely**. It was measured, with one gap noted under "Measured".

**Correction to the premise (first).** The crack is not a mint-order accident, and neither the finish nor the X-side construction is where the fault lies.
- *What the crack is.* A record says the two pieces touch with disjoint interiors: held apart by a gap of zero thickness. The pipeline subtracts by keeping `revert(B)` face by face (Eq. 15.1). Reverting a body whose two faces lie on each other gives its complement plus a zero-thickness sheet of "outside" between those faces, and the crack is `X` intersected with that sheet. Its class is **Out of both pieces**, the only class consistent with disjoint interiors (**likely**). The other mint order, "In both", would mean the pieces overlap by zero thickness, which contradicts the record. So that order is wrong, not an equal alternative.
- *The fault.* The binary pipeline assumes every face of an operand has the operand's material on one side and none on the other. `one_solid` flattens a `⊔` whose pieces touch over an area into one solid with two outer shells, which breaks that assumption: each seat face has material on both sides. For an operand whose pieces the result keeps (`S − X`, `∩`, `∪`), that gap is real structure, and the existing one-wall reading (`vtxfac::Wall`) handles it. For a cut's tool it is information the operation cannot use. **Nothing of a tool's own meetings can survive a cut** (**sure**): where the seat is inside `X`, it has tool material on both sides, so it bounds nothing; where it is outside `X`, it is discarded with the rest of the tool.
- *The row's "Done when".* Both of its directions work inside the pipeline: a finish rule, or no X-side crack. The fault is one level up, at the door, in what the cut takes as its tool. The bodies and tiers the row asks for are correct and are kept.

**Measured** (scratch probe on main `571615865d`, the lane's fixtures, at rest and turned). The probe first built `bolt ∪ plate`, then cut each block by it:

| cut | today | probe result |
|---|---|---|
| post | `ToolSeatCrossed` | ∅ (empty) |
| pin | `ToolSeatCrossed` | 1 solid, 6 faces, 0.020 (the stub) |
| bead | `ToolSeatCrossed` | 1 solid, 6 faces, 0.006 |
| slab | `ToolSeatCrossed` | 1 solid, 14 faces, 2.276 |
| cap | `ToolSeatCrossed` | 1 solid, 10 faces, 0.168 |
| over | `ToolSeatCrossed` | 1 solid, 8 faces, 0.030 |
| collar | `ToolSeatCrossed` | 1 solid, 10 faces, 0.096 |
| split post | `ToolSeatCrossed` | ∅ |
| above, below (a face on the seat) | `VertexReadTwice` | ∅ |
| blade (an edge in the seat) | `VertexReadTwice` | ∅ |
| corner (pairs on the seat) | `VertexReadTwice` | 1 solid, 6 faces, 0.018 |

- Every built body is clean at tier 3 and 3′ and carries no face–face record. Each volume matches the hand-computed one.
- The fold `(t − bolt) − plate` gave identical results.
- **The gap:** the probe let the union *decide* the seat. The design passes the record in as the given verdict, and that path was not exercised.

**The final state under this recommendation.**
- **Cuts.** The tool of a `−` arrives at the pipeline as its material: its pieces are united where its records join them, and pieces with no record stand as they are. Every cut in the table builds.
- **Bead (a wall that turns one-sided).** The union itself divides the plate's top along the bolt's foot edge: the top becomes one face with the foot as a hole, and the foot face is gone. The cut then sees an ordinary concave edge. No reader in the cut divides a patch (**sure**, measured).
- **Faces-only seats (slab, cap).** Covered, as measured. Today the gate refuses them, and before PR 6613 `cap − seat` built a silently wrong body.
- **Curved seat in a cut (a shaft in a bore).** The union glues the two coincident walls if that carrier pair has a glue arm. If it has none, the union refuses with that arm's own typed refusal, never silently. It needs no planar-only area test: the union reads each record as its own kind (area, line or point). **unsure**: I did not build a coaxial fixture.
- **What is deleted.**
  - `BooleanError::ToolSeatCrossed` and its kind, tag and editor mapping.
  - `boolean::seat_crossed`, and the cut branch in `vtxfac` (the refusal, plus "a touch reads both faces only in a cut by the seat").
  - The `ToolSeatCrossed` branch of `ops::Held::apart`.
  - The `VertexReadTwice` refusals in a cut by a seat.
- **`S − X` and `∩`: unchanged.** The one-wall reading stays. It still mints the crack, which reads Out and is dropped, correctly.
- **`∪`: unchanged.** Every `∪` that crosses the seat bridges the two pieces and refuses `TouchesItself`. The crack is kept inside that refused shell as a zero-height strip where the post fills the gap. That strip is the correct end of the gap, but in degenerate form: when area self-touch inside one shell is built, the strip has to collapse into one edge. That is the `TouchesItself` program's problem. Dissolving the seat cannot serve `∪`, because outside `X` the seat must stay held apart. (Derived, not measured: **likely**.)
- **Reversible.** It is one step at one door.

**Alternatives weighed.**
1. **Regularize in the finish.** Drop zero-volume shells, teach the seam bookkeeping that a completed pair may vanish, and divide a seat patch where the other face stops covering it. *For:* it is one rule for every op, so a future `∪` would collapse its strip by the same mechanism. *Against:*
   - The pipeline still mints a degenerate region, then searches for it.
   - It weakens a desync check that catches real bugs ("a seamed operand that lost its kept component").
   - It needs a new reader that divides a face along another operand's edge lying in it (today `EdgeInFace { gap: Crossed }`).
   - It keeps every cut-by-seat special case.

   **Reject for cuts.** It may come back as the `∪` program's answer.
2. **One section per wall.** Cut `X` once along the seat, with one section loop on `X` paired with both seat faces. This is the prior fork's "every reader reads one wall" carried into the join. *For:* it is the most faithful to that fork. *Against:*
   - The join pairs one section loop of `X` with one of the tool (1:1). This needs a 1:2 pairing, which is new structure.
   - For a cut, all that machinery computes a section that no result face uses.
   - It still needs (1)'s patch division for `bead − seat`.

   **Reject for cuts.** Possibly the right answer for `∪` later.
3. **Fold `(t − p) − q`.** Same bodies as the recommendation (measured). *Against:* it takes n−1 extra cuts. The seat has to be carried through each intermediate's lineage into the next step's verdicts. Each intermediate is a place where one order can refuse while another builds. The union states the same law once.
4. **Keep the typed refusal.** Not acceptable for any cut. The body is well defined and builds, and drilling through a stack is the commonest cut an assembly gets. The only refusal a cut by a `⊔` keeps is the union's own typed frontier refusal, for a carrier pair with no glue arm.

**Ratified text.** I would add one sentence to DESIGN `[operations.operators]`, after "`−` and `∩` distribute over `⊔`" (that sentence comes from your D′ PR):
> "A cut's tool is read as its material: `t − (p ⊔ q)` is `t − (p ∪ q)`, the pieces united where the `⊔` recorded their meetings, since nothing a cut keeps holds any of its tool's own meetings."

- This is new binding text, so it waits for your sign-off.
- It does not conflict with "∪ does not absorb ⊔ … never changed by an operator above it". That clause governs what a result carries, and a cut's result carries none of the tool's meetings.
- It does not conflict with `[coincidence.booleans]` "carried": the union reads the record's verdict and decides nothing.
- No README clause names `ToolSeatCrossed` or the wall, so none changes. The `[topology.conventions]` refusal for an area self-touch inside one shell stays for `∪`.

**Confidence.**
- Fault and level: **likely**.
- Outcomes for the planar cuts: **sure**, measured (modulo the decided-versus-given verdict).
- Curved seat: **unsure**.
- `∪` strip: **likely**, not measured.

## For the orchestrator

- **Probe.** The scratch test diff is at `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-crack-C-probe.diff`. It appends a test to `crates/topo/tests/a_vertex_through_a_declared_seat.rs` that builds `topo::union(bolt, plate)`, then runs `X − fused` and `(X − bolt) − plate` for 14 blocks at both poses. It ran on main `571615865d` with `CARGO_TARGET_DIR=/home/user/fork-crack-target`, about 40 s on a warm cache. The clone is restored. The target dir was not deleted, as the dispatch line said.
- **Unverified: the given-verdict union.** The probe's union decided the seat itself. The design needs it to read the record as given (`Verdicts::Given` / `given_verdicts` exist in `boolean/mod.rs`). The records live on the `⊔` body, so they have to become cross-operand glue for a union of its solids, and I did not check that wiring. The prior fork rejected the fold because it "re-decides the seat at its second step". That objection applies to my probe but not to the design, provided this wiring holds.
- **Prior fork's objection to door grouping** (it "cannot reach a slit inside one solid"). An area self-touch inside one shell cannot be made today, because `TouchesItself` refuses it. If such a body becomes possible, dissolving it as a cut's tool needs a self-glue of the record's common region. That is the same surgery as the union's, restricted to one solid. The wall reading would mint the same crack for that body, so the objection does not separate the options. Not checked: whether an import can carry such a body at rest.
- **Curved residue.** If this recommendation lands, `work/seal/a-cut-by-a-curved-seat-keeps-its-two-faces-held-apart-by-the-record.md` narrows to `∪`. `Held::apart` reads only planar area records, so a `∪` bridging a curved seat may still build a shell holding both walls (unmeasured). Its "Done when" should be re-scoped.
- **Rows that flip.** Beyond the seven `ToolSeatCrossed` cuts:
  - `above` and `below` in `a_tool_touching_its_seat_from_one_piece_is_cut_away`;
  - `blade − seat` and `corner − seat` in `a_vertex_on_a_seat_that_is_no_crossing_of_it_reads_twice`;
  - the split post's cut row in `a_seat_divided_along_a_ring_or_met_with_a_seat_refuses_at_the_carry`;
  - the editor row `a_seat_as_a_cuts_tool_is_read_whole_and_refuses_where_the_cut_crosses_it`, which should give ∅;
  - the sentence in the module doc of `editor-core/src/eval/wire/distribute.rs` about a `⊔` as a cut's tool.
- **Lineage.** Result faces born from the tool must name the original pieces' faces through the union's lineage composed with the cut's. I believe this is standard composition, but did not check it against the editor's naming.
- **Brief.** "With the copies minted the other way round it is In both" is a prototype variant, not a free choice: `WallRead`'s docs fix the middle copy as holding neither In run. The brief's list of what refuses in a cut by the seat omits the `VertexReadTwice` rows above, which the same answer retires.
- No defects off the question were filed.
