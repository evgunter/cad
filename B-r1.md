# Fork `crack`: designer B, round 1

## For Ev

**Recommendation (likely).** `X − S` should be `X` minus S's *material*, and the cut should get there by reading its tool as material before it cuts. When a cut's tool holds an area seat, the cut door first fuses the tool's pieces along that seat. The seat's own recorded decision is the glue's verdict, and nothing is decided again. Then it runs an ordinary cut. Under this:
- the crack is never built in a cut;
- `ToolSeatCrossed`, `seat_crossed`, and the cut branches in `vtxfac` and `Held::apart` are deleted;
- the "one-sided wall" division comes from the union's existing coplanar arm;
- the faces-only seats (slab, cap) and the curved seats are covered by the same rule.

The ops that keep a `⊔`'s pieces (`S − X`, `∩`, `∪`) keep today's wall reading unchanged.

**Terms.**
- *Seat*: the face–face record between the bolt's foot and the plate's top.
- *Wall*: that seat as `vtxfac` reads it, two faces, each holding its own piece's material on its own side.
- *Gap reading*: the wall reading's implied model, where the two faces are held apart with an infinitesimal gap between them that belongs to neither piece.
- *Crack*: the zero-thickness region of `X` inside that gap.
- *Material of S*: the point set `bolt ∪ plate`, which has no gap.
- *Fuse*: S's pieces united along S's area records.
- *Revert*: the finish's flip of the tool's kept faces in a cut (`revert(BinA)`, Eq. 15.1).

### Premise check: the crack is not order-dependent, and the fault is the cut's complement
- **The crack's class is not arbitrary (sure).** The order the kernel uses mints it Out of both pieces. That is exactly the gap reading, the one that matches two pieces held apart by a record. The other order (In both) would claim the pieces overlap, which is false. So the gap reading is not a bug: it is the right reading wherever an op keeps S's pieces.
  - `∩` and `S − X` drop the crack, as Out of S.
  - In `∪`, the crack is real union material that fills the gap under `X`. Its zero-height strips become the self-touch's tip.
  - Measured with the held-apart gate switched off: every bridged `∪` then refuses `LaminaWedge`. So does `slab ∪ seat`, which has no crack at all. The `∪` refusal is therefore the ratified self-touch limit, not the crack (likely).
- **The fault is the cut's complement of a held-apart pair (sure).** A cut is `X ∩ complement(S)`, and the finish builds it by reverting S's faces inside `X`. Reverting two faces that lie on each other turns the gap into a zero-thickness sheet of `X`'s material. That sheet is the crack, plus the reverted seat patches it zips with.
  - A regularized `X − S` has no such sheet, because the material of S has no gap.
  - The cut is the only operator that reverts an operand (Eq. 15.1). So it is the only place the gap reading gives a wrong body.
- **The faces-only and curved cases are the same defect (sure).** In `cap − seat`, the crack is the cap's material between the foot and the top, bounded only by the two reverted seat faces: the "floor held apart by the record". The curved row's measured wrong body works the same way: one solid where two belong, joined through a zero-thickness tube of the block inside the shaft–bore gap.
- **The row's "Done when" outcomes are right, but its two directions are not the choice.** "Never mint the crack" would wrongly change the ops that need the crack. "Drop it in the finish" repairs after the fact what the cut should never have built.

### Measured (each case at rest and turned; probe on main `571615865d`)
Each block was cut by the fused tool (`X − (bolt ∪ plate)`). The fold `(X − bolt) − plate` was also run as a check and gave the same results.

| Cut | Result | Faces | Volume |
|---|---|---|---|
| `post` | typed empty | – | – |
| `pin` | the stub below the plate | 6 | 0.020 |
| `bead` | the piece outside both | 6 | 0.006 |
| `slab` | – | 14 | 2.276 |
| `cap` | – | 10 | 0.168 |
| `over` | – | 8 | 0.030 |
| `collar` | – | 10 | 0.096 |
| `corner` | – | 6 | 0.018 |
| `ridge`, `down`, `above`, `below`, `blade` | typed empty | – | – |

- Every built result is at tier 3 and 3′.
- Today `above`, `below`, `blade` and `corner` refuse `VertexReadTwice` when cut by the seat.
- **Curved:** a shaft in a bore, fused, then cut by the block, gives two solids with volume 0.374938. The flush shaft gives two solids with volume 0.576. Turned seams and aligned seams give the same. The curved row measured both as wrong single solids.

### The answers, as final states

**A. The cut reads its tool as material (recommended).**
- **What it makes true.** `t − (p ⊔ q) = t − (p ∪ q)` for every seat a fuse can glue.
  - No cut ever reverts a held-apart pair, so the lamina cannot be built.
  - The cut's inner reduction can take its tool with no area record at all, so a tool wall is unrepresentable there.
- **What goes.**
  - `ToolSeatCrossed` and `seat_crossed`.
  - `vtxfac`'s cut-only branch, which today reads a touch on the wall as material, an ad-hoc form of this same rule.
  - `Held::apart`'s tool branch. Only its `TouchesItself` branch, for `∪`, remains.
- **`bead − seat`'s division.** The fuse's coplanar union leaves the plate's top as one face with the foot cut out of it. The fused tool has 11 faces, as measured. So no reader has to divide a seat face along the other face's edge inside one operand. The `EdgeInFace{Crossed}` gap never arises.
- **Naming.** The cut's lineage is composed through the fused tool, so callers still see S's faces. The editor's row needs no change and flips to building.
- **Its costs.**
  - A second boolean inside the cut.
  - The cut inherits the fuse's frontier: a seat no union arm glues refuses the cut. Fusing only the seats that `X` reaches would avoid that refusal; it is a pure optimisation.
  - The fuse must tell an area seat from a tangency (a rod lying on a plate must not be glued). For planar seats that is read today (`contact::area_record`). For curved seats it is the curved row's open question (carry the record's class), so the two rows share that dependency.
- **Reversible:** yes. It is internal to the door, and B below gives the same bodies.

**B. The finish regularizes.** Keep the single simultaneous pass and drop the zero-volume residue after the zip, the record saying where. It needs four new pieces:
1. seam bookkeeping that lets a completed pair lose both faces;
2. a new reader that parts the plate's top along the foot's edge inside one operand (the gap measured as `EdgeInFace{Crossed}`);
3. fin removal, where the crack hangs off a real piece (`bead`, `slab`);
4. the same again for the faces-only and curved seats.

- **Pro:** one pass, with direct lineage.
- **Con:** it builds a wrong intermediate body and repairs it with four output-stage surgeries keyed by records, and a held-apart pair can still be reverted.
- I would not choose B.

**C. Keep the refusal.** This is not an acceptable final state for any op. Every refused cut has a right body, which the fold that users would write by hand already builds. Cutting a pocket for an assembly is a real modelling act.

(Minting X's section by material, one section for both faces, is not viable. Where the wall turns one-sided, the section of `X` is a branching graph, not a polygon, so the join's polygon pairs cannot hold it.)

### Effect on the rest
- **`S − X`, `∩`, a bead on one piece:** unchanged (sure).
- **Bridged `∪`:** still refuses, `TouchesItself` at the gate, or `LaminaWedge` with the gate off. That refusal is the ratified self-touch limit and is unchanged.
- **Faces-only cuts (slab, cap):** they now build.
- **Curved cuts:** they build once the fuse can read a curved seat's class. The curved row and the flush row (`a-seat-held-apart-across-two-shells…`) both measured cuts. Their remaining scope narrows to ops that keep the pieces, and should be re-read after this change.

### Ratified text
- **Add to `[operations.operators]`, after "`−` and `∩` distribute over `⊔`":** "A cut reads its tool as material: `t − (p ⊔ q)` is `t − (p ∪ q)`, the tool's meeting glued on the `⊔`'s own decision, and none of it reaches the result."
- **Provenance.** The neighbouring text, "a meeting inside an operand is never changed by an operator above it", was written in your D′ `[ev]` PR (`679b4abbf7`). Its subject is `∪`. A cut's tool has no surviving meeting, so the new sentence does not contradict it, but it is new design text and needs your sign-off.

**Confidence:** the recommendation is **likely**. The semantics, and that the gap reading is right for the other ops, are **sure**. The measured bodies are **sure** for the plain door; the fuse in the final design is handed the record's verdict rather than deciding it fresh. `∪`'s crack becoming the self-touch's tip is **likely**.

## For the orchestrator

- **Probes.**
  - `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-crack-B-probe.diff` holds:
    - the topo probes `probe_cut_by_the_tools_material` and `probe_bridged_union_ungated`;
    - an env-gated `PROBE_NO_HELD` skip in `Held::apart`.
  - `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/fork-crack-B-curved-probe.rs` is the sweep probe.
  - The clone at `/root/.local/share/cad-work/designer-crack-b/cad` still carries them. The target dir is left in place, per the dispatch, which overrides the brief's "delete it".
- **Assumption.** In the probes, the fused tool is built by a plain `boolean_op_with(Union, bolt, plate)`, which decides the glue itself. In the final design the fuse is handed the seat's record as its verdict (`Verdicts::Given`, as the n-ary steps are). For these planar and coaxial fixtures that cannot change the body. I did not build the given-verdict fuse.
- **Brief and row errors.**
  - "The crack's class depends on order" mis-frames it: the Out order is the gap reading and is correct.
  - "Out, `X − S` and `∪` keep it, zipped with the reverted seat patches": `∪` keeps no reverted patches. Its measured `LaminaWedge` is the self-touch limit, and the crack-free `slab ∪ seat` gives the same refusal.
- **Rows that flip under A.**
  - The `above` and `below` rows of `a_tool_touching_its_seat_from_one_piece_is_cut_away` (→ empty).
  - The cut-order rows for blade and corner in the "still read twice" test.
  - `a-declared-wall-is-read-only-by-a-vertex-crossing-it` narrows to ops that keep the pieces.
- **Not checked.**
  - Whether a curved bridged `∪` is caught at the finish (`LaminaWedge`) or ships silently.
  - Whether any non-cut op reaches the flush "two shells of one solid" case.
- **Independence.** I did not read designer A's files in the scratchpad.
