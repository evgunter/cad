---
id: cursor-projection-landed-in-marks-for-want-of-a-home
kind: issue
title: cursor_projection is a camera transform living in marks because no module claimed it
status: open
opened: 2026-09-06
refs: [2083]
---


Found by the style review of #2083, which names this the unit's one
judgement call.

`cursor_projection` (`crates/viewer/src/marks.rs:432-448`) takes a
view-projection matrix, a cursor in NDC and a viewport size, and
returns a matrix. It names no index, no selection, no hover and no
document. Nothing about it is a mark: its subject is a camera
transform, and its doc says so — *"composed with
[`crate::camera::Camera::project`] it says that the world point the ray
path un-projects to is the point the id pass rasterizes at the centre
of its target"*.

The reason given for its home is testability, not subject: *"it lives
here, out of the render module, because it is the one part of the id
pass a machine with no GPU can check"*. That is an argument for it not
being in `gpu`; it is not an argument for it being in `marks`. The PR's
own table records it as `none — a matrix`, and its production consumer
set is exactly `gpu.rs:594` — the same consumer it was moved away from.

`crate::camera` is the module the function composes with, the one whose
doc link the move had to rewrite to
`[\`crate::camera::Camera::project\`]` (`marks.rs:428`), and the one a
reader hunting a projection matrix opens. That the move had to widen
that link into a full path is itself the signal: the function's subject
lives in another module.

Taking it made `marks.rs`'s opening universal false (see
`marks-header-asserts-universals-its-own-module-breaks`) and turned a
three-item module into a four-item one whose fourth item the section
title has to mis-describe. Either it should have stayed in `pickindex`
until a unit moved it to `camera`, or the unit should have moved it to
`camera` here.

## Confidence

`likely`. The move is behaviour-preserving and nothing is broken; this
is a judgement that `marks` is the wrong home and that the header
damage is the evidence.
