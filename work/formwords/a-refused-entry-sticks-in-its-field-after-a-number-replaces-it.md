---
id: a-refused-entry-sticks-in-its-field-after-a-number-replaces-it
kind: issue
title: viewer: once a slot field refuses an entry, a corrected entry or a plain number keeps showing it wrong, and its unit reads 'computed'
status: review
opened: 2026-11-22
priority: P1
cost: M
pr: 7378
branch: formwords/refused-entry
---

**Ev reported this** (in chat, 2026-11-22). If variables are typed into a field correctly the first time, it works. If one is typed wrong, then switching it to the right thing, or back to a plain number, keeps saying something is wrong, and the unit shows as "computed" rather than "m". Ev was not sure this happens everywhere.

The cause has two parts, each from reading the code; neither has been reproduced yet.

1. **A refused draft is released only by an expression edit.** A parse refusal holds the typed text in `Drafts::expr_target` / `expr_text` (`crates/viewer/src/drafts.rs:51-62`), set at `crates/viewer/src/app.rs:1599-1602` via `frame::retype_draft` (`crates/viewer/src/frame.rs:3333-3342`). It is released only when a `SetSlotExpression` lands unrefused (`app.rs:1603-1610`), or by the reshape/undo rule (`drafts.rs:1159`, `:1352`). A plain number routes through `props::field_edit` (`crates/viewer/src/props.rs:1209-1217`) to `SessionOp::SetSlot`, which never releases it, so the field keeps showing the refused text: `slot_value_ui` passes it as the draft (`crates/viewer/src/pane/properties.rs:830-831`) and `slot_showing` prefers it to the document (`properties.rs:1543-1544`). The profile pane copies the pattern (`crates/viewer/src/pane/profile.rs:83-115`).
2. **A committed expression refuses a plain number.** If the wrong entry parsed and committed (an expression that evaluates badly, say), the slot is expression-driven. The unit picker then truthfully reads "computed" (`properties.rs:1034-1035`, via `slot_unit_refusal`, `crates/viewer/src/session.rs:2771-2775`, and `props::slot_literal`'s `NotALiteral`, `props.rs:1971-1991`). A plain number typed next goes through `SetSlot` and hits the driven-slot guard (`driven_refusal`, `session.rs:270-287`, `Refusal::DrivenByExpression`). Nothing in the field replaces an expression with a literal.

The fix is one field-edit path: every edit the field makes (number or expression) releases the field's draft, and a number typed into an expression-driven slot replaces the expression rather than being refused. Then "computed" clears once a literal lands. First, write a test reproducing Ev's sequence (good entry; bad entry; corrected entry; plain number) on a slot field and on a profile field.
