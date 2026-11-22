## For Ev

**Recommendation (likely): no identity token for imported bodies. Instead, the only identity the kernel reads at rest — which surface record (key) a face wears — comes from the file's own references, never from comparing floats.**
1. **Import holds a table of surface records.** Each record is a file surface entity, or one that a normalization mints (a knot split's sub-patches). A face names a record and a sense. Two faces citing one `#n` wear one record.
2. **Two distinct entities become one record only through healing's existing reported act** (`[import]` step 3, `one_record`): a Zero at the file's band, faces facing one way, in one solid.
3. **The bit-signature dedup (`surface_sig`) is deleted at all six of its readers.** They are key sharing, the edge-description choice, healing's grouping, and three vertex readings. Each becomes a record comparison.
4. **`step-export` writes one surface entity per surface key**, where today it writes one per face. A round trip then keeps the key partition because the file states it.
5. **`[import]` gains one sentence:** "A surface entity the file states once is one surface record; two entities are two records unless healing joins them, reported."

**Premise check: the row is stale (sure).**
- The types it extends are retired, and you approved the retirement on PR 4322: `GeomSource`, `SourceExpr`, `GeomOrigin`, `AxisSource`, `stamp_minted` and `topo/src/source.rs`. Provenance is the document's, from reads and names.
- `AXIS-DECLARATION-DESIGN.md` is deleted, so "absence refuses" binds nothing.
- The boolean glues where a margin is decided Zero. The chart readings ask only "same key, or shared payload?".
- `[import]` already makes imported geometry "not distinguishable from native geometry", so the row's complaint is the ratified goal.
- Its four design questions have no subject:
  - **Namespace:** nothing is minted that could collide.
  - **Instancing:** each copy is grafted under fresh keys.
  - **Placement order:** `transform_rigid` keeps keys and has nothing to clear.
  - **Scope of a body never evaluated from a document:** `[import]` step 4 already says an imported contact is decided by value and is an `unproven-coincidence` finding.

**What the row is still evidence of (sure on the facts).** Inside the importer, "which record does this face wear" is held twice and reconciled by float bits:
- Each face carries its own copy of the surface value, beside a parallel slice of its file record.
- Sharing is recovered by comparing `to_bits`.
- Healing and `one_record` write each moved surface back to every face by hand.

Three consequences:
- **It is a bit-identity coincidence decision in production.** You retired that (#53) and declined an exception on PR 3410. The gate does not see it because it is hand-rolled.
- **It disagrees with `one_record`.** That act keeps two opposite-facing faces in one solid as two surfaces, and bit-equal records silently override it.
- **It aliases every `Approx` surface** to one key (a filed row, dormant today).

The bit dedup exists only because the exporter flattens sharing (`adopt.rs` says so). It is agent-written M7-3 text, in no DESIGN clause.

**The two final states**

*A. Records follow the file (recommended).*
- **Makes true:**
  - one source of truth for which surface a face wears;
  - no float comparison decides identity;
  - every join of two records is a reported act;
  - a round trip keeps the partition by construction;
  - the `Approx` alias cannot occur;
  - a knot split cannot leave two different sub-nets on one record, because the split mints records.
- **Costs:** a foreign file that states one B-spline surface as two equal entities, with an edge between them, refuses, typed, until `one_record` learns to read a spline pair. Two equal planes or round surfaces are joined, reported. I did not measure how often foreign writers duplicate spline records (unsure).
- **Reversible:** yes; it is internal to the two exchange crates.

*B. Close the row as superseded and keep the bit dedup.*
- **Makes true:** nothing changes, and duplicated spline records keep importing.
- **Leaves:** a production bit-identity consumer outside the gate, the disagreement with `one_record`, the `Approx` alias, and the double bookkeeping.
- If you prefer B, choose it as a gate-listed exception to the bit-identity retirement, not left unseen.

**Worked example: a kernel cylinder wall split into two faces on one key.**
- *Today:* export writes two equal `CYLINDRICAL_SURFACE` entities, and import puts them back on one key because the re-parsed bits agree. It works only while printing and parsing round-trip exactly.
- *Under A:* export writes one entity that both faces cite, and import shares the key because the file says so.
- *A foreign file with two equal cylinder entities:* `one_record` reads the gap where the faces are, decides Zero, joins them and reports it. With two equal B-spline entities, the import refuses typed, naming `one_record`'s missing spline lane.

**Rejected (likely): letting a shared file entity count as structure at the coincidence door.** `[intent.stored]` says adoption recovers descriptions, never intent. A writer sharing an entity is the writer's artefact, not the author's statement. A shared entity decides which record a face wears, nothing more.

**Ratified text:** only the added `[import]` sentence. It needs your sign-off.

**Confidence**
- The premise is stale, and no token should be built: sure.
- A over B: likely. The open risk is how often foreign writers duplicate spline records.
- The exporter change is required for A: likely. Without it, our own round trips of a split spline face would refuse. I read this from `adopt.rs`'s must-carry arm and the writer's `SharedIds`; I did not run it.
