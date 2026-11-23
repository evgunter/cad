---
id: python-selection-builder-docs-call-the-profile-index-canonical
kind: issue
title: Python's selection-builder docs call seg/vertex a canonical chain index, which is false for a program loop
status: open
opened: 2026-09-16
priority: P4
cost: E
---

Filed by EDIT's `dm8-follow-through` unit, sweeping the class the row
`dm8-names-canonical-segments-but-the-published-refs-are-program-anchored`
names: prose that calls a PUBLISHED profile ref canonical, when for a
program loop `eval/anchor.rs` rewrote it canonical to program before
the name table was published.

## The finding

`crates/pncad-py/src/py/select.rs`, the five name-text builders'
section comment and each builder's own doc:

- the section comment says `loop_index` "is the profile's canonical
  loop -- 0 the outer loop, then holes in description order -- and
  `seg`/`vertex` index THAT loop's canonical chain";
- `band`'s doc (and its four siblings') repeats it: "`seg` indexes
  that loop's canonical chain".

These builders mint the text a caller hands to a SELECTION, so the
index has to be the one the published name table carries. For a
profile authored as a loop program that is the program's own step
order, not the canonical chain's -- DM8
(`crates/editor-core/REFERENCES.md`) and `eval::anchor`'s module docs.
A caller who reads these docs and counts along the canonical chain
spells a name that resolves to a different segment, or to nothing.

The Rust builders the comment says it matches were the same prose and
were fixed in the same unit (`crates/editor-core/src/names/role.rs`,
the `band` doc and `ProfileEdgeRef`/`ProfileVertexRef`); their wording
is the one to mirror. `crates/pncad-py/*` is LIB's ground, so the
change was not taken there.

## Also in passing

LIB's own `python-has-no-step-to-profile-edge-door` names the door
`canonical_segments_of` in its title and describes the map as composed
from the permutation. Both moved on EDIT's slate: the door is
`ProfileProgram::profile_edges_of`, and DM8 now says the permutation
is CHECKED rather than applied, with a disagreement asserting rather
than refusing `StepSegmentsError::RecordsDisagree` (that variant is
gone). The `gap: B-STEP-SEGMENTS` disposition itself is unaffected --
it is on the TYPE, and the type still exists.
