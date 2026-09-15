---
id: errors-rs-holds-four-python-visible-maps-and-nothing-enumerates-that-file
kind: unit
title: errors.rs holds Python-visible word maps and no instrument enumerates the file
status: closed
opened: 2026-09-15
branch: census/errors-arrival
closed: 2026-09-15
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
**ten items and 53 literals**, each row naming what holds its words —
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
attributes each to the item that spells it. **An item that spells a
literal is loud however it is written** — form, depth, trait and
literal kind alike, because the population is the shared lexer's answer
and not a grammar of forms. An item that spells NO literal is the one
exception.

That sentence used to read *"there is no form a word can arrive in that
the reader was not taught, because there is no form"*, and to claim a
wrong attribution is always loud because it invents a name the roster
does not carry. **Both were false and were executed as false** — see
the style-review section below. Both are closed; the wording above is
what survives.

The population it reports is **ten items and 53 literals**, not five
maps — `EvalReason::ATTRIBUTE`, `ValidationRefusal::ATTRIBUTES`,
`QuantityOpMismatch`'s `Display` format string and `reads_as_prose`'s
fingerprint are all literals in this file and three of the four are
Python-visible, which the five-map framing did not count. The tenth is
`is_bare_camel_token`, which spells exactly one literal — the char
`'_'` — and was found only when the char-literal hole closed; the count
was nine and 52 until then.

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
- **An `impl` at indentation** — one inside a `mod` — reports an
  arrival loudly, but under a bare, unqualified name. The loudness
  rests on the key rather than on rustfmt: what used to be quiet was
  the collision, and putting the trait in the key took that from two
  `fmt`s to a name Rust itself rejects. On
  `the-errors-arrival-blind-spot-list-claimed-exclusivity-and-was-short`.

Misattribution to the row above WAS listed here, graded loud-on-the-wrong-row.
**That grading was wrong** — an attribute literal landing on a rostered
row is silent and cancels against a deletion — and the case is closed,
not narrowed.

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

### What the style review found, and what the claim says now
(fix pass, 2026-09-15)

**The central claim was false and was executed twice**, against the
real tree, by the style reviewer:

- A CHARACTER literal was read and DROPPED, and with it the item that
  spelled nothing else. `impl ValidationRefusal { pub const fn sep(self)
  -> char { '/' } }` spliced onto `src/errors.rs` gave `[]`.
- A literal in an OUTER ATTRIBUTE was charged to the item ABOVE it, so
  `#[deprecated(note = "…")]` inflated the previous roster row rather
  than inventing an unrostered name — and a deletion in the same item
  cancelled it to `[]`.

Both are **closed**, not narrowed. Every literal counts now, character
literals included and carried as written; a declaration's head starts
at the first attribute of the run above it, so an attribute literal
lands on the item it decorates, and an attribute on an item this
reader cannot name refuses rather than charging it anywhere. Four
tests execute the three cases and the refusal.

**The duplicate-name key was `SelfType::fn_name`**, so
`impl fmt::Debug for QuantityOpMismatch` beside the existing
`impl fmt::Display` hard-stopped the census with *"Qualify them apart
in the same diff"* — an instruction Rust gives no way to follow. The
key is `(self type, trait, item name)` now, spelled `<Type as
Trait>::name`, **which is this program's own unit-2 key**:
`crates/test-utils/tests/hand_written_impl_census.rs` keys on
`(path, trait, self type)` and says at the site why the trait is in it.
This unit had dropped it and walked into the collision that census had
already solved.

`the_errors_mint_census_reds_when_the_file_goes_quiet` drove the
comparison only — `read_minting_items("")` is empty for a broken reader
too — while its doc claimed both halves. It drives both now, over an
empty source and over a legible file holding none of the roster, and
asserts every row BY NAME. Proven by mutation in both directions: a
reader returning empty for every non-empty source reds it (it was green
before), and disabling the missing-row loop reds only it.

Four operations this census had written for itself moved to
`crates/test-utils/src/source.rs`, which is where the tree's shared
lexer already lives: `impl_head`, `type_base`, `ident` and
`line_start`. Each had a second implementation by a different
algorithm in a sibling census; `hand_written_impl_census.rs`,
`deny_unknown_fields_census.rs` and one `editor-core` test read the
shared ones now.

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
