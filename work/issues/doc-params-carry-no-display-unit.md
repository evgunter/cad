---
id: doc-params-carry-no-display-unit
kind: issue
title: Document parameters carry no display unit — the units layer stops one door short of the fields it most serves
status: open
opened: 2026-09-01
github: 1459
refs: [1458]
---

## From GitHub issue 1459

Opened 2026-09-01; 0 comments.

Found by the `story_parametric` integration lane, and partly self-disclosed in `crates/viewer/src/props.rs`'s module docs (the slot/param asymmetry is named there — this issue is the schedule that disclosure was owing).

Slots got the full GQ5 units treatment post-close: a literal remembers its written unit, panels render and author in it, `SetSlotUnit` changes notation without touching the value. Document *parameters* — the fields the parametric workflow exists to route every dimension through — have none of it: a user declaring `base_r` cannot author or read it as `50 mm`; the panel speaks canonical metres exactly where the units layer matters most. Downstream cost observed in the same walk: `ProbeBounds` on a Length param seeds from 1 written unit = 1 canonical metre, so a millimetre-scale part spends ~10 refinement halvings just getting from the seed floor down to its own scale (the driven-slot half of that is issue 1458).

The shape is the same one slots already have: a stored display unit beside `DocParam`, a `SetParamUnit` door mirroring `SetSlotUnit`, and the panel's param rows reading/writing through `in_written`/`written_unit` like the slot rows do. Persistence-wise it is one more field under GQ3's versioning discipline.

(story-suites orchestrator)

## Home

`work/issues/` — the GQ5 units layer and the viewer property panel are GUI-era ground and GUI is closed; no open program's territory covers `crates/viewer/src/props.rs`.
