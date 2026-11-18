---
id: a-face-frame-and-a-signed-extrude-for-a-sketch-on-a-face
kind: unit
title: A sketch on a face is drawn on the face's placement chart (one code with the plane mate's), and an extrude's depth is signed
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

## 2026-11-18 — re-specced after Ev's ruling on `[ev]` PR 7073 (merged)

**This section replaces step 1 above, and amends step 2.** Spec: `docs/DESIGN.md` `[placement]` and `crates/editor-core/REFERENCES.md` DM1, as merged by 7073.

1. **The face's chart, a pose construction.** It reads a planar face, plus a written **orientation** (the 24-way kind `Mate.orient` holds, which the gesture writes nearest the face), a **spin** and **two slides**, each a variable that is the constant 0 unless named. Its value is the chart a plane placement onto that face with those values gives:
   - z is the face's outward normal;
   - x and the origin are the space's, under the orientation, carried onto the face by the least turn, then spun and slid.

   **One code.** The construction calls the plane mate's chart function (`mate::value`, the `charted` path, through `RigidMotion::turned_onto` / `least_turn`). It never re-spells it, so the frame and a placement cannot drift (Ev: "how can we make it impossible for these two to drift? can they use the same code?").

   Within the band about the orientation's pole it refuses typed, naming the orientation to write, as `MateFault::Backwards` does. There are no cell reads and no `Flag` builder: the "toward a cell" text above is retired. The GUI may prefill slides and spin from a clicked corner; the kernel does not.
2. **The drawn-on-face door** takes the orientation (`None` means the nearest, written into the recipe; `nearest_to_face` moves into this door, so it has one home), spin and slides. It also takes the faces to trace (`DrawnOn.faces`, possibly empty: a profile on a face is one act whether or not it traces). Tracing on the chart frame is its own unit, `tracing-reads-the-faces-chart-and-traced-frame-retires`. This unit only lets the list through.
3. **Signed depth:** unchanged from step 3 above.

Check FRAME's 2026-11-03 note (the least turn's pole) against the orientation: a bottom cap writes `x: +x, z: −z`, and the least turn is the identity.
