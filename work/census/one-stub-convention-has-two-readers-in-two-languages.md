---
id: one-stub-convention-has-two-readers-in-two-languages
kind: issue
title: pncad.pyi's Final convention has a Python reader and a Rust reader, held equal by nothing, and they had drifted
status: open
opened: 2026-09-15
priority: P3
cost: D
---


Found by the style review of CENSUS-ARRIVAL-RESIDUE (2026-09-15). The
divergence it names is closed; the pair that produced it is not.

## The two readers

`crates/pncad-py/pncad.pyi` holds one annotation convention
throughout: a `Final` annotation is a CLASS-level constant and a bare
`name: type` is an INSTANCE attribute, which is how every refusal
payload on every `PncadError` subclass is declared. Two readers depend
on it:

* `crates/pncad-py/tests/test_stubs.py`'s `stub_class_names` parses
  the stub with Python's `ast` and argues the convention at length in
  its docstring.
* `crates/pncad-py/src/tests.rs`'s `stub_instance_attributes` reads
  the same file by line prefixes, four-space indentation and
  triple-quote parity.

The Rust doc says so out loud — *"`tests/test_stubs.py` states this
stub's convention and depends on it … This reads the second."*

## They had already drifted

Python admits `annotation == "Final" or annotation.startswith("Final[")`.
The Rust reader tested only `starts_with("Final[")`, so `x: Final`
read as a class attribute in Python and an instance attribute in Rust.
Latent rather than live: `rg ': Final$' crates/pncad-py/pncad.pyi` was
empty when this was written (2026-09-15).

**Closed at the instance** by CENSUS-ARRIVAL-RESIDUE: the Rust reader
admits both spellings and `the_stub_attribute_reader_recognises_what_it_claims`
holds a `Bare: Final` row. Nothing holds the two readers equal, so the
next divergence arrives the same way.

## Why the Rust one was kept

Its other half is `crate::errors::EvalReason::ATTRIBUTES` — a Rust
const read AS a const, which is the property that makes
`the_discriminant_attribute_names_are_declared_in_the_stub` a check
rather than a second spelling of the words. Moving it into
`test_stubs.py`, where the `ast` parser already is, would delete the
second stub reader and re-spell those words in Python: the defect this
program exists to find, traded for the one it found.

## What would close it

Either a single reader both languages consult (the stub's convention
expressed once, as data the Rust side can read), or a check that the
two readers answer the same over the real `pncad.pyi` — which needs a
channel between a `cargo test` and a `pytest` that does not exist
today. Both are bigger than the divergence, which is why this is a row
and not a fix.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.
