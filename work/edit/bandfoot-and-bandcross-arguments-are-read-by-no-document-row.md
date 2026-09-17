---
id: bandfoot-and-bandcross-arguments-are-read-by-no-document-row
kind: issue
title: No document row reads a BandFoot's, BandCross's, BandFace's or BandSlit's argument — four blend mints ride check_total and a count alone
status: spec
opened: 2026-09-16
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
  (`m6_composed_node.rs`, `m6_5_downstream.rs`);
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
