---
id: a-name-pattern-cannot-say-descends-from-this-node
kind: issue
title: A NamePat cannot say 'a face this node minted, through any number of booleans': the author spells the boolean nesting segment by segment
status: open
opened: 2026-10-03
priority: P3
cost: M
---



Found by SHOW's `the-plate-document-never-cuts-its-holes`.

`demos/tour/src/plate.rs` `cut_plate` reads each bore wall off the cut
part (`blank ∖ hole_a ∖ hole_b`). The face hole a cut is named
`FromA(FromB(<hole_a's Lateral piece>))` on the part, so the selector
that picks it is

    face().seg(SegPat::tag(SegTag::FromA).of([
        face().seg(SegPat::tag(SegTag::FromB).of([face().node(hole_a)])),
    ]))

and hole b's is `FromB(<hole_b's>)` — one segment per boolean between
the minting node and the node read at, with the operand side of each.
The author has to know the DAG's nesting to say "the wall hole a cut",
and a third cut ahead of these (or the two cuts reordered) changes
every pattern. `NamePat::node` matches only the OUTERMOST minting node
(`crates/editor-core/src/names/select.rs`, `NamePat::level_matches`),
and `path: None` frees the segments of one level, not the depth.

What a user means is lineage: "faces whose name descends from a name
`hole_a` minted". Unmeasured whether a descendant-of atom keeps the
selector's equivariance arguments (SELECT-DESIGN) intact.
