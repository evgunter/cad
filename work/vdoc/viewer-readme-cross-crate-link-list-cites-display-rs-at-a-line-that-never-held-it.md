---
id: viewer-readme-cross-crate-link-list-cites-display-rs-at-a-line-that-never-held-it
kind: issue
title: the viewer README's cross-crate-link census cites display.rs:262 for a link that has never been near that line
status: open
opened: 2026-09-20
priority: P4
cost: E
---


Filed by the `is-instance-collapses-absent-and-wrong-kind` lane (VNEWS,
PR #2916), whose diff moves the real subject. `crates/viewer/README.md`
is VDOC's, so it is filed rather than repointed.

## The citation

`crates/viewer/README.md` (`:1845` at this lane's merge base), in the
argument that the renderer-free half's cross-crate links are empty of
the case the skip-mode doc gate cannot judge:

> Every cross-crate link in the renderer-free half targets `pncad` —
> twelve sites: `blend.rs:425`, **`display.rs:262`**, `docio.rs:85`, …

**`display.rs:262` is not the link.** On `main` that line is
`/// property of those two functions' error sets, and` — prose inside
`DisplayFault`'s header. The `pncad` link the entry means is
``[`pncad::document::member_of`]`` in `is_instance`'s doc comment, at
`display.rs:345`. The citation was **~83 lines wrong before this lane
touched anything**, so it is disclosed rather than presented as a
repoint (the VIEW register's rule for an out-of-fence citation whose
number never named its subject).

## What the lane's diff does to it

It moves the subject by one line — `is_instance` becomes
`instance_check` and the doc above the link gains and loses lines — so
the link sits at `display.rs:344` on that branch's head. **The census's
CONCLUSION is untouched**: the link still targets `pncad`, `pncad` is
still in `VIEWER_TOOLKIT_SEEDS`, and the population is still twelve.
Only the address is wrong, and it was wrong already.

## The repair

Re-derive by SUBJECT rather than by arithmetic — the whole list is
suspect in the same way, since a list of twelve line numbers in a prose
file has nothing holding it to the tree. Whoever takes it should check
all twelve by reading the line, not shift them.
