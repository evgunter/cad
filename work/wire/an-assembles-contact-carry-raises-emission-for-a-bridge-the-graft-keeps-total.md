---
id: an-assembles-contact-carry-raises-emission-for-a-bridge-the-graft-keeps-total
kind: issue
title: wire_assemble and distribute raise NamingError::Emission when carry_contacts finds no graft image, though the graft's bridge is total over the member it grafted
status: open
opened: 2026-11-01
priority: P3
cost: M
---


Found by `wire-emitraise` (branch `wirehold/emit-raises-argued`), 2026-11-01, sweeping for siblings of the torn-read raises in `names::emit::name_assemble` (`work/wirehold/emit-rs-shared-helpers-raise-emission-for-torn-reads.md`).

Two callers map `product::carry_contacts`'s refusal straight to `NamingError::Emission { what }`:
- `wire_assemble` (`crates/editor-core/src/eval/wire.rs:~4253`), once per member body and its `GraftKeys`;
- the intersect's distribute (`crates/editor-core/src/eval/wire/distribute.rs:~269`), once per assembled unit.

`carry_contacts`'s `what` is "vertex", "edge" or "face" when the bridge has no image for a record's cell, and "cell pair" for the other `topo::CarryRefusal` arms (`Pair`, `Parted`, `Unread`, `Stale`, `MintedMeeting`). As an `Emission` it renders as "...: face", which reads as no sentence.

**The image arms.** These have the argument `name_assemble` now states as `GRAFT_TOTAL` (`names/emit.rs`). `topo::graft_disjoint_all_keyed`'s bridge is total over the source's live entities (`topo/src/instance.rs`, `GraftKeys`). `Framed::read_in` keeps keys, because `transform_rigid` rewrites geometry in place of a clone. A member's records cite live cells of its own body. So those arms look like row-4 `unreachable!` with that why.

**The "cell pair" arms are not argued here.** They need their own D9 argument, arm by arm. `product_recorded`'s use of the same function maps to `ProductError::ContactLineage`, which the function's doc calls defensive. That door is outside this row.

Not fixed in that PR: neither file is under `names/`, and `eval/wire.rs` is WIRE's ground outside WIREHOLD's row.
