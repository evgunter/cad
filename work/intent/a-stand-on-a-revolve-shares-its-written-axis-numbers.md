---
id: a-stand-on-a-revolve-shares-its-written-axis-numbers
kind: issue
title: stand writes a revolve's written axis numbers as shared reads, which the edit door refuses (SharedVarNeedsName), so the viewer's place gesture cannot stand a new revolve
status: open
opened: 2026-10-20
priority: P1
cost: M
design: true
---


Found by INTENT stage 3 D's lane of 2026-10-19/20, chasing `creation_ops::the_ring_stream_reproduces_the_gallery_document_bit_for_bit` (viewer, red).

**The red.** The viewer's "place part" gesture places a new body by `place_at` at the identity (`crates/viewer/src/session.rs`, `place_copy`), while the tour (and every `Recorder::place_part`) stands it by `mate::stand`. For a full ring the two write different documents: `stand` writes an `Axis` mate to the world's `x` axis spun about `z` by the revolve's own heading slot (`turn/4`, read, not copied) and slid by its `axis_through` slots; `place_at` takes the axis path (`axis_in_world`) and writes spins taken by `atan2` as numbers, `-π/2` about `x` then `-6.1e-17` about `z` (an exact zero, rounded: `a-turn-by-the-nearest-quarter-turn-rounds`). So the ops-authored ring is not the gallery document, and the row's D7 comparison fails.

**Why the gesture cannot simply stand the copy.** Switching `place_copy` to `editor_core::stand` makes the ring row pass and every other viewer row but one, and that one shows the defect: `creation_ops::the_revolve_tool_holds_its_profile_pick_and_survives_its_loss` authors a revolve whose `axis_through` is written `[0 m, 0 m]`, and the stand's insert refuses `EditError::SharedVarNeedsName`. `mate::pin::revolve_axis` (`crates/editor-core/src/mate/pin.rs`) writes each move as `Formula::var(var, …)` over the slot's own variable, and a written number is an anonymous variable the edit door will not let two readers share. `revolve_about` (the test fixture) and the tour write the axis as the constant zero to stay clear of this; the viewer's app writes `0.001 mm` (`crates/viewer/src/app.rs`, the batch test's revolve), and Python writes `0 m` wherever a user does.

**The question** (why `design: true`). `stand`'s contract is that the copy follows its body's variables, and `Anchors` counts a written number as a variable. Sharing honours that and is refused for an anonymous one; copying the number keeps the edit door happy and breaks the contract (editing the revolve's axis would move the copy in the world). Options: name the variable at the gesture; let a placement's world read share an anonymous slot variable as one fact; or make `stand` read the revolve's axis by role off the copy, so no number is written. Until it is decided the viewer keeps `place_at` and the ring row stays red.

**Done when** a revolve with written axis numbers stands by `stand`, the viewer's gesture stands every new body as the tour does, and the ring row compares equal.
