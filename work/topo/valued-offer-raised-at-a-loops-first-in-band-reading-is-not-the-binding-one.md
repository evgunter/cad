---
id: valued-offer-raised-at-a-loops-first-in-band-reading-is-not-the-binding-one
kind: issue
title: topo: a valued offer raised at a loop's first in-band reading is not the binding value, and its zero-band readings come into band below it
status: open
opened: 2026-10-01
---


(TOPO, PR 3513's fifth fix pass, from the coincfr4 review's MAJOR-4.)
The rule is D4 ¶1 (i) in `docs/DESIGN.md`: the tolerance is offered
where a smaller ε decides the decision.

## What

A decision read in a loop refuses at the first reading it finds in
band and quotes that reading's margin. Two things make that value
false for the decision as a whole:

- **A later reading binds harder.** The loop has not read the rest, so
  another reading of the same decision with a smaller margin still lies
  in band at 0.9 × the value offered.
- **A zero-band reading comes into band.** A reading the loop decided
  zero (`On`) at this tolerance, with a margin that is not exactly zero,
  enters the band once the tolerance falls below it, so even the
  smallest in-band margin is not the binding value.

Executed on the review's pose `cc2` (a unit block on the unit block's
corner, turned 3e-9 rad about `(−2, 1, ½)`, public union): the planar
vertex sides (`Coincide::VertexOnFace`, `reduce::sweep_direction`,
`crates/topo/src/boolean/reduce.rs:980`, `:1008`) read three vertices,
at 2.62e-9, 1.31e-9 and 6.5e-10. At 1e-9 the first two are in band and
the third is in the zero band. The refusal offered 2.62e-10. At
2.36e-10 the second vertex refuses, and below 1.31e-10 the third. The
operation passes only at 5e-11.

## Done here

PR 3513's fifth pass withdraws `VertexOnFace`'s tolerance
(`LeverPass::Unbound`). The offer that would be made is executed as
`offer_rows::corner_turned_on_a_corner`, which refuses again on
`Coincidence(VertexOnFace)` (F1).

## What remains

By inspection, the same shape stands wherever a valued decision is raised inside a
loop over readings, the coincfr4 review's poses just do not reach it:
`vtxfac::classify_vertex_on_face`'s coplanar sectors
(`crates/topo/src/boolean/vtxfac.rs:201`), `sectors::pair_search`
(`crates/topo/src/boolean/sectors.rs:875`), the curved sweep arm and
the join's matching. To make such an offer true, the site has to read
every reading of the decision before it refuses, and offer the
smallest nonzero margin among those in band or in the zero band. That
value may be one only round-off sets, which is no tolerance a user
chooses. Each site then has to choose between that value and
withdrawing the offer, as `VertexOnFace` did.
