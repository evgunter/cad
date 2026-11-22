**Recommendation (likely).** Do not give imported bodies a file-derived identity token. The only identity the kernel still reads at rest is which *surface record* (arena key) a face wears, and the importer should take that from the file's own references:

1. **Import:** two faces that reference one surface entity `#n` wear one record. Two distinct entities become one record only through the existing reported interpretation act (`one_record`, a Zero at the file's band). The importer's bit-signature dedup (`surface_sig`) is deleted.
2. **Export:** the writer emits one surface entity per surface key. Today it emits one per face.
3. **`[import]` gains one sentence:** a surface entity the file states once is one record; two entities are two records unless interpretation joins them, reported.

**Premise check: the row is stale (sure).** Every type it names is gone from the tree: `GeomSource`, `SourceExpr`, `GeomOrigin`, `AxisSource`, `stamp_minted` and `crates/topo/src/source.rs` do not exist, and `docs/AXIS-DECLARATION-DESIGN.md` was deleted at your request (2026-10-17). PR 4322 ("sounds great!") retired the stamp channel, and `[coincidence.door]` now says a coincidence names its cells "never by a stamp the kernel carries".

So the four questions the row says must be designed have no subject:
- **Namespace collision:** there is no namespace. A record is an arena key, minted fresh.
- **Instancing:** each placed copy is built or moved as its own body and grafted under fresh keys, so copies are distinct by construction. `[assemblies.document]` already says fusing copies is an explicit boolean.
- **Attach before or after placement:** nothing is attached, so nothing is cleared.
- **Scope claim for a body never evaluated from a document:** `[import]` step 4 already answers it. No imported cell is a construction of the document's variables, so its coincidences are decided by value and reported as `unproven-coincidence` findings.

The benefit the row promised (structural coaxiality for imported cylinders) is also covered: `[import]` step 3 heals a coaxial relation that reads Zero at the file's band into an exact one.

**What the row is still evidence of (sure on the facts).** Inside the importer, "which record does this face wear" is held twice and reconciled by comparing float bits.
- Each interpreted face carries its own copy of the surface value, and a parallel slice carries the file's surface entity per face.
- `surface_sig` then recovers sharing by bit equality at six sites: key sharing in `adopt::attach_surfaces`, the edge-description choice in `build::choose`, healing's grouping in `snap::nodes`, and three vertex-rooting reads in `build.rs`.
- Healing and `one_record` keep the copies in step by hand, writing the moved surface back to each face.

Three consequences:
- **It is a bit-identity coincidence decision in production.** You retired that (#53) and declined an exception on PR 3410 ("if nothing needs this then it should not be done"). `scripts/gates/bit-identity-consumer.sh` does not see it, because it is hand-rolled `to_bits`.
- **It disagrees with the reported act.** `one_record` says two plane records facing opposite ways are two surfaces and never asks them. If their bits are equal, `surface_sig` silently puts them on one key anyway, with no ledger entry.
- **It has a filed alias defect.** Every `Approx` surface signs as one bare tag (`step-import-surface-sig-signs-every-approx-surface-with-one-tag`), dormant only because no import path mints one.

It exists only because the exporter flattens sharing; `adopt.rs` says so ("restoring the writer-side sharing that the per-face record emission flattened"). I found no ratification of it: it is agent-written M7-3 spec text, in no DESIGN clause.

**The two defensible final states**

*A. Records follow the file's references (my lean).*
- The interpreted solid holds a table of surface records, keyed by the file's entity id plus records a normalization mints (a knot split's sub-patches). A face names a record and a sense.
- Recognition and healing act once per record. `one_record` re-points faces. Attachment maps record to key and reads no bits.
- The exporter writes each surface key once.
- **Makes true:** one source of truth; no bit comparison decides identity; every join of two records is a reported act; a round trip preserves record structure by construction rather than because printed floats happen to re-parse bit-equal; the `Approx` alias cannot occur.
- **Leaves possible:** a foreign file that states one surface as two bit-equal entities.
  - Planes and round surfaces are still joined, by `one_record`, where their faces face one way in one solid.
  - Two bit-equal B-spline entities with an edge between them would refuse, typed, in the existing "one record stated twice that one_record does not join" lane, until that act learns to read a spline pair.
  - I did not measure whether any corpus file does this.
- **Reversible:** yes; it is internal to the two exchange crates.

*B. Close the row as superseded and keep `surface_sig`.*
- **Makes true:** nothing changes; a foreign duplicate of a B-spline record keeps importing.
- **Leaves possible:** identity decided by bits outside the gate, the opposite-facing disagreement, the alias defect, and the double bookkeeping.
- If you choose B, it should be chosen as an explicit, gate-listed exception to the bit-identity retirement, not left unseen.

**Worked example.** A kernel cylinder wall split into two faces is one key.
- *Today:* export writes two `CYLINDRICAL_SURFACE` entities with equal digits. Import re-parses them, finds equal bits and shares a key, so the edge between them is a curve in one chart. It works only while print and parse round-trip bit-exactly.
- *Under A:* export writes one entity that both faces reference, and import shares the key because the file says so.
- *A foreign file with two equal cylinder entities:* `one_record` reads the gap where the faces are, decides Zero, joins them and reports it.

**Rejected: letting a shared file entity count as "one construction" and quiet the lint (likely).** `[intent.stored]` says adoption recovers descriptions and never intent, and `[import]` step 4 fixes imported contacts as findings. A writer sharing an axis placement is an artefact of the writer, not a statement by this document's author. I would not change that text.

**Ratified text I would change:** only the added `[import]` sentence above. It needs your sign-off.

**Confidence**
- Premise is stale: sure.
- No token should be built: sure.
- A over B: likely. The open risk is how often foreign writers duplicate spline records.
- Exporter change is required for A, or our own round trips of a split spline face refuse: likely. Read from the writer's `SharedIds`, which caches only vertices and edges; not run.
