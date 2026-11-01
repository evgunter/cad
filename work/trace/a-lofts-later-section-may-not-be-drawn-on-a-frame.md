---
id: a-lofts-later-section-may-not-be-drawn-on-a-frame
kind: issue
title: A loft's section after the first may not be a profile drawn on a frame
status: open
opened: 2026-10-29
priority: P3
cost: M
design: true
---


Found by INTENT stage 6 PR C's review (`work/intent/a-sketch-continues-tangent-to-a-surfaces-trace.md`, PR #6043). Departure 5 of #6006's ruling.

A loft's first section stands in its plane, and each later section where its station carries that plane. A drawn section's plane is its frame's, so a later drawn section would name two planes. `wire::loft` therefore refuses it as `WrongOperand` (expected "free profile", found "drawn profile"; `crates/editor-core/src/eval/wire.rs:4277`). The row is `intent_s6_c_trace::a_lofts_first_section_stands_in_its_frame_and_a_later_one_refuses`.

A later section drawn on a face's trace has a use: a loft from a sketch on one body to a sketch on another, each continuing tangent to its own face. That needs a station that reads a frame, or a loft that reads each section's frame, which is a design question for `[placement.spaces]`.

**Done when** that question is answered, and the refusal either stands as the answer or gives way to the station it names.
