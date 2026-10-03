---
id: ring-and-scaffold-refusals-read-three-ways
kind: issue
title: restfront/tess/topo: a ring on a curved face and a null-scaffold edge read three ways across validate, props and mesh
status: open
opened: 2026-09-29
priority: P2
cost: E
---


(CHROME `refusal-residue` fix pass, PR 3457; the review's adjudication.)

## What

Two states have one cause and three readings on screen.

**A ring on a curved face.** No construction mints one: STEP import
refuses it before a body exists (`step-import/src/entities.rs`, the
multi-bound curved-face gate), blend keeps curved faces ring-free
(`sweep/src/blend/surgery.rs`, "a curved face must be RING-FREE"), and
the Boolean slit-zip refuses a slit face carrying rings
(`topo/src/boolean/rest.rs`, "slit-zip face carries rings"). So
reaching it is a construction or file defect.

- `topo::props::MassPropsError::RingOnCurvedFace` (PR 3457) reads it
  as that: "the kernel cannot measure the volume of a curved face with
  a hole", ending in `geom_core::KERNEL_OR_FILE_DEFECT_ENDING`.
- `topo::validate`'s `classify_mass_props` reads it as not yet built:
  "the kernel cannot yet measure a curved face with a hole", ending
  `NOT_YET`.
- `mesh::TessellateError::RingOnCurvedFace` calls it a kernel bug with
  no ending ("this is a kernel bug rather than a mesh to guess at").

**A null-scaffold edge.** Bodies at rest never carry one (tier 2).

- `MassPropsError::NullScaffoldEdge` and `TransformError::NullScaffold`
  (PR 3457) and `classify_mass_props`'s `DEFECT` end in
  `KERNEL_OR_FILE_DEFECT_ENDING`.
- `mesh::TessellateError::NullScaffoldEdge` and `EmptyLoop` tell the
  user to "finish the surgery first", which nobody holding a body at
  rest can do.

## Repair shape

Pick the defect reading for every `RingOnCurvedFace` and
`NullScaffold*` arm, and give the sentence one home: a constant beside
`props.rs`'s arm that `validate.rs` and `mesh/src/types.rs` render,
instead of the two near-copies in `props.rs` and `validate.rs`. Then
retire `validate.rs`'s local `NOT_YET` for `geom_core::NOT_YET_ENDING`
(added in PR 3457). The same literal "There is no way through yet"
stands alone in `topo/src/splitting/mod.rs` (two arms, REACH's) and
`topo/src/shell.rs` (one arm, SHELL's); `topo/src/census.rs` extends it
("… yet for this shape") at eight arms.

## Evidence (TOPO, PR 3513's second fix pass, 2026-09-30)

The literal copies of `geom_core::NOT_YET_ENDING` outside tests, as of
PR 3513's merge base: `topo/src/splitting/mod.rs` (two arms),
`topo/src/shell.rs` (one), `topo/src/census.rs` (eight, each extended
"… yet for …"), and `topo/src/validate.rs`'s local `NOT_YET` (one),
twelve in all; PR 3513's `RestZipFrontier::ending`
(`topo/src/boolean/refusal_routes.rs`) reads the constant. Swept with
`"There is no way through yet"` over `crates/`, `demos/` and `tools/`;
a copy spelled otherwise ("no way through", without "yet") is not
matched.
