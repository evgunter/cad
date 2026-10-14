---
id: typing-a-value-mints-or-offers-a-variable
kind: unit
title: The GUI: a typed value mints an anonymous variable (VR6), the slot offers an existing variable of equal value, and a name is proposed and stored only on commit (VR2)
status: review
opened: 2026-10-03
priority: P0
cost: M
parent: d10-one-way-to-say-intent-is-unbuilt
branch: intent/gui-variables
pr: 4247
needs_ev: true
---

Unparked 2026-10-07: INTENT-LITERALS PR C (#4146) put a `VarId` in every
slot and minted an anonymous variable for a typed value.

## What the unit builds

- **A value typed at a slot mints** (VR6, D10): `SessionOp::SetSlot`
  writes `props::slot_typed_edit`, a `SetParam` of the value, whatever
  the slot read before; a value gesture still moves a typed value in
  place (INTENT-LITERALS Q6).
- **The offer**: after a typed value (a number, or text that is a
  written quantity) lands, `DocSession::offered` answers every other
  variable of the slot's kind whose value equals it, and the slot's row
  draws `same value as` with one button per variable and **keep
  separate**. Accepting is `SessionOp::SetSlotVariable`, the slot-write
  gesture; declining is `SessionOp::DeclineOffer`, which moves no
  document.
- **Naming** (VR2): a slot reading an unnamed variable has a **name…**
  button whose field opens empty (Ev, PR 4247: no proposal); the
  document holds no name until **Name** or Enter commits one `RenameVar`.
- With it, `viewer-param-vocabulary-names-a-variable` and
  `viewer-value-doors-read-a-defined-variable-as-absent`.
