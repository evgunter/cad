---
id: a-work-edit-can-red-pncad-py-unseen
kind: issue
title: deleting or renaming a work/ row can red pncad-py's tracker_row_exists, and a work/ edit runs no build
status: open
opened: 2026-10-16
---


`crates/pncad-py/src/tests.rs` `tracker_row_exists` (about line 9884) reads
`work/` at run time. It holds each suppressed stub attribute to a row filed
under `work/<slate>/<id>.md`. `scripts/ci-filter.py` lists `work/` in
`AUX_PREFIXES` (and a `.md` there is already docs), so a diff that deletes or
renames a cited row runs no pncad-py build. The test then reds on main or in
the nightly instead of in the PR that moved the row.

The read reach that `_markdown_readers` uses does not see this read. The path
is built as `repo_root(...).join("work")` plus a `format!`ed file name, and
`_read_reach` records it only as pncad-py reading the tree (`Reach.tree`),
not as a path the diff can name. So the reader map cannot seed pncad-py from a
`work/` edit without seeding it from every path.

Wanted: a `work/` diff that deletes or renames a `.md` seeds pncad-py. Either
the read reach learns `join("work")` as a directory read, or ci-filter checks
`git diff --diff-filter=DR` under `work/` against the ids that
`crates/pncad-py/src/tests.rs` cites.

— CIW lane, 2026-10-16, from the review of #5228
