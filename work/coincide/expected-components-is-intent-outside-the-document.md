---
id: expected-components-is-intent-outside-the-document
kind: issue
title: ChecksConfig::expected_components is intent stated outside the document
status: open
opened: 2026-10-08
---


`ChecksConfig::expected_components`
(`crates/editor-core/src/checks.rs:283`) is per-run caller configuration
keyed by `(root, output)`, read by the component-count check and its
`CheckEvidence::StaleExpectation` (`checks.rs:291`, `checks.rs:384`). It
is not a document record. DISCIPLINES-DESIGN DS6
(`docs/DISCIPLINES-DESIGN.md:380`) says an acknowledgment is "carried with
provenance like any declaration", which the shipped expectation never was.

Under D10 an expected component count is intent, and intent lives in the
document: the natural home is an assertion on a component-count measure,
which the stage-5 work (FORK-S5C, assertions as the only acknowledgment
vocabulary) is shaping. Found by FORK-S5C's designer A
(`design/intent-s5-s5c-A.md` on branch `design/intent-s5-s5c-A`,
"Off-question finding"); left for the fork's answer to place.
