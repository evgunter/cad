---
id: a-lofts-names-follow-only-its-first-sections-reshaping
kind: issue
title: A loft's names follow only its first section's reshaping: SetProgram on a later section moves none of them
status: open
opened: 2026-09-24
priority: P3
cost: D
---


`Node::anchoring_profile` answers `profiles.first()` for a `Node::Loft`
(`crates/editor-core/src/node.rs:3568`), so the SetProgram door
(`crates/editor-core/src/edit.rs`, the `anchored` filter over
`anchoring_profile`) remaps a loft's names only when its FIRST section is
reshaped.

Since PR #3147 a loft wall's one name is canonical segment `k` of every
section at once (DM8), so section 0 has no special standing in the
naming. What still singles it out is this door. The current behaviour is
pinned by
`edit_set_program::a_lofts_names_follow_its_first_sections_reshaping`:
- A reshape of a later section alone leaves the sections with different
  segment counts, so the loft refuses and no name moves.
- A reshape of section 0 alone rebinds.

Neither is wrong today, because a loft needs every section on one segment
budget. But a reshaping that keeps the budget does exist: it moves a
segment's step without changing the count, on a later section. It
rebinds nothing, and the loft skins again under names now paired with
different segments of that section.

What a fix has to decide:
- Should a loft follow every section? If so, what does the door do when
  two sections' maps disagree (strand, or refuse)?
- Or should the door refuse a SetProgram on a non-first section whose map
  is not the identity?

Not trivial: it is a DM7 question about which edit moves a multi-profile
node's names, so it is left open rather than fixed in #3147.
