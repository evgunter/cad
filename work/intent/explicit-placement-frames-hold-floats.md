---
id: explicit-placement-frames-hold-floats
kind: issue
title: A pattern's or placed union's Explicit placement list holds raw f64 frames, not variables
status: open
opened: 2026-10-07
---

`PatternKind::Explicit(Vec<placement::Frame>)` (`crates/editor-core/src/node.rs`,
the variant's doc: "Instances at ABSOLUTE frames, listed") stores each
placement as a `placement::Frame` — `columns: [[f64; 3]; 3]` and
`translation: [f64; 3]` (`crates/editor-core/src/placement.rs`, `Frame`).
So after INTENT-LITERALS D, a document's snapshot holds floats in two
places: the free variables' values (VR3, by design) and these frames.

Found by the D lane's sweep: every float a committed snapshot holds,
outside the `vars` table, is the epsilon, an appearance metadata value,
or a `PlacedUnion`'s `kind/Explicit/{columns,translation}`
(`crates/editor-core/tests/corpus/die_tool.pncad`: 54 + 18 floats, the
die's twenty-one pip frames).

VR4 says every slot holds a variable id, a placement step included. An
explicit frame is not a slot today (no `SlotId` addresses it, which is
what keeps it out of C's and D's fence), so no token, range or
analysis lane reads it as a value — but nor can a person tolerance a
pip's position, name it, or share it between two placements. Whether
the frames become slots of variables (stage 3's `Frame` kind, D10) or
stay a fixed payload is the program's call; this row records that the
literal sweep's fence stops here.
