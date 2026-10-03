---
id: doc-param-edit-doors-drop-the-python-dimension
kind: issue
title: The document-parameter edit doors drop the dimension the Python payload carries
status: open
opened: 2026-09-16
priority: P3
cost: D
---

## Finding

**The `DocParam` constructors refuse a dimension mismatch between a
parameter and its annotation; the carry-forward EDIT doors accept the
same mismatch silently, because they discard the dimension the Python
value was written with.**

`continuous` (`crates/pncad-py/src/py/doc.rs`) is the shared body of the
three continuous `DocParam` constructors, and it exists to make one
check: a Python `Distribution` carries a `dim`, the declaration carries
another, and a disagreement raises through
`super::analysis::dimension_mismatch` naming the door. Its own rustdoc
says why — a kernel `Distribution` is dimension-free, so there is no
kernel refusal to route, and the binding is the only seam where the two
dimensions meet.

The edit doors on the same file do not have that seam and do not say so
in a refusal:

- `DocEdit::set_doc_param_value` takes a `DocParamValue`, whose
  `length`/`angle`/`scalar` constructors each throw the dimension away
  and keep the bare number. `DocParamValue.angle(90 * deg)` submitted
  against a Length parameter carries `Continuous(1.5707…)` and applies:
  the kernel's `DocParam::with_value` only checks the Count/continuous
  KIND, which matches.
- `DocEdit::set_doc_param_distribution` (added by EDIT's
  `doc-param-distribution-edit-has-no-door`, this same shape at the
  third field) takes a Python `Distribution` whose `dim` it maps away
  in `distribution.map(|d| d.inner)`.

So the two spellings of "annotate this Length parameter with an angle's
uncertainty" answer differently: `DocParam.length(v, dist)` raises, and
`DocEdit.set_doc_param_distribution(name, dist)` applies. The new door's
rustdoc states the position it inherited rather than leaving it
unwritten, which is why this is a row and not a bug report.

## What is needed

A reading, then whichever door it implies. The two candidates:

1. **The edit is a payload and cannot check** — an edit is built
   without the document it will be applied to, so the dimension it
   would check against is not in hand. Then the constructors' check is
   the only one there can be, and the gap is documented rather than
   closed. This is the status quo and what the doors say today.
2. **`Doc.apply` is where both dimensions ARE in hand** — the binding
   could carry the payload's dimension on the Python `DocEdit` and
   refuse at apply, in the binding's own vocabulary. That is a new
   refusal tag and a new field, so it is a design question rather than
   a fix.

Whichever way it goes, `set_doc_param_value`'s half moves with it: the
two doors should not answer this question differently.

## Home

`work/lib/` — `crates/pncad-py/src/py/doc.rs` is LIB's (`work.py
territory`).

## Found by

The `pncad-py` follow-through of EDIT's
`doc-param-distribution-edit-has-no-door` (the annotation door), which
had to decide what its new door does with the `Distribution.dim` it is
handed.
