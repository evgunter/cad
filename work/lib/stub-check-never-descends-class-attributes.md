---
id: stub-check-never-descends-class-attributes
kind: issue
title: Python stub checking never descends into class attributes — a forgotten .pyi enum member leaves the whole suite green
status: closed
opened: 2026-08-30
github: 1309
refs: [1301]
closed: 2026-09-03
---

## From GitHub issue 1309

Opened 2026-08-30; 0 comments.

**Raised by BLEND-5's review round** (PR #1301). The PyO3 mirror of a widened enum is guarded by an exhaustive `growth_tripwire` match (`pncad-py/src/py/select.rs:864`) — good shape. The hand-written stub is not: `test_stubs.py`'s `stub_names()` walks only `tree.body`, so class *attributes* in `pncad.pyi` are never descended into (its own docstring says so: "NAME-level equality of the top-level surface"), and `ty` has nothing to cross-check enum members against. Had BLEND-5's author forgotten to update `pncad.pyi:1624-1628` for `RimSupport::{Host, Mate}`, the entire Python suite would have stayed green while the stub advertised retired variants.

The author did update it correctly — this issue is the class, not an instance: the stub checker's blindness to attribute-level drift means every enum-member and method rename in the bindings relies on authors remembering the `.pyi`. A one-level-deeper walk (class attributes of the top-level classes) would close most of it.

S-QA-shaped (test-infrastructure honesty); flagged for that program's backlog rather than any blend unit.

## Home

`work/lib/` — `crates/pncad-py/tests/test_stubs.py` and `pncad.pyi` are inside LIB's `crates/pncad-py/*` territory glob and the bindings' census/audit gates are LIB's charter.

## Closed

`test_stubs.py` grew a second depth: `TestStubClassDrift` compares, for
every top-level class the stub declares, the names its BODY declares as
class attributes against the names the compiled class carries in its
own `__dict__` — both directions, 136 classes.

- Methods, `@property` doors, `@staticmethod`s, nested-class names and
  `Final[...]` class constants (which is how every enum member in the
  stub is spelled) are compared. `@overload` repetitions of one name
  collapse to that name.
- Bare annotations (`variant: str`) are read as INSTANCE attributes and
  excluded, because they exist on a raised exception and never on the
  class object. The `Final`-vs-bare convention this rests on is
  self-enforcing: miscategorise either way and the name shows up as
  drift in one direction or the other.
- The runtime side reads `vars(cls)`, not `dir(cls)`, so
  `BaseException`'s `args` / `add_note` / `with_traceback` never have to
  be restated in thirty stub bodies and no CPython-version exclusion
  list can go stale.
- A statement kind the walk cannot read (an `if TYPE_CHECKING:` in a
  class body, say) now FAILS
  `test_every_stub_class_body_uses_a_statement_this_walk_understands`
  rather than silently taking its contents out of the comparison.
- `test_declared_operators_exist_on_the_compiled_class` checks the
  stub's `__add__` / `__mul__` / `__len__` family in ONE direction —
  restricted to dunders `object` does not itself provide, because
  `hasattr(cls, "__eq__")` is true of every class and would report a
  pass having asked nothing.

REAL DRIFT FOUND, first run: `Datum.in_plane` — `#[pyo3(get)]` on the
pyclass since the frame work (`crates/pncad-py/src/py/value.rs:386`),
never declared in `pncad.pyi`. Added as a `@property` returning
`Optional[tuple[tuple[float, float], tuple[float, float]]]`, which is
exactly the Rust field's shape. It is the only drift in the file, and
nothing before this walk could have seen it.

PLANTED RED, both directions, against HEAD's checker as the control:
deleting `Mate: Final[RimSupport]` (the motivating case) and adding a
bogus `Flange: Final[RimSupport]` each fail the new check naming the
attribute, and each leave HEAD's `test_stubs.py` green. So does
renaming `Length.in_unit`.

WHAT IT STILL DOES NOT COVER:

- **The walk is ONE level deep.** A class declared inside a top-level
  class has its NAME compared like any other attribute; its BODY is not
  descended into, so its members are unguarded. The stub declares no
  nested classes today, and the module docstring says this so the day
  one appears the gap is stated rather than discovered.
- Signatures — arity, argument names, types, return types — remain
  `ty`'s job over `tests/ty_fixtures/`. This compares NAMES.
- Coverage (does the façade's curated surface have a Python spelling at
  all) remains `test_binding_census.py`'s.
- Instance attributes: a refusal payload the stub declares bare is not
  checked to exist. `test_every_arm_carries_every_attribute` in
  `test_picking.py` / `test_workspace.py` is what checks those.
- Dunders `object` provides — `__init__`, `__eq__`, `__hash__`,
  `__lt__`, `__repr__` — are unchecked in both directions, and every
  dunder is unchecked in the module→stub direction (an operator the
  binding grows and the stub never learns about).
