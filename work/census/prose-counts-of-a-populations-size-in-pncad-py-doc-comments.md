---
id: prose-counts-of-a-populations-size-in-pncad-py-doc-comments
kind: issue
title: Doc comments in pncad-py count populations the code holds, and nothing re-measures them
status: open
opened: 2026-09-15
priority: P4
cost: E
---



Filed by CENSUS-PY-RAISE-LITERALS' fix pass (2026-09-15), from its own
style review, which asked for the class behind three stale copies of
one number to be swept for rather than just repaired.

## The class

A doc comment in `crates/pncad-py/` states how many members a
population has — *"Twenty-two arms"*, *"61 words minted by two or more
maps"*, *"Four items"* — over a population the code itself holds. The
comment is not derived from anything, nothing re-measures it, and the
number is load-bearing in the argument around it often enough that a
reader takes it for a measurement. This is the CENSUS charter's first
shape (*"the list is hand-written and drifts"*) with a prose sentence
in place of the list, in the crate whose other instruments derive
everything they assert.

**Its sharpest form is a count that is the ARGUMENT.** The disposition
for `STEP_IMPORT_WIREFRAME` — why the class cannot carry a type — was
argued in three places as *"the other twenty-one words on this
attribute are the kernel refusal's"*. The map has 23 values. The
argument survives (the property is that the map is the sole other
source of `StepImportError.variant`, and it is), but a reader checking
the reasoning checks a number that has never been right.

## What was measured, and what was repaired here

Sweep shape, stated so the next lane knows what it did not cover:
doc-comment BLOCKS (`///`, `//!`, `//`) joined per block across line
breaks — the shape a line-oriented grep misses, and the one that hid
`61`, which sits with its noun on the next line — then a number word or
digit within fifty characters of a population noun (`arms`, `words`,
`maps`, `functions`, `rows`, `sites`, `entries`, `tags`, `variants`,
`values`, `literals`, `attributes`). 195 raw hits in
`crates/pncad-py/src/**`, most of them `one`/`two`/`three` in ordinary
prose about a pair of things.

**What it cannot match**: a count with no noun beside it (*"all six of
them"*), a count in a Python test, in `pncad.pyi`, in `work/` or in
`docs/`, a count more than fifty characters from its noun, and every
crate but this one. It also cannot tell a count of a WHOLE population
from a count of a subset (*"the two MINT arms"*), which is most of the
noise and has to be read by a person.

Repaired in that unit's fix pass, each verified against the tree:

| site | said | is | done |
| --- | --- | --- | --- |
| `tags.rs`, `step_import_error_tag` | "Twenty-two arms" | 23 | count dropped; the reachability claim kept, and `TAG_INVENTORY`'s row named as the count |
| `tags.rs`, `STEP_IMPORT_WIREFRAME` | "the other twenty-one words" | 23 | replaced by the property ("the sole other source of `StepImportError.variant`") |
| `tests.rs`, `TAG_CONSTS` | same sentence | 23 | same |
| `py/value.rs`, the STEP import arm | "one literal for all twenty-one" | 23 | "for every arm" |
| `tags.rs` header | "**61** words minted by two or more maps" | 62 | names `SHARED_TAG_WORDS` instead |
| `tests.rs`, `SHARED_TAG_WORDS` | "61 words ... the other 54" | 62, 54 | the roster is the count |
| `errors.rs` header | "Four items say what that means here", over a list of four | eight items of that kind in the file | count dropped, list extended |

**The `step_import_error_tag` one was never right**: `git log -S` puts
the sentence at `d9751b13d`, and the map had 23 arms in that same
commit. A count in prose does not have to drift to be wrong.

## What is NOT measured, and is the row

The sweep's ~40 non-trivial hits were checked ONE way only: a
mechanical comparison, for `src/tags.rs`, between each map's doc
comment and the arm count of the function under it. That found the
`step_import_error_tag` row above and cleared three more
(`snapshot_error_tag`'s nineteen, `structure_refusal_tag`'s two,
`census_subject_tag`'s two); the rest of its output is subset phrases a
person has to read.

Unverified, and each a number a reader would take as measured:
`mate_payload.rs`'s *"thirty-one attributes over thirteen arms"*,
`py/mate.rs`'s *"seventeen attributes"*, `tests.rs`'s *"58 arms"* (twice),
*"seventy-one arms"* of `ValidationError`, *"Ten of the eleven arms"*,
*"Six of the seven arms"*, `py/value.rs`'s *"seven variants"*,
`py/resolve.rs`'s *"six arms"*, `tags.rs`'s *"ten `RevolveError` arms"*,
`check_payload.rs`'s *"three of the six arms are unreachable"*. Several
count KERNEL enums, so they go stale on someone else's change and this
crate's gate cannot see it.

## The shape of the work

Not "fix the numbers". The lane that takes this owes a disposition per
hit, and the three dispositions are: derive it (a count that can come
off `TAG_INVENTORY` or a `const ALL` should), drop it (most of them are
scene-setting and the sentence reads the same without), or keep it
WITH a guard (an assertion may carry a number — `tests.rs` already says
so of its own floors: *"they are ASSERTIONS, which is why they may
carry numbers where the prose above may not"*).

Territory: `crates/pncad-py/*` is LIB's fence and CENSUS's `keep_out`
announces its pncad-py rows there.
