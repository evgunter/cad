(Orchestrator's transcription of designer C's `For Ev` section, first report, 2026-11-17. Not verbatim; re-request verbatim if the designer can be resumed.)

**Recommendation: give an imported body no provenance stamp; make the file's own references its identity, in the one form of identity the kernel still has — a shared surface key.**
1. Import: faces citing one STEP surface entity share one surface record (`FaceSurface::Shared`); distinct entities are distinct records, however their numbers compare. This replaces the bitwise dedup (`adopt::surface_sig` / `attach_surfaces`).
2. Export: `step-export` writes one surface entity per kernel surface key, not one per face, so a round trip keeps a body's sharing because the file states it.
3. Build nothing else: no `Imported` origin mark, no minted ids, no namespace partition, no re-stamp after placement.
Confidence: likely.

**Premise (sure on each):** the stamp machinery was retired (PR 4322, FORK-S4-1, INTENT stage 4 unit E); `RecipeNodeId` is a `MintId`, so nothing can collide; `AXIS-DECLARATION-DESIGN.md` is deleted; no listed consumer reads a stamp (the boolean glues on a margin decided Zero; row doors and chart readings ask only same key / shared Arc, PR 3414 / PR 3410); `[import]` says imported geometry is not distinguishable from native, so an `Imported` mark would contradict it; placement and instancing are already right (`transform_rigid` keeps keys, `graft_disjoint` gives each copy fresh keys).

**What the row is evidence of:** the importer restores sharing by comparing f64 bits (`entities.rs` drops the `#surface` id; `attach_surfaces` merges on identical bits). That is a production bit-identity coincidence decision, the channel `scripts/gates/bit-identity-consumer.sh` says is retired (the gate misses it because it hand-rolls `to_bits`), and an unrecorded value-decided coincidence against `[coincidence]` and Ev's PR 3410 question. It fails both ways: it invents sharing the file never stated, and misses sharing a writer meant when numbers print differently. The exporter flattens key sharing (`advanced_face` prints a surface per face), which is why the dedup was needed. Topology is already identified by entity id; only geometry is identified by value.

**Alternatives:** (B) keep the merge but decide it by the carrier ladder's margin and record a row: rejected, likely (value decisions the file never asked for). (C) the row's import-minted identity: rejected, sure (re-creates the retired second identity carrier, has no reader, breaks `[import]`).

**Ratified text:** add one sentence to `[import]`: "The file's references are the imported body's structure: faces citing one surface entity share one surface record; distinct entities are distinct records, and any coincidence between them is decided and recorded as a native one is."

**Feasibility (separate from ranking):** import and export must land together; recognition should resolve once per entity; a surface entity shared across two MSBs stays split per solid; unverified that every third-party file whose half-faces cite distinct bit-identical surfaces still passes through the conventional rung (cylinder halves need a red-first row).
