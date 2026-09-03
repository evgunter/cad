---
id: tier-3-prime-findings-render-through-debug
kind: issue
title: tier-3' census findings render through Debug, so the fourth validator rung cannot raise through the prose gate
status: open
opened: 2026-09-03
refs: [LIB-B-VALIDATE4]
---

Measured at LIB-B-VALIDATE4, which bound `validate_pseudomanifold` —
the first Python door that can produce a tier-3′ census finding. It is
a `crates/topo` rendering, so it is filed rather than taken there.

## The measurement

Three `ValidationError` arms word themselves out of `Debug`:

- `crates/topo/src/validate.rs:1537` — `UndeclaredContact` renders its
  payload as `{contact:?}`, a struct-variant `CensusContact`.
- `crates/topo/src/validate.rs:1561` — `StaleContactDeclaration`
  renders its `StaleDeclaration` the same way.
- `crates/topo/src/census.rs:645` — `witness()` is
  `format!("{p:?}")`, so the position every census finding carries is
  a `Point3 { x: .., y: .., z: .. }` before it reaches any `Display`.

`crates/pncad-py/src/errors.rs:380` (`reads_as_prose`) rejects a
message containing the field-brace fingerprint `" { "`, and
`py::typed_err` asserts it on every raise — live under release, since
the root manifest keeps `debug_assert` on. So the first honest call of
the new door PANICKED:

```python
doc, lower, upper = two_slabs_resting()      # two solids that touch
doc.apply(DocEdit.set_roots([lower, upper]))
product(doc, evaluate(doc)).validate_pseudomanifold()
```

```
thread '<unnamed>' panicked at crates/pncad-py/src/py/mod.rs:456:5:
ValidationError was raised with a `Debug` rendering where its human
message belongs: validate_pseudomanifold reported 8 failure(s):
tier-3′ census: undeclared contact VertexOnFace { vertex:
VertexKey(9v1), face: FaceKey(1v1) } at Point3 { x: 0.25, y: 0.25,
z: 1.0 } — touching must be backed by a declared-contact record ...
```

Only tier 3′ runs the census, so the other three rungs cannot reach
these arms — which is exactly the blind spot the assertion's own
docstring discloses ("what the check cannot see is a door no test
reaches").

## Why it is not a binding defect

The rest of the arm is good prose with real recourse, and the binding
pastes the kernel's `Display` verbatim, as every other door does.
Re-wording the arms at the boundary would fork a diagnosis the kernel
owns, and the `witness` is an opaque `String` no consumer can
re-derive — a faithful re-render would have to DROP the coordinate
that makes the finding actionable.

Note the kernel has already made this fix once, one arm over:
`validate.rs:1587` carries the comment "`{cause}` (Display), NOT
`{cause:?}`: the S6 sweep fixed a Debug-format bug here that dropped
the carrier's recourse sentence from the user-facing message
entirely." Same class, same file, three arms it did not reach.

## What the unit did instead

`run_validator` raises through a new `typed_err_kernel_authored`
(`crates/pncad-py/src/py/mod.rs`) — one caller, message is
`ValidationError::to_string()` by construction, and a doc comment
carrying this argument. The current text is pinned in BOTH directions
so the fix cannot land silently:

- `crates/pncad-py/src/tests.rs::the_census_findings_are_not_prose_by_this_crate_s_own_rule`
  builds the two arms and asserts they are NOT prose (runs in the
  no-interpreter CI row).
- `crates/pncad-py/tests/test_validate.py::TestTheRefusalsShape::test_the_census_findings_still_arrive_as_debug_guts`
  asserts the guts reach Python.

Both go red when this issue is fixed, and the fix should then delete
`typed_err_kernel_authored` and let `run_validator` raise through
`typed_err` again.

## The fix, when someone takes it

`impl Display for CensusContact` and for `StaleDeclaration`, wording
the pair the way `ContactContradicted` already words its own
(`declaration.class.name()` plus `{:?}` on the FaceKeys, which is
tuple-shaped and passes), and `witness()` rendering a point as
coordinates rather than as `Debug`. Arena keys in prose are NOT part
of this: `edge {edge:?}` is how two dozen tier-1/2/3 arms already read
and those cross today without complaint — the fingerprint is the
STRUCT brace, not the key.

A second, larger question this touches and does not decide: whether a
census finding should also cross as a per-arm TAG. `CensusContact` is
`INTERIOR` in the binding census and its row says why; a `Display`
fixes the message without settling that, and the two should not be
bundled.
