---
id: a-minted-reference-direction-follows-the-computing-axes
kind: issue
title: A minted carrier's reference direction branches on the computing axes (orthonormal_basis), so a sketch's zero spin depends on the computing frame
status: open
opened: 2026-10-08
rides_with: an-operation-computes-in-a-frame-of-its-reads
priority: P0
cost: M
---


`Vec3::orthonormal_basis` (`crates/geom-core/src/linalg/vec.rs:640`,
`crates/geom-core/src/linalg/unit_vec.rs:348`) picks its first axis by
comparing the normal's components against the coordinate axes. Carriers
an operation mints take their u-reference from it, so DM1's zero spin on
such a face depends on the frame the operation computes in, by a branch
and not by rounding. A sketch on the face can turn by a finite angle when
the computing frame changes. The sites FORK-S3P's designers found are:
sweep cap planes (`crates/sweep/src/swept.rs`), Newell planes
(`crates/geom-brep/src/newell.rs`), and a split's one-corner section
(`section_loops.rs`, the fallback in `chord_u_ref`).

FORK-S3P (fork log row 95, PR 4324) makes the computing frame the
author's stated first operand, so the dependence is said rather than
hidden. The deeper fix is for each such carrier to derive its reference
from its inputs: a cap from the path frame, a section from the cutting
plane's reference. The frame then moves only rounding.

After Ev's principle on #4324 (the computing frame is chosen for
numerical behaviour and caching), this is required, not optional. Once
the frame is a free numerical choice, nothing semantic may depend on it.


FORK-S3P round 10 (Ev on #4324: a spin within a symmetry "does need to
be set explicitly, but 0 … is always a valid value") takes the other
road. A face reads as a plane, and no reader takes a carrier's
reference direction. A value's zero is charted on the two bodies' own
construction coordinates, so the minted reference stops carrying
meaning instead of being re-derived. What remains is retiring its
readers: the `Frame` mates on a `FaceFrame` and the sketch-on-face zero
spin (DM1). The carrier's reference may then stay a computing
convenience.

Built with stage 3 E (`an-operation-computes-in-a-frame-of-its-reads`), which closes this row. D10 as ratified says "a minted reference direction is a function of the inputs, not of the axes", and that governs the round 10 note above. No reader takes a carrier's reference, but a u-reference also places a seam and its names, which the frame must not move. So E re-derives each of the three sites from its inputs.
