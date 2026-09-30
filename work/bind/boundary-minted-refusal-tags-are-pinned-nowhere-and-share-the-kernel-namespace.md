---
id: boundary-minted-refusal-tags-are-pinned-nowhere-and-share-the-kernel-namespace
kind: issue
title: pncad-py — tags minted at the boundary are outside TAG_INVENTORY's reach, so nothing pins their values or sees them collide with a kernel word
status: open
opened: 2026-09-15
priority: P4
cost: E
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

## What is owed, with the remedy left open

Two properties, stated as what must hold rather than as a mechanism:

1. **Every word this surface can publish is derived from something,
   and pinned.** Today the guard derives its set by reading one file,
   so words outside that file are neither derived nor pinned.
2. **No word is published on one `ErrorClass` by two unrelated
   doors**, or if one is, both doors say so.

**A hand-maintained roster of the words outside `src/tags.rs` is not
the answer**, and is named here so the next unit does not reach for
it: a committed list of a set nothing derives has exactly the defect
this row reports, one file further along. What the closing note on
`python-refusal-tag-values-pinned-nowhere` says about its own guard
applies doubly — that guard at least re-derives its set from source
at test time. Whether the fix is to derive the raise-site words too
(they are reachable from the same kind of source read), to give each
a kernel arm so no word is minted at a boundary at all, or something
else, is the unit's to decide.

That second property is also a third thing from what the closed row
reached: it recorded that an inventory pins the VOCABULARY and not
the MAPPING. This is the vocabulary's EDGES.

## Second evidence: the same fence, on the other side of the wire

Found in the S415 style review (2026-09-15). `TAG_INVENTORY` is blind
to words MINTED outside `src/tags.rs`, above — it is equally blind to
words RESTATED outside it, and the restatements are not rare.

`crates/pncad-py/src/py/doc.rs`'s doc comments name **76 distinct tag
words in backticked prose** that `src/tags.rs` actually returns
(counted by matching backticked `snake_case` tokens in `///` and
`//!` lines against the literals in `tags.rs`) — `unknown_node`,
`placement_rule_mismatch`, `placements_uncertified` and so on, each
telling a Python caller which word a door raises. None is derived and
none is pinned: rename the tag and every one of these doc comments
goes on confidently telling the caller the old word, with the guard
green because `tags.rs` and its inventory moved together.

That is one file. The reviewer's next places to look are
`py/value.rs`, `py/analysis.rs`, `py/mate.rs` and `pncad.pyi` — the
last of which is the file a Python caller actually reads.

## Where to look

- `crates/pncad-py/src/tests.rs` — `TagEntry`, `TAG_INVENTORY`,
  `the_whole_tag_table_matches_its_committed_inventory`.
- `crates/pncad-py/src/py/doc.rs` — `boundary_edit_err` and its three
  callers (the rule for where a `variant` comes from is stated there).
- `crates/pncad-py/src/py/mesh.rs` — `StlRefusal::NotUtf8`.
- `crates/pncad-py/src/py/value.rs` — the `Wireframe` disposition.
- `work/lib/python-refusal-tag-values-pinned-nowhere.md` — the closed
  row this one sits beside; its fence is why this is a separate item.
