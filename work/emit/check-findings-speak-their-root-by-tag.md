---
id: check-findings-speak-their-root-by-tag
kind: unit
title: A check finding names its root by tag; ChecksReport and CheckRefusal take no document to speak it from
status: open
opened: 2026-10-02
priority: P3
cost: M
parent: node-labels-are-document-data
---


Found by `selection-door-refusals-speak-the-node`'s sweep. The rule is DESIGN.md Band 1, "Node labels".

`CheckFinding`'s subject (`checks.rs`, `impl Finding for CheckFinding`, "check {}: root {} output {}") and the separation story ("not certifiably disjoint from root {} output {other_output}") print the root by its tag. `ChecksReport`'s and `CheckRefusal`'s `Display`s render those findings, and the Python `CheckFinding.__str__`, `ChecksReport.__str__` and the `enforce_checks` message (`pncad-py/src/py/checks.rs`) forward them.

A report is the analysis-door shape (`analysis-door-refusals-speak-the-node`): a value whose human form takes the document it was taken of. `pncad-py`'s product memo serves a `ChecksReport` for a (document, evaluation) pair, so the report must keep bare ids, and its rendering needs the document it was taken of (`spoken::assert_taken_of`), which `ChecksReport` does not record today. `CheckFinding.__repr__` already prints the full id (a machine channel).

`ChecksError::Root` speaks through `ChecksError::spoken`, and the Python `run_checks` door uses it.
