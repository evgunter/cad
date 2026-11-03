---
id: a-plane-other-than-a-face-cannot-be-authored-in-the-viewer
kind: issue
title: No viewer gesture defines a plane other than a picked face, so a split or mirror cannot cut elsewhere
status: open
opened: 2026-11-03
priority: P2
cost: M
---


Ev on `[ev]` 6406 (2026-11-03): a split reads a target and a plane, any plane the document can already read, with no offset or plane construction of its own ("i thought we were just using general planes"). The mirror's plane seat is the same type.

The document's plane family (`PoseDef` in `crates/editor-core/src/pose.rs`) has these forms: a face's plane; a plane slid, spun, flipped or carried; a frame's xy/yz/zx. The viewer can author only the first, by picking a face (`matetool::plane_of`). A plane defined any other way is reachable only through Python. The split form's point/normal fields and `frame_off` are gone (PR 6431): a split reads a picked face, or a `Plane` or `Frame` variable the document already holds, picked by selecting its row (`combine::SplitTool::pick_defined`); nothing in the viewer declares such a variable.

The build is a general way to define a plane (and, by the same door, an axis), shared by split, mirror and the placement editor's picks. It is not a split-specific offset.
(FORMS orchestrator)
