---
id: a-loft-takes-its-placements-as-f64
kind: issue
title: loft_body and sweep_body take their placements as f64, so at Sym a document's placement parameters reach the loft as unknowns
status: open
opened: 2026-10-10
priority: P2
cost: M
refs: [sweep-keeps-computed-values-in-t]
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
