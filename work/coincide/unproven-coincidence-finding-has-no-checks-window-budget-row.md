---
id: unproven-coincidence-finding-has-no-checks-window-budget-row
kind: issue
title: The checks-window census renders every CheckEvidence arm but UnprovenCoincidence, so its finding is held to no word budget
status: open
opened: 2026-10-20
priority: P4
cost: E
---


Found by `wire-census2`'s sweep for the class of
`kernel-payload-lists-in-the-concision-census-are-kept-by-hand`.

`check_findings`
(`crates/editor-core/tests/refusal_concision_chains.rs:6045`) renders each
`CheckEvidence` arm (`crates/editor-core/src/checks.rs:413`) as the checks
window draws it and holds it to the 75-word budget
(`every_check_finding_renders_within_the_budget`). It is kept by hand,
and a scripted comparison of its constructors against the enum finds
one of ten arms with no row: `UnprovenCoincidence` (`crates/editor-core/src/checks.rs:521`), whose payload (a
`coincide::NamedCoincidence` and its `Residual`) is not built by hand
anywhere. `crates/editor-core/tests/coincidence_door.rs:176` renders it from a real document but
checks only that it carries "is not proven structural" and a
`Recourse:`, not its length or shape.

## Repair shape

Give `check_findings` an `f6_variants!` roster for `CheckEvidence` and
a set difference over its rows (as `every_forwarded_arm_has_a_row`
does), and a row for `UnprovenCoincidence` built from a fixture (the
glue of two extrudes in `coincidence_door.rs` raises five) or a
payload a test constructor exposes.
