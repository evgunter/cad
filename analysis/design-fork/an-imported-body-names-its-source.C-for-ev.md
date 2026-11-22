## For Ev

**Recommendation: give an imported body no provenance stamp. Make the file's own references its identity, in the one form of identity the kernel still has, which is a shared surface key.** Concretely:
1. **Import.** Faces that reference one STEP surface entity share one surface record (`FaceSurface::Shared`). Two distinct entities are two records, however their numbers compare. This replaces today's bitwise dedup (`step-import/src/adopt.rs`, `surface_sig` / `attach_surfaces`).
2. **Export.** `step-export` writes one surface entity per kernel surface key, not one per face. Then a round trip keeps the body's sharing because the file states it, not because the bits happen to agree.
3. **Build nothing else.** No `Imported` origin mark, no minted ids, no namespace partition, and no re-stamp after placement.

Confidence: likely.

**What the identity means.** An imported body's construction is the file's entity graph. Two faces citing one `#n` surface are one surface: the same claim a shared key makes natively, such as a split's two halves on its one plane. Two entities whose values agree are two surfaces that coincide by value, like a native twin built from two equal literals. Any boolean or census that meets them decides that coincidence by margin and records it, as for native geometry.

### The premise, checked against the tree (sure on each point)
- **The mechanism the row wants to fill no longer exists.** `GeomSource`, `GeomOrigin`, `SourceExpr`, `AxisSource`, `stamp_minted` and `topo/src/source.rs` were all retired. You approved this in PR 4322 (FORK-S4-1: "provenance is the document's", meaning reads plus names, no stamp), and INTENT stage 4's unit E deleted the side tables. A grep of `crates/` finds `GeomSource` only in one test comment that records its removal.
- **No identity collision is possible.** `RecipeNodeId` is now a `MintId` (an ordinal plus a digest), and nothing else is minted that could collide with it.
- **`docs/AXIS-DECLARATION-DESIGN.md` is gone.** It was retired by the intent refactor (PR 3990) and deleted at your request on 2026-10-17, so "absence refuses" no longer governs any code.
- **None of the consumers the row lists reads a stamp any more.**
  - The boolean glues wherever a margin is decided Zero, imported or native alike.
  - The row doors and chart readings ask only "same key, or shared `Arc`?" (PR 3414; your PR 3410 ruling).
  - "Structural" is decided only at the document's coincidence door, from names and the symbolic evaluation.
  - Imported bodies never reach that door: there is no import node. ASSEMBLY.md's A8 says import-as-assembly-document is not implemented.
- **"Indistinguishable from a hand-built body" is the ratified intent, not a defect.** DESIGN `[import]` says: "Imported geometry is not second-class, and it is not distinguishable from native geometry either: no state is reachable only through import." An `Imported` mark would contradict that. I could not find the commit that wrote this in the shallow checkout. It reads as old D7 text.
- **Placement and instancing are already right.**
  - `transform_rigid` keeps every arena key, so sharing inside a copy survives placement.
  - Each placed copy is grafted with fresh keys (`graft_disjoint`), so copies are distinct by construction.
  - Contacts between copies are value-decided `CensusAtRest` rows. In a document these would be `unproven-coincidence` findings, which `[import]` already states.
  - The per-evaluation scope claim has nothing left to apply to. A key is scoped to its own body's lineage.

### What the row is actually evidence of (sure on the facts, likely on the reading)
- **The importer already restores sharing, but by comparing values.**
  - `entities.rs` resolves `ADVANCED_FACE`'s `#surface` reference, keeps only the parsed `Surface` in `FaceSpec`, and drops the id.
  - `attach_surfaces` then puts faces on one key whenever the surface's `f64` bit patterns are identical.
- **That is a production bit-identity coincidence decision.** It is the channel `scripts/gates/bit-identity-consumer.sh` says is retired with an empty allowlist. The gate misses it because it hand-rolls `to_bits` rather than using `bit_identity`.
- **It is also an unrecorded value-decided coincidence**, against `[coincidence]`'s "two distinct records at rest are two surfaces". In PR 3410 you asked why any case should be "an exception to not deducing identity from numerical equality". This is one such case, and it predates that ruling.
- **The bit test fails both ways.** It invents sharing the file never stated whenever two entities print the same bits. And it misses sharing the writer meant whenever the writer prints the numbers differently: M7-2's coplanar records "with different origins" already reach `adopt.rs`'s conventional rung for exactly this reason.
- **The exporter is why the dedup was needed.** It flattens key sharing on the way out: `advanced_face` prints a surface per face, and `SharedIds` caches only vertices and edges.
- **The asymmetry is the tell.** The importer already identifies topology by entity id (vertices by `VERTEX_POINT`, edges by `EDGE_CURVE`). Only geometry is identified by value.

### The final state, and the alternatives I rejected
- **Recommended (A).**
  - What it makes true:
    - A body's surface records are exactly the file's surface entities, per solid.
    - An export followed by an import reproduces the key partition.
    - No import code compares geometry to decide identity.
    - The `Approx` aliasing row filed earlier today (`step-import-surface-sig-signs-every-approx-surface-with-one-tag`) dissolves, because no signature is left.
  - What it costs: a third-party file that gives each face its own entity gets one record per face. The edge between two coincident records takes the existing conventional rung (`coincident_surfaces` plus `carrier_on_surface`). Any boolean that later meets those faces decides them by margin and records the decision. The price is the one you accepted in PR 3410: pcurve rows are re-minted rather than carried across untied keys.
  - Reversibility: easy. The change is local to the two exchange crates.
  - **Worked example.** A writer splits a cylinder into two half-faces.
    - Both cite `#40`: one key, and the seam edges adopt as on a native split body.
    - They cite `#40` and `#41`, which print identical numbers: today, one key, decided silently. Under (A), two records, with the edge adopted conventionally.
    - `#41` differs in its last digit: two records today and under (A). Under (A) the third case is no longer treated differently from the second.
- **(B) Keep the merge, but decide it by the carrier ladder's margin and record a row.** This complies with the letter of D10, but the importer would then make value decisions the file never asked for, at ε_in, and every import would carry coincidence rows no operation needed. Rejected, likely.
- **(C) Do what the row asked: an import-minted identity carried beside the body.** It would re-create the second identity carrier that FORK-S4-1 removed. It has no reader, it breaks `[import]`'s indistinguishability, and it is "if nothing needs this then it should not be done". Rejected, sure.

### Ratified text
- **Add one sentence to `[import]`:** "The file's references are the imported body's structure: faces citing one surface entity share one surface record; distinct entities are distinct records, and any coincidence between them is decided and recorded as a native one is."
- Nothing ratified would need to change. The bit-identity retirement and `[coincidence]` already imply the change.

### Not built, and why
- **No face-to-entity map on `StepImport`.** Nothing reads one. The future import-as-document door is where the file's ids belong: in an import node's cell names, read at the coincidence door as "one construction read twice". That door can re-parse its pinned file, so nothing is expensive to retrofit. Likely.
- **No file axis-placement sharing**, such as two cylinders citing one `AXIS2_PLACEMENT_3D`. The kernel has no axis record now, and coaxiality is a margin that is recorded. If it ever becomes structure, that is the same document door's job. Likely.
