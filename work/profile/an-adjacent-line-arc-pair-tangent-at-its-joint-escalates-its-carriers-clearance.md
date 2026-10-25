---
id: an-adjacent-line-arc-pair-tangent-at-its-joint-escalates-its-carriers-clearance
kind: issue
title: A fillet the door builds between two arcs can escalate its pair's carrier clearance in validation
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Since the joint pass reads a joint's turn at the shorter leg's arm (#5377's fix pass), the arc-fillet door builds some shallow arc × arc corners it used to refuse: its stored-form read (`path_fillet_joint_turn`, `crates/profile/src/path.rs`, the call to `seg::joint_tangency`) reads the fillet's joints as tangent, so it no longer relays the near-tangency to the author. Validation's pair pass then reads the same pair of carriers, the fillet stored as its chord and its leg's circle, meeting within ε at their joint, and escalates `carrier_line_circle` on their clearance (`crates/profile/src/seg.rs:1031`). The door's promise, that no loop it emits carries a tangency validation contradicts, holds: an escalation is not a contradiction. But the door now hands the author a loop validation will not decide.

Measured by `fillet_stored_tangency::a_shallow_arc_x_arc_corner_validates_wherever_its_legs_stand_apart` (`crates/profile/tests/fillet_stored_tangency.rs`), over radii 0.02–1 and turns 1e-8…1e-1:

| ε | corners built | pair escalations |
|---|---|---|
| 1e-9 | 122 | 4 |
| 1e-6 | 119 | 12 |
| 1e-12 | 4 | 0 |

The fix reads the pair's clearance at the door too, when it stores a fillet whose chord sits within the band of its leg's carrier, and relays the escalation as the door did before. Or it lets the pair pass recognise an adjacent pair already decided tangent at its joint. That choice belongs with the unit that owns the pair pass.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILE (`work/profile/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam; PROFILE holds the goal (`docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
