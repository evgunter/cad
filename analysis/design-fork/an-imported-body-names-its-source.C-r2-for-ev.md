## For Ev

**Your concern, answered.** Letting records follow the file reaches no state that native construction cannot also produce (likely). Native verbs already produce every pattern of record sharing the file can state.

I did find one state that today's import reaches and no modelling verb produces. It has nothing to do with records: an imported body can ship with **two adjacent coplanar faces that are not merged** (sure). Concretely:
- `freecad/fuse_boxes.step` imports as 14 faces, with its top and bottom each tiled into 3 faces on one plane (pinned in `tests/freecad.rs` and `tier_gate.rs`).
- The same union built natively has 10 faces.
- The imported body passes the at-rest gate. It then cannot be a boolean operand, because the boolean's operand gate F7 (`gate_maximal_faces`) refuses it as `NonMaximalFaces`.

So the guard your concern calls for belongs at the gate, not in how the importer assigns records.

**Recommendation (likely)**
1. **Records follow the file (unchanged from the PR).**
   - Faces citing one surface entity wear one record.
   - Records a normalization mints are records of their own.
   - Two entities join only through the reported `one_record` act.
   - `surface_sig` goes, and export writes one entity per surface key.
2. **New: "maximal faces" becomes an at-rest check, and import ends by merging, reported.** The rule is that no two faces across an edge lie on one plane facing one way.
   - Tier 3 already checks the other half of DESIGN's "maximal faces and maximal edges" form: maximal edges, check 11. Maximal faces is checked only at the boolean's input.
   - Moving the planar F7 rule into tier 3 makes "no state is reachable only through import" hold by construction. Import holds no idea of validity of its own, so whatever the gate does not check, import can ship.
   - Import then ends with the merge stage, as it already ends with the join. Every merge is reported as a normalization, because merging is never silent.
   - Curved same-key adjacency stays legal, as F7 already says: a periodic wall is canonically cut.

**Why record sharing adds no state (the five cases the orchestrator asked about)**
- **What the gate reads.** The at-rest gate has no rule about which faces share a record. It reads a record only as the surface a face lies on, and its edges are certified on it. The F7 rule and the census read records the same way for native and imported bodies. So the question is only whether native verbs produce each pattern of sharing.
- **Faces on one record that are not edge-connected.** Native: a cut that divides a face leaves every fragment on the face's record (`[topo.one-record]`, the moved set's "every fragment"). Reachable (sure).
- **Faces facing opposite ways in one solid.** Native: a union glues an opposed ("Rest") pair onto one record, with each sense restated (`boolean/ops.rs`, `OneRecordTokens::of_result`, which passes `charts_agree` as false for an opposed pair). Two boxes stacked with an offset leave the lower top and the upper bottom on one plane record in one solid, facing opposite ways. The census already reads opposed pieces on one record (`census.rs`, `opposed_pieces_on_one_record_meet_where_they_overlap`). Reachable (likely; I did not run it).
- **Curved faces on one key across an edge.** This is native's canonical form: F7 skips it, and a union's curved merge skip ships it. Reachable (sure).
- **One record across two solids or two shells.** Import cannot produce this at all. Each `MANIFOLD_SOLID_BREP` is built on its own and grafted under fresh keys (sure). Import is narrower than native here.
- **A face sharing a record with its own void shell.** The file can state it, and no corpus file does. Whether native reaches it I am unsure about. Today's bit test reaches it too whenever the digits agree, so it does not separate the options.

**Today's bit dedup does not avoid the concern; it hides it (sure).**
- One entity always parses to one set of bits, so every sharing the file states by reference already shares today.
- The bits share more: any two bit-equal surfaces in one solid, whatever their facing or connectivity, with nothing reported.
- The `fuse_boxes` state arises today, through `one_record`. Keeping `surface_sig` changes nothing about it.

**The corpus, re-checked** (a text scan of the 114 files; references resolved; not a build):
- **0 of the 6 files** with by-reference sharing state a sharing native construction cannot produce.
  - All 22 shared entities lie in one outer shell of one solid, with one sense each.
  - 20 shared entities have edge-connected faces.
  - 2 do not (`sg1-c5-214` `#194`, a cylinder; `TAIL_TURBINE` `#496`, a plane, not adjacent). Both are native fragments of a divided face.
- **1 of the 114 files** has adjacent coplanar faces facing one way: `fuse_boxes`, with 8 edges, and no sharing by reference. That is the one import-only state in the corpus, and item 2 removes it.

**The options, as final states**
- **A (recommended): records follow the file, plus maximal faces checked at rest.**
  - The gate enforces "no import-only state" for the form DESIGN already claims.
  - Identity is never decided by bits.
  - Every join and every merge is reported.
  - Reversible: yes.
- **A without item 2 (the PR as it stands).** It is sound on records, but leaves `fuse_boxes`-class bodies importing in a form no boolean will take.
- **B: keep `surface_sig`.** The same import-only state stands, and the sharing decision stays silent.
- **Rejected: a rule that honours the file's sharing "only where native could produce it".** Every case I found qualifies, so the rule would be a check with nothing to refuse. The invariant worth stating is the at-rest form, not a rule on sharing.

**Ratified text for sign-off**
- **`[import]`**, as in the PR: "Records follow the file."
- **`[topology]`'s tier 3**: add *maximal faces*, meaning no planar pair across an edge on one plane facing one way, beside maximal edges.
- **`[import]` pipeline**: import ends with the merge as well as the join, each reported.

**Confidence**
- The records proposal adds no state: likely. The opposite-facing case is argued from the code, not run.
- The `fuse_boxes` state is import-only and refused by F7: sure from the code and the pinned counts.
- Native bodies already pass a tier-3 maximal-faces check: likely. The corpus needs measuring.
