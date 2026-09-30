---
id: four-censuses-of-python-visible-vocabulary-in-one-crate
kind: issue
title: four censuses of Python-visible vocabulary live in one crate in four homes, almost but not quite parallel
status: open
opened: 2026-09-15
priority: P3
cost: D
---


Found by the style review of CENSUS-TAG-REACH (2026-09-15) and filed
by that unit's fix pass. It is not a defect in any one of the four; it
is a question about whether four is the right number.

## The four, and what each one's population and instrument are

| census | population | instrument |
| --- | --- | --- |
| `TAG_INVENTORY` (`src/tests.rs`) | every literal in `src/tags.rs` | reads that file as TEXT, with a recogniser that refuses a form it does not know |
| `NODE_KIND_ROSTER` (`src/tests.rs`) | `crate::node_kind`'s words | re-derives from that module and compares against a committed roster |
| `src/surface_census.rs` | the kernel vocabularies Python re-spells (PATHS verbs, arc modes, and the fields of every kernel options struct that reaches a Python door — `StepOptions`, `ImportOptions`, `AsciiOptions`, `BinaryOptions`, `EvalOptions`) | an exhaustive MATCH on the kernel tag, against `pncad.pyi` read as text |
| `src/prose_census.rs` | every `{x:?}` inside every `impl Display` in the workspace | a source walk over the tree |

All four ask one question in four dialects: **can a Python caller
reach every member of this vocabulary, and does the word it reaches
under still say what it said?** They are almost-but-not-quite
parallel — different populations, different instruments, different
failure modes — and nothing relates them.

## Why it is worth a row

CENSUS-TAG-REACH added a fifth population to the FIRST of them (the
evaluation door's `reason` words, via `crate::tags::eval_reason_tag`)
without asking whether the third's device was the better home.
`surface_census`'s *"the witness is a MATCH on the kernel tag, not a
list"* is the strongest of the four — it fails to COMPILE rather than
failing an assertion — and it is the one this crate reaches for least.
The tag-table reader, by contrast, is a source-text reader, which is
this program's standing trap: it needs a guard of its own, and now has
one (`the_tag_table_reader_recognises_every_form_it_claims`), which is
a second instrument bought to hold up the first.

The two rows this couples to, from the other end:

* `work/census/py-discriminant-getters-under-src-py-are-outside-every-inventory.md`
  — 23 Python-visible words that none of the four can see, which is
  the population argument for merging them.
* `work/census/evaluationerror-stub-lists-five-reasons-and-the-door-raises-six.md`
  — a stub docstring restating a vocabulary the Rust side now
  enumerates, and its shape-of-the-fix already points at
  `surface_census.rs` as somewhere such a check could live. That row
  asks about one restatement; this one asks whether the four
  instruments should be fewer.

## What this row is NOT

Not a proposal to delete any of them. Three of the four have a
correctness argument written at the site for the device they chose,
and `prose_census` in particular is answering a different question
(a rendering, not a word). What is owed is the judgement, once, in one
place: which populations belong to which instrument, and what the
fifth one should have joined.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.

## A fifth instrument, found by CENSUS-PY-GETTERS' fix pass (2026-09-15)

`crates/pncad-py/tests/test_binding_census.py` (3710 lines) asks this
row's question on the Python side: every name the Rust façade's
curated `pub use` lists introduce is either bound in Python or listed
with the family it belongs to, and a name that is neither fails there.
Its own header says it is the Python twin of
`crates/pncad/tests/all.rs`'s façade census, reads only source TEXT
(the façade `.rs` files and `pncad.pyi`) and never imports the
compiled module.

It is a fifth device over a fifth population, and it OVERLAPS the four:
it names `Subgroup::`, `ClassAdmission::` and `ClusterMaintenance::`
variants — three of the seven maps CENSUS-PY-GETTERS relocated — and
does not name `MatePrimitive`, `InterfaceCrossing`, `EntityKind` or
`Dimension`. So the same vocabulary is partly covered by two
instruments and partly by one, which is the condition this row exists
to decide.

**The number in this row's title is a design input**, so it should be
five rather than four when the call is made. `crates/pncad-py/tests/`
holds a sixth in the same family — `test_stubs.py`'s depth-2 walk,
which compares every stub class's ATTRIBUTES against the compiled
class name-for-name, and is what actually holds the crate's
`#[pyclass]` enum member vocabulary (see the note on that unit's item).
Whether a stub/module name check belongs in this family or is a
different question is itself part of the call.

## A seventh device, and the answer to whether it joins the family
(CENSUS-ERRORS-ARRIVAL, 2026-09-15)

`ERRORS_MINTING_ITEMS` and `read_minting_items` in `src/tests.rs` are
a further instrument over a further population in this same crate —
every item in `src/errors.rs` that spells a literal — landed by the
unit whose entire subject was that a row predicting the next instance
does not stop it. This paragraph exists because that unit added the
device and left this row untouched, which is that failure once more.

**It is NOT a member of this family, and the reason is the question it
asks.** The four (now six) above all ask *can a Python caller reach
every member of this vocabulary, and does the word it reaches under
still say what it said* — a question about WORDS and their reach.
`ERRORS_MINTING_ITEMS` asks *has an ITEM arrived in this file*, and it
is deliberately indifferent to whether the literals it counts are
vocabulary at all: it counts an ABI string (`"C"` in the fixture) and
a character literal (`'_'` in `is_bare_camel_token`) toward an item's
tally, and its `held_by` column exists precisely because the census
itself answers nothing about the words. Its population is the file's
LITERALS, its verdict is a roster row, and its failure mode is "a new
item nobody has looked at" rather than "a word Python cannot reach".

So it is an arrival alarm, and what it couples to is not this row but
`work/census/errors-rs-holds-four-python-visible-maps-and-nothing-enumerates-that-file.md`.
**It is still a design input here**, in one direction: if the call
made on this row is that a vocabulary should be enumerated by a
compile-time device rather than by a source reader, the arrival alarm
is the counter-case — the thing it detects (an item that did not exist
before) is not compiler-known at all, and no exhaustive `match` can be
written over it. A merged instrument would have to keep it or lose the
arrival question.
