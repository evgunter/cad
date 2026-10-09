# Review R1 — PR #4364, INTENT stage 4 PR A (the join builds what the REST zip builds)

Frozen head `c43e146c`, base `4908d472` (merge-base with origin/main). Concurrent dual review, R1
(docs/DUAL-REVIEW-PROTOCOL.md rule 2). No glimpse of the R2 lane, its branches or the PR's comments.

## Verdict: APPROVE-WITH-FIXES

Every soundness claim I could execute held. The 125 unions the zip built on main are rebuilt by the join on
this head with the same census and the same contact-record counts, at tiers 2/3/3′, to 4.7e-16 relative
volume. The 60 unions main refused that the branch now builds are all sound. No refusal turned into a
silent wrong body. The pins hold at all three ε. The three new mechanisms each have a killing mutant.

The fixes are not about soundness:
- **Test gap.** The half-open rule and the one-period seam shift are unbitten: on every fixture, any
  non-bailing treatment of a row end at the ray gives the same answer (MINOR-1).
- **Stale text.** A visible tour caption and several comments still describe the deleted zip (MINOR-2).
- **PR description.** It credits this branch with the torus peg-in-socket result, which main's own join
  already gives at the base (MINOR-3).

## Method (all execution, private CARGO_TARGET_DIRs)

- **Baseline, clean head, default ε.** topo + sweep + editor-core, `--profile default`: **8043/8044**. The one
  failure is the timing row `name_words_rows::a_large_table_…_in_bounded_time`.
- **Instrumented head**, one worktree, own target dir. It logs `across_tangent` moves (old and new sector
  faces, normal·normal, raw, kept and freshly recomputed `pair_codes`, loci) and `chart_ring_side`
  events (zero row ends, along-ray rows, full window, seam shift, verdict). It also carries a probe at
  `boolean_op_with` on every declared f64 union: an operand fingerprint (sorted vertex bits plus face and
  edge counts), the result's kind, volume, f/e/v/s, contact counts per kind, and tiers 2/3/3′ on the
  result's own contacts. The instrumentation only reads.
  - topo + sweep, `--profile default`: **5126/5126 at default, 5126/5126 at 1e-6, 5125/5126 at 1e-12**.
  - The 1e-12 failure is `parallel_cylinder_join::a_tipped_rod_whose_origin_is_stored_far…`. It fails
    identically on the base (run), as the PR says (`work/tint/tipped-rod-join-escalates-at-1e-12.md`).
- **editor-core on the clean head**: **2917/2918 at 1e-6 and 2917/2918 at 1e-12** (the timing row only, both times).
- **Main (the base) with the same probe** over topo + sweep at default ε: 5133/5133. The zip door built
  **125** unions (the PR measured 125), all from sweep; topo reaches it 0 times.
- **Mutants**, in their own worktree and target dir, over a 186-test filter. The filter holds every suite the
  instrumentation saw reach `across_tangent` or `chart_ring_side`, plus the spec's zip suites. The
  unmutated control is 186/186.
- **Process note.** My first attempt shared one target dir across worktrees. Cargo gives a workspace member
  the same artifact hash whatever path it is built from, so the trees overwrote each other's binaries (a
  backtrace showed the wrong tree's source). I discarded every result from that period except the baseline,
  which ran before any worktree existed, and redid all of it with one target dir per tree. Worth knowing
  for other lanes that use worktrees.

## Claims

**C1, soundness: not falsified.** The probes are matched by operand fingerprint, not by call ordinal, so
a test diverging earlier does not end the pairing as it did for the PR's ordinal pairing.
- **All 125 zip-built unions (not 74) find their twin on the branch** at default ε: 125/125.
  - f, e, v and shells are identical.
  - The contact counts (`vv`, `a_on_b`, `b_on_a`, `ve`, `ee`, `curves`, `patches`) are identical.
  - t2, t3 and t3′ pass.
  - Volume max relative difference: 4.66e-16.
  - The same holds against the branch's 1e-6 and 1e-12 runs (still against main's default-ε run).
- **Inputs main refused and the branch builds: 61 with matched operands**.
  - What main refused with: `Join` 45 (tangent `SectionInvariant` and `Euler(NotSameFace)`),
    `ResultInvalid` 8 (the dip, `RingOutsideOuter`), `RestZipUnsupported` 7 (`SegmentsBetweenIsolatedPierces`
    5, `PinchApex` 2), and one probe-pairing artifact (below).
  - **All 60 real ones pass t2, t3 and t3′**, with volume in [max(va,vb), va+vb]. The two operand orders
    agree to ≤3 ulp, and each test asserts its own closed form.
- **No regression.** The one "BODY→ERR" row is `m9_1_contact_vocabulary::a_wrong_class_declaration…`. It
  calls the same operands twice (once contradicting, once correct), and both trees answer the pair
  identically, so my pool paired them crosswise.
- **Every declared union both trees return as a body that fails t3′** is the same 9 calls on both trees
  (`pierce_strut_at_a_pinch` ×2, `union_flush_onto_edge_contact` ×6, the pinch operand in
  `rest_nested_strut` ×1). They are older than this PR.
- **Every final refusal of `tangent plane×cylinder`, `NotSameFace` or `RingHomingAmbiguous` is gone at all
  three ε** on the branch. Main shows 33 and 12 of them as final answers.

**C2, the half-open azimuth rule and the one-period window: correct by inspection, unexercised by any row
(MINOR-1).**
- The rule reads a row end at the ray's azimuth as "below" (u ≤ u_p). That is the standard half-open
  crossing-number convention. Each run vertex's `u` is shared by exactly two consecutive rows and decided
  once per value, so the reading is consistent.
- Events at default ε across topo + sweep:
  - 134 along-ray junction rows `(Zero,Zero)`, each flanked by `(Zero,Positive)`/`(Negative,Zero)` or
    `(Positive,Zero)`/`(Zero,Negative)` edges;
  - 48 full windows, all in `a_blind_shaft_unions_on_and_off_the_bores_seam` (the wholly-inside span);
  - **0 seam shifts at any of the three ε**.
- **Every integration verdict is `Out`** (1402 + 48 full-window). `In` comes only from the chord_join unit
  rows.
- The case each way:
  - **Vertex exactly at the ray**: reached, but only beside an along-ray row, where the crossing lies below
    the ring vertex either way.
  - **A transversal or touching single-vertex crossing**: never reached.
  - **Window of exactly one period**: reached; M4 kills it.
  - **Just under**: the ordinary path.
  - **Just over**: `SectionInvariant` by inspection; no row reaches it.
  - **A vertex at the seam azimuth on the high edge**: never reached (M6 survives).

**C3, `across_tangent` on a curved bound: not falsified.**
- **Every move is across a crease.** 572 / 604 / 556 moves at default / 1e-6 / 1e-12. Every one has old and
  new sector normals with dot product exactly 0.0, across a 90° crease. 26 per ε are across a curved bound,
  the cylinder rim to the cap plane; the rest are plane → plane.
- **The kept codes agree with fresh codes.** For every move I recomputed `sectors::pair_codes` on the new
  (A, B) sector pair. At every position the fresh reading does not read `On`, the kept code equals it, on
  both operands: 1732/1732.
- **Each guard and direction is tested or idle.** M7 (the two neighbour directions swapped) is killed by
  12 rows. M8 (the "neighbour face is the locus face" guard dropped) survives: in no fixture is the guard
  false, which matches the PR's stated blind spot. Nothing in code asserts the "codes stand" premise (NOTE-1).

**C4, nothing refused loudly now builds silently wrong: not falsified.**
- All 7 `RestZipUnsupported` inputs main reached build sound.
- The wholly-inside span (`ChordEndpointRevisited` in the row) builds at its closed form.
- I re-ran the line kiss (the dip row's shape 2, 10 poses × 2 orders) **on main and on the branch** with a
  scratch test. Both trees give the identical outcome: volume = want to 2.2e-16, and t3′ fails with the same
  `VertexOnFace` and `EdgeFaceOverlap` `UndeclaredContact`s. So this is not a regression. It is still an
  unpinned shipped-invalid body (MINOR-4).

**C5, no pin moved: holds.**
- Topo + sweep + editor-core are green at default ε on the clean head, except the timing row.
- Topo + sweep are green at 1e-6, and at 1e-12 except the tipped rod, which is red on main too.
- Editor-core is green at 1e-6 and 1e-12, except the timing row.
- Main's tour red (the PR 4353 die/teapot count pins) was not run and is not this PR's.

**C6, the rows bite: partly.**

| Mutant | Result |
|---|---|
| M1: old bail on any zero row end | **killed** (8 rows: bore mates, mate2 r1/r2, rest_mate_every_op) |
| M3: `across_tangent` reverted to identity | **killed** (12 rows: join2 r1/r2, reach_continuation ×2, rest_nested_strut, rest_zip_admission ×4) |
| M4: one-period branch removed | **killed** (`a_blind_shaft_unions_on_and_off_the_bores_seam`) |
| M7: across directions swapped | **killed** (12 rows) |
| M9: chart verdict forced `Out` | killed **only** by the unit row `sibling_escalation_rows::a_chart_ring_vertex_escalating…` |
| M10: parity inverted | **killed** (10 rows) |
| M2: half-open flipped (u < u_p is below) | survives; expected, since it is an equally valid convention |
| **M11**: any row with an end at the ray skipped entirely | **survives** |
| **M6**: seam shift removed | **survives** |
| **M5**: along-ray "vertex on the row" check removed | **survives** |
| M8: neighbour-face guard dropped | survives |

## Findings

**MINOR-1 (test-gap, sure; correctness likely fine).** `crates/topo/src/chord_join.rs:3728` (half-open),
`:3687` (seam shift), `:3714-3722` (vertex on an along-ray row).
- No row depends on how a row end at the ray is counted. M11, which drops every such row from the count,
  passes every suite that reaches one. The reason: the only reaching geometry is the other solid's ruling
  (a vertical junction row), whose flanking crossing lies below the ring vertex.
- The seam shift (`u_p − τ`) is reached 0 times at all three ε (M6 survives), and the along-ray vertex test
  is never decisive (M5 survives).
- So the PR's own ruling (1), "a run vertex at the ray's azimuth reads by the half-open rule … a window of
  exactly one period on the branch from its low edge", rests on inspection for every case except "a window
  of one period exists".
- A chart row where the ray passes transversally through a single run vertex, one where it touches a vertex
  and turns back, and a ring vertex at the seam's high edge would each make this bite.

**MINOR-2 (doc/claim, sure). The zip sweep left present-tense zip text, including a user-visible caption.**
- `demos/tour/src/crosslap.rs:88-92`, in a file this PR edited. The comment says "the join-stage REST zip
  removes the coincident contact patches", and the `expect_seamed` label is
  `"declared mated union (M5 S1 REST zip)"`. The PR body says the crosslap captions were changed.
- `crates/topo/src/boolean/refusal_routes.rs:3555`. The doc of
  `the_definite_arms_offer_a_declaration_only_where_their_door_takes_one` still lists "the declared rest zip
  names the geometry alone", though this PR deleted that row (rows 6→5).
- `crates/topo/src/chord_join.rs:324` and `:855` (and `:1564`'s "the zip's seams") still say
  "boolean zip", where the PR rewrote the same phrase at `:1615` to "boolean join".
- `crates/topo/tests/review_m3_pr5.rs:379` ("the M5 S1 REST zip glues the stack") and
  `crates/topo/tests/m3_pr6_tier3prime.rs:250`, both files this PR touched.
- `demos/tour/src/lily.rs:2273`, `demos/README.md:80`,
  `crates/sweep/tests/pi_seam_and_kiss_through_the_boolean.rs:898`.
- The PR's sweep patterns (`declared-REST`, `REST lane`, `rest lane`, `zip lane`, `rest.rs`, `RestZip`)
  cannot match "REST zip", "rest zip" or "boolean zip". That is what they were blind to.

**MINOR-3 (claim, sure, demonstrated). Ruling (3)'s torus half belongs to main, not to this branch.**
- At the base `4908d472`, unmodified main runs `mate7a_torus_rest`'s `peg_in_socket_union_holds` callers at
  ε = 3e-7, 1e-6 and 2e-6 with my door probe. The zip flag never fires: main's own join already builds the
  union (`zip=false`, Seamed, vol 4.885e-2).
- So "Now the join builds it at every ε" is not this PR's effect. Closing
  `join/peg-in-socket-union-refuses-join-desync-at-a-coarse-eps` here is fine, but its closing evidence and
  the PR body should credit main's merge, not the germ fix. Answer to "unbisected": main did it.
- Also: the spec's "stays a refusal" sentence (§2) is now stale on main, independent of this PR.

**MINOR-4 (test-gap, sure).** `work/zip/a-dip-inside-a-rest-contact-is-refused-by-the-result-gate.md`.
- Shape 2 ships a body that fails t3′ on both trees. The row says "Not pinned by a test", and it still is
  not. A re-pointed row with a known shipped-invalid output and no pin cannot tell E (or anything earlier)
  whether the shape changed.
- The scratch probe is about 25 lines over the existing `reflex_beside_a_post`.

**NOTE-1 (likely).** `crates/topo/src/boolean/insert.rs:302-334`.
- `across_tangent`'s premise ("the germ is the same ray in either sector, so its crossing codes stand") holds
  on every one of 1732 measured moves. That is only true because the kept tuple's real-bound code happens to
  sit where the new sector's real bound reads the same. The fresh tuple has its `On` at the *other* position.
- Nothing asserts it. A debug assertion comparing against `pair_codes` on the moved pair, as my
  instrumentation did, would hold the premise mechanically.

**NOTE-2 (sure).** The PR's differential undercounted its own coverage. Matching by operand fingerprint
pairs all 125, not 74. Recommend the fingerprint method for the spec's §11 row 1 record.

## Implementer rulings

1. **The half-open rule and the one-period window: accept.**
   - The rule is the right one (by inspection) and the full window is needed and bitten (M4).
   - The seam-shift sub-branch and the single-vertex semantics are unbitten (MINOR-1).
2. **Arms 2 and 3 as one cause, and the partner-edge chord not built: accept.**
   - M3 kills 12 rows, and no tangent `SectionInvariant` or `NotSameFace` survives as a final answer at any ε.
   - The moves are germ-tangent-to-a-crease-edge (normals perpendicular), as described.
   - Not building an arm no row reaches is the better deviation; it owes nothing.
3. **rest_nested_strut's PinchApex now builds: accept** (additive volume, tiers 2/3/3′; in my
   differential). **The torus half: the attribution is wrong** (MINOR-3).
4. **The dip row re-pointed to E: accept** (shape 2 is measured identical on both trees), but pin it
   (MINOR-4).

## Style

- **Q1 (sure).** The new `RingSide::Undecided` doc (`chord_join.rs:3576`) and `chart_ring_side`'s doc
  (`:3606-3626`) restate the half-open rule in prose twice more beside the code's own comment
  (`:3725-3727`). Three spellings of one rule; no shared helper names it.
- **Q2 (likely).** `across_tangent`'s doc (`insert.rs:302-307`) justifies keeping the codes by "the germ is the
  same ray", which is true of the ray but not of the code tuple's positions (NOTE-1). The prose carries an
  invariant the code does not check.
- **Q3 (sure).** Rows that cannot go red: the C6 survivors M5, M6, M8 and M11. M8 is the PR's own
  acknowledged blind spot; the rest are MINOR-1.
- **Q4 (sure).** `docs/INTENT-STAGE4-SPEC.md` §2 ("the torus's `SectionLoopUndecided` at ε ≥ 3e-7 stays a
  refusal") and §11 row 1 ("the 33 refuse `SectionInvariant`") cite premises that no longer hold on main.
  This is a doc rot, not a drift, and it is the spec's to fix, not this PR's.
- **Q4 (sure).** MINOR-2 is the class: a sweep pattern set blind to "REST zip", "rest zip" and "boolean zip".
  Look also in `docs/KERNEL-VERBS.md` and `demos/` beyond the two named files.
- **Q5 (sure).** `ops.rs`'s module doc now lists "a shaft in a bore, a plate on a rounded plate" as
  boundary-on-boundary unions the join builds. True and tested.
  - `carrier_pair.rs:1-6` promises the doors and delivers them.
  - `flush_pair_relation` still states that it has no in-tree consumer (`:27-30`), carried over verbatim. It
    is a published door with no caller, moved without asking whether it should be.
- **Q6 (sure).** The deviation "partner-edge chord built, then removed" is an improvement. The deviation
  "torus beyond the spec" is mis-attributed (MINOR-3), not unscheduled.
- **Q7 (unsure).** `across_tangent` re-indexes a `PairRecord` while keeping a code tuple read against a
  different sector. I would have re-read the codes on the moved pair (`pair_codes`) and resolved the `On` as
  `recl` does, rather than carry a tuple whose positions mean something else. It works on every fixture.
- **Q8 (likely).** I read `carrier_pair.rs` end to end. It is a verbatim move: 12 lines differ, all
  imports or the header.
  - Its `PairExtent` and `face_witnesses` are now read by `join.rs` (germ frames) and `merge_faces.rs`
    through `boolean::carrier_pair`.
  - A module named for the carrier-pair verdict is host to a join and a merge helper. That is the shape
    that drifts (a core hosted inside one of its consumers).
  - I did not read `chord_join.rs` (≈5200 lines) whole.

## Claims exercised / not

- **Exercised:**
  - C1: full differential, 125/125 plus 60 new builds.
  - C2: event logs at all three ε, plus mutants M1, M2, M4, M5, M6, M9, M10, M11.
  - C3: all three ε, 1732 moves, curved bound included.
  - C4: all 7 typed refusals, plus the line kiss on both trees.
  - C5: all three crates at all three ε.
  - C6: 11 mutants.
- **Not exercised:**
  - A constructed fixture for C2's single-vertex and just-over cases: none exists; I name the gap instead.
  - Mutant survivors over the *full* suite: the 186-test filter holds every suite the logs show reaching the
    mutated code, at default ε.
  - Python, pncad-py, the tour, clippy.
  - A main-side differential at 1e-6 and 1e-12: the branch's ε runs were compared against main's default ε.
- **Style questions exercised:** Q1–Q8 as listed.

Tokens ≈ 270k; wall-clock ≈ 3 h 10 min, of which about 40 min were lost to the shared-target-dir
contamination and rerun.
