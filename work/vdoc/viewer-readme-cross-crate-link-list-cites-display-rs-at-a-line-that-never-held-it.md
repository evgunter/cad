---
id: viewer-readme-cross-crate-link-list-cites-display-rs-at-a-line-that-never-held-it
kind: issue
title: the viewer README's cross-crate-link census cites display.rs:262 for a link that has never been near that line
status: closed
closed: 2026-09-21
branch: vdoc/readme-attributions
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

## Closed — the twelve line numbers are replaced by the command that produces the population, and the population moved (#vdoc/readme-attributions)

**Old citation.** *"Every cross-crate link in the renderer-free half
targets `pncad` — twelve sites: `blend.rs:425`, `display.rs:262`,
`docio.rs:85`, `marks.rs:324`, `matetool.rs:33`, `:54`, `:153`,
`:220`, `parts.rs:11`, `props.rs:652`, `sketch.rs:939`,
`tree.rs:143`"*.

**The subject, re-derived.** The subject of the `display.rs` entry is
the link `` [`pncad::document::member_of`] `` in the doc comment of
what the filing lane knew as `is_instance`. It is at
`crates/viewer/src/display.rs:355` on this tree. The row was right that
`:262` never held it.

**New citation — a command, not a list.** The bullet now prints the
rule that produces the population:

    rg -n -g '!{app,drafts,forms,gpu,widgets,pane}.rs' -g '!pane/**' -g '!bin/**' -e '(\[`|\]\()(pncad|bvh|editor_core|toml)::' crates/viewer/src

extracted back out of the README with `sed -n 2006p` (the indented block in the **Nowhere** bullet) and run: **15**
lines, exit 0. The blind spot is stated at the sentence — a link
resolved through a `use` carries the in-scope name, not the crate, and
`sketch.rs`'s `` [`ProfileVertex`] `` is exactly that member.

**The population is not twelve and it is not all `pncad`.** Fourteen
of the fifteen target `pncad`; the fifteenth is
`crates/viewer/src/session/refuse.rs:151`,
`` [`editor_core::edit::UNDECLARED_PARAM_RECOURSE`] ``, and
`editor-core` is not in `VIEWER_TOOLKIT_SEEDS`. So the bullet's *"that
case is empty today"* was false and the hole it calls theoretical is
open. The page now says so; the CI half is filed as
`work/mirror/renderer-free-link-to-editor-core-opens-the-ungated-hole.md`,
beside the parent row that owns the repair.

**Three further citations in the same bullet, re-derived because a
citation fix is class-wide over the file.**

- `scripts/ci-filter.py:1428` → `:1436` (`VIEWER_TOOLKIT_SEEDS`).
- `ci.yml:1833-1836` → the `rustdoc (gate)` step's `if`/`else` at
  `ci.yml:1890-1894`, with the keying itself now cited where it
  happens, `scripts/ci-filter.py:2495`.
- `work/view/renderer-free-cross-crate-links-are-ungated-off-the-seed-set.md`
  → the row moved to `work/mirror/` with CIW's 2026-09-20 cut; the
  page now names the row rather than a path.
- `nightly.yml:291-293` was checked and is correct — the
  `rustdoc (viewer, all features)` row is at those three lines.

Re-derived on the merged tree at `fb60ba2b7f`.
