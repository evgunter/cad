---
id: boundary-minted-refusal-tags-are-pinned-nowhere-and-share-the-kernel-namespace
kind: issue
title: pncad-py — tags minted at the boundary are outside TAG_INVENTORY's reach, so nothing pins their values or sees them collide with a kernel word
status: open
opened: 2026-09-15
---


## Finding

Found while closing S415's third residue (PORT), which asked whether
anything asserts the Python refusal-tag namespace is injective.
Nothing does, and the reason is structural rather than an oversight in
any one row.

`crates/pncad-py/src/tests.rs`'s
`the_whole_tag_table_matches_its_committed_inventory` re-derives
`TAG_INVENTORY` **by reading `src/tags.rs`**. That is its whole
mechanism and also its fence: a tag that is never written in
`src/tags.rs` is invisible to it. Three refusal words are minted as
string literals at their raise sites and therefore appear in no
inventory, in no `*_tags_are_stable` row, in `pncad.pyi` and in no
test under `crates/pncad-py/tests/`:

- `"name_serialize"` — `src/py/doc.rs`, `name_text`, through
  `boundary_edit_err`. A `serde_json` failure; no kernel arm exists.
- `"not_utf8"` — `src/py/mesh.rs`, `stl_err`'s `StlRefusal::NotUtf8`
  arm. Documented at the arm as *"Not a kernel arm"*.
- `"wireframe"` — `src/py/value.rs`, the `StepImport::Wireframe`
  disposition raised on `ErrorClass::StepImport`. Documented at the
  site as sharing `step_import_error_tag`'s namespace.

Each is individually disclosed at its site, which S415 completed for
`name_serialize`. What is missing is the two properties a reader of
the Python surface would expect the tag census to carry:

1. **Their VALUES are pinned nowhere.** A rename of `"not_utf8"`
   compiles clean, passes the tag-inventory guard (which cannot see
   it) and breaks any Python caller branching on it. This is the
   exact defect `python-refusal-tag-values-pinned-nowhere` (closed
   2026-09-03) fixed for `src/tags.rs`, unfixed for everything
   outside that file.

2. **Nothing checks the namespace is injective, and a collision
   already existed.** `"no_minted_id"` was minted as a literal in
   `Doc::insert` while `declare_error_tag(&DeclareError::NoMintedId)`
   yielded the identical word for `Doc.declare` — two doors, one tag,
   and neither site knew the other existed. S415 ruled that one an
   intended synonym and derived it from the tag function, so the
   collision is gone by construction; **nothing would have reported
   it**, and nothing would report the next one. The three words above
   share `ErrorClass::Edit`, `ErrorClass::StlExport` and
   `ErrorClass::StepImport` with the maps that DO have inventories, so
   an accidental collision is a live shape, not a theoretical one.

## What a fix would look like

The existing guard reads one file and compares against a committed
table. The natural extension is a second committed roster — the words
minted outside `src/tags.rs`, with their raise site and the reason no
enum stands behind each — plus a cross-check that the union of the
inventory's values and that roster has no word published on one
`ErrorClass` by two unrelated doors. That second half is the part
`python-refusal-tag-values-pinned-nowhere`'s closing note did not
reach: it recorded that an inventory pins the VOCABULARY and not the
MAPPING, and this is a third thing again — the vocabulary's edges.

## Where to look

- `crates/pncad-py/src/tests.rs` — `TagEntry`, `TAG_INVENTORY`,
  `the_whole_tag_table_matches_its_committed_inventory`.
- `crates/pncad-py/src/py/doc.rs` — `boundary_edit_err` and its three
  callers (the rule for where a `variant` comes from is stated there).
- `crates/pncad-py/src/py/mesh.rs` — `StlRefusal::NotUtf8`.
- `crates/pncad-py/src/py/value.rs` — the `Wireframe` disposition.
- `work/lib/python-refusal-tag-values-pinned-nowhere.md` — the closed
  row this one sits beside; its fence is why this is a separate item.
