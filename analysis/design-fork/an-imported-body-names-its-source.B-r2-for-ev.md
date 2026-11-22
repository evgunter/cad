## For Ev

**Answer to your concern: no. Taking records from the file's references reaches no at-rest state that native construction cannot, and it reaches fewer than today's bit test does (likely).** The recommendation stands, with one addition: the `[import]` sentence should name the invariant that makes this true.

**The invariant, and where it is enforced.** The importer has no door of its own.
- `step-import` is a separate crate. It attaches surfaces through the same public door `sweep` uses (`Construct::set_face_surface` with `FaceSurface::Shared`), so any face-to-record assignment it makes is one a native caller can make. The compiler enforces this: it cannot reach `topo`'s private writers.
- Only a body the shared at-rest gate passes ships (`[import]` step 4, already ratified).
- A shared record claims one thing: these faces lie on this surface. Each face's edges are certified against its record at ε, so a file that cites one surface for faces that do not lie on it refuses. It cannot produce wrong geometry.

**Each candidate state, checked (sure unless marked).** "Record" means one surface description in the body, which several faces may wear.

- **One record worn by faces facing opposite ways in one solid.** Reachable by import, today and under this design. Also native (likely, by reading, not run): a union of two boxes resting partly on each other glues a one-carrier pair with senses opposed (`GlueClass::Rest`), and `[topo.one-record.boolean]` leaves the surviving fragments on one record, "each sense restated so its material side is unchanged". The token type carries exactly that flag (`OneRecord::others`, "whether its chart faces as the kept one's does"). So the Z-step prism below is one record by union and two records by extrude, both native.
- **One record across two solids, or between a solid and its void shell.** Import cannot produce this under any option: each solid and each void is built as its own body and grafted under fresh keys. Native construction can (you re-ruled "a chart is body-wide" on PR 3412; the contact census reads a key shared across solids with opposed senses). Import reaches strictly less.
- **One record worn by faces that are not edge-connected.** Native: a boolean that cuts a face in two leaves both fragments on the operand's record.
- **One record over faces no native verb would leave shared.** There is no such class beyond the first bullet. Sharing is true or it refuses at certification, and a true sharing is what the boolean's act or the importer's own reported `one_record` act produces from two records.

**Today's bit test does not avoid the concern; it is the wider door.**
- It shares a record whenever the printed numbers agree, including two separate entities facing opposite ways, silently.
- The tree already pins the import-reached state: `step-import/tests/shell_reads_a_chart_worn_both_ways.rs` imports a Z-step prism whose two faces cite one plane, one reversed, as "a finished body" wearing "one chart with both senses within one solid". The `shell` verb then refuses it (`ChartSenseMixed`, not yet supported).
- By reference, that state arises only where the file itself cites one entity both ways. By bits, it also arises where two entities merely print alike.

**Measured.** Of the 6 corpus files with by-reference sharing (22 shared surfaces): none shares across shells, none mixes senses, 20 are edge-connected halves of a round surface (a native split or seam), and 2 are fragments not edge-connected (a native cut face). **Zero state a sharing native construction does not produce.**

**Recommendation (likely), unchanged in substance.**
1. **Import.** Faces citing one surface entity wear one record. Two distinct entities become one record only through `one_record`, the reported act that reads a Zero at the file's band. The bit test (`surface_sig`) is deleted.
2. **Export.** One surface entity per surface key.
3. **`[import]` gains:** "The file's references are taken through the doors a native caller has: faces citing one surface entity wear one record, two entities are two records unless interpretation joins them, reported, and each face is certified against its record."

**The alternative, if you want import narrower than native.** Honour a shared reference only between faces that face one way; give a face citing the same entity reversed its own record.
- **Makes true:** import never produces a record worn both ways in one solid, so `shell` never refuses an imported body for it, and import matches what `one_record` already says of its own joins ("faces facing opposite ways ... are two surfaces to the kernel's own builders").
- **Costs:** a native union's both-ways record does not survive export and re-import as one record; the importer overrides what the file states; and the kernel still has the state natively, so `shell`'s gap remains.
- I lean against it. It becomes the right answer if the union reading above turns out false when run, because then the both-ways record would be reachable only through import or raw topology doors.

**Worked example.** The Z-step prism, `[0,2]×[−1,0] ∪ [1,3]×[0,1]`, extruded: its two faces on y = 0 face opposite ways.
- Extrude writes two planes: two records.
- Two boxes unioned: the y = 0 pair rests opposed, and the fragments left over sit on one record, one with its sense restated.
- A file citing one `PLANE` for both, one reversed: one record under this design, the union's state.
- A file with two `PLANE` entities: two records, the extrude's state. Today, if their numbers print alike, one record.

**Unchanged from round 1.** The row's premise is stale (the stamp types are gone; PR 4322). No file-derived token is built. The bit test is a production bit-identity decision outside the gate, silent, and at odds with `one_record`. No corpus file states a spline surface twice. A shared file entity does not quiet the `unproven-coincidence` lint (`[intent.stored]`).

**Confidence.** No import-only state under this design: likely; it rests on the union reading, which is from the ratified `topo` page and the token type, not from a run. Across-solid and void cases: sure. Bit test is wider: sure. Corpus count: sure, within the method (text equality, per shell).
