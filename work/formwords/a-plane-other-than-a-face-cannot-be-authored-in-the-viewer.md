---
id: a-plane-other-than-a-face-cannot-be-authored-in-the-viewer
kind: issue
title: No viewer gesture defines a plane other than a picked face, so a split or mirror cannot cut elsewhere
status: review
opened: 2026-11-03
priority: P1
cost: M
pr: 7382
branch: formwords/datum
refs: [7373]
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

## The mirror takes the split's plane seat (2026-11-22)

The cheap first step above, landed by PR 7373; this row stays open for the general door. The mirror and the split now share one plane-seat path: `PlaneSeats::feed` (a face is the plane, a `Plane` or `Frame` variable the plane, any other node pick the body) and `PlaneSeats::tool_plane` (the commit's plane, or `SeatError::NoPlane`), in `crates/viewer/src/combine.rs`; `SessionOp::AddMirror` carries the split's plane type, renamed `ToolPlane` (`crates/viewer/src/session/op.rs`), and `DocSession::tool_plane` lowers it for both `add_split` and `add_mirror` (`crates/viewer/src/session.rs`). A pose variable still has no row to click, so the variable pick comes through the selection stream only, as the split's does.

Other plane or axis seats that take a face or edge pick and no pose variable, for the general door to reach:
- the pattern tool's row direction and ring axis (`session::PatternDirection`: a target axis, an edge or a face; no `Axis` or `Frame` variable);
- sketch on a face (`SessionOp::SketchOnFace`, `SessionOp::DrawOnFace`: the profile's plane is a picked face only);
- the placement editor's mate sides (`tools.rs`, `OpenTool::Mate` drops `Selection::Variable`).
- the Transform tool's opener (`tools.rs`, `OpenTool::Transform` drops `Selection::Variable`: the placement editor opens on a picked face, edge, vertex or node only).

## The design (2026-11-22)

A plane or an axis the viewer authors that is not a face read in place
is a **datum**, placed the way a body is placed (DESIGN
`[placement.datums]`). A datum is a node: it reads a target space and a
bundle of mates and values, and its one output is a frame in that
space. Readers take a plane or an axis off it through a role (xy / yz /
zx, x / y / z).

- **New plane / New axis / New frame** inserts a datum into the
  selected body's space (or the world, with nothing selected) at all
  values zero and opens the placement editor on it (create, then
  edit). Its mates' near sides are its own roles, so every pick fills
  a far side.
- **A datum is a node like a body.** It has a tree row, a label,
  delete and selection. It is drawn as a display aid in its space and
  is pickable, and a pick fills an open plane or axis seat. Its output
  is shared unnamed (VR2), so a mirror and a split read one datum with
  no naming step. Editing it moves every reader. Re-pointing a reader
  at another datum is `no-viewer-row-re-points-a-live-nodes-read`.
- **An offset from a face** is a mate whose far side is the face's
  plane slid along its normal (`PoseDef::Slide`), as a body's standoff
  is (`[placement.mates]`). No mate holds a gap. The placement editor
  gains the "offset" row on a mate's far side, which bodies get too.
- **Slide, Spin and Flip stay** as pose moves: a mate's sides are
  spelled with them. `PoseDef::FaceChart` stays as it is for now: folding
  it into the datum waits on a datum's pole being levered over what it
  carries (`face-chart-folds-into-the-datum-once-a-datum-levers-its-pole`).
- **Always a frame** (Ev, `[ev]` 7367): a plane or axis is a role read
  off the datum's frame. If a reader later needs a kind's symmetry, it
  can be added then, perhaps as a forgetting layer over a frame.
- **Anywhere a placement may go**, the world included, and a mate may
  read a datum at all-zero values. No code bans either.
- **A face picked plainly** stays the inline read it is today; a
  one-off face needs no datum.

Kernel: a datum node sharing `mate::Constraint`'s far side, the bundle
door and the solve with `Node::Place`. The near side is a role of the
datum, not a read, so `Constraint` needs a type parameter or a sibling.
`Node::Mirror` and `Node::Split` read a pose of their body's space,
a datum's included.
