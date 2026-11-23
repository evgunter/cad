---
id: validationerror-stub-declares-one-of-the-class-two-shapes
kind: issue
title: pncad.pyi's ValidationError declares three attributes; the class ships seven, and nothing checks that direction
status: open
opened: 2026-09-15
priority: P4
cost: E
---



Found by CENSUS-PY-RAISE-LITERALS (2026-09-15) while giving that
class's discriminant a type. Not fixed there: that unit's spec forbids
changing `pncad.pyi`'s surface.

## The two shapes, and the one the stub knows

`ValidationError` is raised on two paths and carries a different
attribute set on each:

* the four validator doors (`Body.validate` and its siblings) attach
  `door`, `failure_count`, `findings`;
* the two measurement doors (`Body.mass_properties`,
  `Body.validate_geometric_measured`'s continuation) attach `reason`,
  and on a budget refusal `volume_lo`, `volume_hi`, `surface_area`.

**The split is deliberate and asserted**, not a defect:
`tests/test_validate.py`'s
`test_a_body_tier_3_admits_can_still_have_no_number` asserts
`reason == "mass_properties_failed"` AND
`assertFalse(hasattr(gated.exception, "door"))`, so the absence is the
contract on that path.

`crates/pncad-py/pncad.pyi`'s `class ValidationError` declares
`door: str`, `failure_count: int`, `findings: list[ValidationFinding]`
and nothing else, and its docstring describes only the validator
shape. The runtime docstring on the same class — the one
`create_exception!` in `crates/pncad-py/src/py/mod.rs` carries —
describes BOTH shapes and names all seven attributes. So the two
docstrings for one class disagree, and a `ty`-checked caller reading
`err.reason` off a `ValidationError` is reading an attribute the stub
says does not exist.

## Why nothing reds

`work/lib/stub-check-never-descends-class-attributes` closed with the
gap stated: *"Instance attributes: a refusal payload the stub declares
bare is not checked to exist."* This is that sentence's CONVERSE — a
payload the raise attaches and the stub never declares — and the row
names no instance of either. `TestStubClassDrift` compares class
attributes and reads bare annotations as instance attributes, so it
excludes exactly this. `test_every_arm_carries_every_attribute` runs
per class in `test_picking.py` / `test_workspace.py` and there is no
such row for this class.

## The population is unmeasured

One instance, found while working on that class rather than by a
sweep. Whether other exception classes under-declare is the open half:
`SelectRefusal` was checked on the way past and its stub declares all
eight — which is how the same unit found that `py/flush.rs`'s raise
attached one of them, repaired in that change.

## Shape of the fix, if it is taken

Declaring the four missing attributes is the easy half and would leave
the general case where it is. What the class wants is the thing that
would have caught it: a check in the direction the closed row named,
holding each exception class's bare annotations against the attributes
its raise sites attach. `test_every_arm_carries_every_attribute`'s
shape generalises, and the two-shape case is what makes it interesting
— an attribute that is present on one path and absent by contract on
another is a real answer, not a failure.
