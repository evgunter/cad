---
id: python-check-and-assembly-doors-gather-twice
kind: issue
title: pncad-py's run_checks and assemble each gather the product, so a Python caller asking both pays twice
status: open
opened: 2026-09-04
---



## What

DOCM-5 (PR 1871) gave the kernel two doors that take a product the
caller already holds — `run_checks_on(doc, ev, Subject, cfg, tol)` and
`assemble_gathered(product, tol)` — so `DocSession::land` now gathers
ONCE for its three consumers where it used to gather two or three
times. The Python façade did not get them, deliberately: they are
listed `behind-a-door` in `crates/pncad-py/tests/test_binding_census.py`
on the reasoning that Python binds the wrappers and so there is no
question a Python caller cannot ASK.

That reasoning is correct and it is not the whole answer. A Python
caller that asks both questions of one document —

```python
report = pncad.run_checks(doc, ev)
assembly = pncad.assemble(doc, ev)
```

— gathers twice, because `crates/pncad-py/src/py/checks.rs:545` calls
`run_checks` and `crates/pncad-py/src/py/assembly.rs`'s `assemble`
calls `assemble`, and each of those wrappers gathers for itself. It is
exactly the shape DOCM-5 removed from the viewer's landing, still
standing at the Python seam, and nothing books the cost.

The size of it: at the heat sink's 160-fin point (161 solids / 991
faces) the gather is ~250 ms against ~8 ms for the whole check
registry over a subject already in hand (measured by DOCM-5; the
figures of record are the `registry split` row of
`crates/editor-core/tests/m4_pr8_latency.rs`, re-taken nightly). So a
Python caller asking both pays about 250 ms it need not.

## Why it is not a one-line binding

`assemble_gathered` CONSUMES the product, which is what makes one
gather enough on the Rust side. An ownership order is the thing that
does not cross the PyO3 boundary: a Python `Product` object would be
reachable after the assembly took it, so the door would need either a
consuming-by-move discipline Python cannot express or an interior
`Option` that refuses a second use. That is a design question about
what a Python `Product` IS, not a plumbing change — which is why
DOCM-5 disposed of the names rather than binding them.

## Shape of a fix, if it is taken

Three candidates, in increasing order of surface:

1. **A combined door.** One Python call that gathers once and returns
   both answers (`checks_and_assembly(doc, ev)`), so nothing has to
   name a `Product` at all. Smallest surface, least general.
2. **A `Product` object with a consumed flag.** Bind `Product` and the
   two doors; `assemble_gathered` marks it consumed and a second use
   refuses typed. Most faithful to the Rust shape, and the refusal is
   a new error arm the exhaustive mirror will force.
3. **Nothing, with the cost written down.** The census entry says
   `behind-a-door`; this file is the written reason, and a decision to
   leave it is a legitimate outcome as long as it is recorded rather
   than assumed.

## Citations

- `crates/pncad-py/src/py/checks.rs:545` — the Python `run_checks`.
- `crates/pncad-py/src/py/assembly.rs` — the Python `assemble`.
- `crates/pncad-py/tests/test_binding_census.py` — the six
  `behind-a-door` entries and their written reason.
- `crates/editor-core/src/checks.rs`, `assembly.rs` — the doors that
  take a gathered subject.

Filed by DOCM-5's fix pass on the dual review's finding; DOCM's fence
does not reach `pncad-py`'s binding policy, which is LIB's.

## Question for Ev (2026-09-06, LIB orchestrator; `[ev]` PR)

What a Python `Product` is, which decides the shape of the fix. The
three candidates in the body plus a fourth the orchestrator adds, with
the recommendation first:

- **(4) Bind `Product` as a plain value; the Python doors CLONE it.**
  `pncad.gather(doc, ev) -> Product`, and `run_checks(doc, ev,
  product=p)` / `assemble(doc, ev, product=p)` accept it. The kernel's
  `assemble_gathered` consumes; the binding hands it a clone of the
  product it holds, so Python never needs move semantics and a
  `Product` can be used any number of times. Sound only if a clone is
  cheap next to the gather it replaces (the gather is ~250 ms at the
  heat sink's 160-fin point; the clone is a body copy) — the unit
  measures that first and falls back to (1) if it is not.
- **(1) One combined door** `checks_and_assembly(doc, ev)` that
  gathers once and returns both answers. Smallest surface, least
  general: a third consumer of the product needs a third door.
- **(2) `Product` with a consumed flag.** Most faithful to the Rust
  shape; a second use refuses typed. Simulates a linear type in a
  language that has none, and the refusal is a new error arm for a
  mistake Python callers will make often.
- **(3) Nothing**, the cost written down in the census entry.

Recommendation: (4), measured; (1) as the fallback. Either way the
census's six `behind-a-door` entries for the gathered doors get a
written reason that matches what ships.

## Ruled (Ev, PR 2020, 2026-09-06): **(4) — a plain value the doors clone, measured; (1) if the clone is not cheap**

Ev: "plain value doors clone sounds good". The tradeoffs (cost, surface,
generality, staleness, faithfulness) are on the PR thread. The unit's
first job is the measurement: the cost of cloning the product at the
heat sink's 160-fin point against the ~250 ms gather it replaces; if
the clone is not small the unit ships the combined door (1) instead and
says so. Either way: `pncad.gather(doc, ev) -> Product` (or the
combined door), `run_checks` / `assemble` accepting it, a typed refusal
for a product that is not OF the evaluation given (the
`EvaluationOfAnotherDocument` shape), a test pinning the gather count
through `product::gathers_on_this_thread`, `pncad.pyi`, census re-cut of
the six `behind-a-door` entries, stub test. Dispatchable as a LIB unit.
