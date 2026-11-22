## For Ev

**Your concern, answered: no. Taking surface records from the file's references reaches no at-rest state that native construction cannot also reach. It reaches fewer than today's import does** (likely, and the load-bearing case is now run).

What the file decides is only which faces share one surface record. Every pattern of that a file can state, a native verb already produces:
- **One record worn both ways in one solid** (the case that worried us). A probe of `topo::union` on `[0,2]×[−1,0]×[0,1]` and `[1,3]×[0,1]×[0,1]` puts both y = 0 faces on one record, with opposite senses, and tier 3 accepts the body. This holds in both operand orders, and after a split and a move. Extruding the same Z-step outline gives two records. So both states are native, and a file that cites one `PLANE` for both faces, one reversed, reaches the union's state.
  - Downstream treats them alike. `shell` refuses the imported Z-step (`ChartSenseMixed`, pinned in `step-import/tests/shell_reads_a_chart_worn_both_ways.rs`), and it refuses the union's state too.
- **Faces on one record that are not edge-connected.** Native: a boolean that divides a face leaves every fragment on its record.
- **Edge-adjacent faces on one record.**
  - Curved: a merge run the surgery stops stays "the act alone, left unmerged on one record".
  - Planar: a half revolve's two caps across the axis edge are put on one record by `one_record_across`, and are not merged.
- **Opposite facings meeting at one edge (a pinch).** No native verb produces it, and tier 3's wedge check refuses it on geometry, with one record or two.
- **One record across two solids, or between a solid and its void shell.** Native verbs produce these (a disconnecting subtract; a cup set upside down on a plate). Import cannot state them at all: each solid and void is built alone and grafted under fresh keys. This is where import is narrower.

The invariant that keeps this true already exists, and it binds native and imported bodies alike:
- A record claims only that the faces wearing it lie on it. Each face's edges are certified against its record at ε.
- The one at-rest gate (tiers 2, 3, 3′) runs unchanged on both.
- If a sharing pattern ever turns up that the gate admits and no verb produces, the fix belongs in the gate, which binds native bodies too. It does not belong in an importer filter: `[import]` says "import holds no idea of validity of its own".

**Today's bit test does not avoid your concern. It is the wider door.**
- One entity always gives one set of bits, so everything the file shares by reference is shared today.
- The bits add a silent join of any two entities that merely print alike, including two facing opposite ways.

**The corpus.** Of the 6 files that share a surface by reference (22 surfaces), none states a pattern native construction cannot produce. All are in one shell and face one way.
- 20 are edge-connected halves of a round surface.
- 2 are fragments of a divided face.

**Recommendation (likely)**
1. **Import.** Faces citing one surface entity, within one shell, wear one record.
   - Records a normalization mints (knot-split sub-patches) are records of their own.
   - Two distinct entities become one record only through the reported `one_record` act.
   - `surface_sig` and its six reads go.
2. **Export.** Write one surface entity per surface key.
3. **No identity stamp.** Nothing would read one; the channel was retired (PR 4322).
4. **The unit pins your concern.** It adds one import row per pattern above, stated by file reference. Each row holds the import to its native twin's at-rest verdict: the union for both-ways, a divided face, a half revolve, the curved unmerged run. The pinch row holds the import to its refusal. A row that fails names a gate gap.

**Rejected**
- *Honour the file's sharing only where faces face one way.* The probe shows the both-ways record is native. This filter would make import narrower than native and override what the file states, and `shell`'s gap would remain.
- *Keep the bit test.* It is a production bit-identity decision outside the gate, it joins silently, and it can disagree with `one_record`.

**One correction to my earlier answer.** I said `freecad/fuse_boxes.step` reaches an import-only state: its top and bottom each import as three faces on one plane record, edge-adjacent, where the native union gives one face each. It is not import-only. A native half revolve's two caps have the same form: one plane record, facing one way, adjacent across an edge.

Both forms are refused as boolean operands by the boolean's maximal-faces gate (`NonMaximalFaces`). That is how I read the code; I have not run a half-revolve operand. So the gap is in the kernel as a whole, between that gate and the at-rest form, not in import. It is independent of how records are chosen, so it goes to its own row and question, not into this decision.

**Ratified text for your sign-off: one sentence in `[import]`.** "A surface entity the file states is one record, cited by every face of one shell that cites it; two entities are two records unless the import's reported one-record act joins them; export writes one entity per record."

**Confidence**
- No import-only state from records: likely. The both-ways case is run; the rest are cited from code and tests.
- Import narrower than native on cross-solid and void sharing: sure.
- The bit test is the wider door: sure.
- `fuse_boxes` is not import-only: likely. This rests on the revolve code and `[topo.one-record.merge]`.
