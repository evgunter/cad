---
id: LIB-B-CANCEL
kind: unit
title: binding census family B-CANCEL
status: review
opened: 2026-09-03
branch: lib/b-cancel
---

Queued mechanical census family (the B-READBACK/B-CHECKS shape): sweep the
family's bindings against the census contract, construct the previously
unconstructible pins where the surface now allows, and re-cut the census
rows honestly. Families share the census/tags/test files, so at most two
run concurrently, staggered.

## Delivered

The family chartered ONE name, `CancelToken`, and moving it needed
three doors, because a charter that names a door's argument does not
name what the door answers with:

* `CancelToken` — a frozen pyclass over the kernel's
  `Arc<AtomicBool>`; `cancel()`, a read-only `canceled`, no reset.
  Top-level in `pncad.pyi` at the same spelling, so it left the `gap`
  roster with no `BOUND_AS` entry, and `FAMILIES["B-CANCEL"]` went
  with it;
* `evaluate(doc, *, resolver=, prior=, cancel=)` — the token
  threaded through, and the kernel run moved inside `py.detach`,
  because with the GIL held no other Python thread can set the flag
  and the door would have been decoration;
* `Evaluation.canceled` — the answer. `EvalOutcome` stays
  `different-shape` and is RE-CUT in place: the disposition was
  vacuously true while Python's outcome was a constant.

`Evaluation.value` now splits `unknown_node` from
`node_not_evaluated`, the second arm being what a canceled run's
prefix makes reachable; the ladder word is `tags::NODE_NOT_EVALUATED`
and is pinned against the read-back and pick doors in `src/tests.rs`.

`tests/test_cancellation.py` is the positive form — the deterministic
pre-canceled arm as the contract pin, the concurrent arm on
race-independent invariants plus one categorical GIL pin.

Banked:
`work/lib/the-python-feature-half-of-pncad-py-is-linted-by-no-ci-row.md`.
