## For Ev

**Recommendation: build no import identity stamp. Make the file's own references decide which surface record a face wears, and delete the importer's bit-equality dedup.** Confidence: likely.
1. **Import.** Faces citing one surface entity `#n` wear one record.
   - Records a normalization mints (a knot split's sub-patches) are records of their own.
   - Two distinct entities become one record only through the existing reported act, `one_record` (a gap decided Zero at the file's band, faces facing one way).
   - `surface_sig` and every read of it go.
2. **Export.** `step-export` writes one surface entity per surface key. Today it writes one per face (`advanced_face`; its id cache holds only vertices and edges).
3. **Nothing else.** No origin mark, no minted ids, no namespace, no re-stamp after placement.

**The row is stale (sure).**
- `GeomSource`, `SourceExpr`, `GeomOrigin`, `AxisSource`, `ParamSource` and `stamp_minted` are gone, and so are `topo/src/source.rs` and `AXIS-DECLARATION-DESIGN.md`.
- You retired the stamp channel on PR 4322 ("sounds great!"). N6 now reads "The kernel carries no recipe provenance of a description."
- So the row's four design questions have no subject:
  - **Namespace collision:** there is no namespace.
  - **Instancing:** each placed copy is grafted under fresh keys.
  - **Placement order:** `transform_rigid` keeps keys and clears nothing.
  - **The per-evaluation scope sentence:** it left with `source.rs`.
- "Indistinguishable from a hand-built body" is now `[import]`'s stated goal, not a defect.

**What the row is still evidence of (sure on the facts).** The importer keeps the file's surface entity per face, then ignores it. It decides record identity by comparing `f64` bits at about six sites. Three consequences:
- **It is a production bit-identity coincidence decision.** That channel is retired (#53), and its gate does not see this use.
- **It is an unreported join.** `[topo.one-record]` says a value-decided join of two records is a reported act.
- **It can disagree with that act.** `one_record` never joins faces facing opposite ways. `surface_sig` keys on the surface alone, so it would.

**Measured on the 114 STEP files in the tree** (text scan with numbers parsed as floats and references resolved; not a build):
- 15 files hold two or more distinct surface entities in one shell whose values are identical: 96 groups.
  - 8 files are our own exporter's fixtures (e.g. `composed_die`, `die_pips`: 21 sphere groups each).
  - 4 are hand-made fixtures.
  - 3 are from foreign writers (NIST `ftc_09`: 34 groups).
- Every group is a plane, cylinder, cone, sphere or torus: all kinds `one_record` reads. None is a B-spline.
- None mixes facings, so no file today shows the disagreement above.
- 6 files share one entity across faces, all with one facing. Foreign writers among them (stepcode `io1-cm-214`, `sg1-c5-214`) do state sharing by reference, so the file's references carry real information.

**Final states.**
- **A (recommended): records follow the file's references.**
  - What it makes true:
    - One source of truth for record identity.
    - No bit comparison decides identity.
    - Every join of two records is a reported act.
    - A round trip keeps the key partition because the file states it.
    - The filed `Approx`-alias defect has nothing left to live in.
  - What changes: foreign files with duplicated entities (NIST `ftc_09`'s 34 groups) still end with one record per locus, but through `one_record`, reported.
  - What is left open: two bit-equal B-spline entities with an edge between them. `one_record` does not read splines, and the must-carry rung reads no spline pair, so that edge would refuse typed. The corpus has no such case.
  - Reversible: yes. It is local to the two exchange crates.
- **B: keep `surface_sig`.** Nothing changes. If you choose it, list it explicitly as an exception to the bit-identity retirement rather than leaving it unseen.
- **C: a file-derived identity stamped on the body** (the row's ask). It brings back the second identity carrier you retired, nothing would read it, and after a knot split it is wrong. Rejected (sure).

**Worked example.** A kernel sphere wall split into faces is one key.
- Today, export writes one `SPHERICAL_SURFACE` per face, and import re-shares them because the printed digits re-parse to equal bits.
- Under A, export writes one entity that every face cites, and import shares the key because the file says so.
- A foreign file with two equal sphere entities becomes one record through `one_record`, reported. The same happens if they differ in the last digit, within the file's band.

**Where file ids would still belong later:** in an import node's cell names, if import-as-document is ever built (ASSEMBLY.md's A8: not implemented). Three constraints for that row:
- A face's file id is not injective after normalization.
- Copies are told apart by the occurrence, which `PlacedInstance` already records.
- Ids mean something only for one pinned file content.

Not built now: nothing reads them.

**Ratified text.** One sentence in `[import]` is worth your sign-off, because it binds the exporter too: "A surface entity the file states is one record, cited by every face that cites it; two entities are two records unless the import's reported one-record act joins them, and export writes one entity per record." Nothing else ratified changes.

**Confidence.**
- Premise stale; no stamp should be built: sure.
- A over B: likely. The open risk is foreign files that duplicate spline records.
- Export change needed for quiet round trips: sure from the measurement. Without it, every split sphere or cylinder in our own fixtures becomes a reported join.
