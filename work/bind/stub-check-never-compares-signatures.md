---
id: stub-check-never-compares-signatures
kind: issue
title: test_stubs.py pins names but not signatures, so the options census's stub-to-door link is unchecked
status: open
opened: 2026-09-15
refs: [1309]
priority: P4
cost: E
---

## From PORT's `python-cannot-set-options-structs` review (PR #2678)

`crates/pncad-py/src/surface_census.rs` holds each kernel options
struct to the Python door that configures it, field by field. Its
evidence chain has three links:

1. the kernel struct's fields — a destructure with no `..`, so the
   compiler owns this link;
2. the door's own struct literal — `E0063`, so the compiler owns this
   one too;
3. **the door's KEYWORDS — read out of `pncad.pyi` as text.**

Link 3 is where the census stops being mechanical, and it rests on
`pncad.pyi` being a faithful stand-in for the compiled module. That
premise is exactly what `tests/test_stubs.py` exists to hold, and it
holds it **name for name only**: `test_stub_and_module_agree_name_for_name`,
`test_class_attributes_agree_name_for_name`,
`test_declared_operators_exist_on_the_compiled_class`. None of them
compares a SIGNATURE. A stub that declared `def import_step(text, *,
eps_in=None)` over a compiled door taking only `text` passes every one
of them, and the options census would report a bound field over a
keyword no caller can pass.

Nothing is wrong today — the wheel is built and the suite exercises
these keywords — but the census's own guarantee is weaker than it
reads.

The one runtime-signature assertion in the tree is for `evaluate`
(`inspect.signature`, in the cancellation/evaluate rows), and the irony
is sharp: that is the door whose fields are almost all `NotBound`,
while the four doors this unit bound get nothing.

A `inspect.signature` comparison for every `def` the stub declares
would close it; a narrower version covering just the rostered options
doors would close the census's link.

**Not a duplicate of `stub-check-never-descends-class-attributes`**
(closed 2026-09-03): that row was about NAMES one level deeper, and
its fix — `TestStubClassDrift` — compares class attribute names. This
is the parameter list, which neither depth reads.

## Home

`work/lib/` — `crates/pncad-py/*` is LIB's territory. Filed by a PORT
lane.
