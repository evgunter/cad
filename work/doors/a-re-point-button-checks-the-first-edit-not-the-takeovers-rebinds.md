---
id: a-re-point-button-checks-the-first-edit-not-the-takeovers-rebinds
kind: issue
title: A re-point the check accepts can still refuse in the takeover's rebinds, which no button asks
status: open
opened: 2026-11-14
priority: P3
cost: M
---


Found by the review of PR 6948, whose three re-point buttons read one check.

**What.** The check behind every re-point button, `world::repoint_check` (a wrapper over `session::repoint_refusal`), asks the action's static checks: a source, a body to re-point onto, `Takeover::stranded_pick`. It then applies the takeover's first edit, `DefineVar` (`source = onto`), on a scratch copy. The action itself, `DocSession::repoint_placements`, goes further. Its `Creating` closure runs `Recording::take_over` (`crates/editor-core/src/take_over.rs`). After the `DefineVar`, `take_over` evaluates, then applies each round's `DocEdit::Rebind` through the door (`self.apply(edit.clone())?` in its round loop). Those rebinds can refuse: `apply_edit`'s `DocEdit::Rebind` arm (`crates/editor-core/src/edit.rs`) has `RebindIdentity`, `RebindKindMismatch`, `RebindTargetMissingNode` and `check_name_steps`. The static check never sees them, because it would need an evaluation.

So in principle, an offer row, a source row or a "copy of" row can be live while the click is refused one layer later. No witness document is known. The rebinds are composed from successors the evaluation reports, which should make a refused one rare.

**Done when** there is a witness and the gap is closed, or there is an argument (and a row) that a rebind the takeover composes cannot be refused, with the check's doc saying so.
