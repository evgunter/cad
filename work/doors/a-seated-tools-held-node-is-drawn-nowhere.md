---
id: a-seated-tools-held-node-is-drawn-nowhere
kind: issue
title: A seated tool's held node seats are drawn nowhere in the viewport, so a tool can commit against a body the picture does not single out
status: parked
opened: 2026-09-30
priority: P2
cost: M
blocked_on: [5636]
---


Found by AUTH-10's census of latched picks (the held-face-mark unit),
which the spec scoped out: *"If held node seats are also undrawn, that
is a row, not this unit."* They are.

## What

Seven modal tools hold NODE seats in tool state across frames —
revolve, boolean, split, transform, pattern, projection and duplicate,
each over `seats::Seats` (`crates/viewer/src/seats.rs`, the `held`
field; the tools in `combine.rs` and `revolvetool.rs`). A seat is
filled from `Selection::seat_node`, so a face or edge click fills it
with the node whose drawn body the ray met, and a tree click fills it
directly. Nothing clears a seat but the tool.

The viewport draws none of them. `pane::viewport::frame_marks` gathers
every held pick the picture marks — the add-datum form's face, the mate
tool's two faces, the blend tool's edge set — and there is no node in
it: the seated tools are named in its doc as the one holder it does not
draw. The only evidence a seat is filled is the panel's line
(`seats::picks_line`), which says `feature 3` and not where feature 3
is. So a boolean's two operands, picked by clicking two bodies, stop
being marked the moment the selection moves on, which is the defect
`held-face-pick-is-invisible-in-the-viewport` closed for faces.

## Why it is a design question and not the face fix again

A face has one drawn patch, so its held mark is an id in
`marks::Highlight::held`. A node seat names a whole body — possibly
several (a pattern), possibly none drawn (a profile, a datum, a split
the projection hides) — so the mark is an EXTENT, which is what
`marks::focus` answers for the side panel's selection. Whether a held
node wears the held mark (stripes, `marks::Held`) over that extent, and
through which lane — `focus` is a scene flag and rebuilds the mesh,
`Highlight::held` is one uniform id per holder — is the decision.

## Decided (2026-10-25, designer pair; AUTHTAIL orchestrator)

An Opus and a Fable designer weighed this row together with `a-held-face-stays-marked-while-its-form-is-collapsed`, as one problem: what a held pick is. The first round split on three points. In round two each was shown the other's report: A held its position, and B moved to A on all three, giving its reasons. That is a convergence, not a crossover. No ratified decision changes, so this is built and not put to Ev.

**Premise corrections.**
- The add-datum form is gone (`e58fec1fb5`, PR 5506). The live latch is `Drafts::sketch_face`, which `ViewerBehavior::sketch_face_rows` (`pane/create.rs`) writes.
- A seat's extent is not what `marks::focus` answers. Focus is what a node MADE, so it would light a fillet's blends.
- It is not `ids_of_node(seat node)` either. A seat holds the operation whose body a placement places (`world::seat_read`), and the drawn patches belong to the copies.

**The design (both, likely).**
1. **One holder.** `Tools` is the only state that holds a pick across frames. The add-profile form's on-face mode becomes `ToolKind::SketchOnFace`, with one face seat; its checkbox is that tool's open/close door. The mirror is `ToolKind::Mirror`, with a body seat and a face seat. Depth, spin and every tool's numbers stay drafts, as today.
   - `Drafts` loses `sketch_face`, `sketch_face_said`, `sketch_on_face`, `held_face`, `hold_sketch_face` and its `document_replaced`/`respeak` halves.
   - This closes a live double fill. With the mate tool open and "on the picked face" ticked, one face click fills both, which `tools.rs`'s one-tool rule exists to forbid (both designers, by reading `Tools::feed` and `sketch_face_rows`; not run).
2. **One face seat.** `seats::FaceSeat` holds a face, the name its node was last said by, and a survival check through `tree::pick_resolution`. Sketch-on-face, mirror and mate (which keeps its pair compaction) all use it. Today the sketch latch has no survival check at all.
3. **One read door.** `Tools::held()` returns the open tool's picks as a list of `HeldPick`: `Face`, `Edges`, or `Read` (a body seat). The match is exhaustive. `pane::viewport::frame_marks` names no tool.
4. **One drawn lane.** `marks::held` resolves the held picks into a patch set plus line segments:
   - a face is its `drawn_patch`;
   - a body read is the patches of every drawn copy placing it (`world::placements_of_read`, narrowed by `hidden_placements`), so a body placed twice lights both copies;
   - blend edges and a held profile are segments.

   The patch set rides the mesh as a per-corner `SceneMesh::FLAG_HELD` beside `FLAG_FOCUS`, drawn with `Theme::held`'s stripes. `Highlight::held`, `HELD_FACES` and the `held_ids` uniform retire. The scene is keyed on (index, display, focus, held). This adds no rebuild a selection click does not already pay (unmeasured).
5. **Lifetime = the tool is open, and the open tool is always on screen.** The open tool's panel is drawn in one fixed slot at the top of Properties, where the mate panel already sits. The sections keep only the buttons that open each tool. This fixes the row-2 symptom of stripes with no form saying so. It also fixes the tools' twin defect: a tool inside a collapsed section keeps taking clicks with its panel hidden.

**Sequencing.** This lands after INTENT's PR 5636. That PR edits `tools.rs` (`committed_by` gains `SetProduct`, and `rename` goes) and rewrites `world.rs`'s `placements_of_read`, which the extent calls. Keep the extent a single call into `world`. After `patterns-are-index-variables`, a family read lights every member's copy with no change here.

**Tests the unit owes.**
- A headless row asserting a seated body marks exactly its drawn copies' patches, in the style of `tests/focus_highlight.rs`, with a `held_patches` summary beside `focus_patches`.
- A row showing that a face `Select` op moves the sketch-on-face seat with no panel drawn.
- A row showing that ticking on-face closes an open mate tool.

**Also in scope, from the mirror review (PR 5683).** Fold these into the change:
- the three face holders' "last said" containers: mate's `[Option<SpokenNode>; 2]`, the sketch latch's `HeldNodes`, and the mirror's `Option<SpokenNode>`;
- the mirror's plane riding in the mate's slot `a` of `marks::Held::faces`;
- the hand-spelled end-cap `StableName` literals in the viewer tests (`combine_ops`, `node_labels`, `mirror_authoring`, `creation_ops` ×3, `sketch_on_face`), which want one helper.

## Moved (2026-10-26)

AUTHTAIL folded into DOORS (Ev, in chat, 2026-10-26): both were AUTHOR's
successors on the viewer authoring files. Id and body unchanged.
