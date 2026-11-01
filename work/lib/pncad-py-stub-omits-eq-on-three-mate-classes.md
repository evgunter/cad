---
id: pncad-py-stub-omits-eq-on-three-mate-classes
kind: issue
title: pncad-py: the stub omits __eq__ on MateFrame, MatePrimitive and Alignment, which all define it
status: closed
opened: 2026-09-08
refs: [LIB-HASH]
closed: 2026-09-09
parent: LIB-SMALL-2
---


Measured by LIB-HASH while enumerating every class in the compiled
module that compares. Not that unit's fix: the unit's stub change was
`__hash__` where the stub already declares dunders, and adding a
missing `__eq__` to three classes is curation.

## What happens

`MateFrame`, `MatePrimitive` and `Alignment` each define `__eq__` in
the binding:

- `crates/pncad-py/src/py/mate.rs:137` (`MateFrame`)
- `crates/pncad-py/src/py/mate.rs:264` (`MatePrimitive`)
- `crates/pncad-py/src/py/mate.rs:373` (`Alignment`)

`crates/pncad-py/pncad.pyi` declares it for none of them, while it
declares `__eq__` for all twenty-four other classes that define one
(`ValidationFinding`, `LengthUnit`, `NodeId`, `Denotation`, … — the
whole rest of the list). So the stub says these three compare by
identity and the module says they compare by value, and a reader
following the stub writes the list-of-comparisons workaround for a
comparison that in fact works.

## Why the stub guard does not catch it

By construction, and correctly. `crates/pncad-py/tests/test_stubs.py`
compares stub declarations against the compiled class only for dunders
`object` does NOT itself provide — `hasattr(cls, "__eq__")` is `True`
for every class ever written, so a check over `__eq__` would report
success having asked nothing. The reasoning is written out at
`stub_class_operators` and it is right; the consequence is that
`__eq__`/`__hash__` declarations in the stub are documentation nothing
verifies.

Swept with an `ast` read of every `ClassDef` in `pncad.pyi` for a
`__eq__`/`__hash__` `FunctionDef`, cross-checked against every
`fn __eq__` in `crates/pncad-py/src/py/*.rs` attributed to its `impl`
block. **What that pattern could not match**: a class whose `__eq__`
arrives some way other than a `fn __eq__` in an `impl` block — a
`#[pyclass(eq)]` derivation, say. There are none in this crate today
outside the fieldless mirrors, and the mirrors declare no dunders in
the stub at all, which is the stub's own consistent convention for
them rather than drift.

## The fix

Three lines in `pncad.pyi`, or a guard that makes the omission fail.
The second is the larger question: a check that reads `__eq__` off the
compiled class's own `__dict__` (rather than `hasattr`) would catch
this whole family, and `module_class_names` already filters
underscore-prefixed names for a stated reason that this would have to
argue with.

## Closed

Taken as the second option: a guard reading `__eq__` off the compiled
class's own `__dict__`, with the underscore filter argued at
`module_class_names` rather than widened, and the fieldless mirrors
exempt on an interning property the row asserts per member. It found
SEVEN, not three — `Length`, `Angle` and `Count` compare through
`__richcmp__` and `FaceCensus` carries a `fn __eq__` this item's sweep
missed, which is this item's own declared blind spot. All seven are
declared, and `FaceCensus` gained the `__hash__` it implements.
Carried by LIB-SMALL-2.
