---
id: a-chord-between-two-merged-faces-within-no-rim-refuses
kind: issue
title: A pair boolean's chord between two merged faces within no rim of their same-side faces refuses, where widened N3 reads it as a seam of the two Merged parents
status: dispatched
opened: 2026-10-18
priority: P2
cost: M
design: true
pr: 5423
branch: wire/ev-pairseam-order
---


## Finding

`crates/editor-core/src/names/emit_topo.rs`, `name_boolean_edges`'s
`chord_kind`, `(true, true)` arm, and `merged_pair_rim`: a chord between
two faces the pair merged is named as the rim a face of its key's side in
one merged face shares with one in the other, where it lies within one
(`rim_among`). A chord that lies within none refuses
`NamingError::MergedChordOffRim`, and a chord with no key side
(zip-listed) refuses `NamingError::MergedChord`. The arm is reached in
`emit_topo`'s unit rows and by no model in the suite. N3 as widened on PR 5279 says a seam at every
boolean cites, as each side, the parent of the face beside it; a chord
between two merged faces off every rim is a seam between their two
`Merged` parents, which no arm mints.

Two things to settle before it is built:
- the sides' order: a pair seam is `Sided` (`a` the A operand's), and a
  seam between two merged faces, each holding faces of both operands, has
  no A side. `crates/editor-core/src/names/canonical.rs`
  `Seams::of_published` reads a published seam as a union's when both
  sides are minted by the name's own node, which a pair seam between two
  merged parents would be; it relies on one side at least being an
  operand's name.
- whether such a chord occurs in a pair boolean at all outside a union
  (where the other operand filled the gap between the two faces), or only
  through a zip-listed edge.

No witness built; found by the sweep of `wire/seam-cites-parent`.

Signed (WIRE lane `wire-seamparent`).

## Measured (WIRE lane `wire-mchord`, 2026-10-20, origin/main `e763c9c732`)

`MergedChord` (the zip-listed edge between two merged faces) **is
reached by a two-member `Union` of two prisms**, and by `Subtract`, by
3-member union folds and by split-then-union. In every hit the edge is
zip-listed; the derived `(true, true)` arm is never reached. The
editor-core suite (3163 tests) reaches the arm zero times.

Witness 1 (blocks helpers; `prism(o, u, v, loop, d)` = `on_frame` +
`Extrude Along`), a `Union` that refuses `Naming(MergedChord)`:
```rust
let (doc,a)=prism(doc,[0.,0.,0.],X,Y,&[(0.,-1.),(2.,-1.),(2.,0.),(4.,0.),(4.,2.),(0.,2.)],1.0);
let (doc,b)=prism(doc,[1.,0.,0.],Y,Z,&[(0.,-1.),(3.,-1.),(3.,1.),(1.,1.),(1.,2.),(0.,2.)],2.0);
Node::Union{members:Bodies::Spelled(vec![a.into(),b.into()])}
```
A's top crosses B's front along y=0, z=1, x∈[1,2]; z=1 and y=0 each hold
a coincident face pair. `Intersect` of the same pair publishes the edge
as a seam with two `Merged` sides, through the n-ary collapse, which
orders them by name.

Witness 2 (`Subtract`, 40 of 48 rotation/mirror/extent variants refuse):
```rust
let (doc,l)=prism(doc,[0.;3],X,Y,&[(0.,-1.),(2.,-1.),(2.,0.),(4.,0.),(4.,2.),(0.,2.)],1.0);
let (doc,bar)=block(doc,(0.,4.),(1.,2.),1.0,1.0);
let (doc,a)=insert(doc,Node::Union{members:Spelled(vec![l.into(),bar.into()])});
let (doc,b)=prism(doc,[1.,0.,0.],Y,Z,&[(-2.,-1.),(0.,-1.),(0.,2.5),(1.,2.5),(1.,1.),(3.,1.),(3.,3.),(-2.,3.)],2.0);
Node::Subtract{from:a.into(),tool:b.into()}
```

The design question is the seam's side order when both sides are
`Merged` parents. The pair is `Sided`, and `Seams::of_published`
(`canonical.rs` ~103) reads a seam with both sides minted by its own
node as a union's (`ByName`). So the fork is shared with
`should-pair-seams-take-name-ordered-sides` (FORK-PAIRSEAM).

`MergedChordOffRim` has no witness. The lane argues that it is reached
only by a crossing the zip did not list, or by a chord keyed to the wrong
side. It suggests the derived arm try both sides' rims, so that Off on
both sides means a kernel defect.

Raised to P2: a plain two-prism union refuses with a kernel-defect
ending.
