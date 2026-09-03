---
id: focus-marking-is-per-node-not-per-segment
kind: issue
title: Viewport focus marking is per NODE, so a profile step cannot light the walls it swept
status: open
opened: 2026-08-29
github: 1182
---

## From GitHub issue 1182

Opened 2026-08-29; 0 comments.

Banked from the 2026-08-29 GUI tweak batch (`docs/GUI-LOG.md` tail, branch `claude/gui-display-editing-tweaks-w1b8j3`). `pick::focus` marks the drawn patches the side panel's selection is responsible for — every patch a feature drew, and for a node that draws nothing itself (a profile, a datum) the geometry built from it. That covers the half of the request that said "the node you're editing"; it does not cover the half that said "the line of a profile as it corresponds to edges on the body".

**What is already in place.** The answer type is a set of patch ids and nothing about its shape assumes a whole node's worth, so per-segment marking needs no new plumbing in `scene`/`gpu`: the scene already carries `FLAG_FOCUS` per corner, keyed off the id. `PickIndex::name_of` hands back each drawn patch's `StableName`, and the naming vocabulary already carries the provenance the marking wants — `RoleSeg::Lateral(ProfileEdgeRef { loop_index, segment })` is literally "the side-wall face swept from this profile segment", with `Band`/`RimEdge`/`Meridian` the revolve-side twins. So the filter is expressible today as "the drawn patches whose role path mentions this `ProfileEdgeRef`".

**What is missing, and why it was not guessed at.** The panel addresses a profile expression as `SlotId::Profile { loop_, step, arg }`, where `step` is the step's index in the loop's authoring chain. The name carries `ProfileEdgeRef.segment`, a canonical segment index in the lowered loop. Those two are not the same coordinate: a single authored step can lower to several canonical segments (a fused fillet leg, `circle_split`), and the carrier forms pin `step` to 0 regardless. Nothing in the tree states the correspondence, and a wrong guess here is the bad failure mode for this feature — it lights a confidently wrong edge, silently, with the user believing the picture.

So the unit is: establish the authored-step → canonical-segment map as a door with its own rows (it belongs beside the lowering, in `profile`/`editor-core`, not in the viewer), then narrow `pick::focus` through it. Two consequences worth deciding at the same time:

- **A step maps to a SET of segments**, so the focus for one step is a union, and the panel wants a "focused slot" notion — layer-3 state like `hover`, set as a slot widget takes focus — because `Selection` has no slot granularity today.
- **Edges are not drawn.** The viewport draws face patches only, so the marking will show the walls a segment swept rather than the rim edges. That reads correctly for an extrude and is the honest available answer; a real edge pass is separate work and should not be folded in here.

Not a defect of what shipped — the shipped behaviour is stated as a gap in the module docs of `pick::focus` and in the GUI log — but it is the natural next step and the request that produced the feature asked for it by name.

## Home

Viewer ground (`crates/viewer/src/pick.rs`) with a lowering-side door in `profile`/`editor-core`; the GUI program is closed and may hold only closed items, so it lands under `work/issues/`.
