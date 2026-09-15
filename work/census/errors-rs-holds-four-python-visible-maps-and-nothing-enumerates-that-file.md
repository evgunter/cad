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

The five are also not the whole of what this file spells. Its live
answer is `ERRORS_MINTING_ITEMS` in `crates/pncad-py/src/tests.rs` —
nine items and 52 literals, each row naming what holds its words —
and that roster, not this table, is the thing a future arrival has to
join.

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

## The disposition taken (CENSUS-ERRORS-ARRIVAL, 2026-09-15)

**An arrival alarm over the file, keyed on its string LITERALS rather
than on any item form.** `crates/pncad-py/src/tests.rs` holds
`ERRORS_MINTING_ITEMS`, a roster with one row per item in this file
that spells a literal — its owner, how many it spells, and the check
that holds those words — and
`errors_rs_spells_literals_in_exactly_these_items` compares the roster
against the file on every run.

**Why keyed on literals.** The row's own proposal — a looser reader
over the file's items — inherits the defect it is built against. Every
instrument this crate has aimed at its vocabulary was keyed on a FORM,
and each went blind to the arrival that did not wear it: the tag
reader's function forms strip `pub fn `, and all five maps here are
`pub const fn`; a top-level-keyed reader sees three of the five. So
the population is not this reader's to define. `test_utils::source`
says which bytes of the file are inside a literal, the reader
attributes each to the item that spells it, and there is no form a
word can arrive in that the reader was not taught, because there is no
form. Attribution can be wrong, and a wrong attribution is loud: it
invents a name the roster does not carry.

The population it reports is **nine items and 52 literals**, not five
maps — `EvalReason::ATTRIBUTE`, `ValidationRefusal::ATTRIBUTES`,
`QuantityOpMismatch`'s `Display` format string and `reads_as_prose`'s
fingerprint are all literals in this file and three of the four are
Python-visible, which the five-map framing did not count.

**The other two shapes, and why not.** Capitalising at the boundary so
`measurement_dimension_tag` stops existing takes the file from five
maps to four and closes arrival not at all — it is a population
reduction offered against a reach problem. Ruling that a
Python-visible word comes from `tags.rs` only runs into the same
`pub const fn` fact from the other side: none of the five is a form
that reader admits, and two are inherent methods whose call shape
cannot move without the enum moving too.

### What this does not close

- **A word that is not a literal here.** A map forwarding
  `crate::tags`', or a word built from a kernel `Display`, adds no
  literal and no row, and the census passes green.
  `the_errors_mint_census_cannot_see_a_word_that_is_not_a_literal`
  executes exactly that and is the record of it.
- **A word RENAMED in place.** The roster holds each item's literal
  COUNT, so growth and loss are loud and a swap inside one item is
  not; that is the `held_by` column's business, which is why every row
  names a check or says plainly that no Rust check names the word
  (`EvalReason::ATTRIBUTE` is that row).
- **Everywhere else.** This is one file's alarm. The same question over
  the rest of the crate is
  `payload-attribute-names-are-spelled-twice-and-held-equal-by-nothing`,
  filed by the same unit.
- **Misattribution to the row above.** A literal in an attribute
  (`#[doc = "…"]`, a `#[pyo3(name = "…")]` were one ever written
  here) lands on the item ABOVE it, because attribution is by the
  nearest declaration above. The count still moves, so it is loud on
  the wrong row rather than silent; the reader's own guard pins that
  behaviour by execution.

### What was executed

A sixth map was added to this file for real — an inherent
`pub const fn` inside an `impl ValidationRefusal`, the position the
census is weakest against and where both already-unseen maps live. It
reds by name, and **nothing else in the crate's Rust suite moved** —
100 other tests passed over a file that had just grown two
Python-shaped words. That is the row's thesis, measured.

The reader's own guard is
`the_errors_mint_reader_recognises_what_it_claims`, driven over a
fixture holding one of every form it reads, with six more tests
executing its limits: the file going quiet, an arrival inside an
`impl`, the not-a-literal blind spot, and three refusals it must make
loud. One claim in the fixture's first draft was wrong and execution
corrected it: an `extern "C"` ABI string IS a literal and IS counted,
where the draft reasoned that the lexer would blank it.

### What the tag reader does with a `pub const fn`

Measured: it **fails loud**, and before this unit it named the wrong
thing — `TopForm::Const` matches on `pub const `, a prefix of
`pub const fn `, so the line was refused as a malformed `&str` const.
`TopForm::Const` now declines the `fn` case and `top_form`'s
diagnostic ladder names the form, with
`the_tag_table_reader_refuses_a_const_fn_map` driving the rung. The
grammar still admits no `const` map; nothing was widened.

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
