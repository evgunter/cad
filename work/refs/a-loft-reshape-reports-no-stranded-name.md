---
id: a-loft-reshape-reports-no-stranded-name
kind: issue
title: A loft reshape that drops or re-pairs a section reports no stranded name, and no door checks a name's section ids against the mint
status: open
opened: 2026-11-02
priority: P2
cost: M
---


Found by `reads/loft-section-ids` (every loft section now carries a
minted `SectionId`, and a loft's walls, seams and rings spell the ids of
the sections they span; `crates/editor-core/src/names/README.md`, N1,
"Loft walls").

1. **No strand report.** A `SetProgram` that drops a step reports every
   name on it (`Took::Step`, `edit.rs` `stranded_steps`, DM7). A
   `SetLoftSections` that drops a section, or inserts one between two
   kept sections, ends every wall, seam and ring name spelling that
   section or that pair, and reports nothing but the `Took::Reach`
   strands of `write_reads` (`crates/editor-core/src/edit.rs`). The
   names keep their spelling and resolve `Vanished`, so nothing is
   silently re-bound, but DM7's report is missing. The same holds today
   for a degree change across 1 (`SetStructuralParam` on `VDegree`),
   which the README says vanishes the other degree's walls. Done when a
   reshape reports, as `Took::Section` or similar, every carried name
   spelling a section pairing the new loft does not skin.
2. **No mint check of a name's section ids.** `check_name_steps`
   (`edit.rs`) and the load door's `NameStepNotMinted`
   (`crates/editor-core/src/persist/check.rs`, `validate_document`) hold
   every STEP id a carried name spells to the mint log; nothing holds its
   SECTION ids there (`Spelled` in `crates/editor-core/src/names/role.rs`
   collects steps, not sections). A file whose name spells an unminted
   section id loads. Done when both doors refuse it, as they refuse an
   unminted step.
