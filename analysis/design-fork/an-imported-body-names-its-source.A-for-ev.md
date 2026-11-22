## For Ev

**Recommendation: don't build this. Close the row as superseded.** Since the row was filed (2026-09-12), the stamp mechanism it extends has been retired. The identity it wants for imported geometry is either already present in a stronger form (bit-identical records share one surface key), or belongs in names once an import becomes a document node. That node does not exist yet. Confidence: **sure** on the premise corrections, **likely** on the recommendation.

**Premise correction 1: the channel the row extends no longer exists.** (sure)
- `GeomSource`, `GeomOrigin`, `SourceExpr`, `AxisSource`, `ParamSource`, `stamp_minted` and `compose_placed` are gone from `crates/`, and so is `topo/src/source.rs`. INTENT stage 4 unit E removed them. Your approval of the fork is "sounds great!" on PR 4322 (DESIGN-FORK-LOG row 92): provenance is the document's, read from reads plus names, with no stamp.
- N6 now reads "The kernel carries no recipe provenance of a description."
- None of the consumers the row lists reads identity any more:
  - The boolean glues on a margin decided Zero (`[coincidence]`).
  - `AXIS-DECLARATION-DESIGN.md` is deleted, so "absence refuses" no longer locks anything out.
  - The one-chart reading (`Body::same_chart`) compares surface keys, or a shared spline payload.
- `[import]` now says imported geometry "is not distinguishable from native geometry either". The row's complaint ("cannot be told from a hand-built one") is now the ratified goal.
- So its four "must be designed" questions are void:
  - **Namespace disjointness:** there is no namespace to partition.
  - **Instancing:** with no stamps, copies cannot collide.
  - **Placement order:** `transform_rigid` has nothing to clear.
  - **Per-evaluation scope:** the sentence left with `source.rs`.

**Premise correction 2: a file entity id is weaker than what the importer already uses, and it is not sound.** (sure)
- **Sharing already happens by bits.** The parsed face spec does not keep its surface entity id. `adopt::attach_surfaces` instead puts every bit-identical surface in a solid on one kernel surface key.
  - One entity always gives one set of bits, so every case where faces share an entity is already shared.
  - Bit-sharing also catches identical surfaces the writer emitted as separate entities.
  - Our own `step-export` writes one surface entity per face (`writer.rs`, `advanced_face`). On our own corpus, entity ids show no sharing at all; bits do.
- **"Same entity ⇒ same bits" fails after normalization.**
  - `knot_split` cuts one B-spline face into sub-patches with different nets, and the faces it mints keep the replaced face's id. An identity keyed on entity ids would call two different descriptions one source, which is the silent gluing the row feared.
  - The knot split and the seamless-band re-mint also create new ids through `mint()`. Those are the importer's own ids, not the file's, so even the maps' keys are partly invented by the reader.

**What file identity would add under the current design: nothing a check can use.**
- The second structural rung (the coincidence door, evaluating each carrier with every variable as a symbol) reads an imported carrier as a constant, because import has no parameters (`[import]`, non-goal).
- Two cells reached through one read with bit-equal carriers differ by exactly zero, so the door proves them structural.
- Two parts that touch only within ε have a nonzero exact difference. That makes the contact an `unproven-coincidence` finding, quieted by an assertion, which is exactly what `[import]` ratifies.
- Entity ids prove nothing that the bits do not.
- Worked example: two placed copies of one bracket, resting on a plate from the same file.
  - The copies' faces come from one entity, but they are different carriers: different maps, different bits.
  - Each copy meets the plate by value, so each contact is a finding. That is the right answer, and an entity id could not change it.

**Where file identity does belong: in names, when an import is a document node.**
- N6 names a recorded cell by the read it entered through and its name there. An imported body has no names today: `import_step` is a free function, `editor-core` never calls it, and ASSEMBLY.md's A8 says "import-as-assembly-document is not implemented".
- When that node is built, its name table is the one place file identity should live. Constraints for that row, all checked in the tree:
  - **A face's file id is not injective.** After normalization, a name needs the face id plus a piece index.
  - **Copies are told apart by the occurrence.** The A7 record (`PlacedInstance`: occurrence, relationship, transform, applied map) already carries it, beside the body.
  - **Ids mean something only for one pinned file content** (`[assemblies.pins]`). Other writers renumber on export.

**The answers as final states:**
- **A (recommended): close as superseded.**
  - The import's provenance is what already travels beside the body: the A7 instance record and the reported normalizations. Nothing is stamped on descriptions.
  - Native and imported bodies are one kind of thing, as `[import]` says.
  - Fully reversible: building the import node later loses nothing.
- **B: stamp a file-derived identity on descriptions** (the row's proposal).
  - It brings back a kernel provenance channel that N6 just retired, and it states the same fact twice (bits/keys and stamps), to be kept in step by hand.
  - Nothing would read it, and after a knot split it is wrong.
  - Reject. (sure)
- **C: build the import node now, with entity-id names.**
  - It is the right home, but it is a feature (import-as-document) with no consumer asking for it, not a fix for this defect.
  - Lean no, unless that feature is wanted for its own sake.

**Ratified text:** nothing to change. The current `[import]`, `[coincidence]` and N6 already state the answer. I could not trace the commit that wrote `[import]`'s "no imported cell is a construction of the document's variables" because the checkout is shallow. The recommendation agrees with it either way.
