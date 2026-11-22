## For Ev

**Your concern, answered: no. With records following the file's references, an import reaches no at-rest state that native construction cannot reach.** (likely, and now probed on the case that carried the weight)
- **A file can say one thing about records:** "these faces of one shell lie on one surface". Every native verb pattern for that sharing has a producer (table below).
- **The riskiest pattern is now settled by a run.** That pattern is one record worn by faces facing opposite ways in one solid. A native `topo::union` of `[0,2]×[−1,0]×[0,1]` and `[1,3]×[0,1]×[0,1]` leaves both `y = 0` faces on one surface key with opposite senses, in both operand orders, and tier 3 accepts it.
- **Import is narrower than native.** It never shares a record across shells or solids; native verbs do (PR 3412: a chart is body-wide).
- **Today's bit dedup reaches the same states**, and more, whenever printed numbers agree, silently. Keeping it does not answer your concern.

**Each pattern a file can state, and its native producer**

| One record worn by… | Native producer | Gate |
|---|---|---|
| faces not edge-connected | a boolean that divides a face: every fragment keeps the operand's record | admitted |
| faces facing opposite ways, one solid | the union above (probed) | admitted |
| two edge-adjacent faces of one curved surface | a merge run left "the act alone" on one record | admitted |
| two edge-adjacent faces of one plane, facing one way | a half revolve: its two caps across the axis edge wear one record (`revolve_partial.rs` asserts it and all tiers) | admitted (see *a separate defect* below) |
| faces in two shells or two solids | a disconnecting subtract, then the mover | admitted; import never states it |
| opposite facings meeting at one edge (a pinch) | none | refused by tier 3 check 4 (`LaminaWedge`), which reads geometry, so it fires with one record or two |

**The invariant, and where it lives.**
- **What a record means.** A record says only that every face wearing it lies on its carrier. The edge ladder certifies each face's boundary on its record, and the one at-rest gate judges the body, for native and imported bodies alike.
- **A future mismatch is fixed at the gate.** If a pattern ever turns up that the gate admits and no verb produces, that is a gap in the kernel's at-rest statement, and the fix is a gate check. It is not an importer filter: `[import]` step 4 says import "holds no idea of validity of its own".
- **So no filter is proposed.** Not "honour only same-facing sharing", not "only where native could produce it". The probe removed the one case a filter was held in reserve for.

**Recommendation (likely):**
1. **Import.** Faces of one shell that cite one surface entity wear one record.
   - Records a normalization mints (knot-split sub-patches) are their own.
   - Two distinct entities become one record only through the reported `one_record` act.
   - `surface_sig` and its reads are deleted.
2. **Export.** Write one surface entity per surface key, not one per face.
3. **No identity stamp.** The row's premise is stale: the stamp types are gone, and N6 now says the kernel carries no recipe provenance.
4. **Pins, in the same unit.** One import row for each pattern above that a file states by reference, each held to its native twin's at-rest verdict, plus the pinch held to its refusal. They are tests, so no non-test code changes for them, and they keep your question answered as the gate changes.

**The corpus** (a text scan of the tree's 114 STEP files; not a build):
- Six files share by reference: 22 records, each in one shell, every face facing one way.
- 20 are edge-connected halves of a curved surface; 2 are separated fragments.
- None states a pattern without a native producer.
- Today's dedup shares all 22 as well. A changes only the 96 pairs of distinct but bit-equal entities, which become reported `one_record` joins.

**A separate defect, which this decision should not carry: bodies whose faces are not maximal.**
- **The ratified form.** DESIGN says a body is "the unique complex with maximal faces and maximal edges … and the form is checked at rest on the result alone".
- **The gap.** Tier 3 checks maximal edges only. Maximal planar faces are checked only at the boolean's input (`gate_maximal_faces`, F7, which refuses any planar pair on one key across an edge).
- **Who ships such bodies.** Import does: `fuse_boxes.step` gives 14 faces against 10 natively, its tops each tiled 3-on-one-plane-record by `one_record`, with no merge after. So does the native half revolve: its two coplanar caps across the axis edge wear one record and are not merged.
- **Consequence.** Both pass every tier, and (by reading, not run) both are refused as a boolean operand.
- **So the state is not import-only, and it does not depend on how records are chosen.** It is a kernel form the gate does not check. The fix spans `[topology]`'s tier list, the half revolve and the import pipeline's end, which makes it its own row and its own question to you.

**Ratified text needing your sign-off (this decision only).** One sentence in `[import]`: "Faces of one shell that cite one surface entity wear one record; two entities are two records unless the import's reported one-record act joins them; export writes one entity per record."

**Confidence.**
- No import-only state from records: likely. The opposite-facing case is probed; the other rows are read from code and pinned tests.
- No filter: sure, given the probe.
- Records over bits: likely.
- No stamp: sure.
- The half revolve shares the maximal-faces gap: likely. The one record on both caps is asserted in a test; that F7 refuses it is read, not run.
