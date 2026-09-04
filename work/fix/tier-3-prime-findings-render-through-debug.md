---
id: tier-3-prime-findings-render-through-debug
kind: issue
title: tier-3' census findings render through Debug, so the fourth validator rung cannot raise through the prose gate
status: closed
opened: 2026-09-03
refs: [LIB-B-VALIDATE4]
branch: fix/tier3-census-display
pr: 1779
closed: 2026-09-04
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

## Closed

`CensusContact` and `StaleDeclaration` each carry a `Display`, worded
the way `RingContact`'s own already is one screen below them in the same
file — prose around arena keys, which stay `{:?}` because a key is
tuple-shaped and IS the name a caller resolves. The two
`ValidationError` arms render `{contact}` / `{declaration}`, and
`census::witness` renders a coordinate triple. `run_validator` raises
through `typed_err` again and `typed_err_kernel_authored` is deleted —
it had one caller and no other reason to exist.

What a caller reads, measured through the door:

    tier-3′ census: undeclared contact vertex VertexKey(9v1) on face
    FaceKey(1v1)'s interior at (0.25, 0.25, 1.0) — touching must be
    backed by a declared-contact record, never blessed from discovery;
    declare the named contact class, or move the geometry

**Four pins turned, not deleted.** The two the filing unit armed —
`pncad-py/src/tests.rs` and `test_validate.py::TestTheRefusalsShape` —
now assert the findings DO read as prose, and each still checks that the
prose kept the entities and the witness.

That was first written as "so a rewording that reads well and says
nothing fails them", and planted regressions falsified it. Reverting an
arm to `{contact:?}` reddens the `pncad-py` pin, and restoring
`witness`'s `Debug` reddens both topo pins — but rewording
`VertexOnFace` to drop BOTH arena keys left every row in the tree
green, and the keys are the entire justification for rendering them
(`assert!(contains("vertex"))` matches the bare noun; `mate4a`'s golden
pins the error's DERIVED `Debug`, which never calls these impls).
`validate.rs`'s `review_census_display_keys` closes it: exhaustive over
7 of 8 `CensusContact` arms and all 4 `StaleDeclaration` arms, by key
MULTIPLICITY rather than containment so a same-type `a`/`b` pair cannot
hide a dropped half. (`ConformalPatch` is skipped — it needs a
`ContactFinding` to build.)

Two more pins were not named in the brief.
`crates/topo/tests/mate4a_ef_bound_rung.rs` pins the straddle seat's
whole census through `Debug`, witness strings included, so the
coordinate change moved six of them; the grep for the OLD spelling
found it. `crates/topo/tests/review_mate9_r1_probes.rs` matched the
same witnesses by FIELD FRAGMENT — `w.contains("x: 0.45,")` and
`w.contains("z: 0.5 ")` — so it never contained the string `Point3 {`
and no grep for either spelling could see it. **Hosted CI caught it,
not the sweep**, and the local runs missed it because they were scoped
to the rows the change was predicted to touch instead of the suite.
The predicates now match inside the coordinate triple.

**The sweep.** Resolver over every `impl fmt::Display` in `crates/`,
matching `{ident:?}` and resolving `ident` to its declared field type,
then asking whether that type is brace-shaped (named-field struct, or an
enum with a struct variant) — the shape `reads_as_prose` rejects. 370
sites, 32 brace-shaped after the false positives were read out. The
disposition is in the PR body; three families are outside this fence and
one of them is already filed at
`work/docm/debug-in-prose-residue-after-finding-sink.md`, whose own
kernel-side note is the arm this unit just discharged.

**Not settled here**, and now its own file so it survives this one:
whether a census finding should also cross as a per-arm TAG —
`work/issues/census-findings-cross-without-a-per-arm-tag.md`.
`CensusContact` stays `INTERIOR` in the binding census; a `Display`
fixes the message without touching that, and the pin that asserts no tag
crosses is untouched.
