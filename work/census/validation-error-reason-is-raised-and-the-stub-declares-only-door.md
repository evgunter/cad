---
id: validation-error-reason-is-raised-and-the-stub-declares-only-door
kind: issue
title: ValidationError raises `reason` and pncad.pyi declares only `door`, and neither attribute is present on every raise of the class
status: open
opened: 2026-09-15
priority: P3
cost: E
---



Found by CENSUS-ARRIVAL-RESIDUE (2026-09-15) while giving
`EvalReason::ATTRIBUTE`'s word the Rust check its `held_by` column said
it did not have. The check reads `pncad.pyi` and compares each
discriminant-carrying class's attribute set against what the stub
declares; `EvaluationError` agrees and `ValidationError` does not.

## What is true of the tree

`ValidationRefusal::ATTRIBUTES` is `["door", "reason"]`
(`crates/pncad-py/src/errors.rs`, `:691`), and
`ValidationRefusal::attribute` routes the four validator refusals to
`door` and `MassProperties` to `reason` (`:710`).
`crate::py::raise_typed` sets **one** of the two on the exception —
`class_discriminant`'s `attribute` for the value in hand
(`crates/pncad-py/src/py/mod.rs`, `:788`) — so:

* a `Body.validate*` refusal carries `door`, `failure_count` and
  `findings`, and no `reason`;
* the measurement refusal (`crates/pncad-py/src/py/value.rs`, `measurement_err`,
  `:228`) carries `reason` and none of the other three.

`pncad.pyi`'s `ValidationError` declares `door: str`,
`failure_count: int` and `findings: list[ValidationFinding]`, all three
unconditionally, and does not declare `reason` at all
(`crates/pncad-py/pncad.pyi`, `:292`). Its docstring describes only the
validator half.

## Why this is a row and not an edit

Two things are wrong and only one of them is a typo.

1. **`reason` is undeclared.** A Python caller reading the stub cannot
   discover an attribute a raise sets. Adding `reason: str` is one line
   and is safe against `tests/test_stubs.py`, which reads a bare
   annotation as an INSTANCE attribute and compares only `Final[…]`
   ones against the compiled class.
2. **The declarations are unconditional and the raises are not.**
   `EvaluationError` next door declares `kind: Optional[str]` and says
   *"attributes never go missing"*; `ValidationError` promises four
   attributes of which any raise carries three or one. Whether the
   repair is `Optional[…]` on all four, always setting both words, or a
   docstring that says which raise carries which, is a decision about
   this class's public shape — which is why adding the one line without
   taking it would swap a missing promise for a false one.

Held in the meantime by
`tests::the_discriminant_attribute_names_are_declared_in_the_stub`,
whose third column lists `reason` as a gap and reds if the stub gains
the declaration — so closing this row is loud rather than silent.

Neighbour, not a duplicate:
`evaluationerror-stub-lists-five-reasons-and-the-door-raises-six` is
the same file and the same shape one class over, about the VALUES a
declared attribute takes rather than the attribute's declaration.

Territory: `crates/pncad-py/*` is LIB's fence and this program's
`keep_out` announces its pncad-py rows there.
