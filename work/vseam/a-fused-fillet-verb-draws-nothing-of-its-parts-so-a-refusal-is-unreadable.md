---
id: a-fused-fillet-verb-draws-nothing-of-its-parts-so-a-refusal-is-unreadable
kind: issue
title: arc_fillet_arc (and the fused fillet family) draws none of its component arcs, so when it refuses to draw nothing shows what went wrong
status: open
opened: 2026-11-02
priority: P1
cost: M
---

Reported by Ev (in chat, 2026-11-02): "arc_tangent_arc is basically
impossible to use because it has no visualization of its components
and so when it fails to draw it is really not clear what's going wrong.
(it can probably display e.g. the fuller arcs that it'll fillet between
(removing parts of them), in another color)". The verb is
`arc_fillet_arc(spec, r, spec₂)` (`docs/PATHS-DESIGN.md` §2c,
`ProgramStep` in `crates/editor-core/src/program.rs`). Filed by the
VSEAM orchestrator. **Reported against an older build** (Ev's note the
same day): reproduce on main first.

## What is missing

The fused fillet verbs (`fillet_arc`, `arc_fillet`, `arc_fillet_arc`)
are each one step that constructs two carrier arcs, or an arc and a
line, and the fillet between them, then keeps only the trimmed pieces.
The editor's preview (`pane/viewport.rs`, `push_preview`, and
`sketch::ProfilePreview`) draws the resulting loop, or, when the step
refuses, the prefix up to it plus a refusal sentence. It never draws
the step's construction: the full incoming arc, the full arrival arc
and the fillet circle. So when the fit refuses (no tangent circle of
radius `r`, a side that does not exist, an arrival the run-out cannot
reach), the person sees nothing of the geometry the refusal is about.

## What a fix needs

- **Construction marks for a fused step:** its carriers drawn in full,
  in a distinct construction colour, with the kept pieces drawn as
  usual over them. Ev's suggestion is the fuller arcs it fillets
  between, with the trimmed-away parts in another colour.
- **Drawn when the step refuses too.** That is when they are most
  needed. The carriers come from the step's own specs before the fit,
  so they exist even when the fillet does not.
- **The refusal sentence pointing at what it is about** where it can,
  such as "no circle of radius r touches both".
- **Data source:** the replay's structure record already holds each
  fillet's decisions (`ReplayStructure`, PP2), and the carriers are
  each spec's resolved arc. Take the marks from there rather than
  re-deriving the geometry in the viewer.

P1: a GUI defect Ev reported as making a verb nearly unusable.
`pane/viewport.rs` is also VNEWS's and VGEOM's ground; the marks layer
(`marks.rs`) is VGEOM's.
