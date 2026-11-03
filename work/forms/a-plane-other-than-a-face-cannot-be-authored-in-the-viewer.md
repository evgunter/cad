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

The document's plane family (`PoseDef` in `crates/editor-core/src/pose.rs`) has these forms: a face's plane; a plane slid, spun, flipped or carried; a frame's xy/yz/zx. The viewer can author only the first, by picking a face (`matetool::plane_of`). A plane defined any other way is reachable only through Python. The split form's point/normal fields and `frame_off` are gone (PR 6431): a split reads a picked face, or a `Plane` or `Frame` variable the document already holds when the selection names it (`Selection::Variable`, `combine::SplitTool::pick_defined`).

What the viewer can do with plane variables today:
- The placement editor's plane mate (`MateTool::add_mate(VarKind::Plane)`, `pane/create.rs`) writes its sides as inline pose reads. The edit door lowers each one to an anonymous `VarDef::Pose` variable (`editor_core::edit`, `mint_pose`) that the mate alone reads. So it declares a plane variable, but one with no name and one reader (VR2, VR7). A split reading it would be a second reader, which the door refuses.
- No Properties row lists a pose variable, named or not: `props::variable_rows` lists named free variables, and `props::defined_rows` named expression variables. So a split can take a `Plane` variable only through the selection stream (a driver, the op vocabulary), not by a click on a row.

Defining a plane in the viewer, and giving a pose variable a row a split can pick, are both this row's to build.

The build is a general way to define a plane (and, by the same door, an axis), shared by split, mirror and the placement editor's picks. It is not a split-specific offset.
(FORMS orchestrator)
