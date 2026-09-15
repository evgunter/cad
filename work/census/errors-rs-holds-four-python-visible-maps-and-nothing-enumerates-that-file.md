---
id: errors-rs-holds-four-python-visible-maps-and-nothing-enumerates-that-file
kind: issue
title: errors.rs holds four Python-visible word maps and no instrument enumerates the file
status: open
opened: 2026-09-15
---


Filed by CENSUS-PY-GETTERS' fix pass (2026-09-15). That unit moved six
discriminant maps into `crates/pncad-py/src/tags.rs`, where a reader
lexes the file and `TAG_INVENTORY` pins every word — and moved the
seventh into `crates/pncad-py/src/errors.rs` instead, because the tag
reader refuses a value that is not lower snake case and
`Measurement.dimension`'s four words are capitalised. That siting is
right. What it exposed is that `errors.rs` has no reader at all.

## What is in the file, and what holds it

`errors.rs` now holds four maps whose values reach Python:

| map | what it answers | held by |
| --- | --- | --- |
| `dimension_tag` | the lower-case FFI dimension word | `dimension_tags_are_stable` and `dimension_tags_match_the_kernel_prose`, the latter over `Dimension::ALL` |
| `measurement_dimension_tag` | `Measurement.dimension`, capitalised | `the_two_dimension_alphabets_are_one_list_in_two_cases`, over `Dimension::ALL` |
| `canonical_unit` | `"m"` / `"rad"` / `None` | `canonical_units_match_the_gq5_ratification`, four hand-written rows |
| `ErrorClass::class_name` | 35 Python exception class names | `error_classes_name_the_python_hierarchy`, whose expectation is a SECOND exhaustive match, so growth stops the build |

**So no map here is unguarded today.** The row is not about the four.

## The row

**Nothing enumerates the file.** `tags.rs` gets *"NEW tag function
`x`, minting [...] — a new set of public Python words that no
inventory has looked at"* the moment a map appears there, and it is
that sentence, not any one pin, that makes the file safe to add to.
`errors.rs` has no equivalent: a fifth `&'static str` map added beside
these four is Python-visible vocabulary with no pin, and nothing
anywhere says so. The four that are pinned are pinned because four
separate people thought to pin them.

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
  inventory, which is all the gap needs;
- or moving the capitalisation out of the map: if
  `Measurement.dimension` were minted by capitalising
  `dimension_tag`'s word at the boundary, the seventh map would not
  exist and `errors.rs` would be back to three. That changes no word's
  value and is a `pncad-py` call, not a kernel one.

Either way the instrument is a hand-maintained thing needing a guard
of its own, which is this program's standing trap and the reason this
is a row rather than a line in someone's PR.

Territory: `crates/pncad-py/*` is LIB's fence and CENSUS's `keep_out`
announces its pncad-py rows there.
