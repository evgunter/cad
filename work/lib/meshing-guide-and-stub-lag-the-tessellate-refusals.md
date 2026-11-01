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

## Added at TESS-1's fix pass (2026-09-20)

**`tessellate_err`'s `note` attribute now carries three kinds of
content under one name** (`crates/pncad-py/src/py/mesh.rs`): an arm's
own prose about the unbuilt lane (`unsupported_nurbs_face`,
`unsupported_curve`, `missing_entity`); ANOTHER error's rendered
sentence (`unsupported_curved_shape` → props' refusal,
`tolerance_band_unformable` → the band error); and, since TESS-1, a
bare KIND WORD (`meridian_free_curved_face` → `sphere` / `cone` /
`cylinder`). A caller cannot tell from the attribute which it is
holding; the stub's one-line description fits only the first. The
kind word in particular is a discriminant riding in a prose slot — if
the python surface wants it branchable it wants its own attribute
(`surface_kind`, `None` elsewhere), which is a LIB API decision TESS-1
did not take. Note also that the recourse in that arm's message
differs per kind (a seamed restatement for sphere and cone, none for a
cylinder), so a caller branching on the tag alone gets the right
sentence only by reading the message.

**On why these two files were filed rather than fixed.** Not
ownership — LIB also owns `tags.rs`, `py/mesh.rs` and `tests.rs`,
which TESS-1 DID edit. The separator is that those edits were forced
(the compiler's exhaustive matches, and the tag-roster and
binding-census tests, red without them), while the stub's docstring
and the guide's bullet list are forced by nothing and were already
stale before the unit; bringing them current is a documentation pass
over the whole refusal surface, not an arm of this change.

## A second arm, same shape (TESS-5, 2026-09-22)

TESS-5 added `TessellateError::SingleColumnCurvedFace` (tag
`single_column_curved_face`, `note` = the bare kind word). The compiler
and the roster tests forced the same four sites again and TESS-5 took
them; the two hand-kept enumerations this row is about are now short by
two arms, not one. Nothing new to say about the cause — the second
occurrence is the evidence that the lag is structural, and the fix
should be the one this row already proposes rather than a third
hand-edit.
