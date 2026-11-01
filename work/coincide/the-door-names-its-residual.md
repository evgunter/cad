---
id: the-door-names-its-residual
kind: issue
title: The coincidence door's recourse names the residual: two free variables of one kind and equal value read as one, two placed poses placed one relative to the other
status: open
opened: 2026-10-11
priority: P1
cost: M
---

Ratified scope, deferred rather than new: the "recourse (Q3)" paragraph
of `carriers-compare-in-canonical-form`, whose design Ev approved in PR
4322 (FORK-S4F, fork log row 93). The orchestrator left it out of that
unit: the unit replays a document at `Sym`
with every variable a symbol and proves rows by two rungs, but an
unproven row still says only `Recourse::OneConstruction`.

The item asks for more. A residual `v − w`, over two free variables of
one kind and equal value, recourses "make the slot reading `v` read
`w`" (the GUI's offer, `typing-a-value-mints-or-offers-a-variable`). A
residual in two placed poses recourses "place one relative to the
other".

What it needs:
- the deciding margin's DAG node on the reading. A deferred Zero
  already keeps it on the session's list (`geom_core::sym::deferred`),
  so a door over that list reaches the node without growing
  `MarginDiag`;
- a reader of the residual's normal form that recognises `c·(s₁ − s₂)`
  in two parameter symbols, mapped back to their `VarId`s through
  `ParamSymbol::new(var.digest())`;
- for poses, a way to tell which opaque atoms came from which solve
  (one per solve until stage 3).

Done when an unproven row over two separately typed equal values
reports the read-one-variable recourse naming both, pinned beside
`two_variables_typed_equal_are_one_constant_to_the_analysis_lane_and_two_symbols_to_the_door`.

## Evidence (2026-10-23, INTENT's mirror-seam designer pair)

A `Numeric` replay on a pair the document never relates recourses `Budget`. Two examples: the seam of a C-shape's second prong, which is coplanar with a mirror plane by value only, and today any mirror seam. That is the wrong lever: raising the budget cannot prove a coincidence the document does not state. The recourse for that row class should name the statement the user could write: read the plane off a face both prongs are built from, or state the collinearity in the profile. See `work/intent/a-symmetric-parts-seam-is-not-walked-as-one-construction.md`.
