---
id: LIB-PRODUCT
kind: unit
title: the gathered product memoized on the Python Evaluation
status: review
branch: lib/product
refs: [python-check-and-assembly-doors-gather-twice]
opened: 2026-09-08
closed:
pr: 2181
---

Ev's ruling (5) on `python-check-and-assembly-doors-gather-twice`: a
Python `Evaluation` is the immutable (document, evaluation) pair
captured at `evaluate`, and the document's product is a pure function
of that pair and the tolerance. So the four Python doors that gather —
`run_checks`, `assemble`, `product`, `product_named` — keep their
signatures, the first of them gathers and stores the product on the
evaluation, and the rest reuse it.

The unit measures the clone first: a `Product` clone against the
gather it replaces, at the heat sink's 160-fin point. A cheap clone
means the memo keeps the product and hands `assemble_gathered` a copy;
an expensive one means the memo is handed over and a later caller
re-gathers.
