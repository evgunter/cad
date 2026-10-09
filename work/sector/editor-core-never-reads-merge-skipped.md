---
id: editor-core-never-reads-merge-skipped
kind: issue
title: No code outside topo reads BooleanNaming::merge_skipped, so a document union whose merge stage skipped a group publishes the body with no trace
status: open
opened: 2026-09-28
priority: P3
cost: E
---


Found by the area-overlap fork's designers (PR #3350). If Ev takes
that PR's second decision, planar skips become refusals and only curved
skips remain in `merge_skipped`. Those are still published silently. A
document union should say so. Check the consumers in `editor-core`
first; this was reported, not traced.

## Planar skips are refusals now (CONTACT-8, 2026-09-28)

Ev took the second decision, and CONTACT-8 built it: `group_contract`
in `crates/topo/src/merge_faces.rs` gives every planar group the
refusing regime, so a planar group the merge cannot glue fails the
boolean as `BooleanError::Merge` and never reaches `merge_skipped`.
What this row is about is now only the curved record
(`PeriodClosure`, `GroupNotClosed` of a curved run, and a declined
`DeclaredCarrierUnsupported` pair). The consumer check is unchanged:
`grep -rn merge_skipped crates --include=*.rs` finds no reader outside
`crates/topo` but `sweep`'s `curved_mergedoor` test suite.
