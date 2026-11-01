---
id: sym-item-docs-carry-unit-archaeology
kind: issue
title: the sym tier's item docs carry unit archaeology: the §4 sweep the SYM-2 header pass was fenced out of
status: open
opened: 2026-09-14
priority: P4
cost: E
---


**Found by SYM-2's review.** SYM-2 applied
`docs/prompts/implementer-discipline.md` §4 to the tier's module header
and its spec fenced the pass there ("apply §4 to the header sentences you
move or leave"). The same class sits in the ITEM docs, which the unit
moved verbatim and did not read for it. This row is the rest of the
sweep, scheduled rather than done, so that whoever takes it takes it as
one pass with one reviewer rather than folding prose edits into a move.

**The rule**, from `memories/cad-working-style.md`: keep a comment only if
something would go wrong without it. A cut is a sentence whose only
content is how the code came to be — the incident, the retired
alternative, the milestone that measured it — and the repair is the
present-tense invariant it defends, with any MEASUREMENT kept (a number
here is a measured claim and usually the reason a constant has its
value).

The list, at `c9a38f5e4`. Each is a sentence, not a whole doc:

- `sym.rs`, `SymId::UNRECORDED` — "It used to be a second thing as well —
  the id every [`Sym::opaque`] value carried — and that was a soundness
  defect …". The defect is the argument for the current design and reads
  as an invariant once turned around.
- `sym.rs`, `SymOp::Powi` — "The unit's spec listed `powi` among the
  opaque atoms and this is a deliberate departure from it, disclosed as a
  deviation". A deviation from a spec that no longer exists in the tree.
- `sym.rs`, `form_in` — "That split is measured, not assumed: the first
  cut of this unit let a ruled form REPLACE the plain one and lost an
  `arc_span` cancellation and a straight edge's endpoint theorem to it."
- `sym.rs`, `discharge` — "(An earlier cut asked the door here and said in
  its own comment that it asked last; under `SymRules::all()` that
  attributed A/B theorems to `registered`. R1 m1 / R2 MINOR-4.)"
- `sym.rs`, `SymRules::early_ab` — "which is what made the first cut cost
  138 s per nominal plate replay" (SYM-2 cut the header's copy of this
  sentence and named this one as its surviving home, so the MEASUREMENT
  stays wherever this lands).
- `form.rs`, `Poly::mul` — "The first version built the whole product and
  let [`within`] reject it afterwards, which is how a single
  multiplication came to take 10.8 s on a reviewer's bracket."
- `form.rs`, `Form::poisoned` — "Why a poison and not a freeze (the
  reviewer's row `atan(1/(x-x)) - atan(1/(x-x))`)": the row is the
  argument and stays; the reviewer is the archaeology.
- `algebra.rs` ~`:117` and `signed.rs` ~`:62` — two more "the first cut
  …" sentences, the same shape, found by grepping the shape rather than
  the file.
- `SymOp`'s own doc ("Everything outside the ring operations is an OPAQUE
  atom (module docs)") points at the header paragraph that
  `sym-header-dag-paragraph-disagrees-with-the-code` says has no bucket
  for `SymOp::Opaque`. Whoever fixes that paragraph re-points this.

`Rat`'s copy of the i128 story is NOT on this list: SYM-2's fix pass
collapsed it against the module header it now duplicated, and what is
left on the type is the invariant a caller needs.

Not a defect hunt: none of these is wrong, and a reader who knows the
history loses nothing by them. What they cost is the thing the parent
item was filed about — prose that reads as history stops being read as a
contract.
