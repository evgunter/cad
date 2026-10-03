---
id: viewer-free-move-and-place-where-shown-over-a-whole-group
kind: issue
title: viewer: widen G3's free-move probe to a whole group, draw an unplaced group where last shown, and add a one-edit 'place where shown'
status: open
opened: 2026-10-03
priority: P1
cost: M
---


Filed by the PLACE orchestrator as P3 of `docs/EDIT-PLACEMENT-SPEC.md` ("P3: viewer"), now that P2 has merged. It realises Ev's ruling on `[ev]` #3441: an unplaced group lives in its own space, and the viewer draws it where it was last shown, as display state that no logic reads.

## The work

- G3's free-move probe is widened to a whole group.
- A group that becomes unplaced (its gauge deleted, its offset cleared, its placing mate deleted) is drawn where it was last shown. The probe is seeded from the prior draw and discarded when the group's key changes, undo included.
- "Place where shown" is one edit whose frame comes from the gesture: a `SetOffset` on the group's root, or a gauge placement, written as literals.
- The kernel side exists: `SetOffset`, `SetGauge`, `Promote`, `Fold`, and `Evaluation::unplaced_groups` in document order.

Related: `work/offer/viewer-free-move-decides-rigidity-by-its-own-predicate.md`, the free-move rigidity predicate. It should read `topo::check_rigid`, the one predicate the kernel's doors use.
