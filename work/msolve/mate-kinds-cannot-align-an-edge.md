---
id: mate-kinds-cannot-align-an-edge
kind: issue
title: the decided mate kinds cannot align two edges or two corners
status: open
opened: 2026-10-31
priority: P0
design: true
---


## Finding

Ev (in chat, 2026-10-31), trying to align the corners of two stacked
bodies in the viewer: a plane mate stacked them, and a second mate to
make one side flush was refused. Ev wants a mate that aligns edges, and
notes that the set of mate kinds was decided precisely so that every
mate that could be needed exists. **Why that set was not sufficient is
the question this row asks, before any kind is added.**

The decided set is A11's five primitives
(`crates/editor-core/ASSEMBLY.md`, "A `Mate { on, to }` equates two poses
of one kind"): `Frame`–`Frame` coincides, `Axis`–`Axis` is coaxial,
`Plane`–`Plane` rests, `Point`–`Point` is a ball, `Direction`–`Direction`
is parallel. `PoseDef::Axis` already reads a straight edge's line
(`crates/editor-core/src/pose.rs`, `PoseDef`), and `corner_frame`
(`crates/editor-core/src/mate/corner.rs`) reads a frame at a body's
corner.

What the kernel can already say:

- **Two corners made coincident** is one `Frame`–`Frame` mate between two
  `corner_frame` reads: it pins all six freedoms, so it stacks and aligns
  at once. The GUI cannot author it: `matetool::MATE_KINDS`
  (`crates/viewer/src/matetool.rs`) offers `plane` and `axis` only, and
  picks faces only. That half is a GUI gap, filed as
  `work/chrome/mate-tool-commits-one-mate.md`.
- **Two edges made collinear** is one `Axis`–`Axis` mate on the edges'
  lines.

What it cannot say, and is the kernel half:

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
