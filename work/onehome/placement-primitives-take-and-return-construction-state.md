---
id: placement-primitives-take-and-return-construction-state
kind: issue
title: transform_rigid and mirror_body stay Body-to-Body; each placement door finishes after calling them
status: open
opened: 2026-11-02
---


Found by the ONEHOME finished-body lane (branch `onehome/finished-body`),
outside its fence.

The lane made every door that returns a body return the scalar's
finished type, but kept the two rigid-map primitives as construction
state: `topo::transform_rigid` (`crates/topo/src/transform.rs:571`) and
`topo::mirror_body` (`crates/topo/src/mirror.rs:118`) take `&Body<T>` and
return `Body<T>`. Each placement door that calls them finishes at its
own end instead: `editor_core::Framed::read_in`
(`crates/editor-core/src/eval/framed.rs:275`), the twist and mirror ops
and the fused member read in `crates/editor-core/src/eval/wire.rs`,
`pncad::export::body_in_space` and `product_body_in_space`
(`crates/pncad/src/export.rs`), and step-import's placed copies.

A rigid map preserves every tier-3 verdict (it admits only det = +1
maps for `transform_rigid`; the mirror reverts senses so +V holds), so
finishing after it re-runs a gate whose answer the input's finish
already fixed. The question this row holds: should the primitives take
and return `T::Finished` (a finished body in, a finished body out, no
re-run), which needs a constructor that carries a verdict across a
rigid map without re-deriving it — a design reading of
`[topology.tiers]`, since `finish` is today the one constructor — or is
the re-run the price of keeping `finish` the only constructor? Until it
is answered, the placement doors pay one finish per map.
