---
id: bandfoot-and-bandcross-arguments-are-read-by-no-document-row
kind: issue
title: No document row reads a BandFoot's, BandCross's, BandFace's or BandSlit's argument — four blend mints ride check_total and a count alone
status: closed
pr: 2794
branch: edit/ladder-rim-fixture
opened: 2026-09-16
closed: 2026-09-17
---


## Finding

`crates/editor-core/src/names/emit_blend.rs` translates eleven blend
mint channels. After the ruled-carve fixture
(`a-ruled-carve-has-no-editor-fixture-…`) lands, the arguments of nine
of them are read by a document-layer row. Two are not:
`RoleSeg::BandFoot` (`rec.rim_feet`) and `RoleSeg::BandCross`
(`rec.meridian_splits`).

`blend5_rim_support.rs`'s module header already states the gap for
three roles — *"the other three (`rim_feet`, `meridian_splits`,
`meridian_remnants`) ride `check_total`: the emitter refuses a table
that does not name every output entity, so reaching a green table at
all is the statement that they emitted, but no row here reads their
names"* — and the ruled-carve suite `edit_ruled_carve.rs` closes the
third (`meridian_remnants` / `RoleSeg::BandCut`) by asserting the rim
each survivor carries against the vertices it actually runs between.
The other two are still in that state.

What `check_total` buys is that SOME name reaches every output entity.
What it cannot see is a name whose ARGUMENT is wrong: a `BandFoot`
carrying the wrong source rim vertex, or a `BandCross` carrying the
neighbouring meridian, names a real entity and passes totality. The
mutation that proves this is cheap and was run for `BandCut`: permuting
the source argument across `rec.meridian_remnants` in `name_blend`
leaves a total, duplicate-free table and is caught only by the row that
reads the argument.

Every other hit in `crates/editor-core/tests/*.rs` for these two roles
is one of three shapes, and none of them says what argument the role
should carry:

- an exhaustive `match` arm that classifies a role into a word
  (`m6_composed_node.rs` — and NOT `m6_5_downstream.rs`, which this
  finding named when it was written and which carries no arm over any
  of these roles at all: `grep -n Band
  crates/editor-core/tests/m6_5_downstream.rs` is empty. Corrected in
  the fix pass, 2026-09-17; the hit list is the five files under
  `crates/editor-core/tests/` that match
  `BandFoot|BandCross|BandFace|BandSlit`, which are
  `blend5_r1_probes.rs`, `blend5_rim_support.rs`, `fixture/mod.rs`,
  `m4_pr4_resolve.rs` and `m6_composed_node.rs`);
- a covariance walk over whatever `NameRef`s a segment wraps
  (`m4_pr4_resolve.rs`);
- a COUNT filtered on the role —
  `blend5_r1_probes.rs`'s
  `a_cone_on_cone_rim_mints_a_band_foot_though_it_has_no_planar_support`
  filters the table on `Some(RoleSeg::BandFoot(_))` and asserts the
  count is positive. That row's subject is the doc comment's "planar
  support" wording, and the count is the right instrument for it: it
  says a band foot is minted on a rim with no planar support. But a
  count reads no argument — it discards the `NameRef` with the
  wildcard, so it is green under any permutation of the source rim
  vertices, which is exactly the mutant this row is about.

## Two more, measured in the fix pass (2026-09-16)

`RoleSeg::BandFace` (`rec.bands`) and `RoleSeg::BandSlit` (`rec.slits`)
are in the same state, by the same test.
`blend5_rim_support.rs`'s `a_closed_rim_carve_names_its_whole_output`
asserts `count(BandFace(_)) == 1` and `count(BandSlit(_)) == 1` — the
right instrument for what that row says (one band face rounds the rim,
one slit keeps it ring-free), and no reading of either argument.
`BandFace` carries the SET of source rim edge names and `BandSlit` the
meridian it was slit along; permuting either across the mints leaves
both counts and `check_total` green.

`BandTrim` is not in this group: the same suite's `trims` helper reads
its `support` argument against the surface the named edge actually lies
on.

## What a taker owes

A document driving a LADDER rim — the shape the corpus's `die_composed`
pip cavities carve — with two rows in the shape
`edit_ruled_carve.rs`'s: the band foot is the host-support vertex
retracted from the source rim vertex its name carries, and the band
crossing is on the meridian its name carries, each asserted against the
runtime vertex the name resolves to. The mutant to check them against
is the permutation above, applied to `rec.rim_feet` and
`rec.meridian_splits`.

## Filed from inside the fence

`crates/editor-core/src/names/emit_blend.rs` is EDIT's (claimed
2026-09-16 for the ruled cut-off-arc row), so this sits beside the
fixture row it was found from.

## Spec (2026-09-16, EDIT orchestrator) — middle tier, branch `edit/ladder-rim-fixture`

**Premises, verified against the tree.** `emit_blend.rs` mints
`BandFace` from `rec.bands: Vec<(FaceKey, Vec<EdgeKey>)>`, `BandFoot`
from `rec.rim_feet: Vec<(VertexKey, VertexKey)>`, `BandCross` from
`rec.meridian_splits: Vec<(VertexKey, EdgeKey)>`, `BandSlit` from
`rec.slits: Vec<(EdgeKey, EdgeKey)>` (`sweep::blend::naming`, lines
185–203). The corpus's `die_composed` carves LADDER rims through
`Node::Fillet`; `blend5_rim_support.rs` drives an ANNULUS rim and reads
`BandTrim`'s argument; `edit_ruled_carve.rs` is the shape for a row
that reads a role's ARGUMENT against the entity the name resolves to.

**What lands.** `crates/editor-core/tests/edit_ladder_rim.rs` (+ its
`#[path]` line in `tests/all.rs`): a document driving a ladder rim
through `Node::Fillet` — the smallest document that mints all four
roles (measure: a rim whose supports meet at a ladder, the
`die_composed` pip cavity's shape, authored as a recipe through
`DocEdit`; if `die_composed` itself is the smallest, drive it from the
corpus registry and say why the registry's cost is not paid twice).
Four rows, each reading the ARGUMENT against the runtime entity:
- `a_band_foot_is_the_host_support_vertex_retracted_from_its_source_rim_vertex`
  — the foot's coordinates against the source rim vertex its `BandFoot`
  argument names: on the host support, retracted from that vertex and
  no other (the name resolves; the geometry agrees).
- `a_band_crossing_lies_on_the_meridian_its_name_carries` — the
  crossing vertex's coordinates on the meridian edge the `BandCross`
  argument names, and not on its neighbour.
- `a_band_face_carries_the_set_of_rim_edges_it_rounds` — the face's
  boundary against the SET of source rim edge names in `BandFace`'s
  argument, as a set equality, not a count.
- `a_slit_runs_along_the_meridian_it_was_slit_along` — the slit edge's
  two ends on the meridian `BandSlit`'s argument names.
Plus the count row the header already promised is NOT enough
(`check_total` and the counts stay green under every mutant below —
state it at the claim site).

**Mutants, each named with the rows it reds:** permute the source
argument across `rec.rim_feet`; across `rec.meridian_splits` (the
neighbouring meridian); across `rec.bands`' edge sets; across
`rec.slits`. Each leaves `check_total` and the counts green and reds
exactly its row; say which of the four survive any OTHER row in the
tree (`blend5_rim_support`, `blend5_r1_probes`, the sweep crate's own
rows) — the fixture is only load-bearing where it is the first reader.

**The window.** As `edit_ruled_carve.rs`: derive the one tolerance from
the closest separation a row must resolve, measured and pinned by a
row; no chosen constant.

**Not this unit:** `emit_blend.rs` itself (the fixture proves the
names right or files what it finds); the sweep crate's records.

**Territory:** `crates/editor-core/tests/*` (EDIT, also TCOST/TINT);
`tests/all.rs`. If the derivation of the ladder profile copies
`sweep::test_support`, give it one home there (S-BOOL/FILLET's test
support, disclosed, as the carve did). Middle tier.

## Built (2026-09-17, PR #2794)

`crates/editor-core/tests/edit_ladder_rim.rs` lands with six rows over
one document: a square plate with two round holes, extruded, and both
holes' `End`-cap rims filleted in ONE `Node::Fillet`. Each hole rim is a
LADDER rim — a RING of the cap plane, and each of the two wall faces an
extruded circle mints carries exactly one of its arcs.

The four argument rows the spec names, each reading the argument against
the runtime entity the name resolves to, plus
`the_totality_and_the_counts_read_no_argument_at_all` (the count row,
with the statement at the claim site that it stays green under every
mutant) and
`the_closest_pair_a_row_must_tell_apart_is_a_mint_and_its_source` (the
window derivation: the minimum over both rims of a foot-to-rim-vertex
and a crossing-to-meridian-end separation, measured and pinned; `NEAR`
is its 1e-7).

**Mutant table**, each rotating one `BlendNaming` channel's source
argument by one inside `name_blend` and running the whole `editor-core`
`all` binary (baseline 1418 passed, 0 failed):

| channel | role | rows RED |
|---|---|---|
| `rim_feet` | `BandFoot` | `a_band_foot_…`, `a_slit_runs_along_…`, + 2 digest goldens |
| `meridian_splits` | `BandCross` | `a_band_crossing_…`, `a_slit_runs_along_…`, + 2 digest goldens |
| `bands` | `BandFace` | `a_band_face_…`, + 2 digest goldens |
| `slits` | `BandSlit` | `a_slit_runs_along_…`, + 2 digest goldens |

`check_total` and every count stay green under all four. No other
behavioural row in the tree moves: not `blend5_rim_support`, not
`blend5_r1_probes`, not the `match` arms or the covariance walk. The two
rows that do move under all four are the corpus name-DIGEST goldens
(`lib_g16_corpus_name_digests`, `perf2_name_keying_differential`), which
red under any change to the emitted names and cannot say which argument
is right — change detectors, not readers. **The sentence that stood
here about the sweep crate was false for `bands`** and is corrected in
**After the review** below: `verbs_arms1_annulus` and `verbs_arms3` DO
read that channel's source edge set by identity, on the record the
surgery hands back rather than on a name, and cannot see a permutation
applied in the emitter. The crate's other rows on these channels assert
counts, source-key and minted-key injectivity and one minted face key,
never which source entity a row carries.

**What did not land**, and why: one rim (it would make the `bands` and
`slits` mutants vacuous — those channels carry one row per closed rim);
`die_composed` driven from the registry (it is not the smallest document
that mints the four roles, and it already pays the registry battery);
any addition to `sweep::test_support` (the plate's circles are authored
in the document, so nothing is derived that could be copied); any change
to `emit_blend.rs` (all four arguments are correct on this tree, so
there was nothing to file).

## After the review (2026-09-17, PR #2794)

The review returned MERGEABLE with 0 MAJOR, 2 MINOR, 3 NOTE and 7
style findings. Every one was taken.

**The sweep paragraph above was false for `bands`, and is corrected.**
Three of the four roles are first READ here — nothing in the tree says
which rim vertex a foot was retracted from, which meridian a crossing
split, or which meridian a slit ran along. `BandFace`'s SOURCE set
already has two readers in the sweep crate:
`verbs_arms1_annulus::every_annulus_output_entity_is_a_recorded_mint_or_a_survivor`
asserts `rec.bands[0].1 == vec![rim]`, and `verbs_arms3`'s whole-rim
row flattens `naming.bands`' edge sets and compares them to the rim's
own two arcs. Both read the RECORD `sweep::fillet_edges` hands back;
what is new for `BandFace` here is everything downstream of it — the
emitter's translation of an edge-key set into a set of source edge
NAMES, and a document-layer row reading that name. A permutation
planted in `emit_blend` is invisible to those two rows for exactly that
reason. The sweep that missed them matched `rec\.(rim_feet|…|slits)`,
which cannot see `verbs_arms3`'s `naming.bands` spelling and which read
`verbs_arms1_annulus`'s `rec.bands[0].1` as one of the counts beside
it; the widened pattern is
`\.(rim_feet|meridian_splits|bands|slits)\b` over `crates/`, whose
blind spot is a record channel bound to a local before it is read
(`let (_, srcs) = &rec.bands[0]`) — no such site exists on this tree,
and the mutant runs cover it for `editor-core` from the other side.

**The helpers got one home.** The eleven `edit_ladder_rim.rs` and
`edit_ruled_carve.rs` each carried a byte-identical copy of — `tol`,
`run`, `table`, `key_of`, `edge_of`, `vertex_of`, `face_of`, `point`,
`ends`, `count`, `minted` — moved into
`crates/editor-core/tests/fixture/mod.rs`, with `face_vertices`,
`rim_edge` and `cap_vertex` unified there too and `vname` added beside
`fname`/`ename`. The class outside these two suites is filed as
`editor-core-suites-redefine-the-name-table-helpers` and is NOT swept
here.

**Six measurements replaced six assertions that read as measurements.**
`APART` — how far the plate's closest two meridians actually stand —
now backs the crossing row's "off every other meridian" arm, where
`NEAR` had been standing in for a separation it is not; `CLOSEST` stays
a measured literal and its row says why it coincides with the blend
radius on right-angle supports and what a tapered wall would do to it;
the loop index of each hole has one source (`rims`, derived from the
order `plate` pushes them in); the two-equal-terms `max` became
`footprint_gap`, which reads both ends and requires them to agree; and
the face and counts rows now say where a wrong set actually reds (at
the lookup, not the set equality) and that the counts can red only
through totality.

**Mutant table, re-run after the helper move.** Baseline on this
branch: 1419 passed, 0 failed, 5 ignored. Each mutant is planted in
`names::emit_blend::name_blend` by the reviewer's harness
(`scripts/review-ladder-rim-mutants.py` on `review/ladderrim-rv`, a
review artifact that is NOT carried here), which rewrites one
`BlendNaming` channel's SOURCE halves and leaves the minted keys alone.

| # | channel / op | rows RED |
|---|---|---|
| M1 | `rim_feet` rotate | `a_band_foot_…`, `a_slit_runs_along_…`, + 2 digest goldens |
| M2 | `meridian_splits` rotate | `a_band_crossing_…`, `a_slit_runs_along_…`, + 2 digest goldens |
| M3 | `bands` rotate | `a_band_face_…`, + 2 digest goldens |
| M4 | `slits` rotate | `a_slit_runs_along_…`, + 2 digest goldens |
| M5 | `rim_feet` swap inside one rim | `a_band_foot_…`, `a_slit_runs_along_…`, + 2 digest goldens |
| M6 | `meridian_splits` swap inside one rim | `a_band_crossing_…`, `a_slit_runs_along_…`, + 2 digest goldens |
| M7 | `rim_feet` swap ACROSS the two rims | `a_band_foot_…`, `a_slit_runs_along_…` — and NOTHING else |
| M8 | `meridian_splits` swap ACROSS the two rims | `a_band_crossing_…`, `a_slit_runs_along_…` — and NOTHING else |
| M9 | `bands` drop one member of each set | `a_band_face_…` (at the LOOKUP), + 2 digest goldens |
| M10 | `rim_feet` drop a whole record | all six ladder rows, at the table — and no digest golden |

M7 and M8 are the sharpest result and are new since the review: this
suite is the ONLY thing in the tree that sees them. The two digest
goldens (`lib_g16_corpus_name_digests`,
`perf2_name_keying_differential`) red under the other mutants because
any change to an emitted name moves a digest — they are change
detectors that cannot say which argument is right — but the across-rims
swap and the dropped record leave the corpus's own ladder rims'
channels untouched, so the goldens stay green and only these rows
speak. M10 does not red a count: dropping a record makes the fillet
node fail outright with `Naming(MissingUpstream)`, so every row reds at
the table, which is totality refusing rather than a count noticing.
`check_total` and
`the_totality_and_the_counts_read_no_argument_at_all` stay green under
M1–M9. No other behavioural row in the tree moved under any of the ten.

**Rows filed from the fix pass.**
`editor-core-suites-redefine-the-name-table-helpers` (EDIT's slate):
twenty-four `crates/editor-core/tests/` files still define their own
copy of the readers `fixture/mod.rs` now holds one home for. Only the
two `edit_*` suites were re-pointed here.

## Closed (2026-09-17, EDIT orchestrator)

Built and merged as PR #2794 after one opus style review (MERGEABLE:
0 MAJOR, 2 MINOR, 3 NOTE, 7 style — every one taken in the fix pass).
A plate with two round holes, both cap rims filleted in one
`Node::Fillet`, is the smallest document in which all four
permutations move a name; four rows read `BandFoot`, `BandCross`,
`BandFace` and `BandSlit` arguments against the runtime entity each
name resolves to, ten mutants each red on its row with `check_total`
and every count green, and the two across-rim swaps are seen by
nothing else in the tree. The review's corrections stand in the
record: `BandFace`'s source set already has two readers in the sweep
crate's rows (the emitter-side translation is what is new for it), and
`m6_5_downstream` carries no arm over these roles. The eleven helpers
the suite had copied from `edit_ruled_carve.rs` now have one home in
`tests/fixture/mod.rs` (two of the copies had diverged), with `vname`
beside `fname`/`ename`; the class across the other twenty-four suites
is filed as `editor-core-suites-redefine-the-name-table-helpers`.
The orchestrator's rulings on the lane's three questions: the carve
branch was already merged, so no conflict; `face_edges` moves with its
twin; the tight neighbour-arm bound stays.
