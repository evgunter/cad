---
id: a-loft-takes-its-placements-as-f64
kind: issue
title: loft_body and sweep_body take their placements as f64, so at Sym a document's placement parameters reach the loft as unknowns
status: closed
opened: 2026-10-10
priority: P2
cost: M
refs: [4591]
pr: 4741
closed: 2026-10-11
---


`sweep::loft_body` takes `places: &[Affine3<f64>]` (`crates/sweep/src/loft.rs`,
`loft_body`/`build`), and editor-core's Loft node hands it
`ProfilePre::placement_f64`, read out of its own lane
(`crates/editor-core/src/eval/wire.rs`, `wire_loft` / `section_of`).
Since `sweep-keeps-computed-values-in-t` (PR 4591) the loft lifts each
placement through `Real::from_computed`, which is honest about the
`f64` it is given: at `Sym` every placement entry is its own unknown.
But the document had those placements as functions of its parameters,
and the loft can no longer carry them: on that PR's pin prism (two
squares, identity and a translation) the build's own `symbolic_zero`
falls 596 → 428 (numeric 124 → 292), every lost discharge an identity
over the placements. Nothing refuses or escalates on any fixture
measured there.

Owed: the loft takes its placements in `T` (`&[Affine3<T>]`), skinning
from their `f64` values, so a placement the document computed stays
the function it was; `sweep_body`'s `sweep_places` output is genuinely
computed and keeps `from_computed`. A public signature change: run
`demos/tour` and `demos/wild` clippy.

**The library half landed** with `the-skin-computes-its-values-in-t`:
`loft_body` takes `&[Affine3<T>]` and `sweep_body` an `Affine3<T>`
start frame, and the loft skins from their value channel, so a library
caller's placement stays the function it was (`sweep_places` enters
only the path's offsets, turn axes and angles through
`from_computed`, and composes the caller's frame in `T`).

**What editor-core still owes**: its Loft node reads the placement out
of its lane as `f64` (`section_of`, DM1c) and lifts it at the door
through `Real::from_computed` (`wire_loft`), so in a document the
placements still reach the loft as unknowns at `Sym`. Feeding them in
`T` is the document half,
`a-loft-is-the-nominal-surface-in-every-generic-lane`, where (per its
ruling) this item dissolves.

Closed by #4741: dissolved, as
`a-loft-is-the-nominal-surface-in-every-generic-lane`'s ruling says.
The document's loft reads each section's profile value, whose plane
is its placement in `T`, so there is no `f64` placement input left.

Its `Sym` measure (the `symbolic_zero` count this item gave for a
placement parameter at `Sym`) was not re-taken. With the item closed it
names no live code path, because a document's loft no longer takes an
`f64` placement, so no file is owed for it. The `Sym` placement in `T`
is pinned by `a_placement_in_t_stays_a_function_of_its_parameter`
(`crates/sweep/tests/a_loft_computes_its_walls_in_t.rs`).

