---
id: id-lowerings-to-u64-tokens-drop-the-ordinal
kind: issue
title: GeomSource and ParamSymbol lower a node or variable id to its 64-bit digest, which no longer refuses an in-document collision
status: open
opened: 2026-10-07
priority: P3
cost: M
design: true
refs: [name-order-was-insertion-order-under-the-counter]
---


## What

A node, step or variable id is a pair, its mint ordinal and the first
64 bits of its chain digest (`crates/editor-core/src/mint.rs`, `MintId`;
`names/README.md`, N1). Within one document the ordinal alone makes
every id distinct, so the mint no longer refuses a digest it has drawn
before. Two kernel tokens still take an id as one `u64`, and editor-core
lowers the id to its digest for them:

- `topo::GeomSource::minted(node, ..)` and `GeomSource::placed(by, ..)`
  (`crates/topo/src/source.rs`), stamped in
  `crates/editor-core/src/eval/wire.rs` (`stamp_minted_from`'s three stamps,
  `compose_placed`) with the digest. `GeomSource` identity is the
  declared-coincidence rung's whole test (`same_base`).
- `geom_core::ParamSymbol::new` (`crates/geom-core/src/sym.rs`), built in
  `crates/editor-core/src/analysis.rs` (`AxisScalar::axis_of` for
  `Sym<T>`) with `var.0.digest()`. The symbol becomes the payload of an
  interned `SymNode`, a single `u64` slot shared with `Lit`'s float bits.

Before the pair, the mint refused a 64-bit draw it had logged
(`NodeIdCollides`, `VarIdCollides`), so these tokens were injective
within a document by refusal. Now two ids of one document that share a
digest — about 2^-64 per pair — lower to one token: one geometric
source, or one symbolic indeterminate, silently.

Neither token needs ONE integer; each needs an injective, ordered,
hashable identity. The ordinal alone would be injective within one
document but repeats across documents (a part's evaluation meets its
host's), which the digest does not.

## Options

- Widen both tokens to carry the pair (`GeomSource.node`,
  `SourceExpr::Placed.node`, `AxisSource::placed`; `ParamSymbol` and the
  `SymNode` payload it rides in), and lower the whole id.
- Keep the digest lowering and refuse the collision where it is
  lowered, loud rather than silent.
- Accept the 2^-64 per pair, said where the lowering is.
