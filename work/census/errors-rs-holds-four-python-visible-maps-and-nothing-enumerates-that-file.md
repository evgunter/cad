---
id: errors-rs-holds-four-python-visible-maps-and-nothing-enumerates-that-file
kind: unit
title: errors.rs holds Python-visible word maps and no instrument enumerates the file
status: spec
opened: 2026-09-15
branch: census/errors-arrival
---


Filed by CENSUS-PY-GETTERS' fix pass (2026-09-15). That unit moved six
discriminant maps into `crates/pncad-py/src/tags.rs`, where a reader
lexes the file and `TAG_INVENTORY` pins every word — and moved the
seventh into `crates/pncad-py/src/errors.rs` instead, because the tag
reader refuses a value that is not lower snake case and
`Measurement.dimension`'s four words are capitalised. That siting is
right. What it exposed is that `errors.rs` has no reader at all.

## What is in the file, and what holds it

**FIVE maps as of `census/py-raise-literals`, not four** — the row was
opened at four and the fifth landed six commits later, which is the row
happening rather than an error in it. The id keeps `four` because
`docs/DOC-LEDGER.md`'s CENSUS-PY-GETTERS entry cites it as the residue
filed under that name; read it as a name, not as a count.

`errors.rs` holds these maps whose values reach Python:

| map | what it answers | held by |
| --- | --- | --- |
| `dimension_tag` | the lower-case FFI dimension word | `dimension_tags_are_stable` and `dimension_tags_match_the_kernel_prose`, the latter over `Dimension::ALL` |
| `measurement_dimension_tag` | `Measurement.dimension`, capitalised | `the_two_dimension_alphabets_are_one_list_in_two_cases`, over `Dimension::ALL` |
| `canonical_unit` | `"m"` / `"rad"` / `None` | `canonical_units_match_the_gq5_ratification`, four hand-written rows |
| `ErrorClass::class_name` | 35 Python exception class names | `error_classes_name_the_python_hierarchy`, whose expectation is a SECOND exhaustive match, so growth stops the build |
| `ValidationRefusal::attribute` | `door` or `reason` — WHICH Python attribute a validation refusal's word lands on | added by CENSUS-PY-RAISE-LITERALS with no pin; pinned by its fix pass with `the_validation_class_mints_exactly_these_attributes` (the image of the map over `ValidationRefusal::ALL`, both directions) and `every_validation_refusal_writes_the_attribute_it_is_committed_to` (the per-refusal routing) |

**So no map here is unguarded today.** The row is not about the five.

## The row

**Nothing enumerates the file.** `tags.rs` gets *"NEW tag function
`x`, minting [...] — a new set of public Python words that no
inventory has looked at"* the moment a map appears there, and it is
that sentence, not any one pin, that makes the file safe to add to.
`errors.rs` has no equivalent: a further `&'static str` map added
beside these is Python-visible vocabulary with no pin, and nothing
anywhere says so. The ones that are pinned are pinned because someone
thought to pin each.

**That is no longer a prediction.** CENSUS-PY-RAISE-LITERALS added
`ValidationRefusal::attribute` — two Python-visible words, `door` and
`reason`, both declared in `pncad.pyi` and both read by
`tests/test_validate.py` — with no pin, no sentence anywhere saying a
fifth map had arrived, and this row untouched in the same PR. It was a
style reviewer, not an instrument, that found it. Its fix pass pinned
the map and updated the table above.

That is exactly the CENSUS charter's third shape — *"where one DOES
exist but scans the wrong population, the drift is invisible and
certified"* — one file over from where the instrument is.

## What a fix would look like, and the trap in it

The cheap move is to widen the tag-table reader's scan set to
`errors.rs`, which is wrong as stated: the reader's exactness comes
from refusing everything it does not understand, including a
capitalised value, and `errors.rs` holds an enum, an impl block and
prose helpers the recogniser rejects by design.

The shapes worth weighing instead:

- a SECOND, looser reader over `errors.rs` that only has to answer
  "how many `-> &'static str` maps are in this file" and compare
  against a pinned count — an arrival alarm rather than a word
  inventory, which is all the gap needs. **The instance that arrived
  is a counter-example to the cheapest version of it**:
  `ValidationRefusal::attribute` is an inherent method inside an
  `impl` block, `pub const fn attribute(self) -> &'static str`, not a
  free `pub fn` at top level. A reader keyed on the file's top-level
  items — the shape the three `dimension`-family maps have, and the
  shape `tags.rs`'s reader is built on — would not have seen it, and
  would have reported agreement over a population that had grown. It
  is the second map here in that position (`ErrorClass::class_name` is
  the first), so the looser reader has to walk `impl` bodies, which is
  most of what made the tag reader hard;
- or moving the capitalisation out of the map: if
  `Measurement.dimension` were minted by capitalising
  `dimension_tag`'s word at the boundary, that map would not exist and
  `errors.rs` would be back to four. That changes no word's value and
  is a `pncad-py` call, not a kernel one.

Either way the instrument is a hand-maintained thing needing a guard
of its own, which is this program's standing trap and the reason this
is a row rather than a line in someone's PR.

## Two observations about WHY the file keeps growing

Both were raised in CENSUS-PY-RAISE-LITERALS' style review, rated
`likely` taste rather than defect, and adjudicated onto this row as
evidence rather than as work of their own.

- **The file is drifting toward a door-argument dump.** That unit added
  `BoundaryEdit`, `UnmirroredSelect` and `StlRefusal` here — three enums
  that are arguments to raises in `crate::py`, not taxonomy — and each
  is sited here for one reason: `tags.rs`'s source-text recogniser
  admits no `enum`, so an enum whose map must be inventoried cannot
  live beside its map. The module header answers "what is the binding
  error taxonomy" and now also "what values do three raises take",
  which is the growth this row's instrument would have to count.
- **A reader's grammar is deciding where production code lives.** The
  siting argument above is not about the code: it is about what
  `tests.rs`'s lexer can parse. That is worth stating plainly in
  whatever disposition this row takes, because widening the recogniser
  to admit an `enum` would change where five types belong.

Territory: `crates/pncad-py/*` is LIB's fence and CENSUS's `keep_out`
announces its pncad-py rows there.
