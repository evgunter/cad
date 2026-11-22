---
id: a-plane-other-than-a-face-cannot-be-authored-in-the-viewer
kind: issue
title: No viewer gesture defines a plane other than a picked face, so a split or mirror cannot cut elsewhere
status: open
opened: 2026-11-03
priority: P1
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

Moved from FORMS to FORMWORDS on 2026-11-20, by `git mv`, when FORMS merged into FORMWORDS: FORMS held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one FORMWORDS carries.

## Ev hit it in the mirror (2026-11-22): raised to P1

Ev (in chat, 2026-11-22): "in the gui, there seems to be no way to mirror around some arbitrary plane rather than a face of an object". Ev filed it at P1.

The mirror is narrower than the split even before the general plane door exists. The split takes `Selection::Variable` (`crates/viewer/src/tools.rs:942-946`, through `PlaneSeats::pick_defined`, `crates/viewer/src/combine.rs:406-416`, and `SplitPlane::Read` in `SplitTool::op`, `combine.rs:605-623`). The mirror routes only `selection.face()` (`tools.rs:972-975`). `MirrorTool::op` refuses with `SeatError::NoFace` (`combine.rs:716-729`), `SessionOp::AddMirror { body, plane: FaceSelection }` can carry nothing else (`crates/viewer/src/session/op.rs:810-833`), and `add_mirror` reads the plane off the picked face (`crates/viewer/src/session.rs:3801-3826`). Meanwhile `Node::Mirror.plane` is already a `RoleRead` of a `Plane` or a frame's plane (`crates/editor-core/src/node.rs`, ~1852-1865), and `HeldPlane::Defined` exists (`combine.rs:361-372`) unused by the mirror. Giving the mirror the split's `pick_defined` arm is a cheap first step that can land ahead of the general plane-definition door.

## The design (2026-11-22)

A plane or an axis the viewer authors is a pose variable: there is no
separate plane object and no plane-construction gesture. The general
door is **one editor for a pose definition**, over `PoseDef`'s family,
filtered by the kind the seat reads (plane or axis).

- **Create, then edit.** Split and mirror insert on a picked face's
  plane, or on a Plane or Frame variable named in the selection, as
  today. The node's plane row then edits the definition, one undoable
  edit each: offset along its normal (a length row, number or
  formula), turn about a picked edge or axis (an angle row), flip, a
  frame's xy / yz / zx plane, through an edge and a point, re-pick.
  Wrapping is an edit of what the seat reads (the seat's plane P
  becomes `Slide { P, … }`), and unwrapping points the seat back at P.
- **Anonymous at its reader, named to share.** An unnamed pose is shown
  under its one reader's row (VR2). Naming it on that row lists it in
  the Variables pane, whose row is the same editor; editing it there
  moves every reader. A second reader of an unnamed pose opens the
  name field, as Place does for a shared source (VR7).
- **Seen and picked.** A named plane or axis is drawn as a display glyph
  in its space, toggled in the View pane like a body; an unnamed one is
  drawn while its reader or its row is selected. Clicking it gives
  `Selection::Variable`, which every plane and axis seat takes.
- **Axes by the same door**: the mate's axis sides and the placement
  values' slide-along and spin-about reads. The mate sides' own
  lowering (`matetool::plane_of`, `MateReading`) becomes the editor's
  base picks.
- **Variables pane "New plane" / "New axis"**: sugar over the same
  editor, asking for a name and a base pick (the GUI proposes no name),
  built after the seat path.
- **The editor exposes the whole of `PoseDef`'s family**, as structured
  widgets rather than typed text (the reads inside a pose are picks).
  A chain reads on the row as a flat list of steps, in stored order.
- **The world stays unreadable** ([placement.world],
  [variables.poses]): every plane is built from the body's own
  geometry.

Kernel: `VarDecl` (`crates/editor-core/src/var.rs`) gains a pose arm,
so `DeclareVar` and `DefineVar` can declare and redefine a pose
variable. `no-viewer-row-re-points-a-live-nodes-read`'s plane slots are
this editor's re-pick and are built with it.
