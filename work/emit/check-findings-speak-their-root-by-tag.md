---
id: check-findings-speak-their-root-by-tag
kind: unit
title: A check finding names its root by tag; ChecksReport and CheckRefusal take no document to speak it from
status: review
opened: 2026-10-02
priority: P3
cost: M
parent: node-labels-are-document-data
pr: 3833
---


Found by `selection-door-refusals-speak-the-node`'s sweep. The rule is DESIGN.md Band 1, "Node labels".

`CheckFinding`'s subject (`checks.rs`, `impl Finding for CheckFinding`, "check {}: root {} output {}") and the separation story ("not certifiably disjoint from root {} output {other_output}") print the root by its tag. `ChecksReport`'s and `CheckRefusal`'s `Display`s render those findings, and the Python `CheckFinding.__str__`, `ChecksReport.__str__` and the `enforce_checks` message (`pncad-py/src/py/checks.rs`) forward them.

A report is the analysis-door shape (`analysis-door-refusals-speak-the-node`): a value whose human form takes the document it was taken of. `pncad-py`'s product memo serves a `ChecksReport` for a (document, evaluation) pair, so the report must keep bare ids, and its rendering needs the document it was taken of (`spoken::assert_taken_of`), which `ChecksReport` does not record today. `CheckFinding.__repr__` already prints the full id (a machine channel).

`ChecksError::Root` speaks through `ChecksError::spoken`, and the Python `run_checks` door uses it.

**The viewer's Checks window speaks its root from the wrong document** (found by `viewer-panes-speak-the-kernel-refusals-they-draw`'s review, PR 3827). `ViewerApp::checks_window` (`crates/viewer/src/app.rs`) labels each finding's select-the-root button `self.session.doc().spoken(finding.root)`, which is the shown (committed) document. But the report belongs to the landed pair: `DocSession::checks` reads `LandedRun::checks`, which was run over the landed document and evaluation. While a run is outstanding, the button says the committed document's label for an id spelled in the landed one. It should speak from `DocSession::landed_pair`'s document, as `pane::properties::standing_verdict` does. The finding's own sentence beside the button (`finding.to_string()`) is this row's tag case above, and it speaks from the same landed document once `CheckFinding` takes one.
