---
id: a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face
kind: unit
title: A face's frame is built from the face and a cell off it, and an extrude's depth is signed
status: open
opened: 2026-11-02
priority: P1
cost: M
parent: on-the-picked-face-makes-a-profile-and-extrudes-it-in-one-checkbox
---


Cut by the VSEAM orchestrator (2026-11-02) from
`work/vseam/on-the-picked-face-makes-a-profile-and-extrudes-it-in-one-checkbox`.
That row's dated "ruled" section is the build spec. It is Ev's ruling
on `[ev]` PR 6298: `docs/DESIGN.md` `[placement]` and
`crates/editor-core/REFERENCES.md` DM1, as merged. This is the
editor-core half. The library half is
`work/pydoor/doc-sketch-on-face-retires-for-a-profile-drawn-on-the-face`,
and the viewer half is `work/vseam/on-the-picked-face-draws-the-profile-there`.

## The build

1. **A face's frame, a pose construction** (`crates/editor-core/src/pose.rs`).
   It reads a planar face and a cell of the face's body off the face's
   normal. Its answer is `Spin { Through { axis, point }, about z, by spin }`:
   - z is the face's outward normal;
   - x points toward the named cell, projected into the face's plane;
   - on a face with no such cell (a bare disc capping a cylinder), x is
     the space's x carried onto the face by the least turn, measured as a
     placement's spin is (`[placement.values]`);
   - `spin` is an angle slot, the constant 0 unless named.

   The construction is the builder for the `Flag` kind that
   `[variables.kinds]` already lists and nothing builds. A cell that a
   later edit removes (a filleted corner, say) leaves the read
   unresolved, typed and loud, and never re-pointed.
2. **A profile drawn on that frame** (`ProfileProgram.on`, `DrawnOn` with
   no traced faces) is in the face's body's space, and is one node. Give
   it an editor-core door that pncad and the viewer both call. That door
   replaces `edit.rs`'s `sketch_on_face` for their purposes.
3. **The extrude's depth is signed** (`Node::Extrude`'s `distance`, in
   `node.rs`). + is a boss, − is a pocket's tool, and 0 refuses. Lower
   −d at the evaluator as an extrude of |d| whose start cap sits d along
   z. That leaves `crates/sweep` (carve/lathe/strut ground) alone; if
   that does not hold, stop and say so. Update the variant's doc. It
   currently says the size is "refused unless definitely positive" and
   that the side is the mates' to say.

Leave the old fused `sketch_on_face`, `Recording::sketch_on_face` and
`SketchedOnFace` in place: their callers retire in the lib and vseam
units, and the last of those deletes them.
