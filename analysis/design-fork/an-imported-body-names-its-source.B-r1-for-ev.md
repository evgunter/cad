## For Ev

**Recommendation (likely).** Build no file-derived identity token. The only identity the kernel reads at rest is which surface record a face wears; the importer should take it from the file's references and from the one reported act that already exists.

1. **Import.** Faces citing one surface entity wear one record. Two distinct entities become one record only through `one_record`, the reported interpretation act that reads a Zero at the file's band where the faces are. The bit-signature dedup (`surface_sig`) is deleted.
2. **Export.** The writer emits one surface entity per surface key (today: one per face).
3. **`[import]` gains one sentence:** a surface entity stated once is one record; two entities are two records unless interpretation joins them, reported.

**Premise: the row is stale (sure; all three designers agree).** `GeomSource`, `SourceExpr`, `GeomOrigin`, `AxisSource`, `stamp_minted` and `topo/src/source.rs` are gone (PR 4322, "sounds great!"), and `AXIS-DECLARATION-DESIGN.md` was deleted at your request. So:
- **Collision:** no namespace exists. A record is an arena key, minted fresh.
- **Instancing:** each copy is grafted under fresh keys; copies are distinct by construction.
- **Placement order:** nothing is attached, so nothing is cleared.
- **Scope claim:** `[import]` step 4 already says imported contacts are decided by value and reported as `unproven-coincidence` findings.

**What the row is still evidence of (sure on the facts).** The importer holds "which record does this face wear" twice: a copy of the surface value on every face, and a parallel list of the file's entity per face. It reconciles them by comparing float bits at six sites (key sharing, the edge-description choice, healing's grouping, three vertex-rooting reads).
- That is a bit-identity coincidence decision in production, which you retired (#53) and declined to re-admit on PR 3410. The gate misses it because it is hand-rolled.
- It is silent, while the same join made by `one_record` is reported.
- It disagrees with `one_record`: two bit-equal plane entities facing opposite ways are "two surfaces, never asked" there, and one key here.
- It carries a filed alias defect (every `Approx` surface signs alike).
- It exists because our exporter flattens sharing (`adopt.rs` says so). It is agent-written spec text, in no DESIGN clause.

**Measured on the tree's 114 STEP files (1,030 faces), by text equality of fully expanded surface entities within a shell.**
- Sharing by reference occurs in 6 files (22 surfaces), all foreign writers or hand fixtures; none of our exports.
- Two entities with equal numbers occur in 15 files (96 groups). About half are our own exports, which change 2 removes.
- **No spline surface is stated twice, in any file.**
- No duplicate group mixes senses.
- The foreign duplicates are all planes or round surfaces facing one way, the case `one_record` joins. Largest: NIST `nist_ftc_09_asme1_rd.stp`, 34 groups, 33 with an edge between the two halves.

So deleting the bit test changes no corpus outcome, by reading; I did not run it.

**The two defensible final states**

*Mine (and, except for one clause, the third designer's).* A table of surface records per solid: the file's entities plus records a normalization mints (a knot split's sub-patches). A face names a record and a sense. Recognition and healing act once per record; `one_record` re-points faces; attachment reads no bits.
- **Makes true:** one source of truth; every join of two entities is a reported act; a round trip keeps record structure because the file states it; the alias defect cannot occur.
- **Leaves possible:** a file that states a spline surface twice with an edge between refuses, typed, naming the lane. Zero such files in the tree.
- **Reversible:** yes; internal to the two exchange crates.

*Close as superseded, keep the bit test (the first designer).*
- **Makes true:** nothing changes.
- **Leaves possible:** everything listed under "still evidence of".
- Its argument is that bits catch more sharing than entity ids. They do, but the surplus is what `one_record` already decides, with a report. Its knot-split objection applies to keying on face ids, which this design does not do.
- If chosen, it should be an explicit, gate-listed exception to the bit-identity retirement.

**Where I would not go.** "Distinct entities are always distinct records" is cleaner to state but contradicts `[import]` step 3 and refuses the NIST file's 33 half-cylinder pairs.

**Worked example.** A cylinder wall in two half-faces.
- Both cite `#40`: one record, because the file says so.
- They cite `#40` and `#41` with equal numbers: `one_record` reads the gap where the faces are, decides Zero, joins and reports. Today the bit test joins them first, silently.
- Same, but one face reversed: two records under this design and under `one_record`'s own rule; one key today.

**Rejected (likely):** letting a shared file entity quiet the `unproven-coincidence` lint. `[intent.stored]`: adoption recovers descriptions, never intent.

**Ratified text changed:** only the added `[import]` sentence.

**Confidence.** Premise stale: sure. No token: sure. Table plus `one_record` over keeping bits: likely (was likely with an open risk; the count closes it). Exporter change needed so our own split spline faces round-trip: likely, read not run.
