---
id: section-unaccounted-carries-its-cause-as-text
kind: issue
title: SectionError::Unaccounted carries its SsiError cause as rendered text, so its standing aborts as mixed and a consumer matches it by its words
status: open
opened: 2026-11-16
priority: P3
cost: M
---

Re-homed from the closing note on `the-section-tables-refusal-states-no-standing` (2026-11-12, CLEAVETAIL's lily-wall8 lane), when `onecopy/section-error-standing` closed that row.

## What

`SectionError::Unaccounted { cause: String }` (`crates/geom-brep/src/intersect.rs:762`) is what `whole_reading_refused` (`crates/geom-brep/src/pair_section.rs:1299`) makes of every `SsiError` that is neither a tangency nor an escalation (`:1307`). It renders that error to text.

- **Its standing aborts as mixed** (`SectionError::refusal`). An accounting that did not close (a fault) and a branch that did not certify sit beside a budget spent (`SsiError::CellBudget`, a bound on the work), and only the text tells them apart.
- **A consumer matches it by its words.** Lily wall 8 (`demos/tour/src/lily.rs`, `wall_probes`) pins `GermSectionUnbuilt { why: Refused(Unaccounted { cause }) }` and can tell the budget from the other causes only by `cause.contains("cell budget")`.

## What closing it takes

Carry the cause typed (the `SsiError`, or a cause enum read off it), and state its standing per cause. A budget is a bound on the work. A failed accounting or an uncertified branch is the reading's own refusal, by its class. Then the lily pin matches the variant, and `SectionError::refusal`'s mixed arm loses `Unaccounted`.
