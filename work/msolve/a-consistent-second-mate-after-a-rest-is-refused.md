---
id: a-consistent-second-mate-after-a-rest-is-refused
kind: issue
title: a consistent second mate after a rest is refused, so a corner cannot be aligned incrementally
status: open
opened: 2026-10-31
priority: P1
design: true
---


## Finding

Ev (in chat, 2026-10-31), trying to align the corners of two stacked
bodies in the viewer: a plane mate stacked them, and a second mate to
make one side flush was refused.

**The kinds are sufficient for single mates.** A11's five primitives
(`crates/editor-core/ASSEMBLY.md`) already say corner-on-corner
(`Frame`–`Frame` over `corner_frame`, `crates/editor-core/src/mate/corner.rs`)
and edge-collinear-with-edge (`Axis`–`Axis` on edge lines; `PoseDef::Axis`
reads a straight edge, `crates/editor-core/src/pose.rs`). That the GUI
cannot author them is
`work/placing/the-mate-tool-offers-two-of-the-five-mate-kinds.md` (P0),
and it is what blocked Ev.

What remains for the kernel is composition:

- **A rest plus a flush side.** The natural way to align a corner is
  incremental: rest the top face on the bottom face, then make one side
  flush (or one edge collinear). The rest fixes the normal (two turns)
  and the offset; a second plane mate, or an axis mate on an in-plane
  edge, restates turns the rest already fixed. "Constraints are on equal
  footing" (`docs/DESIGN.md`, `[placement.mates]` and the paragraph above
  it) refuses any constraint "any of whose equations the others already
  fix", decided by subgroup algebra "without measuring", so the
  consistent second mate is `MateFault::Overconstrained`
  (`crates/editor-core/src/mate/solve.rs`, `overconstrained`).
  The same paragraph names the intended recourse: "a relation the
  placement can still carry is a mate, of a richer kind where none says
  it yet". An edge laid along an edge within an already-rested plane
  (one turn, one slide) is such a relation, and no kind says it.

## Questions for the design pass

1. Was the five-kind set meant to cover composition, or only single
   mates? If composition, which of the two fixes did the decision
   intend: a richer kind for each residual relation (edge-in-plane,
   point-in-plane, …), or admitting a constraint whose overlapping
   equations agree, which needs a measured check and so reverses
   "without measuring"?
2. Which residual relations does real assembly need? At least: edge
   collinear with edge after a rest; edge flush with a face after a
   rest; a corner on a corner after a rest.
3. Find where the kind set was decided and what it was weighed against:
   `git log -S'is a ball' --all -- crates/editor-core/ASSEMBLY.md` and
   the A11 fork entries in `docs/DESIGN-FORK-LOG.md`.

This is a design fork: it is weighed by the designers
(`docs/prompts/designer.md`) and goes to Ev before any code.
