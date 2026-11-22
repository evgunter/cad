## For Ev

**Your concern, answered: no. Letting records follow the file's references lets import reach no at-rest state that native construction cannot (sure for the case that was in doubt; likely overall).** Import stays narrower than native, and today's bit comparison is the wider door, not the safer one.

"Record" means one surface description in the body, which several faces may wear.

**Why.**
- **The importer has no door of its own.** It is a separate crate that attaches surfaces through the same public door the sweeps use, and ships only what the shared at-rest gate passes (`[import]` step 4).
- **A shared record claims one thing:** these faces lie on this surface. Each face's edges are certified against its record at ε. A file that cites one surface for faces not on it refuses; it cannot produce wrong geometry.
- **Every sharing pattern a file can state has a native producer:**

| A file states one record worn by… | Native producer | At-rest gate |
|---|---|---|
| faces not edge-connected | a boolean that cuts a face in two; both fragments keep the record | admits |
| faces facing opposite ways in one solid | a union of two boxes resting partly on each other. **Run on main:** both y = 0 faces end on one record with opposite senses, in either operand order, and tier 3 accepts the body. An extrude of the same outline gives two records | admits |
| two edge-adjacent faces of one curved surface | the canonical cut of a periodic wall | admits |
| opposite facings meeting at one edge (a pinch) | none | refuses on geometry, with one record or two |

- **Import is narrower.** Each shell is built on its own and grafted under fresh keys, so import never shares a record across shells or solids. Native bodies do (your "a chart is body-wide" ruling, PR 3412).
- **Today's bit comparison reaches all of the above and more.** It shares a record whenever printed numbers agree, including two separate entities facing opposite ways, with nothing reported. A test already pins an import with one plane worn both ways (`shell_reads_a_chart_worn_both_ways.rs`).

**Measured.** Of the tree's 114 STEP files, 6 share a surface by reference (22 surfaces). All 22 sit in one shell with one sense. 20 are edge-connected halves of a round surface; 2 are fragments of a cut face. **None states a sharing native construction does not produce.**

**Recommendation (likely).**
1. **Import.** Faces of one shell citing one surface entity wear one record. A record a normalization mints (a knot split's sub-patches) is its own. Two distinct entities become one record only through `one_record`, the reported act that reads a Zero at the file's band and joins only faces facing one way. The bit comparison (`surface_sig`) is deleted.
2. **Export.** One surface entity per record.
3. **No identity token.** Nothing reads one; the stamp channel was retired (PR 4322).
4. **Tests that pin this answer.** One import per row of the table, stated by file reference, each held to its native twin's at-rest verdict, and the pinch held to its refusal. They are the unit's acceptance, and a failure names a gap in the gate.
5. **`[import]` gains one sentence, for your sign-off:** "A surface entity the file states is one record, worn by every face of one shell that cites it; two entities are two records unless the import's reported one-record act joins them; each face is certified against its record; export writes one entity per record."

**No importer filter.** In the last round I kept a fallback: honour a shared reference only between faces facing one way. It existed in case the union did not produce a record worn both ways. The run shows it does, so the fallback is withdrawn. A filter would be an idea of validity held by import alone, which `[import]` step 4 rules out. If a pattern ever turns up that the gate admits and no verb produces, the fix is at the gate, which binds native bodies too.

**A separate finding that deserves its own question, not a rider here.** One designer found an import-only state that exists today and has nothing to do with records: `freecad/fuse_boxes.step` imports with its top and bottom each tiled into three coplanar faces (14 faces against 10 built natively). It passes the at-rest gate and is then refused as a boolean operand (`NonMaximalFaces`). The proposed cure is a tier-3 maximal-faces check plus a reported merge at the end of import.
- It is a real instance of your concern, and I would act on it.
- It should not be decided inside this one. It changes `[topology]`'s tier-3 list and the `[import]` pipeline, and its premise is unsettled: `[topo.one-record.merge]` says a half revolve's two caps across its axis edge are left "the act alone" on one plane record, which reads as a native verb producing the same adjacent coplanar pair. If so, the state is native, the boolean's operand gate refuses a native body, and a new tier-3 check would refuse the half revolve. That needs a run and its own weighing.
- Nothing in items 1–5 depends on it, and it is equally present if the bit comparison is kept.

**What this leaves possible.** A foreign file stating one B-spline surface as two entities with an edge between them refuses, typed, naming the lane: `one_record` does not read splines. No file in the tree does this.

**Worked example.** The Z-step prism, `[0,2]×[−1,0] ∪ [1,3]×[0,1]`; its two faces on y = 0 face opposite ways.
- Extruded: two records.
- Two boxes unioned: one record, one face's sense restated (run).
- A file citing one `PLANE` for both, one reversed: one record, the union's state.
- A file with two `PLANE` entities: two records, the extrude's state. Today, if the numbers print alike, one record, silently.

**Reversible:** yes; the change is internal to the two exchange crates.

**Confidence.**
- A record worn both ways is native: sure (run by the orchestrator).
- No import-only state from records following references: likely. The remaining rows are read from code and the ratified `topo` page; item 4's tests settle them.
- Records by reference over keeping the bit comparison: likely.
- No token: sure.
- The maximal-faces finding is real: sure from the pinned counts. That it is import-only: unsure, because of the half revolve.
