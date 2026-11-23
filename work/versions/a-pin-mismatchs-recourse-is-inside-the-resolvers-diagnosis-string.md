---
id: a-pin-mismatchs-recourse-is-inside-the-resolvers-diagnosis-string
kind: issue
title: a pin mismatch's recourse is inside the resolver's diagnosis string, so a viewer drawing the accept control prints it again
status: open
opened: 2026-11-23
priority: P3
cost: M
---


Found by OFFER's offers unit (`work/offer/a-refusal-offers-no-action-in-the-viewer.md`, item 4: where an offer is drawn as a control, the message's prose recourse for that same act is not printed again beside it).

The viewer draws a pin-mismatched instance row's accept as a control (`viewer::recourse::offers`, the `NodeErrorClass::PartPinMismatch` arm; label "Accept updated version of <file>, at every instance of it"). The row's failure line above it is the node's own sentence, `PartFault::Unresolved { fault: PinMismatch, message }` (`crates/editor-core/src/eval/parts.rs`, `PartFault::say_by`): "the reference's pin does not hold: {message}". The `message` is a `String` the resolver wrote, `WorkspaceError::PinMismatch`'s `Display` (`crates/pncad/src/workspace.rs`), which ends on `Recourse(PIN_MISMATCH_RECOURSE)`: "record the \"accept updated version\" edit …". So the act is said in prose directly above the control that is it.

The viewer cannot leave that clause out without trimming the kernel's sentence, which the item rules out. The ε seam's arm shows the shape that would allow it: there the resolver's message is the diagnosis alone, and `PartFault::say_by` appends `Recourse(EPSILON_SEAM_RECOURSE)` itself. With the pin's recourse appended the same way, a door could say the fault beside its accept without it (as `CheckFinding::said_beside_its_assertion` does for a check's assertion).

Hazard: `PIN_MISMATCH_RECOURSE` is "stated once at every door" (`workspace.rs`, the `PinMismatch` arm of the resolve recourse), and the Python author suite and the demo's update walk pin that count at one.
