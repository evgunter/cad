## For Ev

**Your concern, answered first: no. Sharing by the file's references reaches no at-rest state that native construction cannot reach.** (likely; argued from the code and the tests, not executed)
- **What a file can state with A.** One surface record worn by a set of faces in one shell.
- **What it cannot state with A.** Any edge or face geometry the existing gate does not already certify. Every boundary edge is still certified against its faces' surfaces, and the gate runs unchanged.
- **Native verbs produce every such pattern I could find.** The table below gives each one.
- **Import is narrower than native, not wider.** It builds its records per shell, so it never shares a record across shells or solids. Native verbs do.
- **Today's bit dedup reaches exactly the same set** whenever printed numbers agree. Keeping it does not avoid the concern; it only hides which faces were joined, and by a rounding accident.

**The patterns, checked against native producers and the gate**

| A file states one record worn by… | Native producer | Gate |
|---|---|---|
| faces not edge-connected | a boolean divides a face and every fragment keeps the operand's record (subtract a slot across a top; one tool drilled through two bosses) | admitted |
| faces facing opposite ways in one solid | a union with an overhang: the glue moves every fragment of the decided face onto one record (`[topo.one-record.token]`), so the overhang's underside (−z) wears the base's top record (+z). Pinned buildable at tiers 3 and 3′ by `a_block_resting_in_the_zero_band_unions` ("overhanging") | admitted |
| two edge-adjacent faces of one plane | a half revolve's two caps across its axis edge, on one record by `one_record_across` | admitted |
| two edge-adjacent faces of one curved surface | a merge run whose surgery the inventory stops stays "the act alone, left unmerged on one record" | admitted |
| an outer face and a void-shell face | a union of a cup set upside down on a plate: the plate's top record bounds both the outer shell and the enclosed air | admitted (tier 3 check 10 reads keys shared across shells). Import never states it: voids are separate shells |
| faces of two solids | a disconnecting subtract, then `move_shells_to_new_solid`; your PR 3412 ruling made a chart body-wide | admitted. Import never states it: records are per solid, copies grafted with fresh keys |
| opposite facings meeting at one edge (a pinch) | none | refused: tier 3 check 4 (`LaminaWedge`) reads the jets, not the keys, so it fires with one record or two |

The one candidate no verb produces is refused on geometry, whatever the record partition.

**The invariant, and where it lives.** A surface record means one thing: every face wearing it lies on its carrier. Two places hold it:
- the edge ladder, which certifies each face's boundary against its record;
- the one at-rest gate (tiers 2, 3, 3′), which runs on native and imported bodies alike (DESIGN's canonical-places row).

Beyond that, sharing changes only how the gate reads an edge (one chart versus a pair), and each such reading has a native producer above. If a sharing pattern ever turns up that the gate admits and no verb produces, fix the gate: it is a gap in the kernel's at-rest statement, which binds native bodies too. It is not an importer filter. `[import]` step 4 says "import holds no idea of validity of its own", and a rule like "honour a file's sharing only where a native construction could produce it" would be one.

**Recommendation (unchanged; likely):**
1. **Import.** Faces citing one surface entity, within one shell, wear one record.
   - Records a normalization mints (knot-split sub-patches) are their own.
   - Two distinct entities become one only through the reported `one_record` act, which joins faces only where they face one way.
   - `surface_sig` and its reads go.
2. **Export.** Write one surface entity per surface key.
3. **No identity stamp.** Nothing reads one, and N6 retired the channel.
4. **New in this round: pin the answer to your concern.** One import row per pattern in the table, each stated by file reference. Each row holds the import to its native twin's at-rest verdict (built by the verb named in the table), and the pinch row to its refusal.
   - These pins are the evidence that no import-only state exists. A pin that fails names the gap and the gate check it needs.

**What it leaves possible.** A foreign file that states two bit-equal B-spline entities with an edge between them still refuses, typed: `one_record` does not read splines. The corpus has no case.

**The corpus, re-measured** (a text scan of the tree's 114 STEP files, references resolved; not a build). Six files state sharing by reference: 22 records, each in one shell.
- All 22 have every face facing one way.
- 20 are edge-connected: halves of split cylinders, spheres, a cone and a torus. Native producer: the unmerged curved run.
- 2 are not edge-connected (a `TAIL_TURBINE` plane pair; a four-face `sg1-c5` cylinder). Native producer: a divided face.
- None states a pattern native construction cannot produce.
- Today's dedup shares all 22 too (same entity, same bits). A changes nothing for them; it changes only the 96 bit-equal pairs of distinct entities, which become reported `one_record` joins.

**Ratified text.** One sentence in `[import]` needs your sign-off: "A surface entity the file states is one record, cited by every face of one shell that cites it; two entities are two records unless the import's reported one-record act joins them; export writes one entity per record."

**Confidence.**
- The answer to your concern: likely. The producers are cited from code, docs and one buildable test row. The two union cases (overhang, inverted cup) are read from the glue's moved-set rule, not executed. Item 4's pins would settle them.
- Records follow references over keeping `surface_sig`: likely.
- No stamp: sure.
