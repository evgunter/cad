---
id: meshing-guide-and-stub-lag-the-tessellate-refusals
kind: issue
title: docs/guide/meshing.md and pncad.pyi enumerate TessellateError short of the enum, and the guide says it has no Display
status: open
opened: 2026-09-18
---


Found by TESS-1's lane while adding `TessellateError::
MeridianFreeCurvedFace` (tag `meridian_free_curved_face`). The compiler
forced the tag table (`crates/pncad-py/src/tags.rs`,
`tessellate_error_tag`), the payload projection
(`crates/pncad-py/src/py/mesh.rs`, `tessellate_err`), the tag roster
(`crates/pncad-py/src/tests.rs`) and the binding census
(`crates/pncad-py/tests/test_binding_census.py`), and TESS-1 took those
arms. Two hand-kept enumerations nothing forces were left for their
owner:

1. **`crates/pncad-py/pncad.pyi`, `class TessellateError` docstring.**
   Its `variant` list ends at `unsupported_curved_domain`. Missing
   before TESS-1: `unsupported_curved_shape`,
   `tolerance_band_unformable`. Missing with it:
   `meridian_free_curved_face`. Its `note` sentence ("the arm's own
   prose about which lane would be needed") also predates two uses:
   `unsupported_curved_shape` and `tolerance_band_unformable` put the
   source error's sentence there, and `meridian_free_curved_face` puts
   the surface kind's name there (`sphere`, `cone`, `cylinder`,
   `torus`) — the one part of that refusal that is not an arena key.
2. **`docs/guide/meshing.md`, "Reading a tessellation refusal".** The
   bullet list covers six tags and none of the curved-lane arms
   (`unsupported_curved_domain` with its `value`, `unsupported_curved_
   shape`, `meridian_free_curved_face`). Its closing paragraph — "the
   message on this class is a `Debug` rendering … because
   `mesh::TessellateError` implements no `Display`" — is false:
   `crates/mesh/src/types.rs` carries `impl Display for
   TessellateError`, and `tessellate_err`'s own doc says the message is
   that prose.

The refusal a guide reader is most likely to meet from an imported part
is the new one: a STEP file stating a dome as one rim circle and no
seam imports as a valid solid and refuses at `tessellate`. What a
caller can do about it (restate the face seamed) is in the variant's
doc and its `Display`.
