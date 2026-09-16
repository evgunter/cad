---
id: bandfoot-and-bandcross-arguments-are-read-by-no-document-row
kind: issue
title: No document row reads a BandFoot's or BandCross's argument — the ladder rim phase's other two mints ride check_total alone
status: open
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
is an exhaustive `match` arm that classifies a role into a word
(`m6_composed_node.rs`, `m6_5_downstream.rs`) or a covariance walk over
whatever `NameRef`s a segment wraps (`m4_pr4_resolve.rs`). None of them
says what argument the role should carry.

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
